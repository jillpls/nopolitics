use clap::Parser;
use nopolitics::benchmark::BenchmarkResult;
use nopolitics::{Error, Part, Solution, SolutionResult};
use std::collections::HashMap;

mod y2016;
mod y2022;
mod y2023;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    all: bool,
    #[arg(short, long)]
    benchmark: bool,
    #[arg(short, long)]
    year: Option<usize>,
    #[arg(short, long)]
    day: Option<usize>,
    #[arg(short, long)]
    part: Option<usize>,
    #[arg(long)]
    parallel: bool,
}

fn main() {
    let mut args = Args::parse();

    if !args.all && args.year.is_none() && args.day.is_none() {
        args.year = Some(2023);
        args.day = Some(1);
    }

    let mut year_solutions: HashMap<usize, _> = HashMap::new();
    year_solutions.insert(2016, y2016::create_solutions());
    year_solutions.insert(2022, y2022::create_solutions());
    year_solutions.insert(2023, y2023::create_solutions());

    let years = if let Some(y) = args.year {
        vec![y]
    } else {
        let mut v = year_solutions.keys().copied().collect::<Vec<_>>();
        v.sort();
        v
    };
    for y in years {
        run_year(y, &year_solutions, &args);
    }
}

fn run_year(year: usize, year_solutions: &HashMap<usize, HashMap<usize, Solution>>, args: &Args) {
    println!("\nRunning year {} ...", year);
    if let Some(solutions) = year_solutions.get(&year) {
        let days = if let Some(y) = args.day {
            vec![y]
        } else {
            let mut v = solutions.keys().copied().collect::<Vec<_>>();
            v.sort();
            v
        };
        let mut benchmark_results = HashMap::new();
        for day in days {
            let r = run_day(day, solutions, args);
            if let Ok(vsr) = r {
                benchmark_results
                    .insert(day, vsr.iter().map(|sr| sr.benchmark).collect::<Vec<_>>());
            }
        }
        if args.benchmark {
            let r = benchmark_results
                .values()
                .map(|x| x.iter().sum::<BenchmarkResult>())
                .sum::<BenchmarkResult>();
            println!("\nOverall time for year {}:\n{}", year, r);
        }
    }
}

fn run_day(
    day: usize,
    solutions: &HashMap<usize, Solution>,
    args: &Args,
) -> Result<Vec<SolutionResult>, Error> {
    println!("\n>Running day {} ...", day);
    if let Some(solution) = solutions.get(&day) {
        let r = match args.part {
            Some(1) => solution.run_main(args.parallel, Part::Part(1)),
            Some(2) => solution.run_main(args.parallel, Part::Part(2)),
            _ => solution.run_main(args.parallel, Part::All),
        };
        if let Ok(res) = &r {
            let mut keys: Vec<_> = res.results.keys().copied().collect();
            keys.sort();
            for k in keys {
                println!("Part {} result: {}", k, res.results[&k]);
            }
            if args.benchmark {
                println!("{}", res.benchmark);
            }
        } else {
            println!("Error: {:?}", r);
        }
        return r.map(|sr| vec![sr]);
    }
    Ok(vec![])
}
