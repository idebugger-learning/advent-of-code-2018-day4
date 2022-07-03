use parser::parse_input;

mod parser;

fn main() {
    let raw_input = include_str!("./data/input_example.txt");

    let (rest_input, duty_records) = parse_input(raw_input).expect("Failed to parse input");

    println!("Rest input length: {}", rest_input.len());
    println!("Duty records: {:?}", duty_records);
}
