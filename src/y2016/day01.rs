use nopolitics::benchmark::BenchmarkResult;
use nopolitics::{parse::ParseError, Error, Matrix2, Part, Point2, Solution, SolutionResult};
use num_traits::One;
use std::collections::HashSet;
use std::path::Path;

pub fn create_solution() -> Result<Solution, Error> {
    Ok(Solution::new(module_path!(), solve, None))
}

#[allow(dead_code, unreachable_code, unused_variables, unused_mut)] // TODO
fn solve(path: &Path, part: Part) -> Result<SolutionResult, Error> {
    let mut benchmark = BenchmarkResult::new_and_start(part);
    let s = std::fs::read_to_string(path).map_err(|e| Error::Parse(ParseError::from(e)))?;
    let steps = s
        .split(',')
        .map(|x| {
            (
                match x.trim().chars().next().unwrap() {
                    'R' => Matrix2::rot90(),
                    'L' => Matrix2::rot270(),
                    _ => Matrix2::one(),
                },
                x.trim()[1..].parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<(Matrix2<i32>, i32)>>();
    benchmark.stop_parse_start_part();
    Ok(match part {
        Part::Part(1) => SolutionResult::part1(part1(&steps), benchmark.stop_part_and_owned()),
        Part::Part(2) => SolutionResult::part2(part2(&steps), benchmark.stop_part_and_owned()),
        Part::All => SolutionResult::part1and2(
            part1(&steps),
            part2(&steps),
            benchmark.stop_part_and_owned(),
        ),
        _ => {
            return Err(Error::Default);
        }
    })
}

fn part1(steps: &[(Matrix2<i32>, i32)]) -> String {
    let r = steps
        .iter()
        .fold(
            (Point2::new(0, 0), Point2::new(1, 0)),
            |(pos, dir), (rot, amount)| {
                let new_rot = rot * dir;
                (pos + new_rot * *amount, new_rot)
            },
        )
        .0;
    (r.x.abs() + r.y.abs()).to_string()
}

fn part2(steps: &[(Matrix2<i32>, i32)]) -> String {
    let mut pos = Point2::new(0, 0);
    let mut visited = HashSet::from([[0, 0]]);
    let mut dir = Point2::new(1, 0);
    steps.iter().find(|(rot, amount)| {
        dir = rot * dir;
        (0..*amount).any(|_| {
            pos = pos + dir;
            !visited.insert(pos.to_array())
        })
    });
    (pos.x.abs() + pos.y.abs()).to_string()
}
