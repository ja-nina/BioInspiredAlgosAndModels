use crate::solution::Solution;
use crate::utils;
use crate::{atsp, deltas};
use bitflags::bitflags;
use rand::rngs::StdRng;
use rand::Rng;

const MAX_NODES: u16 = 2u16.pow(10) - 1;

#[derive(Debug)]
enum OperationType {
    NodeSwap,
    EdgeSwap,
    ThreeOpt, // not implemented
    Invalid,
}

#[derive(Debug)]
pub struct Operation {
    op_type: OperationType,
    first_idx: u16,
    second_idx: u16,
    third_idx: u16,
}

bitflags! {
    pub struct OperationFlags: u32 {
        const NODE_SWAP = 0b01;
        const EDGE_SWAP = 0b10;
        const THREE_OPT = 0b100;
    }
}

impl Operation {
    fn new(op_type: OperationType, first_idx: u16, second_idx: u16, third_idx: u16) -> Operation {
        Operation {
            op_type,
            first_idx,
            second_idx,
            third_idx,
        }
    }

    pub fn to_int(&self) -> u32 {
        // first two bits -> op_type
        // 10 bits -> first_idx
        // 10 bits -> second_idx
        // 10 bits -> third_idx

        let mut result = 0u32;
        result |= match self.op_type {
            OperationType::NodeSwap => 0b00,
            OperationType::EdgeSwap => 0b01,
            OperationType::ThreeOpt => panic!("3-opt not implemented"),
            OperationType::Invalid => panic!("Invalid operation type"),
        };
        result <<= 10;
        result |= self.first_idx as u32;
        result <<= 10;
        result |= self.second_idx as u32;
        result <<= 10;
        result |= self.third_idx as u32;

        result
    }

    pub fn from_int(op: u32) -> Operation {
        let op_type = match op >> 30 {
            0b00 => OperationType::NodeSwap,
            0b01 => OperationType::EdgeSwap,
            _ => panic!("Invalid operation type"),
        };
        let first_idx = (op >> 20) as u16 & 0b1111111111;
        let second_idx = (op >> 10) as u16 & 0b1111111111;
        let third_idx = op as u16 & 0b1111111111;

        Operation {
            op_type,
            first_idx,
            second_idx,
            third_idx,
        }
    }

    pub fn apply(&self, solution: &mut Solution) {
        match self.op_type {
            OperationType::NodeSwap => {
                solution
                    .order
                    .swap(self.first_idx as usize, self.second_idx as usize);
            }
            OperationType::EdgeSwap => {
                let idx_diff = (self.second_idx as i16 - self.first_idx as i16).abs();
                if idx_diff < 2 || idx_diff == solution.order.len() as i16 - 1 {
                    return;
                }
                let mut i = self.first_idx as usize;
                let mut j = self.second_idx as usize;
                if i > j {
                    std::mem::swap(&mut i, &mut j);
                }
                solution.order[i + 1..j + 1].reverse();
            }
            OperationType::ThreeOpt => panic!("3-opt not implemented"),
            OperationType::Invalid => panic!("Invalid operation type"),
        }
    }

    pub fn evaluate(&self, solution: &Solution, instance: &atsp::ATSP) -> i32 {
        match self.op_type {
            OperationType::NodeSwap => deltas::get_node_swap_delta(
                &solution.order,
                self.first_idx as usize,
                self.second_idx as usize,
                &instance.matrix,
            ),
            OperationType::EdgeSwap => deltas::get_edge_swap_delta(
                &solution.order,
                self.first_idx as usize,
                self.second_idx as usize,
                &instance.matrix,
            ),
            _ => panic!("Bad operation!"),
        }
    }
}

