enum Direction {
    Forward,
    Up,
    Down,
}

fn part1(input: &[(Direction, i32)]) {
    let (mut vertical, mut depth) = (0, 0);
    for (d, v) in input {
        match d {
            Direction::Forward => vertical += v,
            Direction::Up => depth -= v,
            Direction::Down => depth += v,
        }
    }
    println!("{}", vertical * depth);
}

fn part2(input: &[(Direction, i32)]) {
    let (mut vertical, mut depth, mut aim) = (0, 0, 0);
    for (d, v) in input {
        match d {
            Direction::Forward => {
                vertical += v;
                depth += aim * v;
            }
            Direction::Up => aim -= v,
            Direction::Down => aim += v,
        }
    }
    println!("{}", vertical * depth);
}

fn main() {
    let input_str = include_str!("input.txt");
    let input = input_str
        .trim()
        .lines()
        .map(|s| {
            let mut t = s.split_whitespace();
            let d_t = t.next().unwrap();
            let v_t = t.next().unwrap();
            let d = if d_t == "forward" {
                Direction::Forward
            } else if d_t == "up" {
                Direction::Up
            } else if d_t == "down" {
                Direction::Down
            } else {
                panic!("Unexpected direction {}", d_t)
            };
            let v = v_t.parse::<i32>().unwrap();
            (d, v)
        })
        .collect::<Vec<(Direction, i32)>>();
    part1(&input);
    part2(&input);
}
