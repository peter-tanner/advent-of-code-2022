use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fs::read_to_string,
};

const PATH: &str = "src/input";
// const SCAN_DEPTH: i32 = 10;
const SCAN_DEPTH: i32 = 2000000;
const P2_SCAN_MAX: i32 = 4000000;

fn main() {
    let binding = read_to_string(PATH).expect("Error reading file");
    let data = binding.trim();
    parse_lines(data);
}

fn parse_lines(data: &str) {
    let mut sensors = HashSet::<(i32, i32)>::new();
    let mut bacons = HashSet::<(i32, i32)>::new();
    let mut bacons_count = HashMap::<i32, HashSet<i32>>::new();
    let mut horizontal_slices = HashMap::<i32, Vec<(i32, i32)>>::new();

    let binding = data.replace(|c: char| c.is_alphabetic() || c == ' ' || c == '=', "");
    let mut lines = binding.split('\n').into_iter();
    while let Some(line) = lines.next() {
        let mut parts = line.split([',', ':']).into_iter();
        let mut parse_token = || parts.next().unwrap().parse::<i32>().unwrap();
        let sensor = (parse_token(), parse_token());
        let beacon = (parse_token(), parse_token());
        let taxicab_radius: i32 = (sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1))
            .try_into()
            .unwrap();

        sensors.insert(sensor);
        bacons.insert(beacon);

        let bacon_count = match bacons_count.entry(beacon.1) {
            Entry::Vacant(entry) => entry.insert(HashSet::new()),
            Entry::Occupied(entry) => entry.into_mut(),
        };
        bacon_count.insert(beacon.0);

        // println!(
        //     "{} {} {} {} -> {}",
        //     sensor.0, sensor.1, beacon.0, beacon.1, taxicab_radius
        // );

        // can we do this in reverse from the beacon to sensors? does that improve anything?
        for j in 0..=taxicab_radius {
            let l = sensor.0 - (taxicab_radius - j) as i32;
            let u = sensor.0 + (taxicab_radius - j) as i32;
            let entry = match horizontal_slices.entry(sensor.1 + j) {
                Entry::Vacant(entry) => entry.insert(Vec::new()),
                Entry::Occupied(entry) => entry.into_mut(),
            };
            entry.push((l, u));

            let entry = match horizontal_slices.entry(sensor.1 - j) {
                Entry::Vacant(entry) => entry.insert(Vec::new()),
                Entry::Occupied(entry) => entry.into_mut(),
            };
            entry.push((l, u));

            // println!(
            //     "{} {} {}",
            //     sensor.0 - (taxicab_radius - j.abs()) as i32,
            //     sensor.0 + taxicab_radius - j.abs() as i32,
            //     sensor.1 + j
            // );
        }
    }

    part_1(
        horizontal_slices
            .get_mut(&SCAN_DEPTH)
            .unwrap_or(&mut vec![]),
        bacons_count.get(&SCAN_DEPTH).unwrap().len() as i32,
    );

    for j in 0..=P2_SCAN_MAX {
        let big_intervals =
            simplify_intervals(horizontal_slices.get_mut(&j).unwrap_or(&mut vec![]));
        if big_intervals.len() > 1 {
            let bacon_obtained = big_intervals.get(0).unwrap();
            let freq: i64 = (bacon_obtained.1 as i64 + 1) * 4000000 + j as i64;
            println!("PART 2 {}", freq);
            return;
        }
    }
    // print_grid(sensors, bacons, horizontal_slices);
}

fn part_1(intervals: &mut Vec<(i32, i32)>, bacons_row_count: i32) {
    let big_intervals = simplify_intervals(intervals);
    let mut sum = 0;
    for biginterval in big_intervals {
        sum += biginterval.1 - biginterval.0 + 1;
    }
    sum -= bacons_row_count;
    println!("PART 1 {}", sum);
}

fn simplify_intervals(intervals: &mut Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    if intervals.len() == 0 {
        return intervals.to_vec();
    }
    intervals.sort_by(|a, b| a.0.cmp(&b.0));
    let mut big_intervals = Vec::<(i32, i32)>::new();
    let mut intervals_iter = intervals.iter_mut();
    let biginterval = intervals_iter.next().unwrap();
    while let Some(interval) = intervals_iter.next() {
        // print!("[{} {}", biginterval.0, biginterval.1);
        // println!("|{} {}]", interval.0, interval.1);
        // println!(
        //     "{} {}",
        //     biginterval.1 >= interval.0,
        //     interval.1 >= biginterval.1
        // );
        // if interval_joined(&biginterval, interval) {
        if biginterval.1 >= interval.0 && interval.1 >= biginterval.1 {
            *biginterval = (biginterval.0, interval.1);
        } else if interval.1 >= biginterval.1 {
            // intervals.push(biginterval);
            big_intervals.push(biginterval.clone());
            *biginterval = interval.clone();
            // println!("-> {} {}", biginterval.0, biginterval.1);
        }
    }
    big_intervals.push(biginterval.clone());
    return big_intervals;
}

fn print_grid(
    sensors: HashSet<(i32, i32)>,
    bacons: HashSet<(i32, i32)>,
    horizontal_slices: HashMap<i32, Vec<(i32, i32)>>,
) {
    for j in -2..22 {
        for i in -2..=25 {
            let mut break_outer = false;
            if sensors.contains(&(i, j)) {
                print!("S");
                continue;
            } else if bacons.contains(&(i, j)) {
                print!("B");
                continue;
            } else {
                for v in horizontal_slices.get(&j).unwrap_or(&vec![]) {
                    if (v.0..=v.1).contains(&i) {
                        print!("#");
                        break_outer = true;
                        break;
                    }
                }
            }
            if !break_outer {
                print!(".");
            }
        }
        println!();
    }
}
