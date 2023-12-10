#![warn(clippy::all)]

extern crate nom;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult, ParseTo,
};

use std::time::Instant;

#[derive(Default, Debug)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Default, Debug)]
struct Game {
    id: u32,
    selections: Vec<CubeSet>,
}

fn parse_cubeset(input: &[u8]) -> IResult<&[u8], CubeSet> {
    let (rest, val) = separated_list1(
        tag(", "),
        tuple((
            digit1,
            tag(" "),
            alt((tag("red"), tag("blue"), tag("green"))),
        )),
    )(input)?;
    let mut cs: CubeSet = CubeSet::default();
    for v in val {
        match v.2[0] {
            b'r' => cs.red = v.0.parse_to().unwrap(),
            b'g' => cs.green = v.0.parse_to().unwrap(),
            b'b' => cs.blue = v.0.parse_to().unwrap(),
            _ => unreachable!(),
        }
    }
    Ok((rest, cs))
}

fn parse_game(input: &[u8]) -> IResult<&[u8], Game> {
    let mut g: Game = Game::default();
    let (rest, id) = delimited(tag("Game "), digit1, tag(": "))(input)?;
    g.id = id.parse_to().unwrap_or_default();

    let (rest, selections) = separated_list1(tag("; "), parse_cubeset)(rest)?;
    g.selections = selections;
    Ok((rest, g))
}

fn part1(input: &str) -> u32 {
    input.lines().fold(0, |acc, s| {
        let game = parse_game(s.as_bytes()).unwrap_or_default().1;
        let found_bad = game
            .selections
            .iter()
            .find(|&cs| cs.red > 12 || cs.blue > 14 || cs.green > 13);
        match found_bad {
            Some(_) => acc,
            None => acc + game.id,
        }
    })
}

fn part2(input: &str) -> u32 {
    input.lines().fold(0, |acc, s| {
        let game = parse_game(s.as_bytes()).unwrap_or_default().1;
        let fewest = game
            .selections
            .iter()
            .fold(CubeSet::default(), |acc, cs| CubeSet {
                red: std::cmp::max(acc.red, cs.red),
                green: std::cmp::max(acc.green, cs.green),
                blue: std::cmp::max(acc.blue, cs.blue),
            });
        acc + fewest.red * fewest.green * fewest.blue
    })
}

fn main() {
    let input = include_str!("../../input/02.txt");

    let timer = Instant::now();
    let p1 = part1(input);
    println!("Part 1: {:?}\n(elapsed: {:.2?})", p1, timer.elapsed());

    let timer = Instant::now();
    let p2 = part2(input);
    println!("Part 2: {:?}\n(elapsed: {:.2?})", p2, timer.elapsed());
}

#[cfg(test)]
mod tests {
    const SAMPLE1: &str = r"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part1_works() {
        let sum = super::part1(SAMPLE1);
        assert_eq!(8, sum);
    }

    #[test]
    fn part2_works() {
        let sum = super::part2(SAMPLE1);
        assert_eq!(2286, sum);
    }
}
