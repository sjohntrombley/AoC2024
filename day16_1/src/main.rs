use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::read_to_string;
use std::rc::{Rc, Weak};

type Num = u32;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::*;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: Num,
    y: Num,
}

impl Point {
    fn move_(&self, dir: Direction) -> Point {
        match dir {
            North => Point {
                x: self.x,
                y: self.y - 1,
            },
            East => Point {
                x: self.x + 1,
                y: self.y,
            },
            South => Point {
                x: self.x,
                y: self.y + 1,
            },
            West => Point {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct NodeLabel {
    location: Point,
    facing: Direction,
}

struct Node {
    label: NodeLabel,
    neighbors: RefCell<Vec<(Weak<Node>, Num)>>,
}

struct HeapElement {
    node: Rc<Node>,
    dist: Num,
}

impl PartialEq for HeapElement {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl Eq for HeapElement {}

// arguments flipped for partial_cmp and cmp so that we get a min heap
impl Ord for HeapElement {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for HeapElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let (start, end, _nodes) = parse_input(&read_to_string("input.txt").unwrap());

    let mut queue = BinaryHeap::new();
    queue.push(HeapElement {
        node: Rc::clone(&start),
        dist: 0,
    });
    let mut visited = HashSet::new();
    while let Some(HeapElement {
        node,
        dist: total_dist,
    }) = queue.pop()
    {
        if visited.contains(&node.label) {
            continue;
        }
        if node.label.location == end {
            println!("{total_dist}");
            break;
        }

        for (neighbor, dist) in node.neighbors.borrow().iter() {
            let neighbor = neighbor.upgrade().unwrap();
            if !visited.contains(&neighbor.label) {
                queue.push(HeapElement {
                    node: neighbor,
                    dist: total_dist + dist,
                });
            }
        }

        visited.insert(node.label);
    }
}

fn parse_input(input: &str) -> (Rc<Node>, Point, HashMap<NodeLabel, Rc<Node>>) {
    let mut nodes = HashMap::new();
    let mut start = None;
    let mut end = None;

    for (y, line) in input.lines().enumerate() {
        let y = y.try_into().unwrap();
        for (x, c) in line.chars().enumerate() {
            let x = x.try_into().unwrap();
            let location = Point { x, y };
            if c == '.' || c == 'S' || c == 'E' {
                // This probably creates 30-40% more nodes than are necessary, but it makes things
                // easier and can be fixed later
                insert_node(&mut nodes, location, North);
                insert_node(&mut nodes, location, East);
                insert_node(&mut nodes, location, South);
                insert_node(&mut nodes, location, West);
            }

            if c == 'S' {
                start = Some(Rc::clone(
                    &nodes[&NodeLabel {
                        location,
                        facing: East,
                    }],
                ));
            }
            if c == 'E' {
                end = Some(location);
            }
        }
    }

    for node in nodes.values() {
        add_neighbor(&nodes, node, North);
        add_neighbor(&nodes, node, East);
        add_neighbor(&nodes, node, South);
        add_neighbor(&nodes, node, West);
    }

    (start.unwrap(), end.unwrap(), nodes)
}

fn insert_node(nodes: &mut HashMap<NodeLabel, Rc<Node>>, location: Point, facing: Direction) {
    let label = NodeLabel { location, facing };
    nodes.insert(
        label,
        Rc::new(Node {
            label,
            neighbors: RefCell::new(Vec::new()),
        }),
    );
}

fn add_neighbor(nodes: &HashMap<NodeLabel, Rc<Node>>, node: &Node, facing: Direction) {
    let label = NodeLabel {
        location: node.label.location.move_(facing),
        facing,
    };
    if nodes.contains_key(&label) {
        let dist = if node.label.facing == facing { 1 } else { 1001 };
        node.neighbors
            .borrow_mut()
            .push((Rc::downgrade(&nodes[&label]), dist));
    }
}
