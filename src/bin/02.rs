advent_of_code::solution!(2);

fn is_valid_sequence

pub fn part_one(input: &str) -> Option<u32> {
    let mut out = 0;
    input.lines().for_each(|line| {
        let splits = line.split(" ");
        let report = splits
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let is_increasing = report
            .windows(2)
            .all(|w| w[0] < w[1] && w[0].abs_diff(w[1]) >= 1 && w[0].abs_diff(w[1]) <= 3);
        let is_decreasing = report
            .windows(2)
            .all(|w| w[0] > w[1] && w[0].abs_diff(w[1]) >= 1 && w[0].abs_diff(w[1]) <= 3);

        if is_increasing || is_decreasing {
            out += 1;
        }
    });

    Some(out)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut out = 0;
    input.lines().for_each(|line| {
        let splits = line.split(" ");
        let report = splits
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let is_increasing = report
            .windows(2)
            .all(|w| w[0] < w[1] && w[0].abs_diff(w[1]) >= 1 && w[0].abs_diff(w[1]) <= 3);
        let is_decreasing = report
            .windows(2)
            .all(|w| w[0] > w[1] && w[0].abs_diff(w[1]) >= 1 && w[0].abs_diff(w[1]) <= 3);

        if is_increasing || is_decreasing {
            out += 1;
        } else {
            //try again with each number removed
            for i in 0..report.len() {
                let mut report = report.clone();
                report.remove(i);

                let is_increasing = report
                    .windows(2)
                    .all(|w| w[0] < w[1] && w[0].abs_diff(w[1]) >= 1 && w[0].abs_diff(w[1]) <= 3);
                let is_decreasing = report
                    .windows(2)
                    .all(|w| w[0] > w[1] && w[0].abs_diff(w[1]) >= 1 && w[0].abs_diff(w[1]) <= 3);

                if is_increasing || is_decreasing {
                    out += 1;
                    break;
                }
            }
        }
    });

    Some(out)
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
