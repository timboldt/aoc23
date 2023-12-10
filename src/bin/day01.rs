#![warn(clippy::all)]

extern crate nom;

use nom::bytes::complete::tag;
use nom::bytes::complete::take_while;
use nom::character::complete::{alpha1, digit1};
use nom::character::is_alphabetic;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;

fn digit_value(input: u8) -> u32 {
    input as u32 - '0' as u32
}

fn parse_calibration(input: &[u8]) -> IResult<&[u8], u32> {
    let (rest, digits) = preceded(take_while(is_alphabetic), separated_list1(alpha1, digit1))(input)?;
    // First digit of first digit sequence.
    let first_digit = digit_value(*digits.first().unwrap().first().unwrap());
    // Last digit of last digit sequence.
    let last_digit = digit_value(*digits.last().unwrap().last().unwrap());
    //println!("first = {} last = {}", first_digit, last_digit);
    Ok((rest, first_digit * 10 + last_digit))
}

// fn part1(parsed: &[Polyline]) -> usize {
//     42
// }

// fn part2(parsed: &[Polyline]) -> usize {
//     42
// }

fn main() {
    let input = include_str!("../../input/01.txt");
    let sum = input.lines().fold(0, |acc, s| {
        acc + parse_calibration(s.as_bytes()).unwrap_or_default().1
    });
    println!("sum = {}", sum);

    // let timer = Instant::now();
    // let p1 = part1(&parsed);
    // println!("Part 1: {:?}\n(elapsed: {:.2?})", p1, timer.elapsed());

    // let timer = Instant::now();
    // let p2 = part2(&parsed);
    // println!("Part 2: {:?}\n(elapsed: {:.2?})", p2, timer.elapsed());
}

// #[cfg(test)]
// mod tests {
//     const SAMPLE: &str = r"
//         1abc2
//         pqr3stu8vwx
//         a1b2c3d4e5f
//         treb7uchet";

//     #[test]
//     fn part1_works() {
//         let input = super::parse(SAMPLE);
//         assert_eq!(13, super::part1(&input));
//     }

//     #[test]
//     fn part2_works() {
//         let input = super::parse(SAMPLE);
//         assert_eq!(29, super::part2(&input));
//     }
// }
