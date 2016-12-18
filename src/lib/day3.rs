pub fn solve(input: &str) -> i32 {
    1
}

mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "    4   21  894\n\
            419  794  987\n\
            424  797  125\n\
            651  305  558\n\
            655  631  963";
        assert_eq!(solve(input), 3);
    }
}
