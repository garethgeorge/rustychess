# MLMODEL

This directory hosts the evaluation function model and training code. 

Datasets are downloaded from https://database.lichess.org/.

Training implementation is based on the following articles:

 * https://towardsdatascience.com/train-your-own-chess-ai-66b9ca8d71e4
 * https://colab.research.google.com/drive/1smI2B7kiwzkr43TqnCYOpxocZlI0kPUh?usp=sharing#scrollTo=ytOPoXcbDbMM

## Dependencies
 
 * [optional] pyenv for python 3.11.0 (e.g. pyenv install 3.11.0)
 * virtualenv e.g. `python3 -m pip install virtualenv`

## Dev Setup

```
python3 -m venv venv
source env/bin/activate # only use this on subsequent invocations
```
