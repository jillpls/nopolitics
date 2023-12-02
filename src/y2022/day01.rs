use nopolitics::benchmark::BenchmarkResult;
use nopolitics::{Error, Part, Solution, SolutionResult};
use std::path::Path;

pub fn create_solution() -> Result<Solution, Error> {
    Ok(Solution::new(module_path!(), solve, None))
}

fn solve(path: &Path, part: Part) -> Result<SolutionResult, Error> {
    let mut benchmark = BenchmarkResult::new_and_start(part);
    let bags = nopolitics::parse::split_on_empty_lines(&nopolitics::parse::file_to_lines(path)?)
        .iter()
        .map(|v| {
            v.iter()
                .map(|s| s.parse::<i64>())
                .collect::<Result<Vec<i64>, _>>()
        })
        .collect::<Result<Vec<Vec<i64>>, _>>()?;
    benchmark.stop_parse_start_part();

    let highest = bags.iter().map(|b| b.iter().sum::<i64>());
    Ok(match part {
        Part::Part(1) => SolutionResult::part1(part1(highest)?, benchmark.stop_part_and_owned()),
        Part::Part(2) => SolutionResult::part2(part2(highest), benchmark.stop_part_and_owned()),
        Part::All => SolutionResult::part1and2(
            part1(highest.clone())?,
            part2(highest),
            benchmark.stop_part_and_owned(),
        ),
        _ => return Err(Error::Default),
    })
}

fn part1<I>(iter: I) -> Result<String, Error>
where
    I: Iterator<Item = i64>,
{
    iter.max().map(|v| v.to_string()).ok_or(Error::Default)
}

fn part2<I>(iter: I) -> String
where
    I: Iterator<Item = i64> + DoubleEndedIterator,
{
    iter.rev().take(3).sum::<i64>().to_string()
}
