use std::fs::read_to_string;

struct the_rock {
    rock_pattern: [u8; 4],
    height: u8,
}

// ROCKS ARE FLIPPED HORIZONTALLY
const ROCKS: [[u8; 4]; 5] = [
    [0b000_0000, 0b000_0000, 0b000_0000, 0b011_1100], // _
    [0b000_0000, 0b000_1000, 0b001_1100, 0b000_1000], // +
    [0b000_0000, 0b000_0100, 0b000_0100, 0b001_1100], // ⅃
    [0b000_0100, 0b000_0100, 0b000_0100, 0b000_0100], // |
    [0b000_0000, 0b000_0000, 0b000_1100, 0b000_1100], // []
];

const PATH: &str = "src/input";

fn main() {
    let binding = read_to_string(PATH).expect("Error reading file");
    assert!(binding.is_ascii());
    let flow = binding.trim().as_bytes();

    let mut chamber: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let mut max_height: usize = 0;
    let mut active_rock_type: usize = 0;
    let mut flow_i = 0;
    for i in 0..5 {
        let mut rock_type = ROCKS[active_rock_type % ROCKS.len()].clone();
        let rock_idx =
            get_collision_point(&mut chamber, &flow, max_height, &mut rock_type, &mut flow_i);
        println!("STOP {}", rock_idx);
        for i in 0..4 {
            chamber[rock_idx + i] |= rock_type[3 - i];
            if chamber[rock_idx + i] != 0 && rock_idx + i > max_height {
                chamber.push(0);
                max_height = rock_idx + i;
            }
        }
        println!("{} U", max_height);
        print_grid(&chamber);
        active_rock_type += 1;
    }
    print_grid(&chamber);
    println!("PART 1 {}", max_height);
}

fn get_collision_point(
    chamber: &mut Vec<u8>,
    flow: &[u8],
    max_height: usize,
    rock_type: &mut [u8; 4],
    flow_i: &mut usize,
) -> usize {
    let mut rock_idx = max_height + 3;
    loop {
        let flow_dir = flow[*flow_i % flow.len()];
        *flow_i += 1;
        let mut shift_valid = true;
        for i in 0..4 {
            let mut row_opt = chamber.get(rock_idx + i + 1);
            let row = row_opt.get_or_insert(&0);
            match flow_dir {
                b'>' => {
                    if rock_type[3 - i] & 0b000_0001 != 0 {
                        shift_valid = false;
                    }
                    if rock_type[3 - i] >> 1 & *row != 0 {
                        shift_valid = false;
                        // return rock_idx + 1;
                    }
                }
                b'<' => {
                    if rock_type[3 - i] & 0b100_0000 != 0 {
                        shift_valid = false;
                    }
                    if rock_type[3 - i] << 1 & *row != 0 {
                        shift_valid = false;
                        // return rock_idx + 1;
                    }
                }
                _ => {}
            }
        }
        if shift_valid {
            match flow_dir {
                b'>' => println!("RIGHT"),
                b'<' => println!("LEFT"),
                _ => panic!(),
            }
        } else {
            println!("NONE");
        }

        if shift_valid {
            for i in 0..4 {
                match flow_dir {
                    // REVERSE
                    b'>' => {
                        rock_type[3 - i] = rock_type[3 - i] >> 1;
                    }
                    b'<' => {
                        rock_type[3 - i] = rock_type[3 - i] << 1;
                    }
                    _ => {
                        panic!();
                    }
                };
            }
        }

        for i in 0..4 {
            let mut row_opt = chamber.get(rock_idx + i);
            let row = row_opt.get_or_insert(&0);
            if *row & rock_type[3 - i] != 0x00 {
                return rock_idx + 1;
            }
            // print_grid(&chamber);
        }
        if rock_idx == 0 {
            return rock_idx;
        }

        if (chamber.get(rock_idx.saturating_sub(1)).unwrap() & rock_type[3]) != 0 {
            return rock_idx + 1;
        }
        rock_idx -= 1;
    }
}

fn print_grid(chamber: &Vec<u8>) {
    println!("CHAMBER: ");
    for i in (0..chamber.len()).rev() {
        let x = chamber[i];
        let string = format!("{x:#09b}\n")
            .replace("0b", "")
            .replace('0', ".")
            .replace('1', "#");
        print!("{}", string);
    }
}
