# Rusty Chess 

[![Build and Deploy](https://github.com/garethgeorge/rustychess/actions/workflows/deploy.yaml/badge.svg)](https://github.com/garethgeorge/rustychess/actions/workflows/deploy.yaml)

## What is it?

Rusty Chess aims to be a high quality embeddable chess engine that runs entirely locally in the browser (no backend required). This is accomplished by writing the move search and board state evaluation in rust and compiling this to WASM that runs natively in the browser. The engine is lightweight enough that it performs well on laptops and mobile devices. High quality board state scoring is accomplished by training a neural network on ~30 million games downloaded from [lichess](https://database.lichess.org/) and evaluating it in rust using the fantastic [candle](https://github.com/huggingface/candle) library.

## Demo

See the [demo](https://garethgeorge.github.io/rustychess/) on GitHub pages. 
