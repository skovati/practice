use std::io;
use std::str;
use std::cmp;
use std::fs;

fn read_line<T>(path: &str) -> io::Result<Vec<T>>
where
    T: str::FromStr,
{
    Ok(fs::read_to_string(path)?
        .lines()
        .filter_map(|l| l.trim().parse::<T>().ok())
        .collect())
}

fn dec_win<T: cmp::PartialOrd>(v: &Vec<T>, size: usize) -> usize {
    v.windows(size)
    .filter(|w| w[0] < w[size - 1])
    .count()
}

fn main() {
    match read_line::<u32>("input/day01.txt") {
        Ok(v) => {
            println!("size 2: {}", dec_win(&v, 2));
            println!("size 4: {}", dec_win(&v, 4));
        },
        Err(e) => panic!("error reading file: {}", e)
    }
}
