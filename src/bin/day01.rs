use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = File::open(filename)
        .expect("file not found.");

    let reader = BufReader::new(contents);

    let depths: Vec<u32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect();

    part_one(&depths);

    part_two(&depths);
}

fn part_one(depths: &Vec<u32>) {
    let increases = depths.windows(2).filter(|w| w[0] < w[1]).count();

    println!("{} measurements are larger than the previous measurement", increases);
}

fn part_two(depths: &Vec<u32>) {
    let mut sums = Vec::new();

    for window in depths.windows(3) {
        sums.push(window.iter().sum::<u32>());
    }

    let increases = sums.windows(2).filter(|w| w[0] < w[1]).count();

    println!("{} sums are larger than the previous sum", increases);
}
