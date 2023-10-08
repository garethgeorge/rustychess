import chess.pgn
import io
from pyzstd import ZstdFile
import numpy as np
import struct

def read_games(path):
  with open(path, 'rb') as file, ZstdFile(file, 'rb') as zstd_file:
    with io.TextIOWrapper(zstd_file, encoding='utf-8') as text_file:
      while True:
        game = chess.pgn.read_game(text_file)
        if game is None:
          break
        yield game

def get_scored_boards(games):
  """
  This function takes an iterable of games and returns an iterable of boards that are scored.
  The boards are scored by the eval() function of the nodes. Note that a reference to a board may not be retained as the references are mutated.
  """
  # This function takes an iterable of games and returns an iterable of boards that are scored.
  # The boards are scored by the eval() function of the nodes.

  for game in games:
    board = game.board()
    for node in game.mainline():
      board.push(node.move) # node.move is the move leading to the node, it must be applied to get the state at the node.
      score = node.eval()
      if score is not None and score.white().score() is not None:
        yield score, board
      else:
        break

piece_types = [chess.PAWN, chess.KNIGHT, chess.BISHOP, chess.ROOK, chess.QUEEN, chess.KING]
piece_type_and_color = [(piece_type, color) for color in [chess.WHITE, chess.BLACK] for piece_type in piece_types]

tensor_len = 64 * 6 * 2 + 6
tensor_packed_len = 97

def to_tensor(board):
  flags = np.array([
    1 if board.turn else 0,
    1 if not board.turn else 0,
    1 if board.has_kingside_castling_rights(chess.WHITE) else 0,
    1 if board.has_queenside_castling_rights(chess.WHITE) else 0,
    1 if board.has_kingside_castling_rights(chess.BLACK) else 0,
    1 if board.has_queenside_castling_rights(chess.BLACK) else 0,
  ], dtype=np.uint8)

  # 64 squares, 6 piece types, 2 colors
  positions = np.zeros(64 * 6 * 2, dtype=np.uint8)
  for sq in range(0, 64):
    piece = board.piece_at(sq)
    if piece is not None:
      piece_type = piece.piece_type
      color = 1 if piece.color else 0
      positions[sq * 6 * 2 + color * 6 + (piece_type - 1)] = 1

  arr = np.concatenate([flags, positions])
  assert(len(arr) == tensor_len)

def to_tensor_array(board):
  arr = to_tensor(board)
  arr = np.packbits(arr, axis=0)
  assert(len(arr) == tensor_packed_len)
  return arr
