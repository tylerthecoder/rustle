use std::io::{BufRead, BufReader};
use std::fs;
use rand::seq::IteratorRandom; // 0.7.3


static WORDS: &'static str = include_str!("../words.txt");


pub fn play(n: u8) -> String {
    WORDS.lines()
        .filter(|word| word.len() == n as usize)
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string()
}
