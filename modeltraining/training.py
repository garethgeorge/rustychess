import argparse 

parser = argparse.ArgumentParser(description='Training script for the model')
parser.add_argument('--output', type=str, default='model.safetensor', help='Path to save the model')
parser.add_arguent('--epochs', type=int, default=1, help='Number of epochs to train the model')
parser.add_arguent('--dataset', type=int, default=1, help='sqlite database hosting the training dataset')
args = parser.parse_args()

