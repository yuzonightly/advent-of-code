use std::path::Path;
use std::process::Command;

use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Advent of Code
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
#[argh(subcommand, name = "puzzle")]
/// Specify the YEAR and DAY the problem was released,
/// in that sequence
struct RunDayArgs {
    #[argh(positional)]
    /// the year [2015..2021]
    year: String,
    #[argh(positional)]
    /// the day [1..25]
    day: String,
}

// Returns the value for --manifest-path
fn find_manifest_path(year: &String) -> String {
    format!("./year/{}/Cargo.toml", year)
}

fn main() {
    let args: Args = argh::from_env();
    let command = args.nested.unwrap_or_else(|| {
        println!("{}", "Something went wrong, try --help");
        std::process::exit(1);
    });
    match command {
        Subcommands::RunDay(subargs) => {
            let manifest_path = find_manifest_path(&subargs.year);
            if !Path::new(&manifest_path).exists() {
                println!("Path {} is invalid", manifest_path);
                std::process::exit(1);
            }
            let cmd = Command::new("cargo")
                .arg("run")
                .args(&["--manifest-path", &manifest_path])
                .args(&["--color", "always"])
                .args(&["--", "day", &subargs.day])
                .output()
                .expect("Failed to execute 'run'");
            if cmd.status.success() {
                println!("{}", String::from_utf8_lossy(&cmd.stdout).to_string());
            } else {
                println!("{}", String::from_utf8_lossy(&cmd.stderr).to_string());
            }
        }
    }
}
