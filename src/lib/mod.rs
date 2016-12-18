mod day1;
mod day2;

pub fn solve(day: &i32, input: &str) -> i32 {
    match *day {
        1 => {
            return day1::solve_p2(input);
        },
        2 => {
            return day2::solve(input);
        },
        3...25 => unimplemented!(),
        _ => panic!("There is no day {}", day),
    }
}
