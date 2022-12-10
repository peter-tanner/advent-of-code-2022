use std::{collections::HashSet, fs::read_to_string};

const PATH: &str = "src/input";

fn main() {
    let binding = read_to_string(PATH).expect("Error reading file");
    assert!(binding.is_ascii());
    let data = binding.trim().split('\n').into_iter();

    // SCREEN OUTPUT
    let mut vga = HashSet::<(i32, i32)>::new();

    // REGISTERS
    let mut rx = 1;
    let mut ra = 0; // SIGNAL STRENGTH SUM
    let mut rclk = 0;

    macro_rules! update_clk {
        () => {
            // OUTPUT TO SCREEN
            // CHECK ALL 3 POSITIONS OF SPRITE.
            for dx in -1..=1 {
                if rclk % 40 == rx + dx {
                    vga.insert((rx + dx, rclk / 40));
                }
            }

            // INCREMENT CLOCK
            rclk += 1;

            // UPDATE VOLTAGE COUNTER REGISTER
            if (rclk - 20) % 40 == 0 {
                ra += rclk * rx;
            }
        };
    }

    for instruction in data {
        let mut tokens = instruction.split(' ');
        // MATCH MNENOMIC AND UPDATE CLOCK
        let mnenomic = tokens.next().unwrap();
        match mnenomic {
            "addx" => {
                let operand = tokens.next().unwrap().parse::<i32>().unwrap();

                // TWO CYCLES
                update_clk!();
                update_clk!();
                rx += operand;
            }
            "noop" | _ => {
                // ONE CYCLE
                update_clk!();
            }
        }
    }

    println!("PART 1 {}", ra);
    println!("PART 2");
    print_screen(vga);
}

fn print_screen(vga: HashSet<(i32, i32)>) {
    for j in 0..6 {
        for i in 0..40 {
            if vga.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
