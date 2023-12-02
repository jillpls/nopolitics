use nopolitics::benchmark::BenchmarkResult;
use nopolitics::{Error, Part, Solution, SolutionResult};
use std::path::Path;

pub fn create_solution() -> Result<Solution, Error> {
    Err(Error::Default)
    // Ok(Solution::new(module_path!(), solve, None))
}

#[allow(dead_code, unreachable_code, unused_variables, unused_mut)] // TODO
fn solve(path: &Path, part: Part) -> Result<SolutionResult, Error> {
    let mut benchmark = BenchmarkResult::new_and_start(part);
    todo!();
    benchmark.stop_parse_start_part();
    Ok(match part {
        Part::Part(1) => SolutionResult::part1(todo!(), benchmark.stop_part_and_owned()),
        Part::Part(2) => SolutionResult::part2(todo!(), benchmark.stop_part_and_owned()),
        Part::All => SolutionResult::part1and2(todo!(), todo!(), benchmark.stop_part_and_owned()),
        _ => {
            return Err(Error::Default);
        }
    })
}
