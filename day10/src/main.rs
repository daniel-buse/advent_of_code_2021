fn is_opening(c: char) -> bool {
    c == '<' || c == '{' || c == '(' || c == '['
}

fn is_matching(opening: char, closing: char) -> bool {
    match opening {
        '(' => closing == ')',
        '[' => closing == ']',
        '{' => closing == '}',
        '<' => closing == '>',
        _ => unreachable!(),
    }
}

fn score(c: char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn part1(input: &[&str]) {
    let mut final_score = 0;
    for line in input {
        let mut scratch = Vec::new();
        for c in line.chars() {
            if is_opening(c) {
                scratch.push(c)
            } else if !is_matching(scratch.pop().unwrap(), c) {
                final_score += score(c);
                break;
            }
        }
    }
    println!("{}", final_score)
}

fn score2(c: char) -> i64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!(),
    }
}

fn closing_to(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

fn part2(input: &[&str]) {
    let mut total_scores = Vec::new();
    for mut scratch in input.iter().filter_map(|line| {
        let mut scratch = Vec::new();
        for c in line.chars() {
            if is_opening(c) {
                scratch.push(c)
            } else if !is_matching(scratch.pop().unwrap(), c) {
                return None;
            }
        }
        return Some(scratch);
    }) {
        let mut line_total_score = 0;
        scratch.reverse();
        for c in scratch {
            line_total_score = line_total_score * 5 + score2(closing_to(c));
        }
        total_scores.push(line_total_score)
    }
    total_scores.sort();
    println!("{}", total_scores[total_scores.len() / 2])
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();
    part1(&input);
    part2(&input);
}
