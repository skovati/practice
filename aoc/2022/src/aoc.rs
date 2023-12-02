use std::fs;
use std::io;
use std::str;

pub fn read_line<T>(path: &str) -> io::Result<Vec<T>>
where T: str::FromStr {
    Ok(fs::read_to_string(path)?
        .lines()
        .filter_map(|l| l.trim().parse::<T>().ok())
        .collect())
}

pub fn read_chunks<T>(path: &str) -> io::Result<Vec<Vec<T>>>
where T: str::FromStr {
    Ok(fs::read_to_string(path)?
        .split("\n\n")
        .map(|g|
            g.split("\n")
            .filter_map(|e| e.parse::<T>().ok())
            .collect::<Vec<T>>())
        .collect::<Vec<Vec<T>>>())
}
