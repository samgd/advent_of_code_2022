use regex;
use std::io::{self, BufRead};

#[derive(Clone, Copy)]
enum Item {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy)]
enum UnknownItem {
    X,
    Y,
    Z,
}

struct UnknownRound {
    us: UnknownItem,
    them: Item,
}

struct Round {
    us: Item,
    them: Item,
}

impl Round {
    fn score(&self) -> u32 {
        let item_score = match self.us {
            Item::Rock => 1,
            Item::Paper => 2,
            Item::Scissors => 3,
        };

        let round_score = match (&self.us, &self.them) {
            (Item::Rock, Item::Rock) => 3,
            (Item::Rock, Item::Paper) => 0,
            (Item::Rock, Item::Scissors) => 6,
            (Item::Paper, Item::Rock) => 6,
            (Item::Paper, Item::Paper) => 3,
            (Item::Paper, Item::Scissors) => 0,
            (Item::Scissors, Item::Rock) => 0,
            (Item::Scissors, Item::Paper) => 6,
            (Item::Scissors, Item::Scissors) => 3,
        };

        item_score + round_score
    }
}

pub fn run(mut args: impl Iterator<Item = String>) -> Result<(), &'static str> {
    let path = match args.next() {
        Some(p) => p,
        None => return Err("missing filepath to input data"),
    };

    let file = std::fs::File::open(path).expect("failed to open file");
    let file = io::BufReader::new(file);

    let unk_rounds: Vec<UnknownRound> = file
        .lines()
        .map(|l| l.expect("failed to load line in file"))
        .map(parse_str)
        .map(|x| x.expect("failed to parse line in file"))
        .collect();

    let p1_score: u32 = unk_rounds
        .iter()
        .map(|ur| Round {
            them: ur.them,
            us: match ur.us {
                UnknownItem::X => Item::Rock,
                UnknownItem::Y => Item::Paper,
                UnknownItem::Z => Item::Scissors,
            },
        })
        .map(|r| r.score())
        .sum();
    println!("Part 1 score: {}", p1_score);

    let p2_score: u32 = unk_rounds
        .iter()
        .map(|ur| Round {
            them: ur.them,
            us: match (ur.them, ur.us) {
                (Item::Rock, UnknownItem::X) => Item::Scissors,
                (Item::Rock, UnknownItem::Z) => Item::Paper,
                (Item::Paper, UnknownItem::X) => Item::Rock,
                (Item::Paper, UnknownItem::Z) => Item::Scissors,
                (Item::Scissors, UnknownItem::X) => Item::Paper,
                (Item::Scissors, UnknownItem::Z) => Item::Rock,
                (item, UnknownItem::Y) => item,
            },
        })
        .map(|r| r.score())
        .sum();
    println!("Part 2 score: {}", p2_score);

    Ok(())
}

fn parse_str(round_str: String) -> Result<UnknownRound, &'static str> {
    let re = regex::Regex::new("([ABC]) ([XYZ])").expect("regex should parse!");
    let caps = match re.captures(round_str.as_str()) {
        Some(caps) => caps,
        None => return Err("failed to parse line"),
    };
    let them = match &caps[1] {
        "A" => Item::Rock,
        "B" => Item::Paper,
        "C" => Item::Scissors,
        _ => return Err("failed to parse them"),
    };
    let us = match &caps[2] {
        "X" => UnknownItem::X,
        "Y" => UnknownItem::Y,
        "Z" => UnknownItem::Z,
        _ => return Err("failed to parse us"),
    };
    Ok(UnknownRound { us, them })
}
