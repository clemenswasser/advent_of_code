pub fn d02(input: &str) -> (u64, u64) {
    let part1 = input
        .lines()
        .map(|line| {
            let (opponent_str, me_str) = line.split_once(' ').unwrap();
            let opponent = opponent_str.chars().next().unwrap();
            let me = me_str.chars().next().unwrap();
            let me_score = match me {
                'X' => 1,
                'Y' => 2,
                'Z' => 3,
                _ => unreachable!(),
            };
            let round_score = match (opponent, me) {
                ('A', 'X') => 3,
                ('A', 'Y') => 6,
                ('A', 'Z') => 0,
                ('B', 'X') => 0,
                ('B', 'Y') => 3,
                ('B', 'Z') => 6,
                ('C', 'X') => 6,
                ('C', 'Y') => 0,
                ('C', 'Z') => 3,
                _ => unreachable!(),
            };
            me_score + round_score
        })
        .sum();

    let part2 = input
        .lines()
        .map(|line| {
            let (opponent_str, me_str) = line.split_once(' ').unwrap();
            let opponent = opponent_str.chars().next().unwrap();
            let me = me_str.chars().next().unwrap();
            let me_score = match (me, opponent) {
                ('X', 'A') => 3,
                ('X', 'B') => 1,
                ('X', 'C') => 2,
                ('Y', 'A') => 1,
                ('Y', 'B') => 2,
                ('Y', 'C') => 3,
                ('Z', 'A') => 2,
                ('Z', 'B') => 3,
                ('Z', 'C') => 1,
                _ => unreachable!(),
            };
            let round_score = match me {
                'X' => 0,
                'Y' => 3,
                'Z' => 6,
                _ => unreachable!(),
            };
            me_score + round_score
        })
        .sum();

    (part1, part2)
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!((15, 12), super::d02("A Y\nB X\nC Z"));
    }
}
