use std::{collections::HashMap, fs};

const PATH: &str = "src/input";

fn main() {
    let translation_tbl = HashMap::<&str, u32>::new();
    let adjacency_list = HashMap::<u32, Vec<u32>>::new();
    let data = fs::read_to_string(PATH)
        .expect("Error reading file")
        .replace("Valve ", "")
        .replace("has flow rate=", "")
        .replace("; tunnels lead to valves", "")
        .replace("; tunnel leads to valve", "")
        .replace(',', "");

    for (i, line) in data.split('\n').enumerate() {
        let mut tokens = line.split([' ', '=']);
        // tokens.advance_by(1);
        // let valve_name = tokens.next();
        // tokens.advance_by(3);
        // let valve_rate = tokens.next();
        // tokens.advance_by(5);
        for (i, token) in line.split([' ', '=']).enumerate() {
            println!("{} {}", i, token);
        }
    }
}
