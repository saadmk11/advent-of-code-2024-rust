pub fn part1(input: String) -> u32 {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input
        .trim()
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("   ").unwrap();
            (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap())
        })
        .unzip();

    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right)
        .fold(0, |acc, (left, right)| acc + left.abs_diff(right))
}

pub fn part2(input: String) -> u32 {
    let (left, right): (Vec<u32>, Vec<u32>) = input
        .trim()
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("   ").unwrap();
            (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap())
        })
        .unzip();

    let right_count_map = right
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, item| {
            *acc.entry(*item).or_insert(0) += 1;
            acc
        });

    left.iter()
        .map(|item| item * right_count_map.get(item).unwrap_or(&0))
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3".to_string();
        assert_eq!(part1(input), 11);
    }

    #[test]
    fn test_part2() {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3".to_string();
        assert_eq!(part2(input), 31);
    }
}
