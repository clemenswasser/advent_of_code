pub fn d01(input: &str) -> usize {
    let measurements: Vec<u64> = input.lines().map(str::parse).flatten().collect();
    measurements
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}

#[cfg(test)]
mod test {
    use super::d01;

    #[test]
    fn example() {
        assert_eq!(7, d01("199\n200\n208\n210\n200\n207\n240\n269\n260\n263"));
    }
}
