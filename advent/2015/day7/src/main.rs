use anyhow::Result;
use pest::{
    Parser,
    iterators::Pair};
use pest_derive::Parser;
use std::collections::{BTreeMap, HashSet};
use std::fmt;

type Wire = String;
type Signal = u16;

#[derive(Debug)]
enum Input {
    Constant(Signal),
    Connection(Wire)
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Input::Constant(n) => write!(f, "{}", n),
            Input::Connection(w) => write!(f, "{}", w),
        }
    }
}

impl Input {
    fn get(self: &Input, sigs: &BTreeMap<Wire, Signal>) -> Option<Signal> {
        match self {
            Input::Constant(c) => Some(*c),
            Input::Connection(w) => sigs.get(w).map(Clone::clone),
        }
    }
}

struct Component {
    ins: Vec<Input>,
    op: fn(&[u16]) -> u16,
    out: Wire,
}

impl Component {
    fn run(self: &Component, sigs: &mut BTreeMap<Wire, Signal>) {
        //let strs = self.ins.iter().map(|i| i.to_string()).collect::<Vec<String>>();
        //println!("{} -> {}", strs.join(", "), self.out);
        let ins = self.ins.iter().map(|i| i.get(sigs)).collect::<Option<Vec<u16>>>();
        if let Some(ins) = ins {
            //this clone is dumb
            sigs.insert(self.out.clone(), (self.op)(&ins));
        }
    }
}

fn parse_input(p: Pair<Rule>) -> Input {
    let inner = p.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::num => Input::Constant(inner.as_str().parse::<u16>().unwrap()),
        Rule::wire => Input::Connection(inner.as_str().to_string()),
        _ => unreachable!()
    }
}

fn parse_component(p: Pair<Rule>) -> Component {
    let mut inner = p.into_inner();
    let conn = inner.next().unwrap();
    let out = inner.next().unwrap().as_str().to_string();
    match conn.as_rule() {
        Rule::binop => {
            let mut inner = conn.into_inner();
            let in1 = parse_input(inner.next().unwrap());
            let op = inner.next().unwrap();
            let in2 = parse_input(inner.next().unwrap());
            let make = |op| Component {
                ins: vec!(in1, in2),
                op: op,
                out: out,
            };

            match op.as_str() {
                "AND" => make(|i| i[0] & i[1]),
                "OR" => make(|i| i[0] | i[1]),
                "RSHIFT" => make(|i| i[0] >> i[1]),
                "LSHIFT" => make(|i| i[0] << i[1]),
                _ => unreachable!(),
            }
        },
        Rule::not => {
            let in1 = parse_input(conn.into_inner().next().unwrap());
            Component {
                ins: vec!(in1),
                op: |i| !i[0],
                out: out,
            }
        },
        Rule::input => {
            let in1 = parse_input(conn);
            Component {
                ins: vec!(in1),
                op: |i| i[0],
                out: out,
            }
        },
        _ => unreachable!()
    }
}

fn parse_components(i: &str) -> Result<Vec<Component>> {
    let parsed = RuleParser::parse(Rule::connections, i)?.next().unwrap();
    Ok(parsed.
        into_inner().
        map(parse_component).
        collect::<Vec<Component>>())
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct RuleParser;

fn all_set(signals: &BTreeMap<Wire, Signal>, wires: &HashSet<Wire>) -> bool {
    for w in wires {
        if !signals.contains_key(w) {
            return false;
        }
    }
    true
}

fn run_circuit(comps: &Vec<Component>) -> BTreeMap<Wire, Signal> {
    let mut signals = BTreeMap::new();
    let mut wires = HashSet::new();

    for comp in comps {
        wires.insert(comp.out.clone());
        for input in &comp.ins {
            if let Input::Connection(w) = input {
                wires.insert(w.clone());
            }
        }
    }

    while !all_set(&signals, &wires) {
        for comp in comps {
            comp.run(&mut signals);
        }
    }

    signals
}

fn main() -> anyhow::Result<()> {
    let components = parse_components(include_str!("input.txt"))?;
    println!("len: {}", components.len());
    let signals = run_circuit(&components);

    println!("a is {}", signals["a"]);

    Ok(())
}
