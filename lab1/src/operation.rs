use crate::solution::Solution;
use rand::rngs::StdRng;
use rand::Rng;

const MAX_NODES: u16 = 2u16.pow(10) - 1;

enum OperationType {
    NodeSwap,
    EdgeSwap,
}

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
                let mut new_order = solution.order.clone();
                new_order[self.first_idx as usize] = solution.order[self.second_idx as usize];
                new_order[self.second_idx as usize] = solution.order[self.first_idx as usize];
                solution.order = new_order;
            }
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

    let op_type = match rng.gen_range(0..2) {
        0 => OperationType::NodeSwap,
        1 => OperationType::EdgeSwap,
        _ => panic!("Invalid operation type"),
    };

    // TODO: generate 3 unique indices
    let first_idx = rng.gen_range(0..num_nodes);
    let second_idx = rng.gen_range(0..num_nodes);
    let third_idx = rng.gen_range(0..num_nodes);

    Operation::new(op_type, first_idx, second_idx, third_idx)
}
