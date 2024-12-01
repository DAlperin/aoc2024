use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let line_count = input.lines().count();
    let mut left = Vec::with_capacity(line_count);
    let mut right = Vec::with_capacity(line_count);

    input.lines().for_each(|line| {
        let splits = line.split("   ");
        let mut iter = splits.map(|s| s.parse::<u32>().unwrap());
        left.push(iter.next().unwrap());
        right.push(iter.next().unwrap());
    });

    left.sort_unstable();
    right.sort_unstable();

    let res = right
        .iter()
        .zip(left.iter())
        .map(|(r, l)| r.abs_diff(*l))
        .sum::<u32>();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let line_count = input.lines().count();
    let mut left = Vec::with_capacity(line_count);
    let mut right = Vec::with_capacity(line_count);

    input.lines().for_each(|line| {
        let splits = line.split("   ");
        let mut iter = splits.map(|s| s.parse::<u32>().unwrap());
        left.push(iter.next().unwrap());
        right.push(iter.next().unwrap());
    });

    left.sort_unstable();
    right.sort_unstable();

    let right_appearance_count = right.iter().fold(HashMap::new(), |mut acc, &x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });

    let res = left.iter().fold(0, |acc, &x| {
        acc + x * right_appearance_count.get(&x).unwrap_or(&0)
    });

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
