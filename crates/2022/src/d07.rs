use std::mem;

fn parse_runs(input: &str) -> Vec<Vec<&str>> {
    let mut commands = Vec::new();
    let mut command = Vec::new();

    for line in input.lines() {
        if line.starts_with("$ ") && !command.is_empty() {
            commands.push(mem::take(&mut command));
        }

        command.push(line);
    }

    if !command.is_empty() {
        commands.push(mem::take(&mut command));
    }

    commands
}

pub fn d07(input: &str) -> (u64, u64) {
    //let fs_map: HashMap<&str, FSEntry> = HashMap::new();
    let runs = parse_runs(input);
    let mut path = String::new();
    let mut sizes = Vec::new();
    let mut part1 = 0;
    let mut sizes_history = Vec::new();

    for run in runs {
        let command = run[0].strip_prefix("$ ").unwrap();
        if command.starts_with("cd ") {
            let dir_name = command.strip_prefix("cd ").unwrap();
            match dir_name {
                ".." => {
                    path.drain(path[..path.len() - 1].rfind('/').unwrap() + 1..);
                    let dir_size = sizes.pop().unwrap();
                    sizes_history.push(dir_size);
                    if dir_size <= 100_000 {
                        part1 += dir_size;
                    }
                    if let Some(last) = sizes.last_mut() {
                        *last += dir_size;
                    }
                }
                "/" => path += "/",
                _ => {
                    path += dir_name;
                    path += "/";
                }
            }
        } else if command == "ls" {
            let dir_size = run
                .iter()
                .skip(1)
                .flat_map(|fs_entry| {
                    fs_entry
                        .split_ascii_whitespace()
                        .next()
                        .and_then(|num_str| num_str.parse::<u64>().ok())
                })
                .sum::<u64>();
            sizes.push(dir_size);
        } else {
            unreachable!()
        }
    }

    let mut total_size = 0;
    for size in sizes.iter().rev() {
        total_size += size;
        sizes_history.push(total_size);
    }
    let size_left = 70_000_000 - total_size;
    let size_to_free = 30_000_000 - size_left;

    sizes_history.sort_unstable();
    let lowest_free = match sizes_history.binary_search(&size_to_free) {
        Ok(_) => size_to_free,
        Err(idx) => *sizes_history.get(idx).unwrap_or(&total_size),
    };

    (part1, lowest_free)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!((95_437, 24_933_642), super::d07("$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k"));
    }
}
