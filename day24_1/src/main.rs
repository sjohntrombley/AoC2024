use std::cmp::Reverse;
use std::collections::HashMap;
use std::fs::read_to_string;

struct GateArgs(String, String);

enum Gate {
    And(GateArgs),
    Or(GateArgs),
    Xor(GateArgs),
}

impl Gate {
    fn get_args(&self) -> &GateArgs {
        match self {
            Self::And(args) => args,
            Self::Or(args) => args,
            Self::Xor(args) => args,
        }
    }
}

fn parse_wires(input: &str) -> HashMap<String, u8> {
    let mut wires = HashMap::new();
    for line in input.lines() {
        let (wire, value) = line.split_once(": ").unwrap();
        wires.insert(wire.to_string(), value.parse().unwrap());
    }

    wires
}

fn parse_gates(input: &str) -> HashMap<String, Gate> {
    let mut gates = HashMap::new();
    for line in input.lines() {
        let (gate_spec, out_wire) = line.split_once(" -> ").unwrap();

        let mut gate_spec_iter = gate_spec.split_whitespace();
        let in_wire0 = gate_spec_iter.next().unwrap().to_string();
        let gate_type = gate_spec_iter.next().unwrap();
        let in_wire1 = gate_spec_iter.next().unwrap().to_string();

        let gate_args = GateArgs(in_wire0, in_wire1);
        let gate = match gate_type {
            "AND" => Gate::And(gate_args),
            "OR" => Gate::Or(gate_args),
            "XOR" => Gate::Xor(gate_args),
            _ => panic!("Invalid gate type ({gate_type})"),
        };

        gates.insert(out_wire.to_string(), gate);
    }

    gates
}

fn parse_input(input: &str) -> (HashMap<String, u8>, HashMap<String, Gate>) {
    let (initial_values_str, gates_str) = input.split_once("\n\n").unwrap();
    let wires = parse_wires(initial_values_str);
    let gates = parse_gates(gates_str);

    (wires, gates)
}

fn get_wire(wires: &mut HashMap<String, u8>, gates: &HashMap<String, Gate>, wire: &String) -> u8 {
    if let Some(&n) = wires.get(wire) {
        return n;
    }

    let GateArgs(in_wire0, in_wire1) = gates[wire].get_args();

    let in_wire0_value = get_wire(wires, gates, in_wire0);
    let in_wire1_value = get_wire(wires, gates, in_wire1);
    let value = match gates[wire] {
        Gate::And(_) => in_wire0_value & in_wire1_value,
        Gate::Or(_) => in_wire0_value | in_wire1_value,
        Gate::Xor(_) => in_wire0_value ^ in_wire1_value,
    };

    wires.insert(wire.clone(), value);
    value
}

fn main() {
    let (mut wires, gates) = parse_input(&read_to_string("input.txt").unwrap());

    let mut z_wires: Vec<_> = gates
        .keys()
        .filter(|wire| wire.starts_with('z'))
        .cloned()
        .collect();
    // Using sort_by_key creates lifetime problems, but this works
    z_wires.sort_by(|wire0, wire1| Reverse(wire0).cmp(&Reverse(wire1)));
    let mut ans: u64 = 0;
    for wire in &z_wires {
        ans *= 2;
        ans += u64::from(get_wire(&mut wires, &gates, wire));
    }

    println!("{ans}");
}
