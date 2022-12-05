use std::{array::from_fn, fs::read_to_string, ops::Deref};

const STACK_COUNT: usize = 9;
const STACK_WIDTH: usize = 4;

const PATH: &str = "src/input";

fn top_of_stack(stacks: [String; STACK_COUNT]) -> String {
    return stacks
        .map(|s| s.chars().last().unwrap_or('?').clone().to_string())
        .join("");
}

fn main() {
    let binding = read_to_string(PATH).expect("Error reading file");
    let mut data = binding.split("\n").into_iter().peekable();

    // NOTE: ALSO COPIES NUMBERS AT BOTTOM OF STACKS. SHOULD HAVE NO EFFECT.
    // READ STACKS
    let mut stacks: [String; STACK_COUNT] = from_fn(|_x| String::new());
    while data.peek().is_some() && data.peek().expect("expected string").deref().len() > 0 {
        let row = data.next().unwrap_or("");
        for i in 0..STACK_COUNT {
            let item = row
                .get(STACK_WIDTH * i + 1..STACK_WIDTH * i + 2)
                .unwrap_or("");
            if item.chars().all(char::is_alphanumeric) {
                stacks[i].push_str(item);
            }
        }
    }

    // inefficient
    for i in 0..STACK_COUNT {
        stacks[i] = stacks[i].chars().rev().collect::<String>();
    }
    let mut stacks_pt_2 = stacks.clone();

    // SKIP EMPTY LINE
    data.next();

    // PROCESS INSTRUCTIONS
    while data.peek().is_some() && data.peek().expect("expected string").deref().len() > 0 {
        let mut tokens = data.next().unwrap_or("").split_ascii_whitespace();

        macro_rules! get_token {
            () => {
                tokens.nth(1).unwrap().parse::<usize>().unwrap()
            };
        }

        let n_move = get_token!();
        let src = get_token!() - 1;
        let dest = get_token!() - 1;
        // println!("move {} from {}->{}", n_move, src, dest);

        let mut pt2_buf = String::new();

        for _i in 0..n_move {
            let c = stacks[src].pop().unwrap_or('\0');
            stacks[dest].push(c as char);

            // I know it is possible to append a whole string at once but this is a quicker solution to write.
            let c = stacks_pt_2[src].pop().unwrap_or('\0');
            pt2_buf.push(c as char);
        }
        stacks_pt_2[dest].push_str(&pt2_buf.chars().rev().collect::<String>());
        print!("");
    }

    println!(
        "PART 1 {}\nPART 2 {}",
        top_of_stack(stacks),
        top_of_stack(stacks_pt_2)
    );
}
