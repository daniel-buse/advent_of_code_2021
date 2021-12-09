#![allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}
#[derive(Debug, Clone)]
struct Grid {
    fields: Vec<i32>,
    width: i32,
    height: i32,
}

impl Grid {
    fn new(width: i32, height: i32) -> Self {
        Grid {
            fields: vec![0; (width * height) as usize],
            width,
            height,
        }
    }

    fn index(&self, x: i32, y: i32) -> usize {
        (self.width * y + x) as usize
    }

    fn inc_field(&mut self, x: i32, y: i32) {
        let i = self.index(x, y);
        self.fields[i] += 1;
    }

    fn field(&self, x: i32, y: i32) -> i32 {
        self.fields[self.index(x, y)]
    }

    fn add_line(&mut self, start: &Point, end: &Point) {
        let x_diff = (start.x - end.x).abs();
        let y_diff = (start.y - end.y).abs();
        let mut x = start.x;
        let mut y = start.y;
        if x_diff == 0 {
            let y_dir = if start.y < end.y { 1 } else { -1 };
            for _ in 0..=y_diff {
                self.inc_field(x, y);
                y += y_dir;
            }
        } else if y_diff == 0 {
            let x_dir = if start.x < end.x { 1 } else { -1 };
            for _ in 0..=x_diff {
                self.inc_field(x, y);
                x += x_dir;
            }
        } else {
            // only "straight" diagonal
            assert!(x_diff == y_diff);
            let x_dir = if start.x < end.x { 1 } else { -1 };
            let y_dir = if start.y < end.y { 1 } else { -1 };
            for _ in 0..=x_diff {
                self.inc_field(x, y);
                x += x_dir;
                y += y_dir;
            }
        }
    }

    fn count_overlapped_fields(&self) -> i32 {
        self.fields.iter().filter(|f| **f >= 2).count() as i32
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{} ", self.field(x, y));
            }
            println!();
        }
    }
}

fn parse_input(input: &str) -> Vec<(Point, Point)> {
    input
        .lines()
        .map(|line| {
            // x1,y1 -> x2,y2
            let mut tokens = line.split_whitespace();
            let mut first = tokens.next().unwrap().split(',');
            let x1 = first.next().unwrap().parse().unwrap();
            let y1 = first.next().unwrap().parse().unwrap();
            tokens.next();
            let mut second = tokens.next().unwrap().split(',');
            let x2 = second.next().unwrap().parse().unwrap();
            let y2 = second.next().unwrap().parse().unwrap();
            (Point::new(x1, y1), Point::new(x2, y2))
        })
        .collect()
}

fn calc_width_height(lines: &[(Point, Point)]) -> (i32, i32) {
    let (mut width, mut height) = (0, 0);
    for (start, end) in lines.iter() {
        width = width.max(start.x);
        width = width.max(end.x);
        height = height.max(start.y);
        height = height.max(end.y);
    }
    (width + 1, height + 1)
}

fn part1(lines: &[(Point, Point)]) {
    let (width, height) = calc_width_height(lines);
    let mut grid = Grid::new(width, height);
    for (start, end) in lines.iter() {
        if start.x != end.x && start.y != end.y {
            continue;
        }
        grid.add_line(start, end);
    }
    println!("{}", grid.count_overlapped_fields());
}

fn part2(lines: &[(Point, Point)]) {
    let (width, height) = calc_width_height(lines);
    let mut grid = Grid::new(width, height);
    for (start, end) in lines.iter() {
        grid.add_line(start, end);
    }
    println!("{}", grid.count_overlapped_fields());
}

fn main() {
    let input = parse_input(include_str!("input.txt"));
    part1(&input);
    part2(&input);
}
