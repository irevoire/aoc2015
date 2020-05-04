use std::collections::HashMap;
use std::io::BufRead;

pub fn parse<R: BufRead>(reader: R) -> HashMap<String, Value> {
    reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let mut split = line.rsplit("->").map(str::trim);
            (
                split.next().unwrap().to_string(),
                Value::Lazy(split.next().unwrap().to_string()),
            )
        })
        .collect()
}

/// used to memoize what has already been computed
#[derive(Debug, Clone)]
pub enum Value {
    Lazy(String),
    Done(u16),
}

impl Value {
    pub fn compute(&self, env: &mut HashMap<String, Value>) -> Value {
        match self {
            Value::Lazy(s) => Value::Done(execute(s, env)),
            done => done.clone(),
        }
    }

    pub fn unwrap(&self) -> u16 {
        match self {
            Value::Done(a) => *a,
            _ => panic!("ntm"),
        }
    }
}

pub fn execute(stmt: &str, env: &mut HashMap<String, Value>) -> u16 {
    let mut split = stmt.split(' ');
    let a = split.next().expect("I need at least one element");
    let b = split.next();
    let c = split.next();

    match (a, b, c) {
        ("NOT", Some(s), _) => !execute(s, env),
        (s, None, None) => s.parse().unwrap_or_else(|_err| {
            let value = env.get(s).unwrap().clone();
            let value = value.compute(env);
            env.insert(s.to_string(), value.clone());
            value.unwrap()
        }),

        (left, Some("AND"), Some(right)) => execute(left, env) & execute(right, env),
        (left, Some("OR"), Some(right)) => execute(left, env) | execute(right, env),
        (left, Some("RSHIFT"), Some(right)) => execute(left, env) >> execute(right, env),
        (left, Some("LSHIFT"), Some(right)) => execute(left, env) << execute(right, env),
        allo => panic!("Unexpected entry: {:?}", allo),
    }
}
