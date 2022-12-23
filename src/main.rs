use std::process;

mod day_01;
mod day_02;

fn main() {
    let mut args = std::env::args();
    let name = args.next().expect("program name should always be 0th args");

    let day = match args.next() {
        Some(day_string) => day_string,
        None => {
            eprintln!("Please specify the day to run (e.g. '{name} 01 ...').");
            process::exit(1);
        }
    };

    let result = match day.as_str() {
        "01" => day_01::run(args),
        "02" => day_02::run(args),
        _ => Err("unknown day"),
    };

    if let Err(msg) = result {
        eprintln!("{}", msg);
    }
}
