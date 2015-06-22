extern crate graphviz_minimal;
use graphviz_minimal::Graph;

fn main() {
    let mut g = Graph::new();
    g   .edge("a", "b")
        .edge("b", "c")
        .edge("c", "a")
        .line("a [label=bla]");
    println!("{}", g);
}
