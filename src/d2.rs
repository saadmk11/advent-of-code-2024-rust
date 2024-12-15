#[derive(Clone)]
struct Report {
    levels: Vec<u32>,
    is_increasing: bool,
}

impl From<&str> for Report {
    fn from(value: &str) -> Self {
        let levels: Vec<u32> = value
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        let mut is_increasing = false;

        if levels.len() >= 2 {
            is_increasing = levels[0] < levels[1];
        }
        Self {
            levels,
            is_increasing,
        }
    }
}

impl Report {
    fn is_safe(&self) -> bool {
        self.levels.windows(2).all(|pairs| {
            let x = pairs[0];
            let y = pairs[1];

            if !((self.is_increasing && y <= x) || (!self.is_increasing && y >= x))
                && (1..=3).contains(&x.abs_diff(y))
            {
                return true;
            }
            false
        })
    }

    fn clone_and_remove(&self, index: usize) -> Report {
        let mut new_report = self.clone();
        new_report.levels.remove(index);

        let mut is_increasing = false;

        if new_report.levels.len() >= 2 {
            is_increasing = new_report.levels[0] < new_report.levels[1];
        }
        new_report.is_increasing = is_increasing;
        new_report
    }

    fn can_correct(&self) -> bool {
        (0..self.levels.len())
            .map(|i| self.clone_and_remove(i))
            .any(|report: Report| report.is_safe())
    }
}

struct UnusualData {
    reports: Vec<Report>,
}

impl From<String> for UnusualData {
    fn from(value: String) -> Self {
        UnusualData {
            reports: value.trim().lines().map(Report::from).collect(),
        }
    }
}

pub fn part1(input: String) -> u32 {
    let data: UnusualData = input.into();

    data.reports.iter().fold(
        0,
        |acc, report| {
            if report.is_safe() {
                acc + 1
            } else {
                acc
            }
        },
    )
}

pub fn part2(input: String) -> u32 {
    let data: UnusualData = input.into();

    data.reports.iter().fold(0, |acc, report| {
        if report.is_safe() || report.can_correct() {
            acc + 1
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            .to_string();
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn test_part2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            .to_string();
        assert_eq!(part2(input), 4);
    }
}
