#![feature(associated_consts)]
#![allow(dead_code)]
mod board;
mod boards;
mod boardset;
mod description;

use board::Board;
use boards::*;

fn main() {
    println!("peg-solitaire rust edition
Copyright (C) 2015-2016 Bernd Amend <berndamend+pegsolitaire@googlemail.com>
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License version 3 as published by
the Free Software Foundation. This program comes with ABSOLUTELY NO WARRANTY");

    match std::env::args().nth(1) {
        None => {
            println!("usage {}", std::env::args().nth(0).unwrap());
            println!("  solve_eng");
            println!("  solve_eng_par");
            println!("  all");
        }
        Some(s) => {
            match s.as_ref() {
                "solve_eng" => {
                    EnglishBoard::solve(8589869055u64);
                }
                "solve_eng_par" => {
                    EnglishBoard::solve_parallel(8589869055u64);
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use board::Board;
    use boards::*;

    #[test]
    fn solve_test() {
        assert!(EnglishBoard::solve(8589869055u64)
            .iter()
            .fold(0, |o, i| o + i.len()) == 23475688);
    }

    #[test]
    fn solve_parallel_test() {
        assert!(EnglishBoard::solve_parallel(8589869055u64)
            .iter()
            .fold(0, |o, i| o + i.len()) == 23475688);
    }
}
