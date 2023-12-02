use std::cmp::max;
use nopolitics::benchmark::BenchmarkResult;
use nopolitics::{Error, Part, Solution, SolutionResult};
use std::path::{Path};

pub fn create_solution() -> Result<Solution, Error> {
    Ok(Solution::new(module_path!(), solve, None))
}

#[allow(dead_code, unreachable_code, unused_variables, unused_mut)] // TODO
fn solve(path: &Path, part: Part) -> Result<SolutionResult, Error> {
    let mut benchmark = BenchmarkResult::new_and_start(part);
    let games = nopolitics::parse::file_to_lines(path)?.iter().map(|s|
        {
            let (game_id, values) = s.split_once(':').unwrap();
            let game_id = game_id.split_whitespace().next_back().unwrap().parse::<usize>().unwrap();
            let reveals = values.split(';').map(|v| {
                v.split(',').map(|s| {
                    let v = s.trim().split_once(' ').unwrap();
                    (v.0.parse::<usize>().unwrap(), match v.1 {
                        "red" => 0usize,
                        "green" => 1usize,
                        "blue" => 2usize,
                        _ => unreachable!()
                    })
                }).collect::<Vec<_>>()
            }).collect::<Vec<_>>();
            (game_id, reveals)
        }).collect::<Vec<_>>();
    benchmark.stop_parse_start_part();
    Ok(match part {
        Part::Part(1) => SolutionResult::part1(part1(&games), benchmark.stop_part_and_owned()),
        Part::Part(2) => SolutionResult::part2(part2(&games), benchmark.stop_part_and_owned()),
        Part::All => SolutionResult::part1and2(part1(&games), part2(&games), benchmark.stop_part_and_owned()),
        _ => {
            return Err(Error::Default);
        }
    })
}

type Games = [(usize, Vec<Vec<(usize, usize)>>)];
fn part1(games : &Games) -> String {
let cubes: [usize; 3] = [12, 13, 14];
games.iter().fold(0, |prev, (val, next)| {
prev + if next.iter().all(|inner| {
inner.iter().all(|(amount, kind)| *amount <= cubes[*kind] )
}) {
*val
} else {
0
}
}).to_string()
}

fn part2(games : &Games) -> String {
    games.iter().fold(0, |prev, (_, next)| {
        let mins = next.iter().fold([0;3], |prev, next| {
            let next = next.iter().fold([0;3], |mut prev, next| {
                prev[next.1] = next.0;
                prev
            });
            [max(prev[0], next[0]),max(prev[1],next[1]),max(prev[2],next[2])]
        });
        prev + mins.iter().product::<usize>()
    }).to_string()
}
