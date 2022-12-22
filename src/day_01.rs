use std::io::{self, BufRead};

pub fn run(mut args: impl Iterator<Item = String>) -> Result<(), &'static str> {
    let path = match args.next() {
        Some(p) => p,
        None => return Err("missing filepath to input data"),
    };

    let file = std::fs::File::open(path).expect("failed to open file");
    let file = io::BufReader::new(file);

    let mut calories = parse(
        file.lines()
            .map(|l| l.expect("failed to load line in file")),
    )?;

    if calories.is_empty() {
        return Err("failed to find any calories in file");
    }
    calories.sort();
    calories.reverse();

    println!(
        "elf with most calories: {}",
        calories.first().expect("length must be >= 1")
    );

    if calories.len() < 3 {
        return Err("fewer than 3 calories in file");
    }

    let top_three: u64 = calories.iter().take(3).sum();
    println!("total calories for top three: {}", top_three);

    Ok(())
}

fn parse(lines: impl Iterator<Item = String>) -> Result<Vec<u64>, &'static str> {
    let mut calories: Vec<u64> = Vec::new();
    let mut myb_total: Option<u64> = None;

    for line in lines {
        if line.is_empty() {
            if let Some(total) = myb_total {
                calories.push(total);
                myb_total = None
            }
            continue;
        }

        let calorie: u64 = line.parse().expect("failed to parse int");
        myb_total = Some(myb_total.unwrap_or(0) + calorie);
    }

    if let Some(total) = myb_total {
        calories.push(total);
    }

    Ok(calories)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::iter;

    const EXAMPLE_STR: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn parse_empty() {
        assert_eq!(parse(iter::empty()), Ok(vec![]));
    }

    #[test]
    fn parse_example() {
        let example_iter = EXAMPLE_STR.split("\n").map(String::from).into_iter();
        let expected = vec![6000, 4000, 11000, 24000, 10000];
        assert_eq!(parse(example_iter), Ok(expected));
    }
}
