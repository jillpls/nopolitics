use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use nopolitics::benchmark::BenchmarkResult;
use nopolitics::{Error, Part, Solution, SolutionResult};
use std::path::Path;

pub fn create_solution() -> Result<Solution, Error> {
    Ok(Solution::new(module_path!(), solve, None))
}

#[allow(dead_code, unreachable_code, unused_variables, unused_mut)] // TODO
fn solve(path: &Path, part: Part) -> Result<SolutionResult, Error> {
    let mut benchmark = BenchmarkResult::new_and_start(part);
    let mut list = InstructionList { map: HashMap::new() };
    nopolitics::parse::file_to_lines(path)?.iter().for_each(|l| {
        let (name, calc) = l.split_once(':').unwrap();
        if let Ok(v) = calc.trim().parse::<i64>() {
            list.map.insert(name.trim().to_string(),  Instruction::Assign(v));
        } else {
            let split = calc.split_whitespace().collect::<Vec<_>>();
            let op1 = Reference::Other(split[0].to_string());
            let op2 = Reference::Other(split[2].to_string());
            list.map.insert(name.trim().to_string(), match split[1] {
                "+" => Instruction::Add(op1, op2),
                "*" => Instruction::Mul(op1, op2),
                "-" => Instruction::Sub(op1, op2),
                "/" => Instruction::Div(op1, op2),
                _ => { unreachable!() }
            });
        }
    });
    benchmark.stop_parse_start_part();
    Ok(match part {
        Part::Part(1) => SolutionResult::part1(list.calc_result_p1("root").unwrap().to_string(), benchmark.stop_part_and_owned()),
        Part::Part(2) => SolutionResult::part2(part2(&list), benchmark.stop_part_and_owned()),
        Part::All => SolutionResult::part1and2(todo!(), todo!(), benchmark.stop_part_and_owned()),
        _ => {
            return Err(Error::Default);
        }
    })
}

fn part2(list: &InstructionList) -> String {
    let mut list = list.clone();
    list.build_dependencies("root", 0);
    let root =  list.map.get_mut("root").unwrap();
    let (op1, op2) = match root {
        Instruction::Add(op1, op2) |
        Instruction::Mul(op1, op2) |
        Instruction::Sub(op1, op2) |
        Instruction::Div(op1, op2) |
        Instruction::Eql(op1, op2) => {
            (op1.clone(), op2.clone())
        }
        _ => { unreachable!() }
    };
    *root = Instruction::Eql(op1, op2);

    list.simplify(Some("humn".to_string()));
    let deps = list.build_dependencies("root", 0);
    let r = list.inverse_for_key("humn", &deps);
    let r = InstructionList {map : r};
    let _ = r.build_dependencies("humn", 0);
    let r = r.calc_result_p1("humn");
    r.unwrap().to_string()
}

#[derive(Clone, Eq, PartialEq)]
enum Reference {
    Other(String),
    Literal(i64)
}

impl Debug for Reference{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Reference::Other(v) => { write!(f, "{}", v)}
            Reference::Literal(v) => {write!(f, "{}", v)}
        }
    }
}

#[derive(Clone)]
enum Instruction {
    Assign(i64),
    Add(Reference, Reference),
    Mul(Reference, Reference),
    Sub(Reference, Reference),
    Div(Reference, Reference),
    Eql(Reference, Reference),
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Assign(v) => { write!(f, "{}", v)}
            Instruction::Add(op1, op2) |
            Instruction::Mul(op1, op2) |
            Instruction::Sub(op1, op2) |
            Instruction::Div(op1, op2) |
            Instruction::Eql(op1, op2)  => {
                let operator = match self {
                    Instruction::Assign(_) => { unreachable!() }
                    Instruction::Add(_, _) => { "+"}
                    Instruction::Mul(_, _) => { "*"}
                    Instruction::Sub(_, _) => {"-"}
                    Instruction::Div(_, _) => {"/"}
                    Instruction::Eql(_, _) => {"=="}
                };
                write!(f, "{:?}{}{:?}", op1, operator, op2)
            }
        }
    }
}

