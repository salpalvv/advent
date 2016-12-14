pub fn solve(input &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1{
        let input = "R2, L3";
        assert_eq!(solve(input),5);
    }

    #[test]
    fn sample_2{
        let input = "R2, R2, R2";
        assert_eq!(solve(input),2);
    }

    #[test]
    fn sample_3{
        let input = "R5, L5, R5, R3";
        assert_eq!(solve(input),12);
    }
}
