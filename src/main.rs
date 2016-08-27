#![feature(associated_consts)]
#![allow(dead_code)]
mod utils;
mod board;
mod boards;
mod boardset;
mod description;

use std::env::args;
use board::Board;
use boards::*;

pub const ENGLISH_START: &'static str = "  xxx  \n  xxx  \nxxxxxxx\nxxx.xxx\nxxxxxxx\n  xxx  \n  xxx  ";
pub const EUROPEAN_START: &'static str = "  xxx  \n xxxxx \nxxxxxxx\nxxx.xxx\nxxxxxxx\n xxxxx \n  xxx  ";
pub const HOLES15_START: &'static str = "x    \nxx   \nxxx  \nxxxx \n.xxxx";

fn main() {
    println!("peg-solitaire rust edition
Copyright (C) 2015-2016 Bernd Amend <berndamend+pegsolitaire@googlemail.com>
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License version 3 as published by
the Free Software Foundation. This program comes with ABSOLUTELY NO WARRANTY");

    match (args().nth(1), args().nth(2)) {
        (Some(s), Some(threads)) => {
            let threads = threads.parse::<usize>().unwrap();
            match s.as_ref() {
                "english" => {
                    let start = EnglishBoard::desc().from_string(ENGLISH_START).unwrap();
                    EnglishBoard::solve(start, threads);
                }
                "euro" => {
                    let start = EuropeanBoard::desc().from_string(EUROPEAN_START).unwrap();
                    EuropeanBoard::solve(start, threads);
                }
                "15" => {
                    let start = Holes15Board::desc().from_string(HOLES15_START).unwrap();
                    Holes15Board::solve(start, threads);
                }
                _ => {}
            }
        }
        (_, _) => {
            println!("usage {} <board> <threads>", std::env::args().nth(0).unwrap());
            println!("  boards: english, euro, 15");
        }
    }

    // let desc = &descriptions[0];

    // let start_fields = boards::possible_start_fields();
    // start_fields.foreach(|x| {
    //     println!("Field {}:\n{}\n", x, desc.to_string(x).unwrap());
    //  });
}

#[cfg(test)]
mod tests {
    use super::*;
    use board::Board;
    use boards::*;

    #[test]
    fn solve_test() {
        let start = EnglishBoard::desc().from_string(ENGLISH_START).unwrap();
        assert!(EnglishBoard::solve(start)
            .iter()
            .fold(0, |o, i| o + i.len()) == 23475688);
    }

    #[test]
    fn solve_parallel_tes_3() {
        let start = EnglishBoard::desc().from_string(ENGLISH_START).unwrap();
        assert!(EnglishBoard::solve_parallel(start, 3)
            .iter()
            .fold(0, |o, i| o + i.len()) == 23475688);
    }
    #[test]
    fn solve_parallel_test_4() {
        let start = EnglishBoard::desc().from_string(ENGLISH_START).unwrap();
        assert!(EnglishBoard::solve_parallel(start, 4)
            .iter()
            .fold(0, |o, i| o + i.len()) == 23475688);
    }
}
