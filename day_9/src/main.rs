use std::{collections::HashSet, fs::read_to_string};

const PATH: &str = "src/input";
fn main() {
    println!("PART 1 {}", rope_sim(2));
    println!("PART 2 {}", rope_sim(10));
}

fn rope_sim(rope_size: usize) -> usize {
    // FOR PART 1, THERE IS A SMARTER SOLUTION BUT WITH THE SIZE OF THE INPUT WE CAN DO IT WITH THE NAIVE SOLUTION.
    let binding = read_to_string(PATH).expect("Error reading file");
    assert!(binding.is_ascii());
    let data = binding.trim().split('\n').into_iter();

    // WTF IS THIS???
    let mut rope = Vec::<(i32, i32)>::new();
    for _ in 0..rope_size {
        rope.push((0, 0));
    }

    let mut tail_points = HashSet::<(i32, i32)>::new();
    tail_points.insert(*rope.last().unwrap());

    for instruction in data {
        let mut tokens = instruction.split_ascii_whitespace().into_iter();
        let dir = tokens.next().unwrap();
        let mag = tokens.next().unwrap().parse::<usize>().unwrap();

        for _ in 0..mag {
            // println!("({},{}), ({},{})", v_t.0, v_t.1, v_h.0, v_h.1);
            move_head(rope.get_mut(0).unwrap(), dir);
            for rope_part_i in 1..rope_size {
                // SEGMENT HEAD AND TAIL.
                let (part_1, part_2) = rope.split_at_mut(rope_part_i);
                let v_dh = part_1.get_mut(rope_part_i - 1).unwrap(); //rope.get_mut(rope_part_i - 1).unwrap()
                let v_dt = part_2.get_mut(0).unwrap(); //rope.get_mut(rope_part_i).unwrap()
                rule_1(&v_dh, v_dt);
                rule_2(&v_dh, v_dt);
            }
            tail_points.insert(*rope.last().unwrap());
        }
        // print_grid(12, &mut tail_points);
    }
    return tail_points.len();
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
        (2, 2) => (1, 1),

        (-2, -1) => (-1, -1),
        (-1, -2) => (-1, -1),
        (-2, -2) => (-1, -1),

        (1, -2) => (1, -1),
        (2, -1) => (1, -1),
        (2, -2) => (1, -1),

        (-1, 2) => (-1, 1),
        (-2, 1) => (-1, 1),
        (-2, 2) => (-1, 1),
        _ => (0, 0),
    };
    v_t.0 += change.0;
    v_t.1 += change.1;
}

fn print_grid(
    r: i32,
    tail_points: &mut HashSet<(i32, i32)>,
    // v_h: &(i32, i32),
    // v_t: &mut (i32, i32),
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
