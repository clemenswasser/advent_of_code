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
            _ => todo!(),
        },
    }
}
