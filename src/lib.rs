use std::*;
use std::collections::HashSet;
use std::fmt::Formatter;

pub struct Graph {
    nodes   : HashSet<Node>,
    edges   : HashSet<(Node, Node)>,
    lines   : Vec<String>,
}

impl Graph {

    pub fn node(&mut self, name : &str) -> &mut Self {
        self.nodes.insert(name.to_string());
        self
    }
    
    pub fn edge(&mut self, head : &str, tail : &str) -> &mut Self {
        if self.edges.insert((head.to_string(), tail.to_string())) {
            self.nodes.insert(head.to_string());
            self.nodes.insert(tail.to_string());
        }
        self
    }
    
    pub fn line(&mut self, line : &str) -> &mut Self {
        self.lines.push(line.to_string());
        self
    }
    
    pub fn new() -> Self {
        Graph {
            nodes: HashSet::new(),
            edges: HashSet::new(),
            lines: Vec::new(),
        }
    }
    
    pub fn write_dot(&self, f : &mut Formatter) -> fmt::Result {
        try!(writeln!(f, "graph {{"));
        for node in self.nodes.iter() {
            try!(writeln!(f, "  {};", node));
        }
        for edge in self.edges.iter() {
            let &(ref head, ref tail) = edge;
            try!(writeln!(f, "  {} -- {};", head, tail));
        }
        for line in self.lines.iter() {
            try!(writeln!(f, "  {};", line));
        }
        try!(writeln!(f, "}}"));
        Ok(())
    }
}

type Node = String;

impl fmt::Display for Graph {
    fn fmt(&self, f : &mut Formatter) -> fmt::Result {
        self.write_dot(f)
    }
}
