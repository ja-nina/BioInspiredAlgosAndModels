pub fn get_node_swap_delta(
    solution: &Vec<u32>,
    mut first_idx: usize,
    mut second_idx: usize,
    cost_matrix: &Vec<Vec<i32>>,
) -> i32 {
    if first_idx == second_idx {
        return 0;
    }
    let n = solution.len();
    if (first_idx > second_idx) || (first_idx == 0 && second_idx == n - 1) {
        (first_idx, second_idx) = (second_idx, first_idx);
    }
    let first = solution[first_idx] as usize;
    let first_prev = solution[(first_idx + n - 1) % n] as usize;
    let first_next = solution[(first_idx + 1) % n] as usize;
    let second = solution[second_idx] as usize;
    let second_prev = solution[(second_idx + n - 1) % n] as usize;
    let second_next = solution[(second_idx + 1) % n] as usize;

    let mut delta = 0;
    delta += cost_matrix[first_prev][second];
    delta += cost_matrix[first][second_next];

    delta -= cost_matrix[first_prev][first];
    delta -= cost_matrix[second][second_next];

    if first_next != second {
        delta += cost_matrix[second][first_next];
        delta += cost_matrix[second_prev][first];
        delta -= cost_matrix[first][first_next];
        delta -= cost_matrix[second_prev][second];
    }

    return delta;
}

pub fn get_edge_swap_delta(
    solution: &Vec<u32>,
    mut first_idx: usize,
    mut second_idx: usize,
    cost_matrix: &Vec<Vec<i32>>,
) -> i32 {
    if first_idx == second_idx {
        return 0;
    }
    let n = solution.len();
    if first_idx > second_idx || (first_idx == 0 && second_idx == n - 1) {
        (first_idx, second_idx) = (second_idx, first_idx);
    }
    let first = solution[first_idx] as usize;
    let first_next = solution[(first_idx + 1) % n] as usize;
    let second = solution[second_idx] as usize;
    let second_next = solution[(second_idx + 1) % n] as usize;

    if first_next == second {
        return 0;
    }

    let mut delta = 0;
    delta += cost_matrix[first][second];
    delta += cost_matrix[first_next][second_next];

    delta -= cost_matrix[first][first_next];
    delta -= cost_matrix[second][second_next];

    return delta;
}
