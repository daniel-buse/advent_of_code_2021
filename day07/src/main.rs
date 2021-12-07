fn part1(input: &[i32]) {
    let max = input.iter().max().copied().unwrap();
    let mut min_sum = i32::MAX;
    for pos in 0..max {
        let sum: i32 = input.iter().map(|val| (*val - pos).abs()).sum();
        min_sum = min_sum.min(sum);
    }
    println!("{}", min_sum);
}

fn fuel(steps: i32) -> i32 {
    (steps * (steps + 1)) / 2
}

fn part2(input: &[i32]) {
    let max = input.iter().max().copied().unwrap();
    let mut min_sum = i32::MAX;
    for pos in 0..max {
        let sum: i32 = input.iter().map(|val| fuel((*val - pos).abs())).sum();
        min_sum = min_sum.min(sum);
    }
    println!("{}", min_sum);
}

fn main() {
    let input: Vec<i32> = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    part1(&input);
    part2(&input);
}
