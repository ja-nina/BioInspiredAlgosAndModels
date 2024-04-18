use std::fs;

use crate::solution::Solution;

fn vec_to_string<T: ToString>(vector: &Vec<T>) -> String {
    let mut result = "[".to_string();
    for (index, element) in vector.iter().enumerate() {
        result.push_str(element.to_string().as_str());
        if index < vector.len() - 1 {
            result.push_str(", ")
        }
    }
    result.push(']');
    result
}

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
    meta_param_1: f64,
    meta_param_2: f64,
    meta_param_3: f64,
    evaluations_history: &Vec<u32>,
    cost_history: &Vec<i32>,
) {
    let mut data: String = "{\n".to_string();
    data.push_str("\t\"order\": ");
    data.push_str(vec_to_string(&solution.order).as_str());
    data.push_str(",\n\t\"cost\": ");
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
    data.push_str("\",\n\t\"evaluations_history\": ");
    data.push_str(vec_to_string(evaluations_history).as_str());
    data.push_str(",\n\t\"cost_history\": ");
    data.push_str(vec_to_string(cost_history).as_str());
    data.push_str(",\n\t\"meta-param-1\": ");
    data.push_str(meta_param_1.to_string().as_str());
    data.push_str(",\n\t\"meta-param-2\": ");
    data.push_str(meta_param_2.to_string().as_str());
    data.push_str(",\n\t\"meta-param-3\": ");
    data.push_str(meta_param_3.to_string().as_str());
    data.push_str("\n}");

    fs::write(filename, data).expect("Failed to write to a file");
}