impl Instruction {
    pub(crate) fn calc_if_possible(&self) -> Option<i64> {
        self.get_2_op().and_then(|ops| {
            match ops {
                (Reference::Literal(i), Reference::Literal(j)) => {
                    Some(
                    self.calc(*i, *j))
                },
                _ => None
            }
        })
    }

    fn calc(&self, val1: i64, val2: i64) -> i64 {
        match self {
            Instruction::Assign(_) => { 0 }
            Instruction::Add(_, _) => { val1 + val2 }
            Instruction::Mul(_, _) => { val1 * val2 }
            Instruction::Sub(_, _) => { val1 - val2 }
            Instruction::Div(_, _) => { val1 / val2}
            Instruction::Eql(_, _) => { 0 }
        }
    }

    fn get_2_op(&self) -> Option<(&Reference, &Reference)> {
        match self {
            Instruction::Add(op1, op2) |
            Instruction::Mul(op1, op2) |
            Instruction::Sub(op1, op2) |
            Instruction::Div(op1, op2) |
            Instruction::Eql(op1, op2) => Some((op1, op2)),
            _ => None
        }
    }

    pub(crate) fn set_values(&mut self, n1: Reference, n2: Reference) {
        match self {
            Instruction::Assign(_) => {}
            Instruction::Add(op1, op2)|
            Instruction::Mul(op1, op2)|
            Instruction::Sub(op1, op2)|
            Instruction::Div(op1, op2)|
            Instruction::Eql(op1, op2) => {
                *op1 = n1;
                *op2 = n2;
            }
        }
    }
}

#[derive(Debug, Clone)]
struct InstructionList {
    map: HashMap<String, Instruction>
}

impl InstructionList {
    pub fn inverse_for_key(&self, key: &str, deps: &HashMap<String,String>) -> HashMap<String, Instruction> {
        let dep_name = match deps.get(key) {
            Some(v) => v,
            _ => return HashMap::new(),
        };
        let i = self.map.get(dep_name).unwrap();
        let mut map = self.inverse_for_key(dep_name, deps);
        let as_ref = Reference::Other(key.to_string());
        match i {
            Instruction::Mul(op1, op2) |
            Instruction::Sub(op1, op2) |
            Instruction::Div(op1, op2) |
            Instruction::Add(op1, op2) |
            Instruction::Eql(op1, op2) => {

                let (_, v) = match (&op1, &op2) {
                    (Reference::Literal(v), o) | (o, Reference::Literal(v)) => { (o, v) }
                    _ => { unreachable!(); }
                };
                let calc = match i {
                    Instruction::Add(_, _) => {
                        Instruction::Sub(Reference::Other(dep_name.to_string()), Reference::Literal(*v))
                    }
                    Instruction::Mul(_, _) => {
                        Instruction::Div(Reference::Other(dep_name.to_string()), Reference::Literal(*v))
                    }
                    Instruction::Sub(_, _) => {
                        if op1 == &as_ref {
                            Instruction::Add(Reference::Other(dep_name.to_string()), Reference::Literal(*v))
                        } else {
                            Instruction::Sub(Reference::Literal(*v), Reference::Other(dep_name.to_string()))
                        }
                    }
                    Instruction::Div(_, _) => {
                        if op1 == &as_ref {
                            Instruction::Mul(Reference::Other(dep_name.to_string()), Reference::Literal(*v))
                        } else {
                            Instruction::Div(Reference::Literal(*v), Reference::Other(dep_name.to_string()))
                        }
                    }
                    Instruction::Eql(_, _) => {
                        let mut r = HashMap::new();
                        r.insert(key.to_string(), Instruction::Assign(*v));
                        return r;
                    }
                    _ => { unreachable!() }
                };
                map.insert(key.to_string(), calc);
            }
            _ => { unreachable!(); }
        }
        map
    }

