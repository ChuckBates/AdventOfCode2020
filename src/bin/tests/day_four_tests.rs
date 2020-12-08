use super::*;

#[test]
fn when_parsing_input_and_there_is_only_one_line() {
    let input_file_path = "src/bin/tests/inputs/day_four/day_four_input_example_one.txt";
    let expected = vec![vec!["ecl:gry", "pid:860033327", "eyr:2020", "hcl:#fffffd"]];

    assert_eq!(parse_input(input_file_path), expected);
}

#[test]
fn when_parsing_input_and_there_are_two_lines_with_no_empty_lines() {
    let input_file_path = "src/bin/tests/inputs/day_four/day_four_input_example_two.txt";
    let expected = vec![vec!["ecl:gry", "pid:860033327", "eyr:2020", "hcl:#fffffd", "byr:1937", "iyr:2017", "cid:147", "hgt:183cm"]];

    assert_eq!(parse_input(input_file_path), expected);
}

#[test]
fn when_parsing_input_and_there_are_four_lines_with_on_empty_line() {
    let input_file_path = "src/bin/tests/inputs/day_four/day_four_input_example_three.txt";
    let expected = vec![
        vec!["ecl:gry", "pid:860033327", "eyr:2020", "hcl:#fffffd", "byr:1937", "iyr:2017", "cid:147", "hgt:183cm"],
        vec!["iyr:2013", "ecl:amb", "cid:350", "eyr:2023", "pid:028048884"]
    ];

    assert_eq!(parse_input(input_file_path), expected);
}

#[test]
fn when_parsing_input_and_there_are_four_passports() {
    let input_file_path = "src/bin/tests/inputs/day_four/day_four_input_example_full.txt";
    let expected = 4;

    assert_eq!(parse_input(input_file_path).len(), expected);
}

#[test]
fn when_parsing_input_example() {
    let input_file_path = "src/bin/tests/inputs/day_four/day_four_input_example_full.txt";
    let expected = vec![
        vec!["ecl:gry", "pid:860033327", "eyr:2020", "hcl:#fffffd", "byr:1937", "iyr:2017", "cid:147", "hgt:183cm"],
        vec!["iyr:2013", "ecl:amb", "cid:350", "eyr:2023", "pid:028048884", "hcl:#cfa07d", "byr:1929"],
        vec!["hcl:#ae17e1", "iyr:2013", "eyr:2024", "ecl:brn", "pid:760753108", "byr:1931", "hgt:179cm"],
        vec!["hcl:#cfa07d", "eyr:2025", "pid:166559648", "iyr:2011", "ecl:brn", "hgt:59in"],
    ];

    assert_eq!(parse_input(input_file_path), expected);
}

#[test]
fn when_validating_passport_and_it_is_empty() {
    let passport = vec![];
    let expected = false;

    assert_eq!(is_passport_fields_present(&passport), expected);
}

#[test]
fn when_validating_passport_and_it_has_less_than_seven_fields() {
    let passport = vec![
        "hcl:#cfa07d".to_string(), 
        "eyr:2025".to_string(), 
        "pid:166559648".to_string(), 
        "iyr:2011".to_string(), 
        "ecl:brn".to_string(), 
        "hgt:59in".to_string()
    ];
    let expected = false;

    assert_eq!(is_passport_fields_present(&passport), expected);
}

#[test]
fn when_validating_passport_and_it_is_missing_byr() {
    let passport = vec![
        "hcl:#cfa07d".to_string(), 
        "eyr:2025".to_string(), 
        "pid:166559648".to_string(), 
        "iyr:2011".to_string(), 
        "ecl:brn".to_string(), 
        "hgt:59in".to_string(), 
        "cid:147".to_string()
    ];
    let expected = false;

    assert_eq!(is_passport_fields_present(&passport), expected);
}

#[test]
fn when_validating_passport_and_it_is_missing_iyr() {
    let passport = vec![
        "hcl:#cfa07d".to_string(), 
        "eyr:2025".to_string(), 
        "pid:166559648".to_string(), 
        "byr:2011".to_string(), 
        "ecl:brn".to_string(), 
        "hgt:59in".to_string(), 
        "cid:147".to_string()
    ];
    let expected = false;

    assert_eq!(is_passport_fields_present(&passport), expected);
}

