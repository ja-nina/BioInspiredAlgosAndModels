enum OperationType {
    NODE_SWAP,
    EDGE_SWAP,
}

struct Operation {
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

    fn to_int(&self) -> u32 {
        // first two bits -> op_type
        // 10 bits -> first_idx
        // 10 bits -> second_idx
        // 10 bits -> third_idx

        let mut result = 0u32;
        result |= match self.op_type {
            OperationType::NODE_SWAP => 0b00,
            OperationType::EDGE_SWAP => 0b01,
        };
        result <<= 10;
        result |= self.first_idx as u32;
        result <<= 10;
        result |= self.second_idx as u32;
        result <<= 10;
        result |= self.third_idx as u32;

        result
    }

    fn from_int(op: u32) -> Operation {
        let op_type = match op >> 30 {
            0b00 => OperationType::NODE_SWAP,
            0b01 => OperationType::EDGE_SWAP,
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
}
