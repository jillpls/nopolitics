use nopolitics::benchmark::BenchmarkResult;
use nopolitics::{Error, Part, Solution, SolutionResult};
use std::path::Path;

pub fn create_solution() -> Result<Solution, Error> {
    Ok(Solution::new(module_path!(), solve, None))
}

fn solve(path: &Path, part: Part) -> Result<SolutionResult, Error> {
    let mut benchmark = BenchmarkResult::new_and_start(part);
    let backpacks = nopolitics::parse::file_to_lines(path)?;
    benchmark.stop_parse_start_part();
    Ok(match part {
        Part::Part(1) => SolutionResult::part1(part1(&backpacks), benchmark.stop_part_and_owned()),
        Part::Part(2) => SolutionResult::part2(part2(&backpacks), benchmark.stop_part_and_owned()),
        Part::All => SolutionResult::part1and2(
            part1(&backpacks),
            part2(&backpacks),
            benchmark.stop_part_and_owned(),
        ),
        _ => {
            return Err(Error::Default);
        }
    })
}

fn part1(backpacks: &[String]) -> String {
    backpacks
        .iter()
        .map(|s| {
            let c1: Vec<_> = s.chars().take(s.len() / 2).collect::<_>();
            let c2: Vec<_> = s.chars().rev().take(s.len() / 2).collect::<_>();
            for c in c1 {
                if c2.contains(&c) {
                    return calculate_priority(c);
                }
            }
            0
        })
        .sum::<u32>()
        .to_string()
}

fn part2(backpacks: &[String]) -> String {
    backpacks
        .chunks(3)
        .map(|v| {
            for c in v[0].chars() {
                if v[1].contains(c) && v[2].contains(c) {
                    return calculate_priority(c);
                }
            }
            0
        })
        .sum::<u32>()
        .to_string()
}

fn calculate_priority(char: char) -> u32 {
    char as u32 - if char.is_lowercase() { 96 } else { 64 - 26 }
}