pub fn random_operation(rng: &mut StdRng, num_nodes: u16, op_flags: u32) -> Operation {
    if num_nodes < 3 {
        panic!("Number of nodes must be at least 3");
    }
    if num_nodes > MAX_NODES {
        panic!("Number of nodes must be at most {}", MAX_NODES);
    }
    let (first_idx, second_idx) = utils::generate_unique_duplet(num_nodes as usize, rng);
    let third_idx = 0; // third index only used in 3-opt
    let op_flags = OperationFlags::from_bits(op_flags).unwrap();
    let num_enabled_ops = op_flags.bits().count_ones();
    if num_enabled_ops == 1 {
        if op_flags.contains(OperationFlags::NODE_SWAP) {
            return Operation::new(
                OperationType::NodeSwap,
                first_idx as u16,
                second_idx as u16,
                0,
            );
        } else if op_flags.contains(OperationFlags::EDGE_SWAP) {
            return Operation::new(
                OperationType::EdgeSwap,
                first_idx as u16,
                second_idx as u16,
                0,
            );
        } else {
            panic!("Invalid operation type");
        }
    }
    let op_type = match rng.gen_range(0..2) {
        0 => OperationType::NodeSwap,
        1 => OperationType::EdgeSwap,
        _ => panic!("Invalid operation type"),
    };

    // for 3-opt, third_idx is also needed
    // third_idx = self.rng.gen_range(0..num_nodes-2);
    // third_idx = third_idx + (third_idx > first_idx) + (third_idx > second_idx)

    Operation::new(op_type, first_idx as u16, second_idx as u16, third_idx)
}

fn initial_operation(op_type: OperationType) -> Operation {
    match op_type {
        OperationType::NodeSwap => Operation::new(OperationType::NodeSwap, 0, 1, 0),
        OperationType::EdgeSwap => Operation::new(OperationType::EdgeSwap, 0, 2, 0),
        _ => panic!("Invalid operation type"),
    }
}
pub struct NeighborhoodIterator {
    num_nodes: u16,
    op_flags: OperationFlags,
    current_op: Operation,
}

impl NeighborhoodIterator {
    pub fn new(num_nodes: u16, op_flags: u32) -> NeighborhoodIterator {
        if (num_nodes < 3) || (num_nodes > MAX_NODES) {
            panic!(
                "Number of nodes must be at least 3 and at most {}",
                MAX_NODES
            );
        }
        let op_flags_parsed = OperationFlags::from_bits(op_flags);
        if op_flags_parsed.is_none() {
            panic!("Invalid operation flags");
        }

        NeighborhoodIterator {
            num_nodes,
            op_flags: op_flags_parsed.unwrap(),
            current_op: Operation::new(OperationType::Invalid, 0, 0, 0),
        }
    }

    pub fn size(&self) -> u32 {
        let n = self.num_nodes as u32;
        let mut size = 0;
        if self.op_flags.contains(OperationFlags::NODE_SWAP) {
            size += n * (n - 1) / 2;
        }
        if self.op_flags.contains(OperationFlags::EDGE_SWAP) {
            size += (n * (n - 1)) / 2 - n;
        }
        size
    }
}

impl Iterator for NeighborhoodIterator {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        match self.current_op.op_type {
            OperationType::Invalid => {
                if self.op_flags.contains(OperationFlags::NODE_SWAP) {
                    self.current_op = initial_operation(OperationType::NodeSwap);
                } else if self.op_flags.contains(OperationFlags::EDGE_SWAP) {
                    self.current_op = initial_operation(OperationType::EdgeSwap);
                }
                return Some(self.current_op.to_int());
            }
            OperationType::NodeSwap => {
                if self.current_op.second_idx >= self.num_nodes - 1 {
                    self.current_op.first_idx += 1;
                    self.current_op.second_idx = self.current_op.first_idx + 1;
                } else {
                    self.current_op.second_idx += 1;
                }
                if self.current_op.first_idx >= self.num_nodes - 1
                    || !self.op_flags.contains(OperationFlags::NODE_SWAP)
                {
                    self.current_op.op_type = OperationType::EdgeSwap;
                    self.current_op.first_idx = 0;
                    self.current_op.second_idx = 2;
                }
                return Some(self.current_op.to_int());
            }
            OperationType::EdgeSwap => {
                // Edge swaps are different only if the distance between the two indices is 2 or more
                // Second condition avoids pair 0 n-1 which also has distance < 2
                if (self.current_op.second_idx >= self.num_nodes - 1)
                    || (self.current_op.first_idx == 0
                        && self.current_op.second_idx >= self.num_nodes - 2)
                {
                    self.current_op.first_idx += 1;
                    self.current_op.second_idx = self.current_op.first_idx + 2;
                } else {
                    self.current_op.second_idx += 1;
                }
                if (self.current_op.first_idx == self.num_nodes - 1)
                    || (self.current_op.second_idx == self.num_nodes)
                    || !self.op_flags.contains(OperationFlags::EDGE_SWAP)
                {
                    return None;
                }
                return Some(self.current_op.to_int());
            }
            _ => (),
        }
        return None;
    }
}
