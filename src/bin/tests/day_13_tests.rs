use super::*;

#[test]
fn when_getting_a_buses_next_departure_at_time_939_and_the_bus_interval_is_7() {
    let time = 939;
    let bus_interval = 7;
    let expected = 945;

    assert_eq!(get_bus_next_departure(time, bus_interval), expected);
}

#[test]
fn when_getting_a_buses_next_departure_at_time_939_and_the_bus_interval_is_13() {
    let time = 939;
    let bus_interval = 13;
    let expected = 949;

    assert_eq!(get_bus_next_departure(time, bus_interval), expected);
}

#[test]
fn when_getting_a_buses_next_departure_at_time_1068700_and_the_bus_interval_is_59() {
    let mut time = 1068710;
    let bus_interval = 59;
    let mut expected = 1068726;

    assert_eq!(get_bus_next_departure(time, bus_interval), expected);

    time = expected + 1;
    expected = 1068785;

    assert_eq!(get_bus_next_departure(time, bus_interval), expected);
}

#[test]
fn when_executing_part_one() {
    let input = "src/bin/tests/inputs/day_13/input_example.txt";
    let expected = 295;

    assert_eq!(execute_part_one(input), expected);
}

#[test]
fn when_executing_part_two_example() {
    let input = "src/bin/tests/inputs/day_13/input_example_part_two.txt";
    let expected = 1068781;

    assert_eq!(execute_part_two(input), expected);
}

#[test]
fn when_executing_part_two_example_1() {
    let input = "src/bin/tests/inputs/day_13/input_example_small_1.txt";
    let expected = 3417;

    assert_eq!(execute_part_two(input), expected);
}

#[test]
fn when_executing_part_two_example_2() {
    let input = "src/bin/tests/inputs/day_13/input_example_small_2.txt";
    let expected = 754018;

    assert_eq!(execute_part_two(input), expected);
}

#[test]
fn when_executing_part_two_example_3() {
    let input = "src/bin/tests/inputs/day_13/input_example_small_3.txt";
    let expected = 779210;

    assert_eq!(execute_part_two(input), expected);
}

#[test]
fn when_executing_part_two_example_4() {
    let input = "src/bin/tests/inputs/day_13/input_example_small_4.txt";
    let expected = 1261476;

    assert_eq!(execute_part_two(input), expected);
}

#[test]
fn when_executing_part_two_example_5() {
    let input = "src/bin/tests/inputs/day_13/input_example_small_5.txt";
    let expected = 1202161486;

    assert_eq!(execute_part_two(input), expected);
}