pub fn parse(file: &str) -> Vec<u32> {
    vec![]
}

pub fn solve(values: Vec<u32>) -> u32 {
    values.iter().sum()
}

#[test]
fn part_one_example() {
    let input = "";
    let answer = 0;
    assert_eq!(solve(parse(input)), answer);
}
