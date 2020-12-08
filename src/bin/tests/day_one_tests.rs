use super::*;

#[test]
fn test_part_one_example() {
    let input_file_path = "src/bin/tests/inputs/day_one/input_example.txt";
    let expected = 514579;
    assert_eq!(execute_part_one(input_file_path), expected);
}

#[test]
fn test_part_two_example() {
    let input_file_path = "src/bin/tests/inputs/day_one/input_example.txt";
    let expected = 241861950;
    assert_eq!(execute_part_two(input_file_path), expected);
}