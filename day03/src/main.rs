fn part1(input: &[&str]) {
    let mut count_zeros = vec![0; 12];
    let mut count_ones = vec![0; 12];
    for num in input {
        for (i, c) in num.chars().enumerate() {
            if c == '0' {
                count_zeros[i] += 1;
            } else {
                count_ones[i] += 1;
            }
        }
    }
    let mut epsilon_rate = 0;
    let mut gamma_rate = 0;
    for i in 0..12 {
        assert!(count_zeros[i] != count_ones[i]);
        if count_ones[i] > count_zeros[i] {
            epsilon_rate += i32::pow(2, 12 - i as u32 - 1);
        } else {
            gamma_rate += i32::pow(2, 12 - i as u32 - 1);
        }
    }
    println!("{}", epsilon_rate * gamma_rate);
}

fn part2(input: &[&str]) {
    let mut input1 = input.to_vec();

    for i in 0..12 {
        let count_zeros = input1
            .iter()
            .filter(|val| val.chars().nth(i).unwrap() == '0')
            .count();
        let count_ones = input1
            .iter()
            .filter(|val| val.chars().nth(i).unwrap() == '1')
            .count();
        // keep val with the most common bit in pos, keep 1 in equal case
        let keep = if count_zeros > count_ones { '0' } else { '1' };
        input1.retain(|val| val.chars().nth(i).unwrap() == keep);
        if input1.len() == 1 {
            break;
        }
    }

    let mut input2 = input.to_vec();

    for i in 0..12 {
        let count_zeros = input2
            .iter()
            .filter(|val| val.chars().nth(i).unwrap() == '0')
            .count();
        let count_ones = input2
            .iter()
            .filter(|val| val.chars().nth(i).unwrap() == '1')
            .count();
        // keep val with the least common bit in pos, keep 0 in equal case
        let keep = if count_zeros <= count_ones { '0' } else { '1' };
        input2.retain(|val| val.chars().nth(i).unwrap() == keep);
        if input2.len() == 1 {
            break;
        }
    }

    assert!(input1.len() == 1);
    assert!(input2.len() == 1);

    let oxygen_generator_rating = i32::from_str_radix(input1[0], 2).unwrap();
    let co2_scrubber_rating = i32::from_str_radix(input2[0], 2).unwrap();
    println!("{}", oxygen_generator_rating * co2_scrubber_rating);
}

fn main() {
    let input_str = include_str!("input.txt");
    let input: Vec<&str> = input_str.trim().lines().collect();
    part1(&input);
    part2(&input);
}
