use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

type Data = Vec<usize>;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Result<Vec<Data>> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Vec<Data>> {
    let mut reports = vec![];
    for line in input.lines() {
        let levels = line.split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect();
        reports.push(levels);
    }
    Ok(reports)
}

fn safe_transition(level: usize, p: usize, increasing: bool) -> bool {
    level != p 
    && !(level > p && (!increasing || level > p + 3)) 
    && !(level < p && (increasing || level + 3 < p))
}

fn is_safe(report: &Vec<usize>) -> bool {
    let mut prev = None;
    let lastvalue = report.last().unwrap();
    let firstvalue = report.first().unwrap();
    let increasing = lastvalue > firstvalue;
    let mut safe = true;
    for &level in report {
        match prev {
            Some(p) => {
                if !safe_transition(level, p, increasing) {
                    safe = false;
                    break;
                }
            },
            None => {
                // do nothing
            }
        }
        prev = Some(level);
    }
    safe
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Data]) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &[Data]) -> usize {
    input.iter()
        .filter(|report| is_safe(report))
        .count()
}

// is it safe if we remove one level?
fn is_dampedsafe(report: &Vec<usize>) -> bool {
    if is_safe(report) {
        return true;
    }
    for i in 0..report.len()
    {
        let mut damped = report.to_vec();
        damped.remove(i);
        if is_safe(&damped) {
            return true;
        }
    }
    false
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Data]) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &[Data]) -> usize {
    input.iter()
        .filter(|report| is_dampedsafe(report))
        .count()
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 4);
    }
}