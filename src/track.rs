use crate::direction::*;
use rand::prelude::*;

pub struct Track {
    pub moves: Vec<Direction>,
    pub length: usize,
    pub h_moves: usize,
    pub v_moves: usize,
}

impl Track {
    pub fn new(rng: &mut StdRng, h_prob_moves: usize, v_prob_moves: usize) -> Track {
        let (moves, h_moves, v_moves) = Track::build_moves(rng, h_prob_moves, v_prob_moves);
        let length = h_moves + v_moves;
        Track {
            moves,
            h_moves,
            v_moves,
            length,
        }
    }
    pub fn build_moves(
        rng: &mut StdRng,
        h_prob_moves: usize,
        v_prob_moves: usize,
    ) -> (Vec<Direction>, usize, usize) {
        let inv_prob = 0.0 / 16.0;
        let right = rng.gen_bool(0.5);
        let horizontally;
        let horizontally_inv;
        if right {
            //println!("Direction::Right");
            horizontally = Direction::Right;
            horizontally_inv = Direction::Left;
        } else {
            //println!("Direction::Left");
            horizontally = Direction::Left;
            horizontally_inv = Direction::Right;
        };
        let down = rng.gen_bool(0.5);
        let vertically;
        let vertically_inv;
        if down {
            //println!("Direction::Down");
            vertically = Direction::Down;
            vertically_inv = Direction::Up;
        } else {
            //println!("Direction::Up");
            vertically = Direction::Up;
            vertically_inv = Direction::Down;
        };
        let length = h_prob_moves + v_prob_moves;
        let mut moves = Vec::with_capacity(length);
        let mut v_moves = 0;
        let mut h_moves = 0;
        for i in 0..length {
            if v_moves >= v_prob_moves {
                let inv = rng.gen_bool(inv_prob);
                if inv {
                    moves.push(horizontally_inv);
                } else {
                    moves.push(horizontally);
                }
                h_moves += 1;
                continue;
            }
            if h_moves >= h_prob_moves {
                let inv = rng.gen_bool(inv_prob);
                if inv {
                    moves.push(vertically_inv);
                } else {
                    moves.push(vertically);
                }
                v_moves += 1;
                continue;
            }
            let must_horizontally = rng.gen_bool(h_prob_moves as f64 / length as f64);
            if must_horizontally {
                let inv = rng.gen_bool(inv_prob);
                if inv {
                    moves.push(horizontally_inv);
                } else {
                    moves.push(horizontally);
                }
                h_moves += 1;
            } else {
                let inv = rng.gen_bool(inv_prob);
                if inv {
                    moves.push(vertically_inv);
                } else {
                    moves.push(vertically);
                }
                v_moves += 1;
            }
        }
        (moves, h_moves, v_moves)
    }
}
