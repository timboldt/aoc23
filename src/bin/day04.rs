#![warn(clippy::all)]

extern crate nom;

use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, recognize},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, preceded, tuple},
    IResult,
};

use std::collections::BTreeSet;
use std::time::Instant;

#[derive(Default, Debug)]
struct ScratchCard {
    id: u32,
    winning: BTreeSet<u32>,
    actual: BTreeSet<u32>,
}

fn parse_num_list(input: &str) -> IResult<&str, Vec<u32>> {
    let (rest, vals) = preceded(
        many0(tag(" ")),
        separated_list1(
            many1(tag(" ")),
            map_res(recognize(digit1), str::parse::<u32>),
        ),
    )(input)?;
    Ok((rest, vals))
}

fn parse_scratchcard(input: &str) -> ScratchCard {
    let mut sc: ScratchCard = ScratchCard::default();

    if input.len() > 0 {
        let (_, (id, numbers)) = tuple((
            delimited(
                tuple((tag("Card"), many1(tag(" ")))),
                map_res(recognize(digit1), str::parse::<u32>),
                tag(":"),
            ),
            separated_list1(tag(" | "), parse_num_list),
        ))(input)
        .unwrap();

        sc.id = id;

        if numbers.len() == 2 {
            numbers[0].iter().for_each(|&v| {
                sc.winning.insert(v);
            });
            numbers[1].iter().for_each(|&v| {
                sc.actual.insert(v);
            });
        }
    }

    sc
}

fn part1(input: &str) -> u32 {
    input.lines().fold(0, |acc, s| {
        let sc = parse_scratchcard(s);
        let winning = sc.actual.intersection(&sc.winning).count() as u32;
        let value = if winning == 0 { 0 } else { 1 << (winning - 1) };
        acc + value
    })
}

fn part2(_input: &str) -> u32 {
    42
}

fn main() {
    let input = include_str!("../../input/04.txt");

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
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part1_works() {
        let sum = super::part1(SAMPLE1);
        assert_eq!(13, sum);
    }

    #[test]
    fn part2_works() {
        let sum = super::part2(SAMPLE1);
        assert_eq!(2286, sum);
    }
}
