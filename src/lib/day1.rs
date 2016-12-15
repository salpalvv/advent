use std::str::FromStr;
use std::error::Error;


pub fn solve_p1(input: &str) -> i32 {
    let instructions = parse_input(input);
    let mut solver = Solver::new();
    let parsed_instructions: Result<Vec<_>, _> = instructions.iter().map(|&x| x.parse()).collect();
    let parsed_instructions = parsed_instructions.expect("couldn't parse instructions");
    solver.execute_p1(&parsed_instructions);
    solver.distance_p1()
}

pub fn solve_p2(input: &str) -> i32 {
    let instructions = parse_input(input);
    let mut solver = Solver::new();
    let parsed_instructions: Result<Vec<_>, _> = instructions.iter().map(|&x| x.parse()).collect();
    let parsed_instructions = parsed_instructions.expect("couldn't parse instructions");
    solver.execute_p2(&parsed_instructions);
    solver.distance_p2()
}

#[derive(Debug, PartialEq)]
struct Solver {
    coords: [i32; 2],
    cardinal: Cardinal,
    coords_visited: Vec<[i32;2]>,
    dupes: Vec<[i32;2]>,
}

#[derive(Debug, PartialEq)]
enum Cardinal {
    North,
    East,
    South,
    West,
}

impl Solver {
    fn new() -> Solver {
        Solver {
            coords: [0; 2],
            cardinal: Cardinal::North,
            coords_visited: vec!([0;2]),
            dupes: Vec::new(),
        }
    }

    /// Part 1 execute {{{
    fn execute_p1(&mut self, instructions: &[Direction]) {
        for instruction in instructions {
            match *instruction {
                Direction::Right(ref distance) => {
                    match self.cardinal {
                        Cardinal::North => {
                            self.coords[0] += *distance;
                            self.cardinal = Cardinal::East;
                        },
                        Cardinal::East => {
                            self.coords[1] -= *distance;
                            self.cardinal = Cardinal::South;
                        },
                        Cardinal::South => {
                            self.coords[0] -= *distance;
                            self.cardinal = Cardinal::West;
                        },
                        Cardinal::West => {
                            self.coords[1] += *distance;
                            self.cardinal = Cardinal::North;
                        },
                    }
                },
                Direction::Left(ref distance) => {
                    match self.cardinal {
                        Cardinal::North => {
                            self.coords[0] -= *distance;
                            self.cardinal = Cardinal::West;
                        },
                        Cardinal::East => {
                            self.coords[1] += *distance;
                            self.cardinal = Cardinal::North;
                        },
                        Cardinal::South => {
                            self.coords[0] += *distance;
                            self.cardinal = Cardinal::East;
                        },
                        Cardinal::West => {
                            self.coords[1] -= *distance;
                            self.cardinal = Cardinal::South;
                        },
                    }
                }
            }
        }
    }
    // }}}

