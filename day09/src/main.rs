use std::collections::{HashSet, VecDeque};

fn part1(heightmap: Vec<u32>, width: usize, height: usize) {
    let mut sum_low_points = 0;
    for y in 0..height {
        for x in 0..width {
            let i = y * width + x;
            let val = heightmap[i];
            if x != 0 {
                if val >= heightmap[i - 1] {
                    continue;
                }
            }
            if x + 1 != width {
                if val >= heightmap[i + 1] {
                    continue;
                }
            }
            if y != 0 {
                if val >= heightmap[i - width] {
                    continue;
                }
            }
            if y + 1 != height {
                if val >= heightmap[i + width] {
                    continue;
                }
            }
            sum_low_points += val + 1;
        }
    }
    println!("{}", sum_low_points);
}

fn part2(heightmap: Vec<u32>, width: usize, height: usize) {
    let mut low_point_indices = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let i = y * width + x;
            let val = heightmap[i];
            if x != 0 {
                if val >= heightmap[i - 1] {
                    continue;
                }
            }
            if x + 1 != width {
                if val >= heightmap[i + 1] {
                    continue;
                }
            }
            if y != 0 {
                if val >= heightmap[i - width] {
                    continue;
                }
            }
            if y + 1 != height {
                if val >= heightmap[i + width] {
                    continue;
                }
            }
            low_point_indices.push(i);
        }
    }
    let mut basin_sizes = Vec::new();
    for low_point_index in low_point_indices {
        let mut indices = VecDeque::new();
        indices.push_back(low_point_index);
        let mut basin_size = 1;
        let mut seen = HashSet::new();
        while !indices.is_empty() {
            let i = indices.pop_front().unwrap();
            let x = i % width;
            let y = i / width;
            let val = heightmap[i];
            if x != 0 {
                let ni = i - 1;
                let nval = heightmap[ni];
                if val < nval && nval < 9 && !seen.contains(&ni) {
                    indices.push_back(ni);
                    seen.insert(ni);
                    basin_size += 1;
                }
            }
            if x + 1 != width {
                let ni = i + 1;
                let nval = heightmap[ni];
                if val < nval && nval < 9 && !seen.contains(&ni) {
                    indices.push_back(ni);
                    seen.insert(ni);
                    basin_size += 1;
                }
            }
            if y != 0 {
                let ni = i - width;
                let nval = heightmap[ni];
                if val < nval && nval < 9 && !seen.contains(&ni) {
                    indices.push_back(ni);
                    seen.insert(ni);
                    basin_size += 1;
                }
            }
            if y + 1 != height {
                let ni = i + width;
                let nval = heightmap[ni];
                if val < nval && nval < 9 && !seen.contains(&ni) {
                    indices.push_back(ni);
                    seen.insert(ni);
                    basin_size += 1;
                }
            }
        }
        basin_sizes.push(basin_size);
    }
    basin_sizes.sort();
    basin_sizes.reverse();
    println!("{}", basin_sizes.iter().take(3).product::<i32>());
}

fn main() {
    let input: Vec<u32> = include_str!("input.txt")
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    part1(input.clone(), 100, 100);
    part2(input.clone(), 100, 100);
}
