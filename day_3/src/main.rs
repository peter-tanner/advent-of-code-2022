use std::{collections::HashSet, fs::read_to_string, ops::Deref};

const PATH: &str = "src/input";

fn value(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        return (c as u8 - b'a' + 1) as u32;
    } else if c.is_ascii_uppercase() {
        return (c as u8 - b'A' + 27) as u32;
    } else {
        panic!()
    }
}

fn main() {
    part_1();
    part_2();
}

fn to_hash(bag: Option<&str>) -> HashSet<&u8> {
    return HashSet::from_iter(bag.expect("must be some").as_bytes().iter());
}

fn part_2() {
    let binding = read_to_string(PATH).expect("Error reading file");
    let mut data = binding.split_ascii_whitespace().into_iter().peekable();
    let mut priority_sum: u32 = 0;
    while data.peek().is_some() {
        let test = **to_hash(data.next())
            .intersection(&to_hash(data.next()))
            .cloned()
            .collect::<HashSet<&u8>>()
            .intersection(&to_hash(data.next()))
            .last()
            .expect("expect") as char;
        priority_sum += value(test)
    }
    println!("PART 2 {}", priority_sum);
}

fn part_1() {
    let data = read_to_string(PATH).expect("Error reading file");
    let mut priority_sum: u32 = 0;
    data.split_ascii_whitespace().for_each(|line| {
        // Expect only bytes.
        let first_bag = line.as_bytes().get(0..line.len() / 2).expect("line").iter();
        let second_bag = line
            .as_bytes()
            .get(line.len() / 2..line.len())
            .expect("line")
            .iter();
        let first_set: HashSet<&u8> = HashSet::from_iter(first_bag);
        let second_set: HashSet<&u8> = HashSet::from_iter(second_bag);
        let common = *first_set
            .intersection(&second_set)
            .last()
            .unwrap_or(&&b'?')
            .deref() as char;

        priority_sum += value(common);

        // println!(
        //     "{} | {} => {} {}",
        //     std::str::from_utf8(line.as_bytes().get(0..line.len() / 2).unwrap()).unwrap(),
        //     std::str::from_utf8(line.as_bytes().get(line.len() / 2..line.len()).unwrap()).unwrap(),
        //     common,
        //     value(common)
        // );
    });

    println!("PART 1 {}", priority_sum);
}
