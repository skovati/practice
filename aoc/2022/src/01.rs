
mod aoc;

fn main() {
    println!("part 1:");
    let lines = aoc::read_chunks::<u32>("inputs/01.txt").expect("error reading file");
    let max = lines
        .iter()
        .map(|g| g.iter().sum::<u32>())
        .max();
    match max {
        Some(m) => println!("max: {m}"),
        None => panic!("error")
    }

    println!("part 2:");
    let lines = aoc::read_chunks::<u32>("inputs/01.txt").expect("error reading file");
    let mut sums = lines
        .iter()
        .map(|g| g.iter().sum::<u32>())
        .collect::<Vec<u32>>();

    sums.sort_by(|a, b| b.cmp(a));

    let top_three = sums.iter().take(3).sum::<u32>();
    println!("top three sum: {}", top_three);
}
