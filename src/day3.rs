use regex::bytes::Regex;
use std::{fs, vec};

pub fn get_data() {
    let day3_path = "./data_inputs/day3_input.txt";

    let content: String = fs::read_to_string(day3_path).expect("Could not find the file");

    let re = Regex::new(r"(do\(\)|don't\(\)).*?(mul\([0-9]{1,3},[0-9]{1,3}\))").unwrap();

    let mut results = vec![];

    for (_, [command_do, operation]) in re.captures_iter(content.as_bytes()).map(|c| c.extract()) {
        results.push((command_do, operation));
    }

    let mut final_multiplication = 0;
    for (command_do, operation) in results {
        let command_do = String::from_utf8(command_do.to_vec()).expect("Could not parse");
        let operation = String::from_utf8(operation.to_vec()).expect("Could not parse");
        let current_mul = calculate_mul(operation.clone());

        if command_do == r"do()" {
            println!("The command : {}", command_do);

            println!("The Operation : {}", operation);

            println!("The current mul : {}", current_mul);
            final_multiplication += current_mul;
        }
    }

    //for op in _results {
    //  let final_ope = String::from_utf8(op.to_vec()).expect("Could not convert");

    // println!("The final operation is {}", final_ope);
    //if final_ope.contains("do()") {
    //  let final_ope: Vec<_> = final_ope.split(r"mul\(").collect();
    // let final_ope = final_ope
    //   .strip_prefix(r"mul(")
    // .expect("Could not strip prefix")
    // .strip_suffix(r")")
    //  .expect("Could not strip suffix");

    //   println!("{}", final_ope[1]);
    //}

    //final_multiplication += multiplication;
    // }

    println!("{}", final_multiplication);
}

fn calculate_mul(mul_op: String) -> i32 {
    let final_ope = mul_op
        .strip_prefix(r"mul(")
        .expect("Could not delete prefix")
        .strip_suffix(r")")
        .expect("Could not delete suffix");
    let operands = final_ope.split(",");

    let mut multiplication = 1;

    for oper in operands {
        let oper: i32 = oper.parse().expect("Could not parse");
        multiplication *= oper;
    }

    multiplication
}
