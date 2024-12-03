use std::{fs, path::Iter, string};
pub fn get_data() -> Vec<String> {
    let content =
        fs::read_to_string("./data_inputs/day2_input.txt").expect("Couldn't find the file");

    let mut reports: Vec<String> = Vec::new();
    // Split the lines
    let rows = content.split("\n");

    for row in rows {
        reports.push(row.to_string());
    }

    reports
}

// Function to check row safety
// Safe if all numbers are all increasing or decreasing
// and if the difference is either 1 or 2, no equality
fn fetch_result() {
    // Convert each element to int32

    let reports = get_data();

    let mut total_safety = 0;

    for report in reports {
        let mut report_int: Vec<i32> = Vec::new();

        let entries = report.split(" ");
        for entry in entries {
            let entry: i32 = entry.trim().parse().expect("Could not parse");
            report_int.push(entry);
        }
        let safety = check_safety(report_int);

        total_safety += safety;
    }

    println!("{}", total_safety);
}

// Calculate the safety of the report
pub fn check_safety(report: Vec<i32>) -> i32 {
    if check_sorted(report.clone()) {
        check_diff(report)
    } else {
        0
    }
}

// A function that checks if the vec is sorted asc and desc
fn check_sorted(report: Vec<i32>) -> bool {
    if report.is_sorted() || report.iter().rev().is_sorted() {
        true
    } else {
        false
    }
}

// A function that calculates if the distance between the element is at least 1 and at most 3
fn check_diff(report: Vec<i32>) -> i32 {
    for element in &report {
        print!("{} ", element);
    }

    let mut safety = 1;
    for i in 1..report.len() {
        let difference = report[i] - report[i - 1];

        if difference.abs() == 1 || difference.abs() == 2 || difference.abs() == 3 {
            continue;
        } else {
            safety = 0;
            break;
        };
    }
    println!(" ==== {} ", safety);
    safety
}
pub fn test_safety() {
    fetch_result();
}
