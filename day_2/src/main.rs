use std::collections::HashMap;
use std::{fs, ops::Deref};

const PATH: &str = "src/input";

const LOSS_PTS: u32 = 0;
const DRAW_PTS: u32 = 3;
const WIN_PTS: u32 = 6;

const ROCK: u8 = 1;
const PAPER: u8 = 2;
const SCISSORS: u8 = 3;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let mut game_score: HashMap<(u8, u8), u32> = HashMap::new();
    game_score.insert((ROCK, ROCK), DRAW_PTS);
    game_score.insert((PAPER, PAPER), DRAW_PTS);
    game_score.insert((SCISSORS, SCISSORS), DRAW_PTS);

    game_score.insert((PAPER, ROCK), LOSS_PTS);
    game_score.insert((ROCK, SCISSORS), LOSS_PTS);
    game_score.insert((SCISSORS, PAPER), LOSS_PTS);

    game_score.insert((ROCK, PAPER), WIN_PTS);
    game_score.insert((SCISSORS, ROCK), WIN_PTS);
    game_score.insert((PAPER, SCISSORS), WIN_PTS);

    let data = fs::read_to_string(PATH).expect("Error reading file");
    let mut total_score = 0;
    for line in data.split("\n") {
        if line.len() > 0 {
            let opponent = line.as_bytes().get(0).unwrap().deref() - b'A' + 1;
            let player = line.as_bytes().get(2).unwrap().deref() - b'X' + 1;
            let score = game_score.get(&(opponent, player)).unwrap() + (player as u32);
            total_score += score;
        }
    }
    println!("PART 1 total_score={}", total_score);
}

fn part_2() {
    let mut choice_score: HashMap<(char, char), u8> = HashMap::new();
    choice_score.insert(('A', 'X'), SCISSORS);
    choice_score.insert(('A', 'Y'), ROCK);
    choice_score.insert(('A', 'Z'), PAPER);

    choice_score.insert(('B', 'X'), ROCK);
    choice_score.insert(('B', 'Y'), PAPER);
    choice_score.insert(('B', 'Z'), SCISSORS);

    choice_score.insert(('C', 'X'), PAPER);
    choice_score.insert(('C', 'Y'), SCISSORS);
    choice_score.insert(('C', 'Z'), ROCK);

    let mut game_score: HashMap<char, u32> = HashMap::new();
    game_score.insert('X', 0);
    game_score.insert('Y', 3);
    game_score.insert('Z', 6);

    let data = fs::read_to_string(PATH).expect("Error reading file");
    let mut total_score: u32 = 0;
    for line in data.split("\n") {
        if line.len() > 0 {
            let opponent = line.chars().nth(0).unwrap();
            let player = line.chars().nth(2).unwrap();

            total_score += *(choice_score.get(&(opponent, player)).unwrap_or(&0)) as u32
                + *game_score.get(&player).unwrap_or(&0);
        }
    }
    println!("PART 2 total_score={}", total_score);
}
