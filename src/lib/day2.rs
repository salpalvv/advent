use std::str::FromStr;
use std::error::Error;

pub fn solve(input: &str) -> i32 {
    let mut solver = Solver::new();
    let mut answer = 0;

    for line in input.lines() {
        let parsed_line = parse_line(line);
        let parsed_directions: Result<Vec<Direction>, _> = parsed_line.iter().map(|&x| x.parse()).collect();
        let parsed_directions = parsed_directions.expect("couldn't parse dirs");
        solver.execute(&parsed_directions);
        answer = answer * 10 + solver.location;
        println!("{}", answer);
    }
    answer
}

#[derive(Debug, PartialEq)]
struct Solver {
    location: i32,
}

impl Solver {
    fn new() -> Solver {
        Solver {
            location: 5,
        }
    }

    fn execute(&mut self, directions: &[Direction] ){
        for direction in directions {
            match *direction {
                Direction::Up => {
                    match self.location {
                        4...9 => self.location -= 3,
                        _ => {},
                    }
                },
                Direction::Down => {
                    match self.location {
                        0...6 => self.location += 3,
                        _ => {},
                    }
                },
                Direction::Left => {
                    match self.location {
                        1 | 4 | 7 => {},
                        _ => self.location -= 1,
                    }
                },
                Direction::Right => {
                    match self.location {
                        3 | 6 | 9 => {},
                        _ => self.location += 1,
                    }
                },
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Direction{
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = Box<Error>;
    fn from_str(s:&str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            other => Err(format!("Invalid Direction {}", other).into()),
        }
    }
}

fn parse_line(line: &str) -> Vec<&str> {
    let split_line = line.split("");
    let mut retvec: Vec<&str> = split_line.collect();
    retvec.reverse();
    retvec.pop();
    retvec.reverse();
    retvec.pop();
    retvec
}

mod tests {
    use super::*;

    #[test]
    fn parse_dir(){
        let dir_u: Direction = "U".parse().expect("couldn't parse");
        let dir_d: Direction = "D".parse().expect("couldn't parse");
        let dir_l: Direction = "L".parse().expect("couldn't parse");
        let dir_r: Direction = "R".parse().expect("couldn't parse");
        assert_eq!(dir_u, Direction::Up);
        assert_eq!(dir_d, Direction::Down);
        assert_eq!(dir_l, Direction::Left);
        assert_eq!(dir_r, Direction::Right);
    }

    #[test]
    fn test_parse_line(){
        let input = "ULLRRDUD";
        assert_eq!(parse_line(&input), ["U","L","L","R","R","D","U","D"])
    }

    #[test]
    fn execute(){
        let mut solver = Solver::new();
        solver.execute(&[Direction::Up]);
        assert_eq!(solver.location, 2);
    }

    #[test]
    fn sample(){
        let input = "ULL\n\
            RRDDD\n\
            LURDL\n\
            UUUUD";
        assert_eq!(1985, solve(input));
    }
}
