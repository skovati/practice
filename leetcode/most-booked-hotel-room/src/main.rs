use std::env;
use std::collections::BTreeMap;

fn main() {
    let bookings: Vec<String> = env::args().skip(1).collect();
    println!("{}", find_max_booked(bookings));
}

fn find_max_booked(bookings: Vec<String>) -> String {
    let mut counts: BTreeMap<String, i32> = BTreeMap::new();
    for b in bookings {
        println!("{}", b);
        let mut chars = b.chars();
        let diff = chars.nth(0).unwrap();
        let room: String = chars.take(2).collect();
        if diff == '+' {
            *counts.entry(room).or_insert(0) += 1;
        }
    }
    let mut vec = counts
        .iter()
        .collect::<Vec<_>>();
    vec.sort_by(|a, b| a.1.cmp(b.1));
    return vec.last().unwrap().0.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input: Vec<String> = vec![
            "+1A", "+3E",
            "-1A", "+4F",
            "+1A", "-3E" ]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(find_max_booked(input), "1A");
    }
}
