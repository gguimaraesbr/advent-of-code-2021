use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("Day 02 First Star Answer: {}", first_star("src/inputs/day02/puzzle.txt"));
    println!("Day 02 Second Star Answer: {}", second_star("src/inputs/day02/puzzle.txt"));
}

fn first_star(filename: &str) -> i32 {
    let contents = File::open(filename)
        .expect("file not found.");

    let reader = BufReader::new(contents);

    let steps: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;

    for step in steps {

        let chunks: Vec<_> = step.split_whitespace().collect();
        let direction = chunks[0];
        let units: i32 = chunks[1].parse::<i32>().unwrap();

        match direction {
            "forward" => horizontal += units,
            "down" => depth += units,
            "up" => depth -= units,
            _ => ()
        }
    }

    return horizontal * depth;
}

fn second_star(filename: &str) -> i32 {
    let contents = File::open(filename)
        .expect("file not found.");

    let reader = BufReader::new(contents);

    let steps: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    for step in steps {

        let chunks: Vec<_> = step.split_whitespace().collect();
        let direction = chunks[0];
        let units: i32 = chunks[1].parse::<i32>().unwrap();

        match direction {
            "forward" => {
                horizontal += units;
                depth += aim * units
            }
            "down" => aim += units,
            "up" => aim -= units,
            _ => ()
        }
    }

    return horizontal * depth;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_star_with_sample_data() {
        assert_eq!(first_star("src/inputs/day02/sample.txt"), 150);
    }

    #[test]
    fn first_star_with_puzzle_data() {
        assert_eq!(first_star("src/inputs/day02/puzzle.txt"), 1947824);
    }

    #[test]
    fn second_star_with_sample_data() {
        assert_eq!(second_star("src/inputs/day02/sample.txt"), 900);
    }

    #[test]
    fn second_star_with_puzzle_data() {
        assert_eq!(second_star("src/inputs/day02/puzzle.txt"), 1813062561);
    }
}
