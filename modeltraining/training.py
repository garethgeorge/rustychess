from collections import OrderedDict
from random import randrange
from safetensors.torch import save_model, save_file
from torch import nn
from torch.utils.data import Dataset, DataLoader, IterableDataset, random_split
import argparse 
import base64
import lib.game
import numpy as np
import os
import pytorch_lightning as pl
import time
import torch
import torch.nn.functional as F
import lmdb
import struct
import math

parser = argparse.ArgumentParser(description='Training script for the model')
parser.add_argument('--lmdb_path', type=str, default="./dataset.lmdb", help='Path to the lmdb store that hosts the dataset.')
parser.add_argument('--output', type=str, default='model.safetensor', help='Path to save the model')
parser.add_argument('--epochs', type=int, default=1, help='Number of epochs to train the model')
args = parser.parse_args()

db = lmdb.open(args.lmdb_path,
      metasync=False,
      sync=False,
      readonly=True,
      lock=False,
      map_async=True,)

class EvaluationDataset(IterableDataset):
  def __init__(self, db):
    self.db = db
    with db.begin() as txn:
      self.count = struct.unpack('i', txn.get('count'.encode('ascii')))[0]

  def __iter__(self):
    return self
  
  def __next__(self):
    idx = randrange(0, self.count)
    return self[idx]
  
  def __len__(self):
    return self.count
  
  def __getitem__(self, idx):
    with self.db.begin() as txn:
      bin = txn.get(str(idx).encode('ascii'))
      (_, score, bitvec) = struct.unpack(f"ii{lib.game.tensor_packed_len}s", bin)
    assert(len(bitvec) == lib.game.tensor_packed_len)
    bin = np.frombuffer(bitvec, dtype=np.uint8)
    bin = np.unpackbits(bin, axis=0).astype(np.single)[0:lib.game.tensor_len]
    score =  math.copysign(math.log2(abs(score / 10.0) + 1), score) # the more extreme the win the less we weight it, we just want to bias towards winning.
    return {
      'binary': bin,
      'eval': np.array([score]).astype(np.single) 
    }
  
dataset = EvaluationDataset(db)

class EvaluationModel(pl.LightningModule):
  def __init__(self,learning_rate=1e-3,batch_size=1024,layer_shapes=[lib.game.tensor_len, lib.game.tensor_len, lib.game.tensor_len, lib.game.tensor_len]):
    super().__init__()
    self.batch_size = batch_size
    self.learning_rate = learning_rate
    layers = []
    prev_shape = lib.game.tensor_len
    for i in range(len(layer_shapes)):
      layers.append((f"linear-{i}", nn.Linear(prev_shape, layer_shapes[i])))
      layers.append((f"relu-{i}", nn.ReLU()))
      prev_shape = layer_shapes[i]
    layers.append((f"linear-{len(layer_shapes)}", nn.Linear(prev_shape, 1)))
    self.seq = nn.Sequential(OrderedDict(layers))
    print(self.seq)

  def forward(self, x):
    return self.seq(x)

  def training_step(self, batch, batch_idx):
    x, y = batch['binary'], batch['eval']
    y_hat = self(x)
    loss = F.l1_loss(y_hat, y)
    self.log("train_loss", loss)
    return loss

  def configure_optimizers(self):
    return torch.optim.Adam(self.parameters(), lr=self.learning_rate)

  def train_dataloader(self):
    return DataLoader(dataset, batch_size=self.batch_size, num_workers=24, pin_memory=True)

print(f"TENSOR LENGTH: {lib.game.tensor_len}")

batch_size = 4096 * 2
layer_shapes = [lib.game.tensor_len // 1, 32, 32]
version_name = f'{time.time()}-batch_size-{batch_size}-layer_count-{len(layer_shapes)}'
logger = pl.loggers.TensorBoardLogger("logs", name="rustychess", version=version_name)
trainer = pl.Trainer(devices=1, accelerator="gpu", precision='bf16-mixed', max_epochs=args.epochs, logger=logger)
model = EvaluationModel(layer_shapes=layer_shapes, batch_size=4096, learning_rate=1e-3)
trainer.fit(model)

# save the version as a .safetensors model
save_model(model, f"{version_name}.safetensors")

# validation data
print(f"Zero-vector output for validation: {model.forward(torch.zeros(1,lib.game.tensor_len))}")
print(f"Ones-vector output for validation: {model.forward(torch.ones(1,lib.game.tensor_len))}")

# check the model against known samples
for i in range(0, 10):
  idx = randrange(0, len(dataset))
  input_tensor = torch.from_numpy(dataset[idx]['binary'])
  print(f"Actual output: {model.forward(input_tensor)}")
  print(f"Expected output: {dataset[idx]['eval']}")

# create validation data, this enables us to validate that we load the model 
# correctly from rust.
validation_data = {}
for i in range(0, 10):
  input_tensor = torch.rand(1,lib.game.tensor_len)
  validation_data[f"input-{i}"] = input_tensor
  validation_data[f"output-{i}"] = model.forward(input_tensor).sum()
save_file(validation_data, f"{version_name}.validation.safetensors")
