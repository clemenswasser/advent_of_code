fn parse_crates(crates: &[&str]) -> Vec<Vec<char>> {
    let (stack_descriptions, stack_lines) = crates.split_last().unwrap();
    let stack_count = (stack_descriptions.len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = Vec::new();
    stacks.resize(stack_count, Vec::new());
    for stack_line in stack_lines.iter().rev() {
        for (i, stack) in stacks.iter_mut().enumerate() {
            let item = stack_line.chars().nth(i * 4 + 1).unwrap();
            if item != ' ' {
                stack.push(item);
            }
        }
    }

    stacks
}

fn simulate_crane(
    stacks: &mut Vec<Vec<char>>,
    moves: &[&str],
    transfer: fn(stacks: &mut Vec<Vec<char>>, from: usize, to: usize, count: usize),
) {
    for line in moves {
        let instrs: Vec<_> = line.split_ascii_whitespace().collect();
        let count: usize = instrs[1].parse().unwrap();
        let from = instrs[3].parse::<usize>().unwrap() - 1;
        let to = instrs[5].parse::<usize>().unwrap() - 1;
        transfer(stacks, from, to, count);
    }
}

fn transfer_crates(stacks: &mut Vec<Vec<char>>, from: usize, to: usize, count: usize) {
    for _ in 0..count {
        let val = stacks[from].pop().unwrap();
        stacks[to].push(val);
    }
}

fn transfer_crates_ordered(stacks: &mut Vec<Vec<char>>, from: usize, to: usize, count: usize) {
    let count_idx = stacks[from].len() - count;
    let (_, crates) = stacks[from].split_at(count_idx);
    let crates = crates.to_owned();
    stacks[to].extend(crates);
    stacks[from].drain(count_idx..);
}

pub fn d05(input: &str) -> (String, String) {
    let lines: Vec<_> = input.lines().collect();
    let separator_line = lines.iter().position(|line| line.is_empty()).unwrap();
    let (crates, moves) = lines.split_at(separator_line);
    let moves = &moves[1..];
    let mut stacks = parse_crates(crates);
    let mut unordered_stacks = stacks.clone();
    simulate_crane(&mut unordered_stacks, moves, transfer_crates);
    let part1: String = unordered_stacks
        .into_iter()
        .flat_map(|stack| stack.last().copied())
        .collect();

    simulate_crane(&mut stacks, moves, transfer_crates_ordered);
    let part2: String = stacks
        .into_iter()
        .flat_map(|stack| stack.last().copied())
        .collect();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(
            ("CMZ".to_owned(), "MCD".to_owned()),
            super::d05("    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2")
        );
    }
}
