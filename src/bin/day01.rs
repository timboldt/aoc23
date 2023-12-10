#![warn(clippy::all)]

extern crate nom;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::value,
    multi::many1,
};

use std::time::Instant;

const INVALID_DIGIT: u32 = 0xFF;

fn parse_part1(input: &str) -> u32 {
    let (_, digits) = many1(alt((
        alt((
            value(1, tag::<&str, &str, nom::error::Error<&str>>("1")),
            value(2, tag("2")),
            value(3, tag("3")),
            value(4, tag("4")),
            value(5, tag("5")),
            value(6, tag("6")),
            value(7, tag("7")),
            value(8, tag("8")),
            value(9, tag("9")),
        )),
        value(INVALID_DIGIT, take(1usize)),
    )))(input)
    .unwrap();

    // Remove invalid digits.
    let digits: Vec<u32> = digits.into_iter().filter(|&x| x != INVALID_DIGIT).collect();

    digits.first().unwrap() * 10 + digits.last().unwrap()
}

fn parse_part2(input: &str) -> u32 {
    let (_, first_digits) = many1(alt((
        alt((
            value(1, tag::<&str, &str, nom::error::Error<&str>>("one")),
            value(2, tag("two")),
            value(3, tag("three")),
            value(4, tag("four")),
            value(5, tag("five")),
            value(6, tag("six")),
            value(7, tag("seven")),
            value(8, tag("eight")),
            value(9, tag("nine")),
        )),
        alt((
            value(1, tag("1")),
            value(2, tag("2")),
            value(3, tag("3")),
            value(4, tag("4")),
            value(5, tag("5")),
            value(6, tag("6")),
            value(7, tag("7")),
            value(8, tag("8")),
            value(9, tag("9")),
        )),
        value(INVALID_DIGIT, take(1usize)),
    )))(input)
    .unwrap();

    // Remove invalid digits.
    let first_digits: Vec<u32> = first_digits
        .into_iter()
        .filter(|&x| x != INVALID_DIGIT)
        .collect();

    let (_, last_digits) = many1(alt((
        alt((
            // Tricky problem: some numbers can be overlapping and we need the last valid one.
            value(1, tag::<&str, &str, nom::error::Error<&str>>("twone")),
            value(2, tag("eightwo")),
            value(3, tag("eighthree")),
            value(8, tag("oneight")),
            value(8, tag("threeight")),
            value(8, tag("fiveight")),
            value(8, tag("nineight")),
            value(9, tag("sevenine")),
        )),
        alt((
            value(1, tag("one")),
            value(2, tag("two")),
            value(3, tag("three")),
            value(4, tag("four")),
            value(5, tag("five")),
            value(6, tag("six")),
            value(7, tag("seven")),
            value(8, tag("eight")),
            value(9, tag("nine")),
        )),
        alt((
            value(1, tag("1")),
            value(2, tag("2")),
            value(3, tag("3")),
            value(4, tag("4")),
            value(5, tag("5")),
            value(6, tag("6")),
            value(7, tag("7")),
            value(8, tag("8")),
            value(9, tag("9")),
        )),
        value(INVALID_DIGIT, take(1usize)),
    )))(input)
    .unwrap();

    // Remove invalid digits.
    let last_digits: Vec<u32> = last_digits
        .into_iter()
        .filter(|&x| x != INVALID_DIGIT)
        .collect();

    first_digits.first().unwrap() * 10 + last_digits.last().unwrap()
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
