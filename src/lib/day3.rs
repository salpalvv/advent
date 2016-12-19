pub fn solve(input: &str) -> i32 {
    let mut possible = 0;

    for line in input.lines() {
        let parsed_line = parse_line(line);
        let parsed_lengths: Result<Vec<i32>, _> = parsed_line.iter().map(|&x| x.parse::<i32>()).collect();
        let parsed_lengths = parsed_lengths.expect("couldn't parse lines");
        if is_possible(&parsed_lengths) {
            possible += 1;
        }
    }
    possible
}

fn is_possible(input: &[i32]) -> bool {
    if input[0] < input[1] + input[2] {
        if input[1] < input[0] + input[2] {
            if input[2] < input[1] + input[0] {
                return true;
            }
        }
    }
    false
}

fn parse_line(line: &str) -> Vec<&str> {
    let parsed = line.split_whitespace().collect();
    parsed
}

mod tests {
    use super::*;

    #[test]
    fn test_parsed_line(){
        let input = " 1 2 3 ";
        assert_eq!(parse_line(&input), ["1","2","3"])

    }

    #[test]
    fn test_is_possible(){
        let input = [4,5,6];
        assert_eq!(is_possible(&input),true);
    }

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
