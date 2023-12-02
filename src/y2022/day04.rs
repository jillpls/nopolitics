use nopolitics::benchmark::BenchmarkResult;
use nopolitics::parse::vec_to_tuple;
use nopolitics::{Error, Part, Solution, SolutionResult};
use std::path::Path;

pub fn create_solution() -> Result<Solution, Error> {
    Ok(Solution::new(module_path!(), solve, None))
}

fn solve(path: &Path, part: Part) -> Result<SolutionResult, Error> {
    let mut benchmark = BenchmarkResult::new_and_start(part);
    let assignments = nopolitics::parse::file_to_lines(path)?;
    let pairs: Vec<((i32, i32), (i32, i32))> = assignments
        .iter()
        .map(|s| {
            vec_to_tuple(
                &s.split(',')
                    .map(|x| nopolitics::parse::to_coordinate(x, '-').unwrap())
                    .collect::<Vec<(i32, i32)>>(),
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    benchmark.stop_parse_start_part();
    Ok(match part {
        Part::Part(1) => SolutionResult::part1(part1(&pairs), benchmark.stop_part_and_owned()),
        Part::Part(2) => SolutionResult::part2(part2(&pairs), benchmark.stop_part_and_owned()),
        Part::All => SolutionResult::part1and2(
            part1(&pairs),
            part2(&pairs),
            benchmark.stop_part_and_owned(),
        ),
        _ => {
            return Err(Error::Default);
        }
    })
}

type Pair = ((i32, i32), (i32, i32));

fn part1(pairs: &[Pair]) -> String {
    pairs
        .iter()
        .filter(|((l1, h1), (l2, h2))| l1 <= l2 && h1 >= h2 || l2 <= l1 && h2 >= h1)
        .count()
        .to_string()
}
fn part2(pairs: &[Pair]) -> String {
    pairs
        .iter()
        .filter(|((l1, h1), (l2, h2))| h1 >= l2 && l1 <= h2)
        .count()
        .to_string()
}
