use regex::Regex;

advent_of_code::solution!(3);

enum Instr {
    Mul(u32, u32),
    Do,
    Dont,
}

fn parse_instrs(input: &str) -> anyhow::Result<Vec<Instr>> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|don't|do").unwrap();
    re.captures_iter(input)
        .map(|c| {
            let instr = match &c[0] {
                "do" => Instr::Do,
                "don't" => Instr::Dont,
                _ => Instr::Mul(c[1].parse()?, c[2].parse()?),
            };
            Ok(instr)
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let instrs = parse_instrs(input).ok()?;
    let res = instrs.iter().fold(0, |acc, instr| match instr {
        Instr::Mul(a, b) => acc + a * b,
        _ => acc,
    });

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let instrs = parse_instrs(input).ok()?;
    let mut enabled = true;
    let res = instrs.iter().fold(0, |acc, instr| match instr {
        Instr::Mul(a, b) if enabled => acc + a * b,
        Instr::Do => {
            enabled = true;
            acc
        }
        Instr::Dont => {
            enabled = false;
            acc
        }
        _ => acc,
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
