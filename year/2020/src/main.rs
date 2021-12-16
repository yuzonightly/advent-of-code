use argh::FromArgs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

#[derive(FromArgs, PartialEq, Debug)]
/// Advent of Code 2020
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
                // 8 => {
                //     day8::run();
                // }
                // 9 => {
                //     day9::run();
                // }
                // 10 => {
                //     day10::run();
                // }
                // 11 => {
                //     day11::run();
                // }
                // 12 => {
                //     day12::run();
                // }
                // 13 => {
                //     day13::run();
                // }
                // 14 => {
                //     day14::run();
                // }
                // 15 => {
                //     day15::run();
                // }
                // 16 => {
                //     day16::run();
                // }
                // 17 => {
                // day17::run();
                // }
                // 18 => {
                // day18::run();
                // }
                // 19 => {day19::run();}
                // 20 => {day20::run();}
                // 21 => {day21::run();}
                // 22 => {day22::run();}
                // 23 => {day23::run();}
                // 24 => {day24::run();}
                // 25 => {
                //     day25::run();
                // }
                _ => {
                    println!("Day {} is invalid/not solved yet", day);
                }
            }
        }
    }
}
