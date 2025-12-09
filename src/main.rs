use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::collections::HashSet;

fn trace<'a>(root: &'a Value) -> (Vec<&'a Value>, Vec<(&'a Value, &'a Value)>) {
    fn build<'a>(
        v: &'a Value,
        nodes: &mut Vec<&'a Value>,
        edges: &mut Vec<(&'a Value, &'a Value)>,
        seen: &mut HashSet<*const Value>,
    ) {
        let ptr = v as *const Value;
        if !seen.insert(ptr) {
            return;
        }
        nodes.push(v);
        for child in &v._children {
            edges.push((child, v));
            build(child, nodes, edges, seen);
        }
    }

    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut seen = HashSet::new();
    build(root, &mut nodes, &mut edges, &mut seen);
    (nodes, edges)
}
fn to_dot(root: &Value, rankdir: &str) -> String {
    assert!(matches!(rankdir, "LR" | "TB"));

    let (nodes, edges) = trace(root);
    let mut s = String::new();
    s.push_str("digraph G {\n");
    s.push_str(&format!("  rankdir={};\n", rankdir));
    s.push_str("  node [shape=record];\n");

    for n in &nodes {
        let id = format!("{:p}", *n as *const Value);
        s.push_str(&format!("  \"{}\" [label=\"{{ data {} }}\"];\n", id, n.data));

        if let Some(op) = &n._operation {
            let op_id = format!("{}{}", id, op);
            s.push_str(&format!("  \"{}\" [label=\"{}\"];\n", op_id, op));
            s.push_str(&format!("  \"{}\" -> \"{}\";\n", op_id, id));
        }
    }

    for (n1, n2) in &edges {
        if let Some(op) = &n2._operation {
            let from = format!("{:p}", *n1 as *const Value);
            let to = format!("{}{}", format!("{:p}", *n2 as *const Value), op);
            s.push_str(&format!("  \"{}\" -> \"{}\";\n", from, to));
        }
    }

    s.push_str("}\n");
    s
}

#[derive(Debug)]
struct Value {
    data: i32,
    _children: Vec<Value>,
    _operation: Option<String>,
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Value {
            data: self.data + other.data,
            _children: vec![self, other],
            _operation: Some(String::from("+")),
        }
    }
}

impl AddAssign for Value {
    fn add_assign(&mut self, other: Self) {
        self.data += other.data;

    }

}

impl Sub for Value {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Value {
            data: self.data - other.data,
            _children: vec![self, other],
            _operation: Some(String::from("-")),
        }
    }
}

impl SubAssign for Value {
    fn sub_assign(&mut self, other: Self) {
        self.data -= other.data;
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Value {
            data: self.data * other.data,
            _children: vec![self, other],
            _operation: Some(String::from("*")),
        }
    }
}

impl MulAssign for Value {
    fn mul_assign(&mut self, other: Self) {
        self.data *= other.data;
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Value {
            data: self.data / other.data,
            _children: vec![self, other],
            _operation: Some(String::from("/")),
        }
    }
}

impl DivAssign for Value {
    fn div_assign(&mut self, other: Self) {
        self.data /= other.data;
    }
}

fn main() {
    println!("Hello, world!");
    let value_1 = Value { data: 10, _children: vec![], _operation: None };
    let value_2 = Value { data: 5, _children: vec![], _operation: None };
    let sum = value_1 + value_2;
    println!("Sum: {:?}", sum);
    let dot = to_dot(&sum, "LR");
    std::fs::write("graph.dot", dot).unwrap();
    println!("Wrote compute graph to graph.dot");
}


