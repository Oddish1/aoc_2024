use std::fs;

fn main() {
    // a list of numbers two at a time from file
    let input = fs::read_to_string("infile").expect("Should have been able to read the file");
    // split on newlines
    let numbers: Vec<String> = input.split("\n").map(|s| s.to_string()).collect();

    // initialize empty vectors for final lists
    let mut list_1: Vec<i32> = Vec::new();
    let mut list_2: Vec<i32> = Vec::new();

    // iterate over numvers
    for pair in numbers.iter() {
        // split on spaces
        let nums = pair.split(" ");
        // iterate over each 'line' of split input to pull out numbers
        for (idx, value) in nums.enumerate() {
            if (idx == 0) & !(value.is_empty()) {
                let result: Result<i32, _> = value.parse();
                match result {
                    Ok(value) => {
                        list_1.push(value);
                    }
                    Err(error) => {
                        println!("Failed to parse: {}", error);
                    }
                }
            } else if (idx == 3) & !(value.is_empty()) {
                let result: Result<i32, _> = value.parse();
                match result {
                    Ok(value) => {
                        list_2.push(value);
                    }
                    Err(error) => {
                        println!("Failed to parse: {}", error);
                    }
                }
            }
        }
    }

    // sort the lists in ascending order
    list_1.sort();
    list_2.sort();
    // find difference between each index, store in diff_list
    let diff_list: Vec<i32> = list_1
        .iter()
        .zip(list_2.iter())
        .map(|(a, b)| (a - b).abs())
        .collect();
    // sum diff_list
    let sum: i32 = diff_list.into_iter().sum();
    println!("The sum of differences is: {}", sum);

    // begin part 2

    // find unique values in list 1
    // since lists are sorted already, use dedup() on list_1
    list_1.dedup();

    // count occurences in list 2
    let mut counts: Vec<[i32; 2]> = list_1.iter().map(|&val| [val, 0]).collect();
    for &value in list_2.iter() {
        if let Some(count_pair) = counts.iter_mut().find(|pair| pair[0] == value) {
            count_pair[1] += 1;
        }
    }
    // calc similarity score = sum(unique number * occurences)
    let mut similarity_score: i32 = 0;
    for &value in counts.iter() {
        similarity_score += value[0] * value[1];
    }

    println!("Similarity Score: {}", similarity_score);
}
