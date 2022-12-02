pub fn d01(input: &str) -> (u64, u64) {
    let mut elves_calories: Vec<u64> = input
        .split("\n\n")
        .map(|elve_section| {
            elve_section
                .lines()
                .map(|line| line.parse::<u64>().unwrap())
                .sum()
        })
        .collect();
    elves_calories.select_nth_unstable_by(2, |a, b| b.cmp(a));
    (elves_calories[0], elves_calories[..3].into_iter().sum())
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(
            (24000, 24000 + 11000 + 10000),
            super::d01("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000")
        );
    }
}
