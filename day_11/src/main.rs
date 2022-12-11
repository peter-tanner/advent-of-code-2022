use std::ops::RangeInclusive;

struct Monkey {
    items: Vec<u64>,
    operation: fn(u64) -> u64,
    test_divisor: u64,
    throw_true: usize,
    throw_false: usize,
    inspections: usize,
}

enum ManagementStrategy {
    Div3,
    Span,
}

// COULDN'T BE BOTHERED TO WRITE A PARSER FOR THIS DATA
// FORMATTED IT USING VSCODE MULTICURSORS

fn main() {
    // CHANGE TO input_monkeys() TO USE PUZZLE INPUT
    let monkeys = demo_monkeys();
    println!(
        "PART 1 {}",
        parts(monkeys, 1..=20, ManagementStrategy::Div3)
    );
    // CHANGE TO input_monkeys() TO USE PUZZLE INPUT
    let monkeys = demo_monkeys();
    println!(
        "PART 2 {}",
        parts(monkeys, 1..=10_000, ManagementStrategy::Span)
    );
}

fn parts(
    mut monkeys: Vec<Monkey>,
    range: RangeInclusive<i32>,
    management_strategy: ManagementStrategy,
) -> usize {
    let modulo_span = monkeys.iter().map(|m| m.test_divisor).product::<u64>();

    for _ in range {
        for m_i in 0..monkeys.len() {
            let items_len = monkeys.get(m_i).unwrap().items.len();
            for i in (0..items_len).rev() {
                let monkey = monkeys.get_mut(m_i).unwrap();

                monkey.inspections += 1;
                let item = monkey.items.get_mut(i).unwrap();
                // ALL THE test_divisor SPAN modulo_span.
                // WE CAN MANAGE WORRIED LEVELS BY KEEPING IT IN modulo_span
                // WHICH AVOIDS NUMBERS GETTING TOO LARGE.
                *item = (monkey.operation)(*item);
                match management_strategy {
                    ManagementStrategy::Div3 => *item /= 3,
                    ManagementStrategy::Span => *item %= modulo_span,
                };

                let throw_monkey_i = if *item % monkey.test_divisor == 0 {
                    monkey.throw_true
                } else {
                    monkey.throw_false
                };

                let throw_item = monkey.items.swap_remove(i);
                let throw_monkey = monkeys.get_mut(throw_monkey_i).unwrap();
                throw_monkey.items.push(throw_item);
                // println!("THROW {} FROM {} TO {}", throw_item, m_i, throw_monkey_i);
            }
        }
        //     for monkey in monkeys.iter() {
        //         println!(
        //             "{}",
        //             monkey
        //                 .items
        //                 .iter()
        //                 .map(|x| x.to_string() + ",")
        //                 .collect::<String>()
        //         )
        //     }
    }
    return get_monkey_business(&monkeys);
}

fn get_monkey_business(monkeys: &Vec<Monkey>) -> usize {
    let mut monkey_inspections = monkeys
        .iter()
        .map(|x| x.inspections)
        .collect::<Vec<usize>>();

    monkey_inspections.sort_by(|a, b| b.cmp(a));
    return monkey_inspections.get(0).unwrap() * monkey_inspections.get(1).unwrap();
}

// COULDN'T BE BOTHERED TO WRITE A PARSER FOR THIS DATA
// FORMATTED IT USING VSCODE MULTICURSORS
fn demo_monkeys() -> Vec<Monkey> {
    let mut monkeys = Vec::<Monkey>::new();
    monkeys.push(Monkey {
        items: vec![79, 98],
        operation: |old: u64| old * 19,
        test_divisor: 23,
        throw_true: 2,
        throw_false: 3,
        inspections: 0,
    });

    monkeys.push(Monkey {
        items: vec![54, 65, 75, 74],
        operation: |old: u64| old + 6,
        test_divisor: 19,
        throw_true: 2,
        throw_false: 0,
        inspections: 0,
    });

    monkeys.push(Monkey {
        items: vec![79, 60, 97],
        operation: |old: u64| old * old,
        test_divisor: 13,
        throw_true: 1,
        throw_false: 3,
        inspections: 0,
    });

    monkeys.push(Monkey {
        items: vec![74],
        operation: |old: u64| old + 3,
        test_divisor: 17,
        throw_true: 0,
        throw_false: 1,
        inspections: 0,
    });
    return monkeys;
}

fn input_monkeys() -> Vec<Monkey> {
    let mut monkeys = Vec::<Monkey>::new();
    monkeys.push(Monkey {
        items: vec![84, 66, 62, 69, 88, 91, 91],
        operation: |old: u64| old * 11,
        test_divisor: 2,
        throw_true: 4,
        throw_false: 7,
        inspections: 0,
    });
    monkeys.push(Monkey {
        items: vec![98, 50, 76, 99],
        operation: |old: u64| old * old,
        test_divisor: 7,
        throw_true: 3,
        throw_false: 6,
        inspections: 0,
    });
    monkeys.push(Monkey {
        items: vec![72, 56, 94],
        operation: |old: u64| old + 1,
        test_divisor: 13,
        throw_true: 4,
        throw_false: 0,
        inspections: 0,
    });
    monkeys.push(Monkey {
        items: vec![55, 88, 90, 77, 60, 67],
        operation: |old: u64| old + 2,
        test_divisor: 3,
        throw_true: 6,
        throw_false: 5,
        inspections: 0,
    });
    monkeys.push(Monkey {
        items: vec![69, 72, 63, 60, 72, 52, 63, 78],
        operation: |old: u64| old * 13,
        test_divisor: 19,
        throw_true: 1,
        throw_false: 7,
        inspections: 0,
    });
    monkeys.push(Monkey {
        items: vec![89, 73],
        operation: |old: u64| old + 5,
        test_divisor: 17,
        throw_true: 2,
        throw_false: 0,
        inspections: 0,
    });
    monkeys.push(Monkey {
        items: vec![78, 68, 98, 88, 66],
        operation: |old: u64| old + 6,
        test_divisor: 11,
        throw_true: 2,
        throw_false: 5,
        inspections: 0,
    });
    monkeys.push(Monkey {
        items: vec![70],
        operation: |old: u64| old + 7,
        test_divisor: 5,
        throw_true: 1,
        throw_false: 3,
        inspections: 0,
    });
    return monkeys;
}
