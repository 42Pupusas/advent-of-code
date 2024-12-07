fn main() {
    println!("Hello, advent of code day two!");
    let input_file = std::fs::read_to_string("days/two/input.txt").unwrap();
    let lines: Vec<&str> = input_file.lines().collect();
    let mut danger_reports = vec![];
    let mut safe_reports = vec![];
    lines.iter().for_each(|line| {
        let danger_report: Vec<i32> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();
        let is_safe = check_if_report_is_safe(danger_report.clone(), true);
        if is_safe {
            safe_reports.push(danger_report);
        } else {
            danger_reports.push(danger_report);
        }
    });
    danger_reports.iter_mut().for_each(|report| {
        report.remove(0);
        let is_safe = check_if_report_is_safe(report.clone(), false);
        if is_safe {
            println!("Danger report: {:?} is safe: {}", report, is_safe);
        }
    });
    // safe_reports.iter().for_each(|report| {
    //     println!("{:?}", report);
    // });
    println!("Safe reports: {}", safe_reports.len());
}

fn check_if_report_is_safe(report: Vec<i32>, first_check: bool) -> bool {
    let mut is_ascending_or_descending = None;
    let mut is_safe = true;

    // Traverse the report to check ascending/descending pattern
    for (index_of_current_report, current_report_value) in report.iter().enumerate() {
        if let Some(next_report_value) = report.get(index_of_current_report + 1) {
            let local_difference = current_report_value - next_report_value;

            // Initialize the direction of the sequence (ascending or descending)
            if is_ascending_or_descending.is_none() {
                is_ascending_or_descending = Some(local_difference.signum());
            }

            let local_sign = local_difference.signum();

            // Check if direction is inconsistent (i.e., from ascending to descending or vice versa)
            if local_sign != is_ascending_or_descending.unwrap() || local_sign == 0 {
                is_safe = false;

                if !first_check {
                    break;
                }

                // Try removing the current level and check again
                let mut left_index_removed_list = report.clone();
                left_index_removed_list.remove(index_of_current_report);
                if check_if_report_is_safe(left_index_removed_list, false) {
                    is_safe = true;
                    break;
                }

                // Try removing the next level and check again
                let mut right_index_removed_list = report.clone();
                right_index_removed_list.remove(index_of_current_report + 1);
                if check_if_report_is_safe(right_index_removed_list, false) {
                    is_safe = true;
                    break;
                }
                println!("Failed Report: {:?}", report);
                break; // Stop processing after trying both removals
            }

            // Check if the difference between levels is valid (between 1 and 3)
            if local_difference.abs() > 3 || local_difference.abs() < 1 {
                is_safe = false;

                if !first_check {
                    break;
                }

                // Try removing the current level and check again
                let mut left_index_removed_list = report.clone();
                left_index_removed_list.remove(index_of_current_report);
                if check_if_report_is_safe(left_index_removed_list, false) {
                    is_safe = true;
                    break;
                }

                // Try removing the next level and check again
                let mut right_index_removed_list = report.clone();
                right_index_removed_list.remove(index_of_current_report + 1);
                if check_if_report_is_safe(right_index_removed_list, false) {
                    is_safe = true;
                    break;
                }
                break; // Stop processing after trying both removals
            }
        }
    }

    // Return the result
    is_safe
}
