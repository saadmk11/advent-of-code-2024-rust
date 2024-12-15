use regex::Regex;

pub fn part1(input: String) -> u32 {
    let regex = Regex::new(r"mul\((?P<opl>[0-9]{1,3}),(?P<opr>[0-9]{1,3})\)").unwrap();
    regex
        .captures_iter(&input)
        .map(|cap| cap["opl"].parse::<u32>().unwrap() * cap["opr"].parse::<u32>().unwrap())
        .sum()
}

pub fn part2(input: String) -> u32 {
    let regex = Regex::new(
        r"(?P<enabled>do\(\))|(?P<disabled>don't\(\))|mul\((?P<opl>[0-9]{1,3}),(?P<opr>[0-9]{1,3})\)"
    ).unwrap();
    let mut enabled = true;

    regex
        .captures_iter(&input)
        .map(|cap| match (&cap[0], enabled) {
            ("do()", _) => {
                enabled = true;
                0
            }
            ("don't()", _) => {
                enabled = false;
                0
            }
            (_, true) => cap["opl"].parse::<u32>().unwrap() * cap["opr"].parse::<u32>().unwrap(),
            _ => 0,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();
        assert_eq!(part1(input), 161);
    }

    #[test]
    fn test_part2() {
        let input =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();
        assert_eq!(part2(input), 48);
    }
}
