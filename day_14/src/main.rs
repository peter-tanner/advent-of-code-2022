use std::{collections::HashSet, fs::read_to_string, ops::RangeInclusive};

const PATH: &str = "src/input";

fn parse_pt(p_str: &str) -> Vec<i32> {
    p_str
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn any_range(a: i32, b: i32) -> RangeInclusive<i32> {
    if a < b {
        return a..=b;
    }
    return b..=a;
}

const SAND_SOURCE: (i32, i32) = (500, 0);

fn main() {
    let data = read_to_string(PATH).expect("Error reading file");
    assert!(data.is_ascii());
    println!("PART 1 {}", sand_sim(data, true));
    let data = read_to_string(PATH).expect("Error reading file");
    assert!(data.is_ascii());
    println!("PART 2 {}", sand_sim(data, false));
}

fn sand_sim(data: String, part_1: bool) -> u32 {
    // TODO: DETERMINE MAX DEPTH AUTOMATICALLY.
    let (mut walls, max_depth) = scan_walls(data);

    let mut stop_sand = false;
    let mut sand_units = 0;
    while !stop_sand && !walls.contains(&SAND_SOURCE) {
        let mut sand_pos = SAND_SOURCE;

        loop {
            // If the tile immediately below is blocked (by rock or sand), ...
            if walls.contains(&(sand_pos.0, sand_pos.1 + 1)) {
                // ... the unit of sand attempts to instead move diagonally one step down and to the left
                if !walls.contains(&(sand_pos.0 - 1, sand_pos.1 + 1)) {
                    sand_pos.0 -= 1;
                // If that tile is blocked, the unit of sand attempts to instead move diagonally one step down and to the right
                } else if !walls.contains(&(sand_pos.0 + 1, sand_pos.1 + 1)) {
                    sand_pos.0 += 1;
                // If all three possible destinations are blocked, the unit of sand comes to rest and no longer moves
                } else {
                    walls.insert(sand_pos);
                    sand_units += 1;
                    break;
                }
            }
            if sand_pos.1 > max_depth {
                walls.insert(sand_pos);
                if part_1 {
                    stop_sand = true;
                }
                sand_units += 1;
                break;
            }
            // A unit of sand always falls down one step if possible.
            sand_pos.1 += 1;
        }
    }
    // visualize_sand(walls, max_depth);
    return sand_units;
}

fn scan_walls(data: String) -> (HashSet<(i32, i32)>, i32) {
    let paths = data.split('\n');
    let mut walls = HashSet::<(i32, i32)>::new();
    let mut max_depth = 0;
    for path in paths {
        let mut points = path.split(" -> ").into_iter();
        let mut previous_point = parse_pt(points.next().unwrap());
        while let Some(next_point) = points.next() {
            let next_point = parse_pt(next_point);
            if previous_point[1] > max_depth {
                max_depth = previous_point[1];
            }
            if next_point[1] > max_depth {
                max_depth = next_point[1];
            }
            // println!(
            //     "{} {} -> {} {}",
            //     previous_point[0], previous_point[1], next_point[0], next_point[1]
            // );
            for x in any_range(previous_point[0], next_point[0]) {
                for y in any_range(previous_point[1], next_point[1]) {
                    walls.insert((x, y));
                }
            }
            previous_point = next_point;
        }
    }
    return (walls, max_depth);
}

fn visualize_sand(walls: HashSet<(i32, i32)>, max_depth: i32) {
    for j in 0..=max_depth + 2 {
        for i in 480..550 {
            if walls.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
