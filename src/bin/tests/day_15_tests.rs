use super::*;

#[test]
fn when_executing_part_one_for_example_1() {
    let input = "src/bin/tests/inputs/day_15/input_example_1.txt";
    let expected = 436;

    assert_eq!(execute(input, 2020), expected);
}

#[test]
fn when_executing_part_one_for_example_2() {
    let input = "src/bin/tests/inputs/day_15/input_example_2.txt";
    let expected = 1;

    assert_eq!(execute(input, 2020), expected);
}

#[test]
fn when_executing_part_one_for_example_3() {
    let input = "src/bin/tests/inputs/day_15/input_example_3.txt";
    let expected = 10;

    assert_eq!(execute(input, 2020), expected);
}

#[test]
fn when_executing_part_one_for_example_4() {
    let input = "src/bin/tests/inputs/day_15/input_example_4.txt";
    let expected = 27;

    assert_eq!(execute(input, 2020), expected);
}

#[test]
fn when_executing_part_one_for_example_5() {
    let input = "src/bin/tests/inputs/day_15/input_example_5.txt";
    let expected = 78;

    assert_eq!(execute(input, 2020), expected);
}

#[test]
fn when_executing_part_one_for_example_6() {
    let input = "src/bin/tests/inputs/day_15/input_example_6.txt";
    let expected = 438;

    assert_eq!(execute(input, 2020), expected);
}

#[test]
fn when_executing_part_one_for_example_7() {
    let input = "src/bin/tests/inputs/day_15/input_example_7.txt";
    let expected = 1836;

    assert_eq!(execute(input, 2020), expected);
}

#[test]
fn when_executing_part_two_for_example_1() {
    let input = "src/bin/tests/inputs/day_15/input_example_1.txt";
    let expected = 175594;

    assert_eq!(execute(input, 30000000), expected);
}

#[test]
fn when_executing_part_two_for_example_2() {
    let input = "src/bin/tests/inputs/day_15/input_example_2.txt";
    let expected = 2578;

    assert_eq!(execute(input, 30000000), expected);
}

#[test]
fn when_executing_part_two_for_example_3() {
    let input = "src/bin/tests/inputs/day_15/input_example_3.txt";
    let expected = 3544142;

    assert_eq!(execute(input, 30000000), expected);
}

#[test]
fn when_executing_part_two_for_example_4() {
    let input = "src/bin/tests/inputs/day_15/input_example_4.txt";
    let expected = 261214;

    assert_eq!(execute(input, 30000000), expected);
}

#[test]
fn when_executing_part_two_for_example_5() {
    let input = "src/bin/tests/inputs/day_15/input_example_5.txt";
    let expected = 6895259;

    assert_eq!(execute(input, 30000000), expected);
}

#[test]
fn when_executing_part_two_for_example_6() {
    let input = "src/bin/tests/inputs/day_15/input_example_6.txt";
    let expected = 18;

    assert_eq!(execute(input, 30000000), expected);
}

#[test]
fn when_executing_part_two_for_example_7() {
    let input = "src/bin/tests/inputs/day_15/input_example_7.txt";
    let expected = 362;

    assert_eq!(execute(input, 30000000), expected);
}