use std::fs::read_to_string;

const PATH: &str = "src/input";

macro_rules! val_in_range {
    ($test:expr, $l:expr, $u:expr) => {
        $test >= $l && $test <= $u
    };
}

// [l1,u1] inside of [l2,u2]
macro_rules! range_in_range {
    ($l1:expr, $u1:expr, $l2:expr, $u2:expr) => {
        $l1 <= $l2 && $u1 >= $u2
    };
}

fn main() {
    let binding = read_to_string(PATH).expect("Error reading file");
    let data = binding.split_ascii_whitespace().into_iter();
    let mut overlapping_ranges_pt1: u32 = 0;
    let mut overlapping_ranges_pt2: u32 = 0;
    for pair in data {
        let ranges: Vec<u32> = pair
            .split(&['-', ','])
            .map(|x| x.parse().unwrap())
            .collect();

        overlapping_ranges_pt2 += (val_in_range!(ranges[0], ranges[2], ranges[3])
            || val_in_range!(ranges[1], ranges[2], ranges[3])
            || val_in_range!(ranges[2], ranges[0], ranges[1])
            || val_in_range!(ranges[3], ranges[0], ranges[1]))
            as u32;

        overlapping_ranges_pt1 += (range_in_range!(ranges[0], ranges[1], ranges[2], ranges[3])
            || range_in_range!(ranges[2], ranges[3], ranges[0], ranges[1]))
            as u32
    }
    println!(
        "PART 1 {}\nPART 2 {}",
        overlapping_ranges_pt1, overlapping_ranges_pt2
    );
}
