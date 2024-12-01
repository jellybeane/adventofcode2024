use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

type Data = (i64, i64);

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Result<Vec<Data>> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Vec<Data>> {
    let mut result: Vec<(i64, i64)> = vec![];
    for line in input.lines(){
        let (left, right) = line.split_once("   ").unwrap();
        let left: i64 = left.parse().unwrap();
        let right: i64 = right.parse().unwrap();
        result.push((left, right));
    }
    Ok(result)
}

// Sort the left and right lists
// Then, sum the distance of the pairs
#[aoc(day1, part1)]
pub fn solve_part1(input: &[Data]) -> i64 {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &[Data]) -> i64 {
    let mut lefts: Vec<i64> = vec![];
    let mut rights: Vec<i64> = vec![];
    for (left, right) in input {
        lefts.push(*left);
        rights.push(*right);
    }
    lefts.sort();
    rights.sort();
    
    let mut sumofdiffs = 0;
    for i in lefts.iter().zip(rights.iter()){
        let (left, right) = i;
        let subtract = left - right;
        sumofdiffs += subtract.abs();
    }

    sumofdiffs
}

// Find the number of times the left value appears in the right list
// Then, sum left value * num occurrences
#[aoc(day1, part2)]
pub fn solve_part2(input: &[Data]) -> i64 {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &[Data]) -> i64 {
    let mut lefts: Vec<i64> = vec![];
    let mut rights: Vec<i64> = vec![];
    for (left, right) in input {
        lefts.push(*left);
        rights.push(*right);
    }

    let mut similarity = 0;
    for left in lefts {
        let numappears = rights.iter()
                                      .filter(|&x| *x == left)
                                      .count();
        // cannot subtract usize from i64
        // try_into: in case we exceed 2^63
        let numappears: i64 = numappears.try_into().unwrap();
        similarity += left * numappears;
    }

    similarity
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"3   4
4   3
2   5
1   3
3   9
3   3
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 11);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 31);
    }
}