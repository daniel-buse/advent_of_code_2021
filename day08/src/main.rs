// 0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....
//
//   5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg

use std::collections::{BTreeSet, HashMap};

fn part1(input: Vec<Vec<&str>>) {
    // input:
    // 0..10 is signal_patern
    // 10 is sep
    // 11..15 is output_digit
    let mut count = 0;
    for display in input {
        let _signal_paterns = &display[0..10];
        let output_digits = &display[11..15];
        for output_digit in output_digits {
            if output_digit.len() == 2
                || output_digit.len() == 3
                || output_digit.len() == 4
                || output_digit.len() == 7
            {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

fn part2(input: Vec<Vec<&str>>) {
    // digit1 = len() == 2
    // digit7 = len() == 3
    // digit4 = len() == 4
    // digit8 = len() == 7
    // digit2 = len() == 5 && subtract(4) == 3
    // digit3 = len() == 5 && contains(7)
    // digit5 = len() == 5 && !contains(7) && subtract(4) == 2
    // digit0 = len() == 6 && !contains(4) && contains(1)
    // digit6 = len() == 6 && !cotains(1)
    // digit9 = len() == 6 && contains(4)
    let mut overall_sum = 0;
    for display in input {
        fn contains(a: &str, b: &str) -> bool {
            b.chars().all(|c| a.chars().any(|oc| oc == c))
        }

        fn subtract(a: &str, b: &str, len: usize) -> bool {
            a.chars().filter(|c| !b.chars().any(|oc| oc == *c)).count() == len
        }
        let mut lookup = HashMap::new();
        let signal_paterns = &display[0..10];
        let digit1 = signal_paterns.iter().find(|x| x.len() == 2).unwrap();
        let digit7 = signal_paterns.iter().find(|x| x.len() == 3).unwrap();
        let digit4 = signal_paterns.iter().find(|x| x.len() == 4).unwrap();
        let digit8 = signal_paterns.iter().find(|x| x.len() == 7).unwrap();
        let digit2 = signal_paterns
            .iter()
            .find(|x| x.len() == 5 && subtract(x, digit4, 3))
            .unwrap();
        let digit3 = signal_paterns
            .iter()
            .find(|x| x.len() == 5 && contains(x, digit7))
            .unwrap();
        let digit5 = signal_paterns
            .iter()
            .find(|x| x.len() == 5 && !contains(x, digit7) && subtract(x, digit4, 2))
            .unwrap();
        let digit0 = signal_paterns
            .iter()
            .find(|x| x.len() == 6 && !contains(x, digit4) && contains(x, digit7))
            .unwrap();
        let digit6 = signal_paterns
            .iter()
            .find(|x| x.len() == 6 && !contains(x, digit1))
            .unwrap();
        let digit9 = signal_paterns
            .iter()
            .find(|x| x.len() == 6 && contains(x, digit4))
            .unwrap();
        lookup.insert(
            digit0
                .chars()
                .collect::<BTreeSet<char>>()
                .into_iter()
                .collect::<String>(),
            0,
        );
        lookup.insert(
            digit1
                .chars()
                .collect::<BTreeSet<char>>()
                .into_iter()
                .collect::<String>(),
            1,
        );
        lookup.insert(
            digit2
                .chars()
                .collect::<BTreeSet<char>>()
                .into_iter()
                .collect::<String>(),
            2,
        );
        lookup.insert(
            digit3
                .chars()
                .collect::<BTreeSet<char>>()
                .into_iter()
                .collect::<String>(),
            3,
        );
        lookup.insert(
            digit4
                .chars()
                .collect::<BTreeSet<char>>()
                .into_iter()
                .collect::<String>(),
            4,
        );
        lookup.insert(
            digit5
                .chars()
                .collect::<BTreeSet<char>>()
                .into_iter()
                .collect::<String>(),
            5,
        );
        lookup.insert(
            digit6
                .chars()
                .collect::<BTreeSet<char>>()
                .into_iter()
                .collect::<String>(),
            6,
        );
        lookup.insert(
            digit7
                .chars()
                .collect::<BTreeSet<char>>()
                .into_iter()
                .collect::<String>(),
            7,
        );
        lookup.insert(
            digit8
                .chars()
                .collect::<BTreeSet<char>>()
                .into_iter()
                .collect::<String>(),
            8,
        );
        lookup.insert(
            digit9
                .chars()
                .collect::<BTreeSet<char>>()
                .into_iter()
                .collect::<String>(),
            9,
        );

        let mut sum = 0;
        for (i, digit) in (&display[11..15]).iter().enumerate() {
            let num = lookup[&digit
                .chars()
                .collect::<BTreeSet<char>>()
                .into_iter()
                .collect::<String>()];
            let exp = 3 - i as u32;
            sum += num * 10i32.pow(exp);
        }
        println!("{}", sum);
        overall_sum += sum;
    }
    println!("{}", overall_sum);
}

fn main() {
    let input: Vec<Vec<&str>> = include_str!("input.txt")
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();
    part1(input.clone());
    part2(input);
}
