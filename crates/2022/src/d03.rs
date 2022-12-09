use std::{collections::HashSet, hash::Hash};

fn item_type_priority(item_type: &char) -> u64 {
    let item_type_priority = if ('a'..='z').contains(&item_type) {
        *item_type as u8 - b'a' + 1
    } else if ('A'..='Z').contains(&item_type) {
        *item_type as u8 - b'A' + 27
    } else {
        unreachable!()
    };
    item_type_priority as u64
}

pub fn d03(input: &str) -> (u64, u64) {
    let part1 = input
        .lines()
        .map(|line| {
            let (comp1, comp2) = line.split_at(line.len() / 2);
            let comp1: HashSet<_> = comp1.chars().collect();
            let comp2: HashSet<_> = comp2.chars().collect();
            comp1
                .intersection(&comp2)
                .map(item_type_priority)
                .sum::<u64>()
        })
        .sum::<u64>();

    let lines: Vec<_> = input.lines().collect();
    let part2 = lines
        .chunks_exact(3)
        .map(|group| {
            group
                .into_iter()
                .map(|group| group.chars().collect::<HashSet<char>>())
                .reduce(|a, b| a.intersection(&b).copied().collect())
                .map(|shared_item_types| {
                    shared_item_types
                        .iter()
                        .map(item_type_priority)
                        .sum::<u64>()
                })
                .unwrap()
        })
        .sum();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!((157, 70), super::d03("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw"));
    }
}
