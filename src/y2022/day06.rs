use nopolitics::benchmark::BenchmarkResult;
use nopolitics::parse::ParseError;
use nopolitics::{Error, Part, Solution, SolutionResult};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn create_solution() -> Result<Solution, Error> {
    Ok(Solution::new(module_path!(), solve, None))
}

fn solve(path: &Path, part: Part) -> Result<SolutionResult, Error> {
    let mut benchmark = BenchmarkResult::new_and_start(part);
    let signal = fs::read_to_string(path).map_err(|e| Error::Parse(ParseError::from(e)))?;

    benchmark.stop_parse_start_part();
    Ok(match part {
        Part::Part(1) => {
            SolutionResult::part1(solve_inner(&signal, 4), benchmark.stop_part_and_owned())
        }
        Part::Part(2) => {
            SolutionResult::part2(solve_inner(&signal, 14), benchmark.stop_part_and_owned())
        }
        Part::All => SolutionResult::part1and2(
            solve_inner(&signal, 4),
            solve_inner(&signal, 14),
            benchmark.stop_part_and_owned(),
        ),
        _ => {
            return Err(Error::Default);
        }
    })
}

fn solve_inner(signal: &str, len: usize) -> String {
    let chars = signal.chars().collect::<Vec<_>>();
    (len..chars.len())
        .find(|x| chars[x - len..*x].iter().collect::<HashSet<_>>().len() == len)
        .unwrap()
        .to_string()
}
