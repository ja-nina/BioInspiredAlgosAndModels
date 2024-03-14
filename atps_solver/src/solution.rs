#[derive(Clone)]
pub struct Solution {
    pub dimension: usize,
    pub order: Vec<u32>,
}

impl Solution {
    pub fn new(order: &Vec<u32>) -> Result<Self, Box<dyn std::error::Error>> {
        let dimension = order.len();
        Ok(Self {
            dimension,
            order: order.clone(),
        })
    }
}
