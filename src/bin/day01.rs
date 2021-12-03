use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("Day 01 First Star Answer: {}", first_star("src/inputs/day01/puzzle.txt"));
    println!("Day 01 Second Star Answer: {}", second_star("src/inputs/day01/puzzle.txt"));
}

fn first_star(filename: &str) -> u32 {
    let contents = File::open(filename)
        .expect("file not found.");

    let reader = BufReader::new(contents);

    let depths: Vec<u32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect();

    return depths.windows(2).filter(|w| w[0] < w[1]).count().try_into().unwrap();
}

fn second_star(filename: &str) -> u32 {
    let contents = File::open(filename)
        .expect("file not found.");

    let reader = BufReader::new(contents);

    let depths: Vec<u32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect();

    let mut sums = Vec::new();

    for window in depths.windows(3) {
        sums.push(window.iter().sum::<u32>());
    }

    return sums.windows(2).filter(|w| w[0] < w[1]).count().try_into().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_star_with_sample_data() {
        assert_eq!(first_star("src/inputs/day01/sample.txt"), 7);
    }

    #[test]
    fn first_star_with_puzzle_data() {
        assert_eq!(first_star("src/inputs/day01/puzzle.txt"), 1167);
    }

    #[test]
    fn second_star_with_sample_data() {
        assert_eq!(second_star("src/inputs/day01/sample.txt"), 5);
    }

    #[test]
    fn second_star_with_puzzle_data() {
        assert_eq!(second_star("src/inputs/day01/puzzle.txt"), 1130);
    }
}
