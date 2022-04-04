use itertools::izip;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, Error};

fn input(year: u32, day: u32) -> io::Result<String> {
    let mut file = File::open(format!("../inputs/{}/day{}.txt", year, day))?;
    let mut inp = String::new();
    file.read_to_string(&mut inp)?;

    Ok(inp)
}

fn main() {
    let depths: Vec<i32> = input(2021, 1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let resOne = depths[..]
        .iter()
        .zip(depths[1..].iter())
        .filter(|(p, n)| p < n)
        .count();

    println!("Part1 : {}", resOne);

    let resTwo = izip!(
        depths[..].iter(),
        depths[1..].iter(),
        depths[2..].iter(),
        depths[3..].iter()
    )
    .filter(|(a, b, c, d)| (*a + *b + *c) < (*b + *c + *d))
    .count();

    println!("Part2 : {}", resTwo);
}
