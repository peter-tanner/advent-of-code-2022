use std::{collections::HashSet, fs::read_to_string};

fn parse_direction(dir: &str, mag: &str) -> (i32, i32) {
    let dir_p = dir.chars().nth(0).unwrap();
    let mag_p = mag.parse::<i32>().unwrap();
    let v = match dir_p {
        'U' => (0, -mag_p),
        'D' => (0, mag_p),
        'L' => (-mag_p, 0),
        'R' => (mag_p, 0),
        _ => {
            panic!()
        }
    };
    return v;
}

fn move_head(v_h: &mut (i32, i32), dir: &str) {
    let dir_p = dir.chars().nth(0).unwrap();
    match dir_p {
        'U' => v_h.1 -= 1,
        'D' => v_h.1 += 1,
        'L' => v_h.0 -= 1,
        'R' => v_h.0 += 1,
        _ => {
            panic!()
        }
    };
}

fn rule_1(v_h: &(i32, i32), v_t: &mut (i32, i32)) {
    let di = v_h.0 - v_t.0;
    let dj = v_h.1 - v_t.1;

    let change = match (di, dj) {
        (2, 0) => (1, 0),
        (0, 2) => (0, 1),
        (-2, 0) => (-1, 0),
        (0, -2) => (0, -1),
        _ => (0, 0),
    };
    v_t.0 += change.0;
    v_t.1 += change.1;
}

fn rule_2(v_h: &(i32, i32), v_t: &mut (i32, i32)) {
    let di = v_h.0 - v_t.0;
    let dj = v_h.1 - v_t.1;

    let change: (i32, i32) = match (di, dj) {
        (1, 2) => (1, 1),
        (2, 1) => (1, 1),

        (-2, -1) => (-1, -1),
        (-1, -2) => (-1, -1),

        (1, -2) => (1, -1),
        (2, -1) => (1, -1),

        (-1, 2) => (-1, 1),
        (-2, 1) => (-1, 1),
        _ => (0, 0),
    };
    v_t.0 += change.0;
    v_t.1 += change.1;
}

fn print_grid(
    r: i32,
    tail_points: &mut HashSet<(i32, i32)>,
    v_h: &(i32, i32),
    v_t: &mut (i32, i32),
) {
    for j in -r..r {
        for i in -r..r {
            if i == 0 && j == 0 {
                print!("S");
            // } else if i == v_h.0 && j == v_h.1 {
            //     print!("H")
            // } else if i == v_t.0 && j == v_t.1 {
            //     print!("T")
            } else if tail_points.contains(&(i, j)) {
                print!("X");
            } else {
                print!("-");
            }
        }
        println!();
    }
}

const PATH: &str = "src/input";
fn main() {
    // FOR PART 1, THERE IS A SMARTER SOLUTION BUT WITH THE SIZE OF THE INPUT WE CAN DO IT WITH THE NAIVE SOLUTION.
    let mut tail_points = HashSet::<(i32, i32)>::new();
    let binding = read_to_string(PATH).expect("Error reading file");
    assert!(binding.is_ascii());
    let data = binding.trim().split('\n').into_iter();

    let v_h = &mut (0, 0);
    let v_t = &mut (0, 0);
    for instruction in data {
        let mut tokens = instruction.split_ascii_whitespace().into_iter();
        let dir = tokens.next().unwrap();
        let mag = tokens.next().unwrap().parse::<usize>().unwrap();

        // println!("{} {}", dir, mag);

        tail_points.insert(*v_t);

        for i in 0..mag {
            // println!("({},{}), ({},{})", v_t.0, v_t.1, v_h.0, v_h.1);
            move_head(v_h, dir);
            rule_1(v_h, v_t);
            rule_2(v_h, v_t);
            tail_points.insert(*v_t);
        }
    }
    print_grid(5, &mut tail_points, v_h, v_t);
    println!("PART 1 {}", tail_points.len());
}
