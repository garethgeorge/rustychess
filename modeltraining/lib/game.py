import chess.pgn
import io
from pyzstd import ZstdFile
import numpy as np
import pickle
import math

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

squares_index = {
  'a': 0,
  'b': 1,
  'c': 2,
  'd': 3,
  'e': 4,
  'f': 5,
  'g': 6,
  'h': 7
}


# example: h3 -> 17
def square_to_index(square):
  letter = chess.square_name(square)
  return 8 - int(letter[1]), squares_index[letter[0]]

def board_to_tensor(board):
  # this is the 3d matrix (piece type, rank, file) 
  board3d = np.zeros((15, 8, 8), dtype=np.int8)

  # here we add the pieces's view on the matrix
  for piece in chess.PIECE_TYPES:
    for square in board.pieces(piece, chess.WHITE):
      idx = np.unravel_index(square, (8, 8))
      board3d[piece - 1][7 - idx[0]][idx[1]] = 1
    for square in board.pieces(piece, chess.BLACK):
      idx = np.unravel_index(square, (8, 8))
      board3d[piece + 5][7 - idx[0]][idx[1]] = 1

  # add attacks and valid moves too
  # so the network knows what is being attacked
  aux = board.turn
  board.turn = chess.WHITE
  for move in board.legal_moves:
      i, j = square_to_index(move.to_square)
      board3d[12][i][j] = 1
  board.turn = chess.BLACK
  for move in board.legal_moves:
      i, j = square_to_index(move.to_square)
      board3d[13][i][j] = 1
  board.turn = aux

  # add the current player's turn
  board3d[14] = np.full((8, 8), 1 if board.turn == chess.WHITE else 0, dtype=np.int8)

  return board3d

def encode_tensor(tensor):
  shape = tensor.shape # the shape of the tensor
  tensor = tensor.flatten()
  arr = np.packbits(tensor, axis=0)
  return pickle.dumps((shape, arr))

def decode_tensor(binary):
  shape, arr = pickle.loads(binary)
  expbits = np.prod(shape)
  arr = np.unpackbits(arr, axis=0).astype(np.single)[0:expbits]
  return arr.reshape(shape)

def board_tensor_to_binary(tensor):
  return encode_tensor(tensor)

def binary_to_board_tensor(binary):
  return decode_tensor(binary)

base_board_tensor = board_to_tensor(chess.Board())
assert(np.array_equal(binary_to_board_tensor(board_tensor_to_binary(base_board_tensor)), base_board_tensor))
