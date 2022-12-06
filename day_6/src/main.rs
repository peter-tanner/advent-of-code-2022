use std::{collections::HashSet, fs::read_to_string, ops::Add};

const PATH: &str = "src/input";

const HEADER_PACKET_LENGTH: usize = 4;
const MESSAGE_LENGTH: usize = 14;

fn is_unique(chunk: &str) -> bool {
    let mut set = HashSet::<u8>::new();
    for c in chunk.bytes().into_iter() {
        if set.contains(&c) {
            return false;
        } else {
            set.insert(c);
        }
    }
    return true;
}

fn main() {
    assert!(HEADER_PACKET_LENGTH > 0);
    assert!(MESSAGE_LENGTH > 0);

    let datagram = read_to_string(PATH)
        .expect("Error reading file")
        .replace(char::is_whitespace, "")
        .add(&"?".repeat(HEADER_PACKET_LENGTH))
        .add(&"?".repeat(MESSAGE_LENGTH));

    assert!(datagram.is_ascii());

    let mut header_idx: usize = 0;
    let mut message_idx: usize = 0;

    for i in 0..datagram.len() {
        // There are certainly more efficient methods that save computation by only
        // computing whether the addition of one character and removal of the last
        // are sufficient to be unique. This is easier though.
        if header_idx == 0 {
            let window_header = datagram.get(i..i + HEADER_PACKET_LENGTH).unwrap();
            if is_unique(window_header) {
                header_idx = i + HEADER_PACKET_LENGTH;
            }
        }
        if message_idx == 0 {
            let window_message = datagram.get(i..i + MESSAGE_LENGTH).unwrap();
            if is_unique(window_message) {
                message_idx = i + MESSAGE_LENGTH;
            }
        }
        if header_idx != 0 && message_idx != 0 {
            println!("PART 1 {}", header_idx);
            println!("PART 2 {}", message_idx);
            break;
        }
    }
}
