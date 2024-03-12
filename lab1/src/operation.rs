use crate::solution::Solution;
use crate::utils;
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
                // TODO: implement proper edge swap
                let mut new_order = solution.order.clone();
                new_order[self.first_idx as usize] = solution.order[self.second_idx as usize];
                new_order[self.second_idx as usize] = solution.order[self.first_idx as usize];
                solution.order = new_order;
            }
            OperationType::ThreeOpt => panic!("3-opt not implemented"),
            OperationType::Invalid => panic!("Invalid operation type"),
        }
    }

    pub fn evaluate(solution: &Solution) -> i32 {
        return 0;
    }
}

pub fn random_operation(rng: &mut StdRng, num_nodes: u16) -> Operation {
    if num_nodes < 3 {
        panic!("Number of nodes must be at least 3");
    }
    if num_nodes > MAX_NODES {
        panic!("Number of nodes must be at most {}", MAX_NODES);
    }
    let (first_idx, second_idx) = utils::generate_unique_duplet(num_nodes as usize, rng);
    let third_idx = 0; // third index only used in 3-opt
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

pub struct NeighborhoodIterator {
    num_nodes: u16,
    current_op: Operation,
}

impl NeighborhoodIterator {
    pub fn new(num_nodes: u16) -> NeighborhoodIterator {
        if (num_nodes < 3) || (num_nodes > MAX_NODES) {
            panic!(
                "Number of nodes must be at least 3 and at most {}",
                MAX_NODES
            );
        }

        NeighborhoodIterator {
            num_nodes,
            current_op: Operation::new(OperationType::Invalid, 0, 0, 0),
        }
    }

    pub fn size(&self) -> u32 {
        // n choose 2 for node swaps
        // n choose 2 - n for edge swaps

        let n = self.num_nodes as u32;
        let n_choose_2 = (n * (n - 1)) / 2;
        return (n_choose_2 * 2) - n;
    }
}

impl Iterator for NeighborhoodIterator {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        match self.current_op.op_type {
            OperationType::Invalid => {
                self.current_op.op_type = OperationType::NodeSwap;
                self.current_op.first_idx = 0;
                self.current_op.second_idx = 1;
                return Some(self.current_op.to_int());
            }
            OperationType::NodeSwap => {
                if self.current_op.second_idx >= self.num_nodes - 1 {
                    self.current_op.first_idx += 1;
                    self.current_op.second_idx = self.current_op.first_idx + 1;
                } else {
                    self.current_op.second_idx += 1;
                }
                if self.current_op.first_idx >= self.num_nodes - 1 {
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
