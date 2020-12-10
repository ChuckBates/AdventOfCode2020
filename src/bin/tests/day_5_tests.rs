use super::*;

#[test]
fn when_taking_the_front_half() {
    let direction = 'F';
    let vector = vec![0,1,2,3,4,5];
    let expected = vec![0,1,2];

    assert_eq!(take_half(direction, vector), expected);
}  

#[test]
fn when_taking_the_back_half() {
    let direction = 'B';
    let vector = vec![0,1,2,3,4,5];
    let expected = vec![3,4,5];

    assert_eq!(take_half(direction, vector), expected);
}  

#[test]
fn when_taking_the_left_half() {
    let direction = 'L';
    let vector = vec![0,1,2,3,4,5];
    let expected = vec![0,1,2];

    assert_eq!(take_half(direction, vector), expected);
}  

#[test]
fn when_taking_the_right_half() {
    let direction = 'R';
    let vector = vec![0,1,2,3,4,5];
    let expected = vec![3,4,5];

    assert_eq!(take_half(direction, vector), expected);
}  

#[test]
fn when_finding_the_row() {
    let directions = "FBFBBFF";
    let expected = 44;

    assert_eq!(find_row(&directions), expected);
}  

#[test]
fn when_finding_the_column() {
    let mut directions = "RLR";
    let expected = 5;

    assert_eq!(find_column(&directions), expected);

    directions = "RRR";
    assert_eq!(find_column(&directions), 7);

    directions = "RLL";
    assert_eq!(find_column(&directions), 4);
}  

#[test]
fn when_getting_the_seat_id() {
    let row = 44;
    let column = 5;
    let expected = 357;

    assert_eq!(get_seat_id(row, column), expected);
}  

#[test]
fn when_executing_part_one() {
    let input_file_path = "src/bin/tests/inputs/day_5/day_five_input_example.txt";
    let expected = 820;

    assert_eq!(execute_part_one(input_file_path), expected);
}