    pub fn build_dependencies(&self, key: &str, depth: usize) -> HashMap<String, String> {
        let mut result = HashMap::new();

        if let Some(i) = self.map.get(key) {
            match i {
                Instruction::Add(op1, op2)|
                Instruction::Mul(op1, op2)|
                Instruction::Sub(op1, op2)|
                Instruction::Div(op1, op2)|
                Instruction::Eql(op1, op2) => {
                    if let Reference::Other(k) = op1 {
                        result.insert(k.to_string(), key.to_string());
                        let r = self.build_dependencies(k, depth+1);
                        result.extend(r);
                }
                    if let Reference::Other(k) = op2 {
                            result.insert(k.to_string(), key.to_string());
                        let r = self.build_dependencies(k, depth+1);
                        result.extend(r);
                    }
            },
                _ => {}

            }
        }
        result
    }

    pub fn simplify(&mut self, filter: Option<String>) {
        let keys = self.map.keys().filter(|k| if let Some(f) = filter.as_ref() {
            k != &f
        } else {
            true
        }).cloned().collect::<Vec<String>>();
        let mut changed = true;
        while changed {
            changed = false;
            for k in &keys {
                changed |= self.simplify_key(k, &filter);
            }
        }
    }

    fn simplify_key(&mut self, key: &str, filter: &Option<String>) -> bool {
        if let Some(v) = self.map.get_mut(key) {
            if let Some(r) = v.calc_if_possible() {
                *v = Instruction::Assign(r);
                return true;
            }
        }
        if let Some((n1,n2)) = if let Some(i) = self.map.get(key) {
            match i {
                Instruction::Assign(_) => { None }
                Instruction::Add(op1, op2) |
                Instruction::Mul(op1, op2) |
                Instruction::Sub(op1, op2)|
                Instruction::Div(op1, op2) |
                Instruction::Eql(op1, op2) => {
                    let mut nop1 = op1.clone();
                    let mut nop2 = op2.clone();
                    if let Reference::Other(s) = op1 {
                        if Some(s) != filter.as_ref() {
                            let inner = self.map.get(s).unwrap();
                            if let Instruction::Assign(v) = inner {
                                nop1 = Reference::Literal(*v)
                            }
                        }
                    }
                    if let Reference::Other(s) = op2 {
                        if Some(s) != filter.as_ref() {
                            let inner = self.map.get(s).unwrap();
                            if let Instruction::Assign(v) = inner {
                                nop2 = Reference::Literal(*v)
                            }
                        }
                    }
                    if &nop1 != op1 || &nop2 != op2 {
                        Some((nop1, nop2))
                    } else {
                        None
                    }

                }
            }
        } else { None } {
            let v = self.map.get_mut(key).unwrap();
            v.set_values(n1, n2);
            true
        } else {
            false
        }

    }

    pub fn calc_result_p1(&self, key: &str) -> Option<i64> {
        self.map.get(key).map(|inner| {
            match inner {
                Instruction::Assign(value ) => {
                    *value
                }
                Instruction::Add(op1, op2) |
                Instruction::Mul(op1, op2) |
                Instruction::Sub(op1, op2) |
                Instruction::Div(op1, op2) => {
                    let (op1, op2) = match (op1, op2) {
                        (Reference::Literal(v), Reference::Other(o)) => { (*v, self.calc_result_p1(o).unwrap()) }
                        (Reference::Other(o), Reference::Literal(v)) => { (self.calc_result_p1(o).unwrap(), *v) }
                        (Reference::Other(o1), Reference::Other(o2)) => { (self.calc_result_p1(o1).unwrap(),self.calc_result_p1(o2).unwrap())}
                        (Reference::Literal(v1), Reference::Literal(v2)) => { (*v1, *v2) }
                    };
                    match inner {
                        Instruction::Add(_, _) => { op1 + op2 }
                        Instruction::Mul(_, _) => { op1 * op2 }
                        Instruction::Sub(_, _) => { op1 - op2 }
                        Instruction::Div(_, _) => { op1 / op2 }
                        _ => { unreachable!(); }
                    }
                }
                _ => { unreachable!() }
            }
        })
    }
}