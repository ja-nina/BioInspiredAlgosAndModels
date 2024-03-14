use crate::solution::Solution;

fn get_node_swap_delta(solution: &Solution, mut first_idx: u16, mut second_idx: u16) -> i32 {
    if first_idx == second_idx {
        return 0;
    }
    if ((first_idx > second_idx) || (first_idx == 0 && second_idx == solution.order.len() - 1)) {
        (first_idx, second_idx) = (second_idx, first_idx);
    }
    let first = solution[first_idx];
    let first_prev = solution[(first_idx - 1 + solution.order.len()) % solution.order.len()];
    let first_next = solution[(first_idx + 1) % solution.order.len()];
    let second = solution[second_idx];
    let second_prev = solution[(second_idx - 1 + solution.order.len()) % solution.order.len()];
    let second_next = solution[(second_idx + 1) % solution.order.len()];

    let mut delta = 0;
    delta += nodes.dist[first_prev][second];
    delta += nodes.dist[first][second_next];

    delta -= nodes.dist[first_prev][first];
    delta -= nodes.dist[second][second_next];

    if (first_next != second) {
        delta += nodes.dist[second][first_next];
        delta += nodes.dist[second_prev][first];
        delta -= nodes.dist[first][first_next];
        delta -= nodes.dist[second_prev][second];
    }

    return delta;
}

fn get_edge_swap_delta(solution: &Solution, mut first_idx: u16, mut second_idx: u16) -> i32 {
    if first_idx == second_idx {
        return 0;
    }
    if (first_idx > second_idx || (first_idx == 0 && second_idx == solution.size() - 1)) {
        (first_idx, second_idx) = (second_idx, first_idx);
    }
    let first = solution[first_idx];
    let first_next = solution[(first_idx + 1) % solution.size()];
    let second = solution[second_idx];
    let second_next = solution[(second_idx + 1) % solution.size()];

    if first_next == second {
        return 0;
    }

    let mut delta = 0;
    delta += nodes.dist[first][second];
    delta += nodes.dist[first_next][second_next];

    delta -= nodes.dist[first][first_next];
    delta -= nodes.dist[second][second_next];

    return delta;
}
