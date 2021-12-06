fn part1(mut lantern_fish: Vec<u8>) {
    for _day in 0..80 {
        let mut new_fish_count = 0;
        for fish in lantern_fish.iter_mut() {
            if *fish == 0 {
                *fish = 6;
                new_fish_count += 1;
            } else {
                *fish -= 1;
            }
        }
        lantern_fish.resize(lantern_fish.len() + new_fish_count, 8);
    }
    println!("{}", lantern_fish.len());
}

fn part2(lantern_fish: Vec<u8>) {
    let mut fish_counts: Vec<usize> = vec![0; 9];
    for fish in lantern_fish {
        fish_counts[fish as usize] += 1;
    }
    for _day in 0..256 {
        let new_fish_count = fish_counts[0];
        for i in 1..=8 {
            fish_counts[i - 1] = fish_counts[i];
        }
        fish_counts[6] += new_fish_count;
        fish_counts[8] = new_fish_count;
    }
    println!("{}", fish_counts.iter().sum::<usize>());
}

fn main() {
    let input: Vec<u8> = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    part1(input.clone());
    part2(input.clone());
}
