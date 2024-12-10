use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    height: u8,
    neighbors: RefCell<HashMap<(usize, usize), Weak<Node>>>,
}

fn main() {
    let nodes = parse_input(read_to_string("input.txt").unwrap());
    let trailheads: Vec<Rc<Node>> = nodes
        .iter()
        .flat_map(|row| row.iter().filter(|node| node.height == 0).map(Rc::clone))
        .collect();
    for _ in 1..=8 {
        for trailhead in &trailheads {
            let mut new_neighbors = HashMap::new();
            for neighbor in trailhead
                .neighbors
                .borrow()
                .values()
                .map(|node| Weak::upgrade(node).unwrap())
            {
                for (new_neighbor_coord, new_neighbor) in neighbor.neighbors.borrow().iter() {
                    new_neighbors.insert(*new_neighbor_coord, Weak::clone(new_neighbor));
                }
            }
            *trailhead.neighbors.borrow_mut() = new_neighbors;
        }
    }
    println!(
        "{}",
        trailheads
            .iter()
            .map(|node| node.neighbors.borrow().len())
            .sum::<usize>()
    );
}

fn parse_input(input: String) -> Vec<Vec<Rc<Node>>> {
    let mut nodes: Vec<Vec<Rc<Node>>> = Vec::new();
    for (r, line) in input.lines().enumerate() {
        let mut row: Vec<Rc<Node>> = Vec::new();
        for (c, height) in line.chars().enumerate() {
            let node = Rc::new(Node {
                height: u8::try_from(height).unwrap() - b'0',
                neighbors: RefCell::new(HashMap::new()),
            });
            if r > 0 {
                if nodes[r - 1][c].height == node.height + 1 {
                    node.neighbors
                        .borrow_mut()
                        .insert((r - 1, c), Rc::downgrade(&nodes[r - 1][c]));
                } else if nodes[r - 1][c].height + 1 == node.height {
                    nodes[r - 1][c]
                        .neighbors
                        .borrow_mut()
                        .insert((r, c), Rc::downgrade(&node));
                }
            }
            if c > 0 {
                if row[c - 1].height == node.height + 1 {
                    node.neighbors
                        .borrow_mut()
                        .insert((r, c - 1), Rc::downgrade(&row[c - 1]));
                } else if row[c - 1].height + 1 == node.height {
                    row[c - 1]
                        .neighbors
                        .borrow_mut()
                        .insert((r, c), Rc::downgrade(&node));
                }
            }
            row.push(node);
        }
        nodes.push(row);
    }

    nodes
}
