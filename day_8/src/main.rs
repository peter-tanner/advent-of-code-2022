use std::{collections::HashSet, fs::read_to_string};

const WIDTH: usize = 99;
const HEIGHT: usize = 99;

const PATH: &str = "src/input";

fn main() {
    // ASSUMPTIONS:
    // ALL ASCII, NUMERICAL [0-9]. ONE TREE PER CHAR.
    // WIDTH AND HEIGHT IS POSTIVE BUT BELOW isize.
    let binding = read_to_string(PATH)
        .expect("Error reading file")
        .replace('\n', "")
        .into_bytes()
        .iter()
        .map(|c| c - b'0')
        .collect::<Vec<u8>>();
    let data = binding.as_slice();

    part_1(data);
    part_2(data);
}

fn part_2(data: &[u8]) {
    fn valid_cell(i: isize, j: isize) -> bool {
        return i >= 0 && i < WIDTH as isize && j >= 0 && j < HEIGHT as isize;
    }
    // isize is more convenient for bounds checking but introduces a lot of casting.
    let rtx = |tree_height: u8, i: isize, j: isize, di: isize, dj: isize| -> usize {
        let mut count: usize = 0;
        let mut i_: isize = i;
        let mut j_: isize = j;
        while valid_cell(i_, j_) {
            count += 1;
            if data[(j_ as usize) * WIDTH + (i_ as usize)] >= tree_height {
                break; // Need to break after add for this case.
            }
            i_ += di;
            j_ += dj;
        }
        return count;
    };

    let mut max_score = 0;

    // brute-forceable for the input puzzle size.
    for i in 0..WIDTH as isize {
        for j in 0..HEIGHT as isize {
            let tree_height = data[(j as usize) * WIDTH + (i as usize)];
            let score = rtx(tree_height, i, j + 1, 0, 1)
                * rtx(tree_height, i - 1, j, -1, 0)
                * rtx(tree_height, i, j - 1, 0, -1)
                * rtx(tree_height, i + 1, j, 1, 0);
            if score > max_score {
                max_score = score;
            }
            // println!("SCORE {} ({}) ({},{})", u * l * d * r, tree_height, i, j);
        }
    }
    println!("PART 2 {}", max_score);
}

fn part_1(data: &[u8]) {
    let mut pvs = HashSet::<(usize, usize)>::new();

    macro_rules! tree_cell {
        ($i:expr, $j:expr) => {
            data[$j * WIDTH + $i]
        };
    }

    let mut prev_tree: u8;
    let mut first: bool;

    macro_rules! check {
        ($i: expr, $j: expr) => {
            let tree = tree_cell!($i, $j);
            if !first && tree <= prev_tree {
                // println!("STOP AT {},{}", $i, $j);
            } else {
                // println!("{},{}", $i, $j);
                pvs.insert(($i, $j));
                prev_tree = tree;
            }
            first = false;
        };
    }

    macro_rules! reset_check {
        () => {
            prev_tree = 0;
            first = true;
        };
    }

    // Test horizontal
    for j in 0..HEIGHT {
        reset_check!();
        for i in 0..WIDTH {
            check!(i, j);
        }
        reset_check!();
        for i in (0..WIDTH).rev() {
            check!(i, j);
        }
    }

    // Test vertical
    for i in 0..WIDTH {
        reset_check!();
        for j in 0..HEIGHT {
            check!(i, j);
        }
        reset_check!();
        for j in (0..HEIGHT).rev() {
            check!(i, j);
        }
    }

    println!("PART 1 {}", pvs.len());

    // PRINT FOREST
    // for j in 0..HEIGHT {
    //     for i in 0..WIDTH {
    //         if pvs.contains(&(i, j)) {
    //             print!("X");
    //         } else {
    //             print!("-");
    //         }
    //     }
    //     println!();
    // }
}
