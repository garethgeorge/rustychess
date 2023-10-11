use super::Evaluator;

use anyhow::Context;
use candle_core::{self, Device, Module, Tensor};
use candle_nn::Linear;
use chess::{Rank, Square};

struct Model {
    layers: Vec<Linear>,
}

impl Model {
    fn forward(&self, input: &Tensor) -> candle_core::Result<Tensor> {
        let mut x = input.clone();
        for layer in &self.layers[0..self.layers.len() - 1] {
            x = layer.forward(&x)?;
            x = x.relu()?;
        }
        x = self.layers[self.layers.len() - 1].forward(&x)?;
        return Ok(x);
    }
}

pub struct NnetEval {
    model: Model,
}

impl NnetEval {
    pub fn new(safetensors: &[u8], prefix: &str) -> anyhow::Result<NnetEval> {
        let device = Device::Cpu;
        let tensors = candle_core::safetensors::load_buffer(safetensors, &device)
            .context("Failed to load safetensors")?;

        let mut keys = tensors.keys().collect::<Vec<_>>();
        keys.sort();

        let mut model = Model { layers: Vec::new() };

        for key in keys {
            println!("Loading model, examining tensor key: {}", key);
            if !key.starts_with(prefix) || !key.ends_with(".weight") {
                continue;
            }

            let Some(weight) = tensors.get(key) else {
                return Err(anyhow::anyhow!("Failed to find weight for key {}", key));
            };
            let Some(bias) = tensors.get(&key.replace(".weight", ".bias")) else {
                return Err(anyhow::anyhow!("Failed to find bias for key {}", key));
            };
            model
                .layers
                .push(Linear::new(weight.clone(), Some(bias.clone())));
            println!("adding layer {} with shape {:?}", key, weight.shape());
        }

        return Ok(NnetEval { model: model });
    }

    fn board_to_input(board: &chess::Board) -> candle_core::Result<Tensor> {
        let mut input: Vec<f32> = Vec::new();
        input.push(b2f(board.side_to_move() == chess::Color::White));
        input.push(b2f(board.side_to_move() == chess::Color::Black));
        input.push(b2f(board.castle_rights(chess::Color::White).has_kingside()));
        input.push(b2f(board
            .castle_rights(chess::Color::White)
            .has_queenside()));
        input.push(b2f(board.castle_rights(chess::Color::Black).has_kingside()));
        input.push(b2f(board
            .castle_rights(chess::Color::Black)
            .has_queenside()));
        let offset = input.len();
        input.resize(64 * 6 * 2 + input.len(), 0.0);

        for file in 0..8 {
            for rank in 0..8 {
                let sq = Square::make_square(Rank::from_index(rank), chess::File::from_index(file));
                let Some(piece) = board.piece_on(sq) else {
                    continue;
                };
                let color = board.color_on(sq).unwrap();
                let color_offset: usize = match color {
                    chess::Color::White => 6,
                    chess::Color::Black => 0,
                };
                let piece_offset = match piece {
                    chess::Piece::Pawn => 0,
                    chess::Piece::Knight => 1,
                    chess::Piece::Bishop => 2,
                    chess::Piece::Rook => 3,
                    chess::Piece::Queen => 4,
                    chess::Piece::King => 5,
                };
                input[offset + sq.to_index() * 12 + color_offset + piece_offset] = 1.0;
            }
        }

        return Tensor::new(input, &Device::Cpu)?.reshape((1, 774));
    }
}

impl Evaluator for NnetEval {
    fn evaluate(&self, board: &chess::Board) -> anyhow::Result<f32> {
        let tensor =
            NnetEval::board_to_input(board).context("failed to create tensor for board")?;
        let output = self
            .model
            .forward(&tensor)
            .context("failed to evaluate model for board")?;
        let scalar_score: f32 = output.sum_all()?.to_scalar()?;

        match board.side_to_move() {
            chess::Color::White => return Ok(scalar_score),
            chess::Color::Black => return Ok(-scalar_score),
        };
    }
}

fn b2f(b: bool) -> f32 {
    return if b { 1.0 } else { 0.0 };
}
