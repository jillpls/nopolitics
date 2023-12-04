use std::collections::HashSet;
use nopolitics::benchmark::BenchmarkResult;
use nopolitics::{Error, Part, Solution, SolutionResult};
use std::path::Path;

pub fn create_solution() -> Result<Solution, Error> {
    Ok(Solution::new(module_path!(), solve, None))
}

#[allow(dead_code, unreachable_code, unused_variables, unused_mut)] // TODO
fn solve(path: &Path, part: Part) -> Result<SolutionResult, Error> {
    let mut benchmark = BenchmarkResult::new_and_start(part);
    let lines = nopolitics::parse::file_to_lines(path)?.iter().map(|w| {
        let split = w.split_once(':').unwrap().1.split_once('|').unwrap();

        (split.0.split_whitespace().filter_map(|x| x.parse::<usize>().ok()).collect::<HashSet<_>>(),
            split.1.split_whitespace().filter_map(|x| x.parse::<usize>().ok()).collect::<HashSet<_>>()

        )}).collect::<Vec<_>>();
    benchmark.stop_parse_start_part();
    let wins = lines.iter().map(|(win, actual)| win.intersection(actual).count()).collect::<Vec<_>>();
    Ok(match part {
        Part::Part(1) => SolutionResult::part1(part1(&wins), benchmark.stop_part_and_owned()),
        Part::Part(2) => SolutionResult::part2(part2(&wins), benchmark.stop_part_and_owned()),
        Part::All => SolutionResult::part1and2(part1(&wins), part2(&wins), benchmark.stop_part_and_owned()),
        _ => {
            return Err(Error::Default);
        }
    })
}

fn part1(wins: &[usize]) -> String {
    wins.iter().fold(0usize, |prev, next | {
        prev + if *next > 0 {
            2usize.pow(*next as u32 - 1)
        } else {
            0
        }
    }).to_string()
}

fn part2(wins: &[usize]) -> String {
    let mut copies = vec![1;wins.len()];
    wins.iter().enumerate().for_each(|(i, w)| {
        (0..copies[i]).for_each(|_|
        (i+1..i+1+*w).for_each(|idx| {
            if let Some(v) = copies.get_mut(idx) {
                *v += 1;
            }
        }
        )
        );

    });
    copies.iter().sum::<usize>().to_string()

}
