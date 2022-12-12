use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fs::read_to_string,
};

const PATH: &str = "src/input";

fn get_dim(input: &[u8]) -> (usize, usize) {
    let height = input.iter().filter(|c| **c == b'\n').count() + 1;
    let width = input.iter().position(|c| *c == b'\n').unwrap();
    (height, width)
}

fn get_start(input: &[u8]) -> usize {
    get_char(input, b'S')
}
fn get_end(input: &[u8]) -> usize {
    get_char(input, b'E')
}

fn get_char(input: &[u8], needle: u8) -> usize {
    let i = input.iter().position(|c| *c == needle).unwrap();
    // (i % width, i / width)
    i
}

// COPIED FROM RUST DOCS
// https://doc.rust-lang.org/std/collections/binary_heap/index.html
#[derive(Copy, Clone, Eq, PartialEq)]
struct Vert {
    distance: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Vert {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Vert {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
// END

fn main() {
    let binding = read_to_string(PATH).expect("Error reading file");
    assert!(binding.is_ascii());
    let (b_height, b_width) = get_dim(binding.trim().as_bytes());

    let binding = binding.replace('\n', "");
    let data = binding.trim().as_bytes();

    macro_rules! ij {
        ($index: expr) => {
            ($index % b_width, $index / b_width)
        };
    }

    macro_rules! linear {
        ($i:expr, $j:expr) => {
            $i + $j * b_width
        };
    }

    println!("BOARD SIZE {} {}", b_width, b_height);
    let start = get_start(data);
    println!("start {} {}", ij!(start).0, ij!(start).1);
    let end = get_end(data);
    println!("end {} {}", ij!(end).0, ij!(end).1);

    let mut previous = (0..b_width * b_height)
        .map(|_| None)
        .collect::<Vec<Option<usize>>>();
    let mut distances = (0..b_width * b_height)
        .map(|_| usize::MAX)
        .collect::<Vec<usize>>();
    distances[end] = 0;
    let mut visited = (0..b_width * b_height)
        .map(|_| false)
        .collect::<Vec<bool>>();
    let mut vertex_q = (0..b_width * b_height)
        .map(|v| Vert {
            position: v,
            distance: distances[v],
        })
        .collect::<BinaryHeap<Vert>>();
    let mut frontier_scanned = Vec::<usize>::new();

    while let Some(u) = vertex_q.pop() {
        // GET ADJACENT VERTEXES
        if visited[u.position] {
            continue;
        }
        let u_tuple = ij!(u.position);
        let mut adjacent = Vec::<usize>::new();
        if u_tuple.0 >= 1 {
            adjacent.push(linear!(u_tuple.0 - 1, u_tuple.1));
        }
        if u_tuple.0 < b_width - 1 {
            adjacent.push(linear!(u_tuple.0 + 1, u_tuple.1));
        }
        if u_tuple.1 >= 1 {
            adjacent.push(linear!(u_tuple.0, u_tuple.1 - 1));
        }
        if u_tuple.1 < b_height - 1 {
            adjacent.push(linear!(u_tuple.0, u_tuple.1 + 1));
        }
        // END
        // println!(
        //     "@ {} {} ADJ TO {}",
        //     u.position % b_width,
        //     u.position / b_width,
        //     adjacent.len()
        // );
        for v in &adjacent {
            let v = *v;
            // UNIFORM STEP COST OF 1
            let alt_path = if data[u.position] != b'E' && data[u.position].abs_diff(data[v]) <= 1
                || data[u.position] != b'E' && data[u.position] < data[v]
                || data[u.position] == b'E' && data[v] == b'z'
                || data[u.position] == b'a' && data[v] == b'S'
            {
                frontier_scanned.push(v);
                distances[u.position].saturating_add(1)
            } else {
                usize::MAX
            };
            if alt_path < distances[v] {
                // println!(
                //     "ADJACENT TO ({},{}) : ({},{}) {} {} {}",
                //     u.position % b_width,
                //     u.position / b_width,
                //     v % b_width,
                //     v / b_width,
                //     data[u.position] as char,
                //     data[v] as char,
                //     data[u.position].abs_diff(data[v])
                // );
                distances[v] = alt_path;
                previous[v] = Some(u.position);
                vertex_q.push(Vert {
                    distance: alt_path,
                    position: v,
                })
            }
            visited[u.position] = true;
        }
    }

    //PRINT GRID
    let mut shortest_path = HashSet::<usize>::new();
    let mut x = start;
    shortest_path.insert(start);
    while previous[x].is_some() {
        shortest_path.insert(previous[x].unwrap());
        x = previous[x].unwrap();
    }

    for j in 0..b_height {
        for i in 0..b_width {
            let index = i + j * b_width;
            if shortest_path.contains(&(index)) {
                print!("\x1b[31m{}\x1b[0m", data[index] as char);
            } else if visited[index] {
                print!("{}", data[index] as char);
            } else if frontier_scanned.contains(&(index)) {
                print!("\x1b[43m{}\x1b[0m", data[index] as char);
            } else {
                print!("\x1b[93m{}\x1b[0m", data[index] as char);
            }
        }
        println!();
    }
    //END
    println!("PART 1 {}", distances[start]);

    let a_ends = data
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == b'a')
        .map(|(i, _)| distances[i])
        .min()
        .unwrap();
    println!("PART 2 {}", a_ends);
}
