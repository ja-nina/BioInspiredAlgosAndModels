use std::fs;

use crate::solution::Solution;

pub fn export_to_file(
    filename: &String,
    solution: &Solution,
    score: i32,
    time_per_run: f64,
    iterations: u32,
    steps: u32,
    evaluations: u32,
    method: &str,
) {
    let mut data: String = "{\n".to_string();
    data.push_str("\t\"order\": [");
    let mut i = 0;
    for ord in solution.order.iter() {
        data.push_str(ord.to_string().as_str());
        i += 1;
        if i < solution.dimension - 1 {
            data.push(',');
            data.push(' ');
        }
    }
    data.push_str("],\n\t\"score\": ");
    data.push_str(score.to_string().as_str());
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
    data.push_str("\"\n}");

    fs::write(filename, data).expect("Failed to write to a file");
}