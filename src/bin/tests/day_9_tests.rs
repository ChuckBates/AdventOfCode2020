use super::*;

#[test]
fn when_getting_summation_values_and_the_previous_five_have_only_one_option() {
    let sequence = vec![1,2,3,4,5];
    let next_value = 9;
    let expected = vec![(5,4)];

    assert_eq!(get_summation_values(sequence, next_value), expected);
} 

#[test]
fn when_getting_summation_values_and_the_previous_five_have_only_one_option_that_is_the_same_number_twice() {
    let sequence = vec![11,12,5,14,15];
    let next_value = 10;
    let expected = vec![];

    assert_eq!(get_summation_values(sequence, next_value), expected);
} 

#[test]
fn when_getting_summation_values_and_the_previous_five_have_two_options() {
    let sequence = vec![1,2,3,4,5];
    let next_value = 6;
    let expected = vec![(5,1),(4,2)];

    assert_eq!(get_summation_values(sequence, next_value), expected);
} 

#[test]
fn when_getting_summation_values_and_the_previous_five_have_no_options() {
    let sequence = vec![1,2,3,4,5];
    let next_value = 20;
    let expected = vec![];

    assert_eq!(get_summation_values(sequence, next_value), expected);
} 

#[test]
fn when_getting_first_non_sum_entry_for_preamble_five_and_look_back_five() {
    let sequence = vec![1,2,3,4,5,6,7,8,9,10,11,78,12];
    let look_back_length = 5;
    let preamble_lenth = 5;
    let expected = 10;

    assert_eq!(get_first_non_sum_entry(sequence, look_back_length, preamble_lenth), expected);
} 

#[test]
fn when_getting_first_non_sum_entry_for_example_input() {
    let sequence = vec![35,20,15,25,47,40,62,55,65,95,102,117,150,182,127,219,299,277,309,576];
    let look_back_length = 5;
    let preamble_lenth = 5;
    let expected = 127;

    assert_eq!(get_first_non_sum_entry(sequence, look_back_length, preamble_lenth), expected);
} 

#[test]
fn when_executing_part_one_for_example_input() {
    let input_file_path = "src/bin/tests/inputs/day_9/input_example.txt";
    let look_back_length = 5;
    let preamble_lenth = 5;
    let expected = 127;

    assert_eq!(execute_part_one(input_file_path, look_back_length, preamble_lenth), expected);
}

#[test]
fn when_getting_series_summation_equaling_target_and_there_is_no_valid_series() {
    let input_sequence = vec![1,2,3,4,5];
    let target_value = 45;
    let expected = vec![];

    assert_eq!(get_series_summation_equaling_target(input_sequence, target_value), expected);
}

#[test]
fn when_getting_series_summation_equaling_target_and_there_is_a_valid_series() {
    let input_sequence = vec![1,2,3,4,5];
    let target_value = 9;
    let expected = vec![2,3,4];

    assert_eq!(get_series_summation_equaling_target(input_sequence, target_value), expected);
}

#[test]
fn when_getting_series_summation_equaling_target_and_there_is_a_valid_series_example() {
    let input_sequence = vec![35,20,15,25,47,40,62,55,65,95,102,117,150,182,127,219,299,277,309,576];
    let target_value = 127;
    let expected = vec![15,25,47,40];

    assert_eq!(get_series_summation_equaling_target(input_sequence, target_value), expected);
}

#[test]
fn when_executing_part_two_example() {
    let input_file_path = "src/bin/tests/inputs/day_9/input_example.txt";
    let target_value = 127;
    let expected = 62;

    assert_eq!(execute_part_two(input_file_path, target_value), expected);
}