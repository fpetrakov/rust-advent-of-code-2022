use rust_advent_of_code_2022::get_input_from_file;
use std::str::FromStr;

#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn beats(self, other: Shape) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper)
        )
    }
}

impl TryFrom<char> for Shape {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' | 'X' => Ok(Shape::Rock),
            'B' | 'Y' => Ok(Shape::Paper),
            'C' | 'Z' => Ok(Shape::Scissors),
            _ => Err("Unknown shape!"),
        }
    }
}

impl From<Shape> for u32 {
    fn from(shape: Shape) -> Self {
        match shape {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

struct Round {
    player: Shape,
    opponent: Shape,
}

impl FromStr for Round {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let opponent = chars.next().unwrap();
        chars.next();
        let player = chars.next().unwrap();

        Ok(Self {
            player: player.try_into()?,
            opponent: opponent.try_into()?,
        })
    }
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl From<Round> for Outcome {
    fn from(round: Round) -> Self {
        let p = round.player;
        let o = round.opponent;

        if p.beats(o) {
            Outcome::Win
        } else if o.beats(p) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }
}

impl From<Outcome> for u32 {
    fn from(outcome: Outcome) -> Self {
        match outcome {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

fn main() {
    let input = get_input_from_file("day_2-part_1.txt");
    let result: u32 = input
        .lines()
        .map(|line| {
            let round: Round = line.parse().unwrap();
            let player_score: u32 = round.player.into();
            let outcome: Outcome = round.into();
            let outcome_score: u32 = outcome.into();
            player_score + outcome_score
        })
        .sum();

    println!("{}", result);
}
