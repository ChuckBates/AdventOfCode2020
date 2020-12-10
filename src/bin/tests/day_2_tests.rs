use super::*;

#[test]
fn test_part_one_example() {
    let input_file_path = "src/bin/tests/inputs/day_2/day_two_input_example.txt";
    let expected = 2;

    assert_eq!(execute_part_one(input_file_path), expected);
}

#[test]
fn test_part_two_example() {
    let input_file_path = "src/bin/tests/inputs/day_2/day_two_input_example.txt";
    let expected = 1;

    assert_eq!(execute_part_two(input_file_path), expected);
}