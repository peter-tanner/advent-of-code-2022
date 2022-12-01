use std::fs;

const PATH: &str = "src/input";

fn main() {
    let data = fs::read_to_string(PATH).expect("Error reading file");
    let mut cals_list: Vec<u32> = Vec::new();
    let mut cals: u32 = 0;
    for line in data.split("\n") {
        if line.len() == 0 {
            cals_list.push(cals);
            cals = 0;
        } else {
            cals += line.parse::<u32>().unwrap();
        }
    }
    cals_list.sort_by(|a, b| b.cmp(a));
    println!("PART 1: {}", cals_list[0]);
    println!("PART 2: {}", cals_list[0] + cals_list[1] + cals_list[2]);
}
