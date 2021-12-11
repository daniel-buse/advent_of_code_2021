fn neighbors(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    if x != 0 && y != 0 {
        result.push((x - 1, y - 1));
    }
    if x != 0 {
        result.push((x - 1, y));
    }
    if x != 0 && y < 9 {
        result.push((x - 1, y + 1));
    }
    if y != 0 {
        result.push((x, y - 1));
    }
    if y < 9 {
        result.push((x, y + 1));
    }
    if x < 9 && y != 0 {
        result.push((x + 1, y - 1));
    }
    if x < 9 {
        result.push((x + 1, y));
    }
    if x < 9 && y < 9 {
        result.push((x + 1, y + 1));
    }
    result
}

fn part1(mut input: Vec<i8>) {
    let mut flashes = 0;
    for _step in 0..100 {
        for y in 0..10 {
            for x in 0..10 {
                let i = y * 10 + x;
                input[i] += 1;
            }
        }
        loop {
            let prev = flashes;

            for y in 0..10 {
                for x in 0..10 {
                    let i = y * 10 + x;
                    if input[i] > 9 {
                        for (xx, yy) in neighbors(x, y) {
                            let j = yy * 10 + xx;
                            // ignore already flashed ones
                            if input[j] != 0 {
                                input[j] += 1;
                            }
                        }
                        flashes += 1;
                        input[i] = 0;
                    }
                }
            }
            if prev == flashes {
                break;
            }
        }
    }
    println!("{}", flashes);
}

fn part2(mut input: Vec<i8>) {
    let mut result = 0;
    let mut flashes = 0;
    for step in 0..999 {
        for y in 0..10 {
            for x in 0..10 {
                let i = y * 10 + x;
                input[i] += 1;
            }
        }
        loop {
            let prev = flashes;

            for y in 0..10 {
                for x in 0..10 {
                    let i = y * 10 + x;
                    if input[i] > 9 {
                        for (xx, yy) in neighbors(x, y) {
                            let j = yy * 10 + xx;
                            // ignore already flashed ones
                            if input[j] != 0 {
                                input[j] += 1;
                            }
                        }
                        flashes += 1;
                        input[i] = 0;
                    }
                }
            }
            if prev == flashes {
                break;
            }
        }
        if input.iter().filter(|v| **v == 0).count() == 100 {
            result = step + 1;
            break;
        }
    }
    println!("{}", result);
}

fn main() {
    let input: Vec<i8> = include_str!("input.txt")
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10).unwrap() as i8)
        .collect();
    part1(input.clone());
    part2(input);
}
