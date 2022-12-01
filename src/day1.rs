use std::collections::BinaryHeap;

pub fn generate(input: &str) -> BinaryHeap<i64> {
    input
        .split("\n\n")
        .map(|s| s.lines().filter_map(|n| n.parse::<i64>().ok()).sum())
        .collect()
}

pub fn part1(input: &BinaryHeap<i64>) -> i64 {
    input.iter().cloned().next().unwrap_or_default()
}

pub fn part2(input: &BinaryHeap<i64>) -> i64 {
    input.iter().take(3).sum()
}

#[cfg(test)]
mod test {
    static INPUT: &'static str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;
    #[test]
    fn test_day1() {
        assert_eq!(super::part1(&super::generate(INPUT)), 24000);
    }

    #[test]
    fn test_day2() {
        assert_eq!(super::part2(&super::generate(INPUT)), 45000);
    }
}
