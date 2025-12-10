use std::ops::{Add, Div, Mul, Sub};
use std::collections::HashSet;
use std::{cell::RefCell, rc::{Rc}};

fn trace(root: &Value) -> (Vec<Value>, Vec<(Value, Value)>) {
    fn build(
        v: &Value,
        nodes: &mut Vec<Value>,
        edges: &mut Vec<(Value, Value)>,
        seen: &mut HashSet<*const RefCell<Node>>,
    ) {
        let ptr = Rc::as_ptr(&v.0);
        if !seen.insert(ptr) {
            return;
        }

        nodes.push(v.clone());

        for child in v.children() {
            edges.push((child.clone(), v.clone()));
            build(&child, nodes, edges, seen);
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
        let id = format!("{:p}", Rc::as_ptr(&n.0));
        let node = n.0.borrow();

        s.push_str(&format!(
            "  \"{}\" [label=\"{{ {} | data: {} | grad: {} }}\"];\n",
            id, node.label, node.data, node.grad
        ));

        if let Some(op) = &node._operation {
            let op_id = format!("{}{}", id, op);
            s.push_str(&format!("  \"{}\" [label=\"{}\"];\n", op_id, op));
            s.push_str(&format!("  \"{}\" -> \"{}\";\n", op_id, id));
        }
    }

    for (n1, n2) in &edges {
        let n2_node = n2.0.borrow();
        if let Some(op) = &n2_node._operation {
            let from = format!("{:p}", Rc::as_ptr(&n1.0));
            let to = format!("{}{}", format!("{:p}", Rc::as_ptr(&n2.0)), op);
            s.push_str(&format!("  \"{}\" -> \"{}\";\n", from, to));
        }
    }

    s.push_str("}\n");
    s
}

struct Node {
    data: f32,
    grad:f32,
    label: String,
    _children: Vec<Value>,
    _operation: Option<String>,
    _backward: Option<Box<dyn Fn()>>,

}
#[derive(Clone)]
struct Value(Rc<RefCell<Node>>);

impl Value {

    fn new(data: f32, label: impl Into<String>) -> Self {
        Value(Rc::new(RefCell::new(Node {
            data,
            grad: 0.0,
            label: label.into(),
            _children: vec![],
            _operation: None,
            _backward: None,
        })))
    }

    fn data(&self) -> f32 {
        self.0.borrow().data
    }
    fn grad(&self) -> f32 {
        self.0.borrow().grad
    }

    fn set_label(&self, s: impl Into<String>) {
        self.0.borrow_mut().label = s.into();
    }

    fn add_grad(&self, g: f32) {
        self.0.borrow_mut().grad += g;
    }

    fn children(&self) -> Vec<Value> {
        self.0.borrow()._children.clone()
    }

    fn operation(&self) -> Option<String> {
        self.0.borrow()._operation.clone()
    }

    fn tanh(&self) -> Self {
        let t = self.data().tanh();
        let out = Value::new(t, String::new());

        {
            let mut out_borrow = out.0.borrow_mut();
            out_borrow._children.push(self.clone());
            out_borrow._operation = Some(String::from("tanh"));
        }
        out
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        let sum = Value::new(self.data() + other.data(), "");

        {
            let mut sum_node = sum.0.borrow_mut();
            sum_node._children.push(self.clone());
            sum_node._children.push(other.clone());
            sum_node._operation = Some(String::from("+"));


        }
        sum
    }
}
impl Sub for Value {
    type Output = Value;

    fn sub(self, other: Value) -> Value {
        let difference = Value::new(self.data() - other.data(), "");

        {
            let mut difference_node = difference.0.borrow_mut();
            difference_node._children.push(self.clone());
            difference_node._children.push(other.clone());
            difference_node._operation = Some(String::from("-"));
        }
        difference
    }
}


impl Mul for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Value {
        let product = Value::new(self.data() * other.data(), String::new());
        {
            let mut product_borrow = product.0.borrow_mut();
            product_borrow._children.push(self.clone());
            product_borrow._children.push(other.clone());
            product_borrow._operation = Some(String::from("*"));
        }
        product
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, other: Value) -> Value {
        let quotient = Value::new(self.data() / other.data(), String::new());
        {
            let mut quotient_borrow = quotient.0.borrow_mut();
            quotient_borrow._children.push(self.clone());
            quotient_borrow._children.push(other.clone());
            quotient_borrow._operation = Some(String::from("/"));
        }
        quotient
    }
}


fn main() {
    let x1 = Value::new(2.0, "x1");
    let x2 = Value::new(0.0, "x2");
    let w1 = Value::new(-3.0, "w1");
    let w2 = Value::new(1.0, "w2");
    let b  = Value::new(6.8813735, "b");

    let x1w1 = x1.clone() * w1.clone();
    x1w1.set_label("x1w1");

    let x2w2 = x2.clone() * w2.clone();
    x2w2.set_label("x2w2");

    let sum = x1w1.clone() + x2w2.clone() + b.clone();
    sum.set_label("sum");

    let activated = sum.tanh();
    activated.set_label("activated");

    let dot = to_dot(&activated, "LR");
    std::fs::write("graph.dot", dot).unwrap();
}