#[test]
fn when_validating_passport_and_it_is_missing_eyr() {
    let passport = vec![
        "hcl:#cfa07d".to_string(), 
        "iyr:2025".to_string(), 
        "pid:166559648".to_string(), 
        "byr:2011".to_string(), 
        "ecl:brn".to_string(), 
        "hgt:59in".to_string(), 
        "cid:147".to_string()
    ];
    let expected = false;

    assert_eq!(is_passport_fields_present(&passport), expected);
}

#[test]
fn when_validating_passport_and_it_is_missing_hgt() {
    let passport = vec![
        "hcl:#cfa07d".to_string(), 
        "eyr:2025".to_string(), 
        "pid:166559648".to_string(), 
        "byr:2011".to_string(), 
        "ecl:brn".to_string(), 
        "iyr:59in".to_string(), 
        "cid:147".to_string()
    ];
    let expected = false;

    assert_eq!(is_passport_fields_present(&passport), expected);
}

#[test]
fn when_validating_passport_and_it_is_missing_hcl() {
    let passport = vec![
        "iyr:#cfa07d".to_string(), 
        "eyr:2025".to_string(), 
        "pid:166559648".to_string(), 
        "byr:2011".to_string(), 
        "ecl:brn".to_string(), 
        "hgt:59in".to_string(), 
        "cid:147".to_string()
    ];
    let expected = false;

    assert_eq!(is_passport_fields_present(&passport), expected);
}

#[test]
fn when_validating_passport_and_it_is_missing_ecl() {
    let passport = vec![
        "hcl:#cfa07d".to_string(), 
        "eyr:2025".to_string(), 
        "pid:166559648".to_string(), 
        "byr:2011".to_string(), 
        "iyr:brn".to_string(), 
        "hgt:59in".to_string(), 
        "cid:147".to_string()
    ];
    let expected = false;

    assert_eq!(is_passport_fields_present(&passport), expected);
}

#[test]
fn when_validating_passport_and_it_is_missing_pid() {
    let passport = vec![
        "hcl:#cfa07d".to_string(), 
        "eyr:2025".to_string(), 
        "iyr:166559648".to_string(), 
        "byr:2011".to_string(), 
        "ecl:brn".to_string(), 
        "hgt:59in".to_string(), 
        "cid:147".to_string()
    ];
    let expected = false;

    assert_eq!(is_passport_fields_present(&passport), expected);
}

#[test]
fn when_executing_part_one() {
    let input_file_path = "src/bin/tests/inputs/day_four/day_four_input_example_full.txt";
    let expected = 2;

    assert_eq!(execute_part_one(input_file_path), expected);
}

#[test]
fn when_validating_year_and_it_is_not_in_range() {
    let mut byr = "byr:201".to_string();
    let expected = false;

    assert_eq!(is_year_valid(&byr, 1000, 9999), expected);

    byr = "byr:20111".to_string();
    assert_eq!(is_year_valid(&byr, 1000, 9999), expected);
}

#[test]
fn when_validating_year_and_it_is_in_range() {
    let byr = "byr:2011".to_string();
    let expected = true;

    assert_eq!(is_year_valid(&byr, 1000, 9999), expected);
}

#[test]
fn when_validating_height_and_it_is_not_valid() {
    let mut hgt = "hgt:2cm".to_string();
    let expected = false;

    assert_eq!(is_height_valid(&hgt), expected);

    hgt = "hgt:1656in".to_string();
    assert_eq!(is_height_valid(&hgt), expected);

    hgt = "hgt:58in".to_string();
    assert_eq!(is_height_valid(&hgt), expected);

    hgt = "hgt:77in".to_string();
    assert_eq!(is_height_valid(&hgt), expected);

    hgt = "hgt:71ind".to_string();
    assert_eq!(is_height_valid(&hgt), expected);

    hgt = "hgt:149cm".to_string();
    assert_eq!(is_height_valid(&hgt), expected);

    hgt = "hgt:194cm".to_string();
    assert_eq!(is_height_valid(&hgt), expected);

    hgt = "hgt:191cmfg".to_string();
    assert_eq!(is_height_valid(&hgt), expected);
}

