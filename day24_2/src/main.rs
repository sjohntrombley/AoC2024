// This isn't really a solution, but it gets you close enough to
// work out a solution by hand. For my input, there was only one
// case where the wire that needed to be swapped was ambiguous,
// and it was trivial to figure out by hand. I'll come back later
// and try to flesh it out and clean it up.
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

struct GateArgs {
    in0: String,
    in1: String,
    out: String,
}

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

    fn get_type(&self) -> &str {
        match self {
            Gate::And { .. } => "AND",
            Gate::Or { .. } => "OR",
            Gate::Xor { .. } => "XOR",
        }
    }

    fn is_and(&self) -> bool {
        match self {
            Gate::And(_) => true,
            _ => false,
        }
    }

    fn is_or(&self) -> bool {
        match self {
            Gate::Or(_) => true,
            _ => false,
        }
    }

    fn is_xor(&self) -> bool {
        match self {
            Gate::Xor(_) => true,
            _ => false,
        }
    }
}

fn parse_gates(input: &str) -> HashMap<String, Gate> {
    let mut gates = HashMap::new();
    for line in input.lines() {
        let (gate_spec, out_wire) = line.split_once(" -> ").unwrap();

        let mut gate_spec_iter = gate_spec.split_whitespace();
        let in_wire0 = gate_spec_iter.next().unwrap();
        let gate_type = gate_spec_iter.next().unwrap();
        let in_wire1 = gate_spec_iter.next().unwrap();

        let gate_args = GateArgs {
            in0: in_wire0.to_string(),
            in1: in_wire1.to_string(),
            out: out_wire.to_string(),
        };
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

fn parse_input(input: &str) -> HashMap<String, Gate> {
    let gates_str = input.split("\n\n").nth(1).unwrap();
    let gates = parse_gates(gates_str);

    gates
}

fn get_gates_by_input(
    gates: &HashMap<String, Gate>,
) -> HashMap<String, HashMap<String, HashMap<&str, &Gate>>> {
    let mut gates_by_input: HashMap<String, HashMap<String, HashMap<&str, &Gate>>> = HashMap::new();

    for gate in gates.values() {
        let args = gate.get_args();
        let in0 = &args.in0;
        let in1 = &args.in1;
        gates_by_input
            .entry(in0.clone())
            .or_default()
            .entry(in1.clone())
            .or_default()
            .insert(gate.get_type(), &gate);
        gates_by_input
            .entry(in1.clone())
            .or_default()
            .entry(in0.clone())
            .or_default()
            .insert(gate.get_type(), &gate);
    }

    gates_by_input
}

fn get_ha1_inputs(gates: &HashMap<String, Gate>) -> HashSet<(&str, &str)> {
    let mut ha1_inputs: HashSet<(&str, &str)> = HashSet::new();
    for gate in gates.values() {
        match gate {
            Gate::Xor(GateArgs { in0, in1, .. }) => {
                if !in0
                    .strip_prefix(['x', 'y'])
                    .is_some_and(|s| s.chars().next().unwrap().is_ascii_digit())
                {
                    ha1_inputs.insert({
                        if in1 < in0 {
                            (in1, in0)
                        } else {
                            (in0, in1)
                        }
                    });
                }
            }
            _ => (),
        }
    }

    ha1_inputs
}

fn get_ha_carry_outs(gates: &HashMap<String, Gate>) -> HashSet<(&str, &str)> {
    let mut ha_carry_outs: HashSet<(&str, &str)> = HashSet::new();
    for gate in gates.values() {
        match gate {
            Gate::Or(GateArgs { in0, in1, .. }) => {
                ha_carry_outs.insert({
                    if in0 < in1 {
                        (in0, in1)
                    } else {
                        (in1, in0)
                    }
                });
            }
            _ => (),
        }
    }

    ha_carry_outs
}

fn main() {
    let gates = parse_input(&read_to_string("input.txt").unwrap());
    let gates_by_input = get_gates_by_input(&gates);
    let ha1_inputs = get_ha1_inputs(&gates);
    let ha_carry_outs = get_ha_carry_outs(&gates);

    // check if both inputs to an or gate come from reasonable places
    for &(in0, in1) in &ha_carry_outs {
        println!("Checking ({}, {})", in0, in1);

        // check if the output gate for in0 or in1 is not an AND gate
        let mut bad_input = false;
        if !gates[in0].is_and() {
            println!(
                "\tInput {} comes from an {} gate, not an AND gate.",
                in0,
                gates[in0].get_type()
            );
            bad_input = true;
        }
        if !gates[in1].is_and() {
            println!(
                "\tInput {} comes from an {} gate, not an AND gate.",
                in1,
                gates[in1].get_type()
            );
            bad_input = true;
        }
        if bad_input {
            continue;
        }

        // Check that one half adder uses a pair of input wires and the other uses internal wires.

        // Assumes that if one of the inputs to the AND gate is an input wire, so is the other.
        // Since it is assumed that there are no superfluous gates and one of the two half adders
        // in each adder adds both input wires for a particular bit, this assumption should be
        // true.
        let GateArgs { in0: in00, .. } = gates[in0].get_args();
        let in0_is_ha0 = in00
            .strip_prefix(['x', 'y'])
            .is_some_and(|s| s.chars().all(|c| c.is_ascii_digit()));
        let GateArgs { in0: in10, .. } = gates[in1].get_args();
        let in1_is_ha0 = in10
            .strip_prefix(['x', 'y'])
            .is_some_and(|s| s.chars().all(|c| c.is_ascii_digit()));
        if in0_is_ha0 == in1_is_ha0 {
            if in0_is_ha0 {
                println!(
                    "\tBoth inputs are the carry out from the first half adder in a full adder."
                );
            } else {
                println!(
                    "\tBoth inputs are the carry out from the second half adder in a full adder."
                );
            }
        }
    }

    println!("");

    // check if both inputs to the second half adder in a full adder are reasonable
    for &(in0, in1) in &ha1_inputs {
        println!("Checking ({in0}, {in1})");

        let gate0 = &gates[in0];
        let gate1 = &gates[in1];

        let mut bad_input = false;
        let gate0_is_00_carry_out = gate0
            .get_args()
            .in0
            .strip_prefix(['x', 'y'])
            .is_some_and(|s| s == "00");
        if gate0.is_and() && !gate0_is_00_carry_out {
            println!("\tInput {in0} comes from an AND gate.");
            bad_input = true;
        }
        let gate1_is_00_carry_out = gate1
            .get_args()
            .in0
            .strip_prefix(['x', 'y'])
            .is_some_and(|s| s == "00");
        if gate1.is_and() && !gate1_is_00_carry_out {
            println!("\tInput {in0} comes from an AND gate.");
            bad_input = true;
        }
        if bad_input {
            continue;
        }

        if gate0_is_00_carry_out {
            if !gate1.is_xor() {
                println!(
                    "\t{in1} comes from an {} gate, not an XOR gate.",
                    gate1.get_type()
                );
            }
            continue;
        }
        if gate1_is_00_carry_out {
            if !gate0.is_xor() {
                println!(
                    "\t{in0} comes from an {} gate, not an XOR gate.",
                    gate1.get_type()
                );
            }
            continue;
        }

        if gate0.is_or() == gate1.is_or() {
            if gate0.is_or() {
                println!("\tBoth inputs come from the carry outs of a full adder.");
            } else {
                println!("\tBoth inputs come from the output of a half adder.");
            }
        }
    }

    println!("");

    // check output wires
    for i in 0..=45 {
        let wire = format!("z{i:02}");
        println!("Checking {wire}");
        let gate = &gates[&wire];
        // TODO: Detect output size instead of hard coding 45
        if !gate.is_xor() && !(gate.is_and() && i == 45) {
            println!(
                "{wire} comes from an {} gate, not an XOR gate.",
                gate.get_type()
            );
            continue;
        }

        if i != 0
            && gate
                .get_args()
                .in0
                .strip_prefix(['x', 'y'])
                .is_some_and(|s| s.chars().all(|c| c.is_ascii_digit()))
        {
            println!("{wire} comes from the first half adder in a full adder, not the second.");
        }
    }
}
