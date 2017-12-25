use std::collections::{HashMap, HashSet};
use std::collections::VecDeque;

use nom::{alphanumeric, digit};

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq, Hash)]
struct Label(String);

#[derive(Copy, Clone, Debug, Eq, Ord, PartialOrd, PartialEq, Hash)]
struct Weight(u32);

#[derive(Debug, Eq, PartialEq)]
struct Node {
    pub label: Label,
    pub weight: Weight,
    pub neighbors: HashSet<Label>,
}

#[derive(Debug)]
struct Graph(HashMap<Label, Node>);

named!(label_parser<&str, Label>, do_parse!(
    s: alphanumeric >>
    (Label(String::from(s)))
));

named!(weight_parser<&str, Weight>, do_parse!(
    s: digit >>
    (Weight(s.parse().expect("parse as u32")))
));

named!(node_parser<&str, Node>, ws!(do_parse!(
    label: label_parser >>
    weight: delimited!(char!('('), weight_parser, char!(')')) >>
     // this complete!() is necessary:
     // https://github.com/Geal/nom/blob/master/doc/upgrading_to_nom_2.md
     // (see #parsers-returning-incomplete-instead-of-an-error-on-empty-input)
    opt!(complete!(tag!("->"))) >>
    targets: separated_list_complete!(ws!(char!(',')), label_parser) >>
    (Node{ label, weight, neighbors: targets.into_iter().collect() })
)));

fn parse_graph(lines: &[&str]) -> Graph {
    Graph(
        lines
            .iter()
            .map(|&s| {
                let node = node_parser(s).unwrap().1;
                (node.label.clone(), node)
            })
            .collect(),
    )
}

fn toposort(g: &Graph) -> Vec<Label> {
    let mut outgoing_count: HashMap<Label, usize> = HashMap::new();
    let mut incoming: HashMap<Label, HashSet<Label>> = HashMap::new();
    let mut q: VecDeque<Label> = VecDeque::new();
    for node in g.0.values() {
        outgoing_count.insert(node.label.clone(), node.neighbors.len());
        for neighbor in &node.neighbors {
            incoming
                .entry(neighbor.clone())
                .or_insert(HashSet::new())
                .insert(node.label.clone());
        }
        if node.neighbors.is_empty() {
            q.push_back(node.label.clone());
        }
    }

    let mut sorted = Vec::new();
    while let Some(label) = q.pop_front() {
        sorted.push(label.clone());
        for sources in incoming.get(&label) {
            for source in sources {
                for cnt in outgoing_count.get_mut(&source) {
                    *cnt -= 1;
                    if *cnt == 0 {
                        q.push_back(source.clone());
                    }
                }
            }
        }
    }
    sorted
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult::Done;

    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    #[test]
    fn parse_node() {
        assert_eq!(
            node_parser("asdf (15)"),
            Done(
                "",
                Node {
                    label: Label(String::from("asdf")),
                    weight: Weight(15),
                    neighbors: HashSet::new(),
                },
            )
        );
        assert_eq!(
            node_parser("asdf (15) -> a, b, c"),
            Done(
                "",
                Node {
                    label: Label(String::from("asdf")),
                    weight: Weight(15),
                    neighbors: vec![
                        Label(String::from("a")),
                        Label(String::from("b")),
                        Label(String::from("c")),
                    ].into_iter()
                        .collect(),
                },
            )
        );
    }

    #[test]
    fn small() {
        let mut fin = File::open(Path::new("data/day7/small.txt")).expect("open file");
        let mut buf = String::new();
        fin.read_to_string(&mut buf).expect("read file");
        let graph = parse_graph(&buf.lines().collect::<Vec<_>>());
        assert_eq!(
            *toposort(&graph).last().unwrap(),
            Label(String::from("tknk"))
        );
    }

    #[test]
    fn main() {
        let mut fin = File::open(Path::new("data/day7/input.txt")).expect("open file");
        let mut buf = String::new();
        fin.read_to_string(&mut buf).expect("read file");
        let graph = parse_graph(&buf.lines().collect::<Vec<_>>());
        assert_eq!(
            *toposort(&graph).last().unwrap(),
            Label(String::from("tknk"))
        );
    }
}
