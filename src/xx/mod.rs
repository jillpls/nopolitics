use nopolitics::Solution;
use std::collections::HashMap;

pub(crate) mod day01;
pub(crate) mod day02;
pub(crate) mod day03;
pub(crate) mod day04;
pub(crate) mod day05;
pub(crate) mod day06;
pub(crate) mod day07;
pub(crate) mod day08;
pub(crate) mod day09;
pub(crate) mod day10;
pub(crate) mod day11;
pub(crate) mod day12;
pub(crate) mod day13;
pub(crate) mod day14;
pub(crate) mod day15;
pub(crate) mod day16;
pub(crate) mod day17;
pub(crate) mod day18;
pub(crate) mod day19;
pub(crate) mod day20;
pub(crate) mod day21;
pub(crate) mod day22;
pub(crate) mod day23;
pub(crate) mod day24;
pub(crate) mod day25;

#[cfg(test)]
mod test {
    use super::*;
    use nopolitics::SolutionTester;

    #[test]
    pub fn test_all() {
        let solutions = create_solutions();
        for (_, sol) in solutions
            .into_iter()
            .collect::<std::collections::BTreeMap<_, _>>()
        {
            SolutionTester::new(sol).test_all();
        }
    }
}

pub(crate) fn create_solutions() -> HashMap<usize, Solution> {
    let mut map = HashMap::new();
    map.insert(1, day01::create_solution());
    map.insert(2, day02::create_solution());
    map.insert(3, day03::create_solution());
    map.insert(4, day04::create_solution());
    map.insert(5, day05::create_solution());
    map.insert(6, day06::create_solution());
    map.insert(7, day07::create_solution());
    map.insert(8, day08::create_solution());
    map.insert(9, day09::create_solution());
    map.insert(10, day10::create_solution());
    map.insert(11, day11::create_solution());
    map.insert(12, day12::create_solution());
    map.insert(13, day13::create_solution());
    map.insert(14, day14::create_solution());
    map.insert(15, day15::create_solution());
    map.insert(16, day16::create_solution());
    map.insert(17, day17::create_solution());
    map.insert(18, day18::create_solution());
    map.insert(19, day19::create_solution());
    map.insert(20, day20::create_solution());
    map.insert(21, day21::create_solution());
    map.insert(22, day22::create_solution());
    map.insert(23, day23::create_solution());
    map.insert(24, day24::create_solution());
    map.insert(25, day25::create_solution());
    map.into_iter()
        .filter(|(_, x)| x.is_ok())
        .map(|(i, x)| (i, x.unwrap()))
        .collect::<_>()
}
