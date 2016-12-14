use std;
use std::error::Error;

#[derive(Debug, PartialEq)]
enum Direction {
    Right(i32),
    Left(i32)
}

impl std::str::FromStr for Direction {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, amt) = s.split_at(1);
        let amt_val: i32 = amt.parse().expect("expecting number");
        match direction {
            "R" => Ok(Direction::Right(amt_val)),
            "L" => Ok(Direction::Left(amt_val)),
            _ => panic!("Invalid direction")
        }
    }
}

fn parse_input(input: &str) -> Vec<&str> {
    let mut split_input = input.split(", ");
    let retvec: Vec<&str> = split_input.collect();
    retvec
}

pub fn solve(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_dir(){
        let dir: Direction = "R2".parse().expect("couldn't parse");
        assert_eq!(
            dir,
            Direction::Right(2)
        );
    }

    #[test]
    fn test_parse_input(){
        let input = "R5, L5, R5, R3";
        assert_eq!(parse_input(&input),["R5", "L5", "R5", "R3"]);
    }

    #[test]
    fn sample_1() {
        let input = "R2, L3";
        assert_eq!(solve(input),5);
    }

    #[test]
    fn sample_2() {
        let input = "R2, R2, R2";
        assert_eq!(solve(input),2);
    }

    #[test]
    fn sample_3() {
        let input = "R5, L5, R5, R3";
        assert_eq!(solve(input),12);
    }
}
