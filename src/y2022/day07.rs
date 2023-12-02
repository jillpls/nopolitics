use nopolitics::benchmark::BenchmarkResult;
use nopolitics::{Error, Part, Solution, SolutionResult};
use std::collections::HashMap;
use std::path::Path;

pub fn create_solution() -> Result<Solution, Error> {
    Ok(Solution::new(module_path!(), solve, None))
}
fn solve(path: &Path, part: Part) -> Result<SolutionResult, Error> {
    let mut benchmark = BenchmarkResult::new_and_start(part);
    let lines = nopolitics::parse::file_to_lines(path)?;
    benchmark.stop_parse_start_part();
    let structure = build_structure(&lines);
    Ok(match part {
        Part::Part(1) => SolutionResult::part1(part1(&structure), benchmark.stop_part_and_owned()),
        Part::Part(2) => SolutionResult::part2(part2(&structure), benchmark.stop_part_and_owned()),
        Part::All => SolutionResult::part1and2(
            part1(&structure),
            part2(&structure),
            benchmark.stop_part_and_owned(),
        ),
        _ => {
            return Err(Error::Default);
        }
    })
}

#[derive(Default, Debug)]
struct Dir {
    name: String,
    size: i64,
    parent: Option<usize>,
    children: Vec<usize>,
}

#[derive(Default, Debug)]
struct FileStructure {
    nodes: Vec<Dir>,
    map: HashMap<String, usize>,
}

fn build_structure(lines: &[String]) -> FileStructure {
    let mut structure = FileStructure::default();
    let mut current_dir: Option<usize> = None;
    for (i, l) in lines.iter().enumerate() {
        if l.starts_with('$') {
            match &l[2..4] {
                "cd" => {
                    let new_dir_name = get_dir_name(l);
                    if new_dir_name == ".." {
                        current_dir = structure.nodes[current_dir.unwrap()].parent;
                        continue;
                    }
                    let parent = current_dir;
                    let new_dir_name = format!(
                        "{}/{}",
                        parent
                            .map(|x| structure.nodes[x].name.as_str())
                            .unwrap_or_default(),
                        new_dir_name
                    );
                    structure
                        .map
                        .insert(new_dir_name.clone(), structure.nodes.len());
                    let new_dir = Dir {
                        name: new_dir_name,
                        size: 0,
                        parent,
                        children: vec![],
                    };
                    if let Some(p) = parent {
                        let idx = structure.nodes.len();
                        structure.nodes[p].children.push(idx);
                    }
                    current_dir = Some(structure.nodes.len());
                    structure.nodes.push(new_dir);
                }
                "ls" => {
                    let size = get_dir_size(&lines[i + 1..]);
                    structure.nodes[current_dir.unwrap()].size = size;
                }
                &_ => {}
            }
        }
    }
    update_sizes(&mut structure, 0);
    structure
}

fn update_sizes(structure: &mut FileStructure, idx: usize) {
    for i in 0..structure.nodes[idx].children.len() {
        let child = structure.nodes[idx].children[i];
        update_sizes(structure, child);
        structure.nodes[idx].size += structure.nodes[child].size;
    }
}

fn part1(structure: &FileStructure) -> String {
    let mut sum = 0;
    for n in &structure.nodes {
        if n.size <= 100000 {
            sum += n.size;
        }
    }
    sum.to_string()
}

fn part2(structure: &FileStructure) -> String {
    let unused = 70_000_000 - structure.nodes[0].size;
    println!("{}", unused);
    structure
        .nodes
        .iter()
        .filter(|x| unused + x.size >= 30_000_000)
        .min_by(|a, b| a.size.cmp(&b.size))
        .unwrap()
        .size
        .to_string()
}

fn get_dir_name(l: &str) -> &str {
    l.split_whitespace().next_back()
        .unwrap_or_default()
        .trim()
        .trim_matches('/')
}

fn get_dir_size(lines: &[String]) -> i64 {
    let mut sizes = vec![];
    for l in lines {
        if l.starts_with('$') {
            break;
        }
        if let Some(Ok(v)) = l.split_whitespace().next().map(|x| x.parse::<i64>()) {
            sizes.push(v);
        }
    }
    sizes.iter().sum()
}
