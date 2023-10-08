# Rusty Chess 

Rusty Chess is a rust implementation of a chess bot that is written with WASM compilation in mind.

The chessbot uses minimax with alpha beta pruning to decide the best move. The scoring function is a deep neural network trained on the lichess database of ~30 million games. The model is evaluated in rust using the fantastic [candle](https://github.com/huggingface/candle) library.
