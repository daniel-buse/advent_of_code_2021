#[derive(Clone)]
struct BingoCardField {
    number: i32,
    marked: bool,
}

impl BingoCardField {
    fn new(number: i32) -> Self {
        BingoCardField {
            number,
            marked: false,
        }
    }
}

#[derive(Clone)]
struct BingoCard {
    fields: Vec<BingoCardField>,
}

impl BingoCard {
    fn new(fields: Vec<BingoCardField>) -> Self {
        // boards is 5x5, row major
        assert!(fields.len() == 25);
        BingoCard { fields }
    }

    fn mark_field(&mut self, number: i32) {
        for field in self.fields.iter_mut() {
            if field.number == number {
                field.marked = true;
            }
        }
    }

    fn has_won(&self) -> bool {
        let mut row_all_marked;
        for col in 0..5 {
            row_all_marked = true;
            for row in 0..5 {
                if !self.fields[5 * col + row].marked {
                    row_all_marked = false;
                    break;
                }
            }
            if row_all_marked {
                return true;
            }
        }

        let mut col_all_marked;
        for row in 0..5 {
            col_all_marked = true;
            for col in 0..5 {
                if !self.fields[5 * col + row].marked {
                    col_all_marked = false;
                    break;
                }
            }
            if col_all_marked {
                return true;
            }
        }
        false
    }

    fn unmarked_number(&self) -> i32 {
        self.fields
            .iter()
            .filter(|field| !field.marked)
            .map(|field| field.number)
            .sum()
    }
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<BingoCard>) {
    let mut lines = input.lines().peekable();
    // firt line is comma sep list of numbers
    let numbers = lines
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    // next is an empty line
    lines.next().unwrap();
    // next is block of 5 lines with field numbers, separated with an empty line
    let mut bingo_cards = Vec::new();
    loop {
        let mut fields = Vec::new();
        for _ in 0..5 {
            fields.extend(
                lines
                    .next()
                    .unwrap()
                    .trim()
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .map(BingoCardField::new),
            )
        }
        bingo_cards.push(BingoCard::new(fields));
        // remove empty line and exit if last line in input
        if lines.next().is_none() {
            break;
        }
    }
    (numbers, bingo_cards)
}

fn part1(numbers: &[i32], mut bingo_cards: Vec<BingoCard>) {
    for number in numbers {
        for bingo_card in bingo_cards.iter_mut() {
            bingo_card.mark_field(*number);
            if bingo_card.has_won() {
                println!("{}", number * bingo_card.unmarked_number());
                return;
            }
        }
    }
    println!("something went wrong");
}

fn part2(numbers: &[i32], mut bingo_cards: Vec<BingoCard>) {
    let mut win_count = 0;
    let bingo_cards_count = bingo_cards.len();
    for number in numbers {
        for bingo_card in bingo_cards.iter_mut() {
            if bingo_card.has_won() {
                continue;
            }
            bingo_card.mark_field(*number);
            if bingo_card.has_won() {
                win_count += 1;
                if win_count == bingo_cards_count {
                    println!("{}", number * bingo_card.unmarked_number());
                    return;
                }
            }
        }
    }
    println!("something went wrong");
}

fn main() {
    let (numbers, bingo_cards) = parse_input(include_str!("input.txt"));
    part1(&numbers, bingo_cards.clone());
    part2(&numbers, bingo_cards);
}
