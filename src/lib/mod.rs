mod day1;

pub fn solve(day: &i32, input: &str) -> i32 {
    match *day {
        1 => {
            return day1::solve_p2(input);
        },
        2...25 => unimplemented!(),
        _ => panic!("There is no day {}", day),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test(){
        assert!(true);
    }
}
