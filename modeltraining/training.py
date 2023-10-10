from random import randrange
from torch import optim, nn
from torch.nn import functional as F
import argparse 
import lib.game
import numpy as np
import lmdb
import struct

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

class EvaluationDataset:
  def __init__(self, db):
    self.db = db
    with db.begin() as txn:
      self.count = struct.unpack('i', txn.get('count'.encode('ascii')))[0]

    self.scaler = 1
    self.scaler = max(abs(self[idx]['target'][0]) for idx in range(0, self.count))

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
    tensor = lib.game.binary_to_board_tensor(bitvec)
    score = score / 100
    return {
      'input': tensor,
      'target': np.array([score], dtype=np.float32),
    }
  
  dataset = EvaluationDataset(db)


class LunaNN(nn.Module):
    """Reinforcement Learning Neural Network"""

    # Optimizer
    optimizer: optim.Optimizer

    # Learning Rate
    learning_rate: float

    # Number of channels
    num_channels: int

    def __init__(self, num_channels: int) -> None:
        super(LunaNN, self).__init__()

        self.board_x, self.board_y, self.board_z = (15, 8, 8)
        self.action_size = game.getActionSize()
        self.game = game
        self.num_channels = 

        # Define neural net
        self.define_architecture()
        self.learning_rate = 1e-3
        self.optimizer = pytorch.optim.Adam(self.parameters(), lr=self.learning_rate)

    def define_architecture(self) -> None:
        """Define Net
            - Input: serialized chess.Board
            - Output:
                - predicted board value (tanh)
                - policy distribution over possible moves (softmax)
        """
        # Args shortcut
        args = self.args

        # Input
        self.conv1 = nn.Conv3d(1, args.num_channels, 3, stride=1, padding=1)
        
        ## Hidden
        self.conv2 = nn.Conv3d(args.num_channels, args.num_channels * 2, 3, stride=1, padding=1)
        self.conv3 = nn.Conv3d(args.num_channels * 2, args.num_channels * 2, 3, stride=1)
        self.conv4 = nn.Conv3d(args.num_channels * 2, args.num_channels * 2, 3, stride=1)
        self.conv5 = nn.Conv3d(args.num_channels * 2, args.num_channels, 1, stride=1)

        self.bn1 = nn.BatchNorm3d(args.num_channels)
        self.bn2 = nn.BatchNorm3d(args.num_channels * 2)
        self.bn3 = nn.BatchNorm3d(args.num_channels * 2)
        self.bn4 = nn.BatchNorm3d(args.num_channels * 2)
        self.bn5 = nn.BatchNorm3d(args.num_channels)

        self.fc1 = nn.Linear(args.num_channels*(self.board_x-4)*(self.board_y-4)*(self.board_z-4), 1024) #4096 -> 1024
        self.fc_bn1 = nn.BatchNorm1d(1024)

        self.fc2 = nn.Linear(1024, 512)
        self.fc_bn2 = nn.BatchNorm1d(512)

        self.fc3 = nn.Linear(512, 512)
        self.fc_bn3 = nn.BatchNorm1d(512)

        # output p dist        
        self.fc4 = nn.Linear(512, self.action_size)

        # output scalar
        self.fc5 = nn.Linear(512, 1)

    def forward(self, boardsAndValids):
        """Forward prop"""
        x, valids = boardsAndValids

        x = x.view(-1, 1, self.board_x, self.board_y, self.board_z)
        x = F.relu(self.bn1(self.conv1(x)))
        x = F.relu(self.bn2(self.conv2(x)))
        x = F.relu(self.bn3(self.conv3(x)))
        x = F.relu(self.bn4(self.conv4(x)))
        x = F.relu(self.bn5(self.conv5(x)))
        x = x.view(-1, self.args.num_channels*(self.board_x-4)*(self.board_y-4)*(self.board_z-4))
        x = F.dropout(F.relu(self.fc_bn1(self.fc1(x))), p=self.args.dropout, training=self.training)
        x = F.dropout(F.relu(self.fc_bn2(self.fc2(x))), p=self.args.dropout, training=self.training)
        x = F.dropout(F.relu(self.fc_bn3(self.fc3(x))), p=self.args.dropout, training=self.training)

        pi = self.fc4(x)
        v = self.fc5(x)

        pi -= (1 - valids) * 1000
        return F.log_softmax(pi, dim=1), torch.tanh(v)

checkpoint_filepath = './checkpoints'
model_checkpointing_callback = ModelCheckpoint(
    filepath = checkpoint_filepath,
    save_best_only= True,
)

print("INPUT: ", dataset[123]['input'])
print("TARGET: ", dataset[123]['target'])

train_tfds = tf.data.Dataset.from_tensor_slices(([dataset[i]['input'] for i in range(0, len(dataset) * 9 // 10)], [dataset[i]['target'] for i in range(0, len(dataset) * 9 // 10)]))
val_tfds = tf.data.Dataset.from_tensor_slices(([dataset[i]['input'] for i in range(len(dataset) * 9 // 10, len(dataset))], [dataset[i]['target'] for i in range(len(dataset) * 9 // 10, len(dataset))]))

print("Num GPUs Available: ", len(tf.config.list_physical_devices('GPU')))

model.fit(train_tfds,
          batch_size=4096,
          epochs=2,
          verbose=1,
          validation_data=val_tfds,
          callbacks=[callbacks.ReduceLROnPlateau(monitor='loss', patience=10),
                     callbacks.EarlyStopping(monitor='loss', patience=15, min_delta=1e-4),model_checkpointing_callback])

model.save('model.keras')

print("expected: " + str(dataset[123]['target']))
print(model.call(tf.constant(dataset[123]['input'].transpose())))
