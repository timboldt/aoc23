#![warn(clippy::all)]

extern crate nom;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{value},
    multi::many1,
};

use std::time::Instant;

fn parse_part1(input: &str) -> u32 {
    let (_, digits) = many1(
        alt((
            alt((
                value(1u32, tag::<&str, &str, nom::error::Error<&str>>("1")),
                value(2u32, tag("2")),
                value(3u32, tag("3")),
                value(4u32, tag("4")),
                value(5u32, tag("5")),
                value(6u32, tag("6")),
                value(7u32, tag("7")),
                value(8u32, tag("8")),
                value(9u32, tag("9")),
            )),
            value(0xFFu32, take(1usize)),
        )),
    )(input)
    .unwrap();

    let digits: Vec<u32> = digits.into_iter().filter(|&x| x != 0xFFu32).collect();

    digits.first().unwrap_or(&0u32) * 10 + digits.last().unwrap_or(&0u32)
}

fn parse_part2(input: &str) -> u32 {
    let (_, first_digits) = many1(alt((
        alt((
            value(1u32, tag::<&str, &str, nom::error::Error<&str>>("one")),
            value(2u32, tag("two")),
            value(3u32, tag("three")),
            value(4u32, tag("four")),
            value(5u32, tag("five")),
            value(6u32, tag("six")),
            value(7u32, tag("seven")),
            value(8u32, tag("eight")),
            value(9u32, tag("nine")),
        )),
        alt((
            value(1u32, tag("1")),
            value(2u32, tag("2")),
            value(3u32, tag("3")),
            value(4u32, tag("4")),
            value(5u32, tag("5")),
            value(6u32, tag("6")),
            value(7u32, tag("7")),
            value(8u32, tag("8")),
            value(9u32, tag("9")),
        )),
        value(0xFFu32, take(1usize)),
    )))(input)
    .unwrap();

    let (_, last_digits) = many1(alt((
        alt((
            // Tricky problem: some numbers can be overlapping and we need the last valid one.
            value(1u32, tag::<&str, &str, nom::error::Error<&str>>("twone")),
            value(2u32, tag("eightwo")),
            value(3u32, tag("eighthree")),
            value(8u32, tag("oneight")),
            value(8u32, tag("threeight")),
            value(8u32, tag("fiveight")),
            value(8u32, tag("nineight")),
        )),
        alt((
            value(1u32, tag("one")),
            value(2u32, tag("two")),
            value(3u32, tag("three")),
            value(4u32, tag("four")),
            value(5u32, tag("five")),
            value(6u32, tag("six")),
            value(7u32, tag("seven")),
            value(8u32, tag("eight")),
            value(9u32, tag("nine")),
        )),
        alt((
            value(1u32, tag("1")),
            value(2u32, tag("2")),
            value(3u32, tag("3")),
            value(4u32, tag("4")),
            value(5u32, tag("5")),
            value(6u32, tag("6")),
            value(7u32, tag("7")),
            value(8u32, tag("8")),
            value(9u32, tag("9")),
        )),
        value(0xFFu32, take(1usize)),
    )))(input)
    .unwrap();
    let first_digits: Vec<u32> = first_digits.into_iter().filter(|&x| x != 0xFFu32).collect();
    let last_digits: Vec<u32> = last_digits.into_iter().filter(|&x| x != 0xFFu32).collect();
    first_digits.first().unwrap_or(&0u32) * 10 + last_digits.last().unwrap_or(&0u32)
}

fn part1(input: &str) -> u32 {
    input.lines().fold(0, |acc, s| acc + parse_part1(s))
}

fn part2(input: &str) -> u32 {
    input.lines().fold(0, |acc, s| acc + parse_part2(s))
}

fn main() {
    let input = include_str!("../../input/01.txt");

    let timer = Instant::now();
    let p1 = part1(input);
    println!("Part 1: {:?}\n(elapsed: {:.2?})", p1, timer.elapsed());

    let timer = Instant::now();
    let p2 = part2(input);
    println!("Part 2: {:?}\n(elapsed: {:.2?})", p2, timer.elapsed());
}

#[cfg(test)]
mod tests {
    const SAMPLE1: &str = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const SAMPLE2: &str = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn part1_works() {
        let sum = super::part1(SAMPLE1);
        assert_eq!(142, sum);
    }

    #[test]
    fn part2_works() {
        let sum = super::part2(SAMPLE2);
        assert_eq!(281, sum);
    }
}