    /// Part 2 execute
    fn execute_p2(&mut self, instructions: &[Direction]) {
        for instruction in instructions {
            match *instruction {
                Direction::Right(ref distance) => {
                    match self.cardinal {
                        Cardinal::North => {
                            let sp = self.coords[0] + 1;
                            let ep = self.coords[0] + *distance + 1;
                            for x in sp..ep {
                                if self.coords_visited.contains(&[x,self.coords[1]]){
                                    self.dupes.push([x,self.coords[1]]);
                                }
                                self.coords_visited.push([x,self.coords[1]]);
                            }
                            self.coords[0] += *distance;
                            self.cardinal = Cardinal::East;
                        },
                        Cardinal::East => {
                            let ep = self.coords[1];
                            let sp = self.coords[1] - *distance;
                            for y in sp..ep {
                                if self.coords_visited.contains(&[self.coords[0],y]){
                                    self.dupes.push([self.coords[0],y]);
                                }
                                self.coords_visited.push([self.coords[0],y]);
                            }
                            self.coords[1] -= *distance;
                            self.cardinal = Cardinal::South;
                        },
                        Cardinal::South => {
                            let ep = self.coords[0];
                            let sp = self.coords[0] - *distance;
                            for x in sp..ep {
                                if self.coords_visited.contains(&[x,self.coords[1]]){
                                    self.dupes.push([x,self.coords[1]]);
                                }
                                self.coords_visited.push([x,self.coords[1]]);
                            }
                            self.coords[0] -= *distance;
                            self.cardinal = Cardinal::West;
                        },
                        Cardinal::West => {
                            let sp = self.coords[1] + 1;
                            let ep = self.coords[1] + *distance + 1;
                            for y in sp..ep {
                                if self.coords_visited.contains(&[self.coords[0],y]){
                                    self.dupes.push([self.coords[0],y]);
                                }
                                self.coords_visited.push([self.coords[0],y]);
                            }
                            self.coords[1] += *distance;
                            self.cardinal = Cardinal::North;
                        },
                    }
                },
                Direction::Left(ref distance) => {
                    match self.cardinal {
                        Cardinal::North => {
                            let ep = self.coords[0];
                            let sp = self.coords[0] - *distance;
                            for x in sp..ep {
                                if self.coords_visited.contains(&[x,self.coords[1]]){
                                    self.dupes.push([x,self.coords[1]]);
                                }
                                self.coords_visited.push([x,self.coords[1]]);
                            }
                            self.coords[0] -= *distance;
                            self.cardinal = Cardinal::West;
                        },
                        Cardinal::East => {
                            let sp = self.coords[1] + 1;
                            let ep = self.coords[1] + *distance + 1;
                            for y in sp..ep {
                                if self.coords_visited.contains(&[self.coords[0],y]){
                                    self.dupes.push([self.coords[0],y]);
                                }
                                self.coords_visited.push([self.coords[0],y]);
                            }
                            self.coords[1] += *distance;
                            self.cardinal = Cardinal::North;
                        },
                        Cardinal::South => {
                            let sp = self.coords[0] + 1;
                            let ep = self.coords[0] + *distance + 1;
                            for x in sp..ep {
                                if self.coords_visited.contains(&[x,self.coords[1]]){
                                    self.dupes.push([x,self.coords[1]]);
                                }
                                self.coords_visited.push([x,self.coords[1]]);
                            }
                            self.coords[0] += *distance;
                            self.cardinal = Cardinal::East;
                        },
                        Cardinal::West => {
                            let ep = self.coords[1];
                            let sp = self.coords[1] - *distance;
                            for y in sp..ep {
                                if self.coords_visited.contains(&[self.coords[0],y]){
                                    self.dupes.push([self.coords[0],y]);
                                }
                                self.coords_visited.push([self.coords[0],y]);
                            }
                            self.coords[1] -= *distance;
                            self.cardinal = Cardinal::South;
                        },
                    }
                }
            }
        }
    }

    fn distance_p1(&self) -> i32 {
        let distance = self.coords.iter().map(|&x| x.abs()).sum();
        distance
    }

    fn distance_p2(&self) -> i32 {
        let distance = self.dupes[0].iter().map(|&x| x.abs()).sum();
        distance
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Right(i32),
    Left(i32),
}

impl FromStr for Direction {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, amt) = s.split_at(1);
        let amt_val: i32 = amt.trim().parse().expect("expecting number");
        match direction {
            "R" => Ok(Direction::Right(amt_val)),
            "L" => Ok(Direction::Left(amt_val)),
            other => Err(format!("Invalid direction {}", other).into()),
        }
    }
}

fn parse_input(input: &str) -> Vec<&str> {
    let split_input = input.split(", ");
    let retvec: Vec<&str> = split_input.collect();
    retvec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_dir(){
        let dir_r: Direction = "R2".parse().expect("couldn't parse");
        let dir_l: Direction = "L55".parse().expect("couldn't parse");
        let dir_long: Direction = "R525".parse().expect("couldn't parse");
        assert_eq!(dir_r,Direction::Right(2));
        assert_eq!(dir_l,Direction::Left(55));
        assert_eq!(dir_long,Direction::Right(525));
    }

    #[test]
    fn test_parse_input(){
        let input = "R5, L5, R5, R3";
        assert_eq!(parse_input(&input),["R5", "L5", "R5", "R3"]);
    }

    #[test]
    fn execute_instructions_p1(){
        let mut solver = Solver::new();
        solver.execute_p1(&[Direction::Right(2)]);
        assert_eq!(solver.distance_p1(), 2);
    }

    #[test]
    fn sample_1() {
        let input = "R2, L3";
        assert_eq!(solve_p1(input),5);
    }

    #[test]
    fn sample_2() {
        let input = "R2, R2, R2";
        assert_eq!(solve_p1(input),2);
    }

    #[test]
    fn sample_3() {
        let input = "R5, L5, R5, R3";
        assert_eq!(solve_p1(input),12);
    }
}
