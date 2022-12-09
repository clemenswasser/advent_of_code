fn count_sections_by(sections: &mut Vec<(u64, u64)>, pred: fn((u64, u64), u64) -> bool) -> u64 {
    sections.sort_unstable_by(|(_, end1), (_, end2)| end2.cmp(end1));
    sections.sort_by_key(|(begin, _)| *begin);
    let mut contained_sections = 0;
    let mut max_end = 0;
    for &mut (begin, end) in sections {
        if pred((begin, end), max_end) {
            contained_sections += 1;
        }

        if end > max_end {
            max_end = end;
        }
    }

    contained_sections
}

fn count_contained_sections(sections: &mut Vec<(u64, u64)>) -> u64 {
    count_sections_by(sections, |(_, end), max_end| max_end >= end)
}

fn count_overlapping_sections(sections: &mut Vec<(u64, u64)>) -> u64 {
    count_sections_by(sections, |(begin, _), max_end| max_end >= begin)
}

fn parse_and_count_sections_by(input: &str, pred: fn(&mut Vec<(u64, u64)>) -> u64) -> u64 {
    input
        .lines()
        .map(|line| {
            let mut sections: Vec<_> = line
                .split(',')
                .map(|section| {
                    let (begin, end) = section.split_once('-').unwrap();
                    (begin.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
                })
                .collect();

            pred(&mut sections)
        })
        .sum()
}

pub fn d04(input: &str) -> (u64, u64) {
    let contained_sections = parse_and_count_sections_by(input, count_contained_sections);
    let overlapping_sections = parse_and_count_sections_by(input, count_overlapping_sections);

    (contained_sections, overlapping_sections)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(
            (2, 4),
            super::d04("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8")
        );
    }
}
