use argh::FromArgs;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

#[derive(FromArgs, PartialEq, Debug)]
/// Advent of Code 2021
struct Args {
    #[argh(subcommand)]
    nested: Option<Subcommands>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Subcommands {
    RunDay(RunDayArgs),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "day")]
/// Specify the DAY the problem was released
struct RunDayArgs {
    #[argh(positional)]
    /// the day [1..25]
    day: String,
}

fn main() {
    let args: Args = argh::from_env();
    let command = args.nested.unwrap_or_else(|| {
        println!("{}", "Something went wrong, try --help");
        std::process::exit(1);
    });
    match command {
        Subcommands::RunDay(subargs) => {
            let day = subargs.day.parse::<u32>().unwrap();
            match day {
                1 => {
                    day1::run();
                }
                2 => {
                    day2::run();
                }
                3 => {
                    day3::run();
                }
                4 => {
                    day4::run();
                }
                5 => {
                    day5::run();
                }
                6 => {
                    day6::run();
                }
                7 => {
                    day7::run();
                }
                8 => {
                    day8::run();
                }
                9 => {
                    day9::run();
                }
                10 => {
                    day10::run();
                }
                11 => {
                    day11::run();
                }
                12 => {
                    day12::run();
                }
                13 => {
                    day13::run();
                }
                // 14 => {}
                // 15 => {}
                // 16 => {}
                // 17 => {}
                // 18 => {}
                // 19 => {}
                // 20 => {}
                // 21 => {}
                // 22 => {}
                // 23 => {}
                // 24 => {}
                25 => {
                    day25::run();
                }
                _ => {
                    println!("Day {} is invalid/not solved yet", day);
                }
            }
        }
    }
}
