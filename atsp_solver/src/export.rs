use std::fs;

use crate::solution::Solution;

pub fn export_to_file(
    filename: &String,
    solution: &Solution,
    initial_cost: i32,
    cost: i32,
    time_per_run: f64,
    iterations: u32,
    steps: u32,
    evaluations: u32,
    method: &str,
    instance: &str,
    neighborhood: &str,
) {
    let mut data: String = "{\n".to_string();
    data.push_str("\t\"order\": [");
    let mut i = 0;
    for ord in solution.order.iter() {
        data.push_str(ord.to_string().as_str());
        if i < solution.dimension - 1 {
            data.push(',');
            data.push(' ');
        }
        i += 1;
    }
    data.push_str("],\n\t\"cost\": ");
    data.push_str(cost.to_string().as_str());
    data.push_str(",\n\t\"initial_cost\": ");
    data.push_str(initial_cost.to_string().as_str());
    data.push_str(",\n\t\"time\": ");
    data.push_str(time_per_run.to_string().as_str());
    data.push_str(",\n\t\"iterations\": ");
    data.push_str(iterations.to_string().as_str());
    data.push_str(",\n\t\"steps\": ");
    data.push_str(steps.to_string().as_str());
    data.push_str(",\n\t\"evaluations\": ");
    data.push_str(evaluations.to_string().as_str());
    data.push_str(",\n\t\"method\": \"");
    data.push_str(method);
    data.push_str("\",\n\t\"instance\": \"");
    data.push_str(instance);
    data.push_str("\",\n\t\"neighborhood\": \"");
    data.push_str(neighborhood);
    data.push_str("\"\n}");

    fs::write(filename, data).expect("Failed to write to a file");
}
