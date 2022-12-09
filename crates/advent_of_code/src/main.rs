use std::io::Read;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(ValueEnum, Clone)]
enum Day {
    D01,
    D02,
    D03,
    D04,
    D05,
    D06,
    D07,
    D08,
    D09,
    D10,
    D11,
    D12,
    D13,
    D14,
    D15,
    D16,
    D17,
    D18,
    D19,
    D20,
    D21,
    D22,
    D23,
    D24,
}

#[derive(Args)]
struct DayArgs {
    day: Day,
}

#[derive(Subcommand)]
enum Commands {
    Y2021(DayArgs),
    Y2022(DayArgs),
}

fn main() {
    let cli = Cli::parse();
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let input = input.replace("\r\n", "\n");
    match &cli.command {
        Commands::Y2021(DayArgs { day }) => match day {
            Day::D01 => println!(
                "there are {} measurements that are larger than the previous measurement",
                advent_of_code_2021::d01::d01(&input)
            ),
            Day::D02 => advent_of_code_2021::d02::d02(&input),
            Day::D03 => advent_of_code_2021::d03::d03(&input),
            _ => todo!(),
        },
        Commands::Y2022(DayArgs { day }) => match day {
            Day::D01 => {
                let (part1, part2) = advent_of_code_2022::d01::d01(&input);
                println!("The most carried Calories by a single Elf are {part1}");
                println!("The most carried Calories by a three Elves are {part2}");
            }
            Day::D02 => {
                let (part1, part2) = advent_of_code_2022::d02::d02(&input);
                println!("The encrypted strategy guide score is {part1}");
                println!("The decrypted strategy guide score is {part2}");
            }
            Day::D03 => {
                let (part1, part2) = advent_of_code_2022::d03::d03(&input);
                println!("The item priority sum is {part1}");
                println!("The item priority sum for groups of 3 is {part2}");
            }
            Day::D04 => {
                let (part1, part2) = advent_of_code_2022::d04::d04(&input);
                println!("The count of contained sections is {part1}");
                println!("The count of overlapping sections is {part2}");
            }
            Day::D05 => {
                let (part1, part2) = advent_of_code_2022::d05::d05(&input);
                println!("The resulting crate positions are {part1}");
                println!("The resulting ordered crate positions are {part2}");
            }
            Day::D06 => {
                let (part1, part2) = advent_of_code_2022::d06::d06(&input);
                println!("The marker for 4 unique characters was detected at {part1}");
                println!("The marker for 14 unique characters was detected at {part2}");
            }
            Day::D07 => {
                let (part1, part2) = advent_of_code_2022::d07::d07(&input);
                println!("The sum of all directories with a size of at most 100000 is {part1}");
                println!("The directory to free has a size of {part2}");
            }
            _ => todo!(),
        },
    }
}
