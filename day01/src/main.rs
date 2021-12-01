fn part1(input: &[i32]) {
    let mut count = 0;
    let mut prev = input.first().unwrap();
    for val in input.iter().skip(1) {
        if val > prev {
            count += 1;
        }
        prev = val;
    }
    println!("Increases: {}", count);
}

fn window_val(window: &[i32]) -> i32 {
    assert!(window.len() == 3);
    window[0] + window[1] + window[2]
}

fn part2(input: &[i32]) {
    let mut count = 0;
    let mut windows_iter = input.windows(3);
    let mut prev = windows_iter.next().unwrap();
    for val in windows_iter {
        if window_val(val) > window_val(prev) {
            count += 1;
        }
        prev = val;
    }
    println!("Increases: {}", count);
}

fn main() {
    let input_str = include_str!("input.txt");
    let input = input_str
        .trim()
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<i32>, _>>()
        .unwrap();
    part1(&input);
    part2(&input);
}
