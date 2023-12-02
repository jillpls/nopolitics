use nopolitics::benchmark::BenchmarkResult;
use nopolitics::{Error, Part, Solution, SolutionResult};
use std::path::Path;

pub fn create_solution() -> Result<Solution, Error> {
    Ok(Solution::new(module_path!(), solve, None))
}

fn solve(path: &Path, part: Part) -> Result<SolutionResult, Error> {
    let mut benchmark = BenchmarkResult::new_and_start(part);
    let lines = nopolitics::parse::file_to_lines(path)?;
    let split = lines.split(|s| s.trim().is_empty()).collect::<Vec<_>>();
    let stacks = parse_stacks(split[0]);
    let instructions = parse_instructions(split[1]);
    benchmark.stop_parse_start_part();
    Ok(match part {
        Part::Part(1) => SolutionResult::part1(
            solve_inner(stacks, &instructions, false),
            benchmark.stop_part_and_owned(),
        ),
        Part::Part(2) => SolutionResult::part2(
            solve_inner(stacks, &instructions, true),
            benchmark.stop_part_and_owned(),
        ),
        Part::All => SolutionResult::part1and2(
            solve_inner(stacks.clone(), &instructions, false),
            solve_inner(stacks, &instructions, true),
            benchmark.stop_part_and_owned(),
        ),
        _ => {
            return Err(Error::Default);
        }
    })
}

fn solve_inner(
    mut stacks: Vec<Vec<char>>,
    instructions: &[(usize, usize, usize)],
    part2: bool,
) -> String {
    for (count, from, to) in instructions {
        if part2 {
            let mut r = stacks[*from]
                .iter()
                .rev()
                .take(*count)
                .rev()
                .copied()
                .collect();
            stacks[*to].append(&mut r);
            for _ in 0..*count {
                stacks[*from].pop();
            }
        } else {
            for _ in 0..*count {
                if let Some(c) = stacks[*from].pop() {
                    stacks[*to].push(c)
                }
            }
        }
    }
    stacks.iter().fold(String::new(), |mut s, v| {
        if let Some(c) = v.last() {
            s.push(*c);
        }
        s
    })
}

fn parse_stacks(lines: &[String]) -> Vec<Vec<char>> {
    let len = lines
        .last()
        .unwrap()
        .chars()
        .filter(|c| c.is_numeric())
        .count();
    let lines = &lines[..lines.len() - 1];
    let lines = lines
        .iter()
        .map(|x| {
            {
                x.trim_end()
                    .chars()
                    .enumerate()
                    .filter(|(i, _)| (i + 3) % 4 == 0)
                    .map(|(_, c)| if c.is_alphabetic() { Some(c) } else { None })
                    .collect::<Vec<Option<char>>>()
            }
        })
        .rev()
        .collect::<Vec<Vec<Option<char>>>>();
    let mut stacks = vec![vec![]; len];
    for l in lines {
        for (i, c) in l.iter().enumerate() {
            if let Some(el) = c {
                stacks[i].push(*el);
            }
        }
    }
    stacks
}

fn parse_instructions(lines: &[String]) -> Vec<(usize, usize, usize)> {
    lines
        .iter()
        .map(|x| {
            let s = x.split_whitespace().collect::<Vec<&str>>();
            (
                s[1].parse::<usize>().unwrap(),
                s[3].parse::<usize>().unwrap() - 1,
                s[5].parse::<usize>().unwrap() - 1,
            )
        })
        .collect::<_>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_stacks() {
        let lines = nopolitics::parse::file_to_lines("input/y2022/day05/example.txt").unwrap();
        let split = lines.split(|s| s.trim().is_empty()).collect::<Vec<_>>();
        let r = parse_stacks(split[0]);
        assert_eq!(r.len(), 3);
        assert_eq!(r[0], vec!['Z', 'N'])
    }

    #[test]
    fn test_parse_instructions() {
        let lines = nopolitics::parse::file_to_lines("input/y2022/day05/example.txt").unwrap();
        let split = lines.split(|s| s.trim().is_empty()).collect::<Vec<_>>();
        let r = parse_instructions(split[1]);
        assert_eq!(r.len(), 4);
        assert_eq!(r[0], (1, 1, 0));
    }
}
