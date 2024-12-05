use std::fs;

fn determine_safe(reports: Vec<[i32; 5]>) -> i32 {
    // Each report is safe if:
    //  - levels are *all* increasing or *all* decreasing
    //  - any two adjacent levels differ by at least one and at most three
    let mut safety_report: Vec<bool> = Vec::new();
    for report in reports.iter() {}
    0
}

fn main() {
    // part 1 begin
    let input = fs::read_to_string("infile").expect("Should have been able to read the file");

    // split on newline to seperate into reports of integers
    let report_strings: Vec<String> = input.split("\n").map(|s| s.to_string()).collect();
    let mut reports: Vec<[i32; 5]> = Vec::new();
    for (idx, level) in report_strings.into_iter().enumerate() {
        let mut processed_level: [i32; 5] = [0; 5];
        let temp = level.split(" ");
        for (idx, value) in temp.enumerate() {
            if !value.is_empty() {
                let result: Result<i32, _> = value.parse();
                match result {
                    Ok(value) => {
                        processed_level[idx] = value;
                    }
                    Err(error) => {
                        println!("Failed to parse: {}", error);
                    }
                }
            }
        }
        reports[idx] = processed_level;
    }

    // determine safety score
    let safety_score = determine_safe(reports);
    println!("Safety Score: {}", safety_score);
}
