use nopolitics::benchmark::BenchmarkResult;
use nopolitics::{Error, Part, Solution, SolutionResult};
use std::path::PathBuf;

pub fn create_solution() -> Result<Solution, Error> {
    Ok(Solution::new(module_path!(), solve, None))
}

#[allow(dead_code, unreachable_code, unused_variables, unused_mut)] // TODO
fn solve(path: &PathBuf, part: Part) -> Result<SolutionResult, Error> {
    let mut benchmark = BenchmarkResult::new_and_start(part);
    let mut triangles = nopolitics::parse::file_to_lines(&path)?
        .iter()
        .map(|s| {
            s.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    benchmark.stop_parse_start_part();
    Ok(match part {
        Part::Part(1) => SolutionResult::part1(part1(triangles), benchmark.stop_part_and_owned()),
        Part::Part(2) => SolutionResult::part2(part2(triangles), benchmark.stop_part_and_owned()),
        Part::All => SolutionResult::part1and2(
            part1(triangles.clone()),
            part2(triangles),
            benchmark.stop_part_and_owned(),
        ),
        _ => {
            return Err(Error::Default);
        }
    })
}

fn part1(mut triangles: Vec<Vec<u32>>) -> String {
    triangles
        .iter_mut()
        .filter_map(|v| {
            v.sort();
            if v[0] + v[1] > v[2] {
                Some(v)
            } else {
                None
            }
        })
        .count()
        .to_string()
}

fn part2(triangles: Vec<Vec<u32>>) -> String {
    let vertical_triangles = triangles
        .chunks(3)
        .map(|v| {
            vec![
                v[0][0], v[1][0], v[2][0], v[0][1], v[1][1], v[2][1], v[0][2], v[1][2], v[2][2],
            ]
        })
        .flatten()
        .collect::<Vec<u32>>()
        .chunks(3)
        .map(|x| x.to_vec())
        .collect();
    part1(vertical_triangles)
}
