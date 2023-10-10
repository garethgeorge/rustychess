import argparse 
import lib.game
import struct 
from tqdm import tqdm
import lmdb

parser = argparse.ArgumentParser(description='Training script for the model')
parser.add_argument('gamedata_path', type=str, help='Path to gamedata downloaded from https://database.lichess.org/.')
parser.add_argument('--lmdb_path', type=str, default="./dataset.lmdb", help='Path to the lmdb store that hosts the dataset.')
parser.add_argument('--lmdb_size', type=int, default=64*1024*1024*1024, help='Size of the LMDB file.')
args = parser.parse_args()

db = lmdb.open(args.lmdb_path,
      map_size=args.lmdb_size,
      metasync=False,
      sync=False,
      map_async=True,)
try:
  for idx, (score, board) in tqdm(enumerate(lib.game.get_scored_boards(lib.game.read_games(args.gamedata_path)))):
    with db.begin(write=True) as txn:
      txn.put(
        str(idx).encode('ascii'), 
        struct.pack(
          f"ii{lib.game.tensor_packed_len}s", 
          idx, 
          score.white().score(), 
          lib.game.board_tensor_to_binary(lib.game.board_to_tensor(board)).tobytes()))
      txn.put('count'.encode('ascii'), struct.pack("i", idx+1))
finally:
  db.close()