#[test]
fn when_validating_height_and_it_is_valid() {
    let mut hgt = "hgt:60in".to_string();
    let expected = true;

    assert_eq!(is_height_valid(&hgt), expected);

    hgt = "hgt:190cm".to_string();
    assert_eq!(is_height_valid(&hgt), expected);
}

#[test]
fn when_validating_hair_color_and_it_is_not_valid() {
    let mut hcl = "hcl:#123abz".to_string();
    let expected = false;

    assert_eq!(is_hair_color_valid(&hcl), expected);

    hcl = "hcl:123abc".to_string();
    assert_eq!(is_hair_color_valid(&hcl), expected);

    hcl = "hcl:##123abc".to_string();
    assert_eq!(is_hair_color_valid(&hcl), expected);
}

#[test]
fn when_validating_hair_color_and_it_is_valid() {
    let mut hcl = "hcl:#123abc".to_string();
    let expected = true;

    assert_eq!(is_hair_color_valid(&hcl), expected);

    hcl = "hcl:#456789".to_string();
    assert_eq!(is_hair_color_valid(&hcl), expected);
}

#[test]
fn when_validating_eye_color_and_it_is_not_valid() {
    let mut ecl = "ecl:wat".to_string();
    let expected = false;

    assert_eq!(is_eye_color_valid(&ecl), expected);

    ecl = "ecl:asdf".to_string();
    assert_eq!(is_eye_color_valid(&ecl), expected);
}

#[test]
fn when_validating_eye_color_and_it_is_valid() {
    let mut ecl = "ecl:amb".to_string();
    let expected = true;

    assert_eq!(is_eye_color_valid(&ecl), expected);

    ecl = "ecl:blu".to_string();
    assert_eq!(is_eye_color_valid(&ecl), expected);

    ecl = "ecl:brn".to_string();
    assert_eq!(is_eye_color_valid(&ecl), expected);

    ecl = "ecl:gry".to_string();
    assert_eq!(is_eye_color_valid(&ecl), expected);

    ecl = "ecl:grn".to_string();
    assert_eq!(is_eye_color_valid(&ecl), expected);

    ecl = "ecl:hzl".to_string();
    assert_eq!(is_eye_color_valid(&ecl), expected);

    ecl = "ecl:oth".to_string();
    assert_eq!(is_eye_color_valid(&ecl), expected);
}

#[test]
fn when_validating_passport_id_and_it_is_not_valid() {
    let mut pid = "pid:0123456789".to_string();
    let expected = false;

    assert_eq!(is_passport_id_valid(&pid), expected);

    pid = "pid:123".to_string();
    assert_eq!(is_passport_id_valid(&pid), expected);

    pid = "pid:123x56789".to_string();
    assert_eq!(is_passport_id_valid(&pid), expected);
}

#[test]
fn when_validating_passport_id_and_it_is_valid() {
    let mut pid = "pid:123456789".to_string();
    let expected = true;

    assert_eq!(is_passport_id_valid(&pid), expected);

    pid = "pid:555555555".to_string();
    assert_eq!(is_passport_id_valid(&pid), expected);

    pid = "pid:987654321".to_string();
    assert_eq!(is_passport_id_valid(&pid), expected);
}

#[test]
fn when_executing_part_two_for_valid_passports() {
    let input_file_path = "src/bin/tests/inputs/day_four/day_four_input_example_valid.txt";
    let expected = 4;

    assert_eq!(execute_part_two(input_file_path), expected);
}

#[test]
fn when_executing_part_two_for_invalid_passports() {
    let input_file_path = "src/bin/tests/inputs/day_four/day_four_input_example_invalid.txt";
    let expected = 0;

    assert_eq!(execute_part_two(input_file_path), expected);
}