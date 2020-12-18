use std::time::Instant;
mod input;

fn main() {
    let input_file_path = "src/bin/inputs/day_16_input.txt";
    let now = Instant::now();

    // Part One
    let part_one_result = execute_part_one(input_file_path);
    println!("({} seconds) ticket scanning error rate: {}", now.elapsed().as_secs_f32(), part_one_result);

    // Part Two
    let part_two_result = execute_part_two(input_file_path);
    println!("({} seconds) product of departure fields: {}", now.elapsed().as_secs_f32(), part_two_result);
}

fn execute_part_one(file_path: &str) -> usize {
    let lines = input::parse_strings(file_path);
    let ticket_information = parse_ticket_information(lines);
    let invalids = evaluate_rules_for_invalid_values(ticket_information.0, ticket_information.2);
    return invalids.iter().sum();
}

fn execute_part_two(file_path: &str) -> usize {
    let lines = input::parse_strings(file_path);
    let ticket_information = parse_ticket_information(lines);
    let own_ticket = ticket_information.clone().1;
    let valid_tickets = evaluate_rules_for_valid_only_tickets(ticket_information.clone().0, ticket_information.2);
    let tickets_count = valid_tickets.len();
    let rules = parse_rules(ticket_information.0);
    let mut rules_for_position: Vec<Vec<String>> = vec![];
    for ticket in valid_tickets {
        let mut ticket_index = 0;
        for value in ticket {
            let matched_rules = rules.clone().into_iter().filter(|(_,r)| (value >= r.0.0 && value <= r.0.1) || (value >= r.1.0 && value <= r.1.1)).collect::<Vec<(String, ((usize,usize),(usize,usize)))>>();
            let mut rules = vec![];
            for matched_rule in matched_rules {
                    rules.push(matched_rule.0);
            }
            let previous_rules = rules_for_position.get(ticket_index);
            if previous_rules.is_some() {
                let previous = rules_for_position.remove(ticket_index);
                for rule in previous {
                        rules.push(rule.to_string());
                }
            }
            rules_for_position.insert(ticket_index, rules);
            ticket_index += 1;
        }
    }

    let mut valid_rules_for_position = vec![];
    for rules_at_position in rules_for_position {
        let mut valid_rules = vec![];
        for rule in &rules {
            let rule_matches = rules_at_position.clone().into_iter().filter(|r| r.to_string() == rule.0).count();
            if rule_matches == tickets_count {
                valid_rules.push(rule.clone().0);
            }
        }
        valid_rules_for_position.push(valid_rules);
    }
    
    let mut rule_map = vec!["".to_string(); own_ticket.len()];
    let mut rules_for_positions = valid_rules_for_position.clone();
    for _ in 0..own_ticket.len() {
        let lonely_rule = get_lonely_rule(rules_for_positions.clone());
        rules_for_positions = remove_rule_from_positions(lonely_rule.1.clone(), rules_for_positions);
        rule_map[lonely_rule.0] = lonely_rule.1;
    }

    let mut result = 1;
    let mut rule_index = 0;
    for rule in rule_map {
        if rule.starts_with("departure") {
            result *= own_ticket[rule_index];
        }
        rule_index += 1;
    }

    return result;
}

fn get_lonely_rule(rules_for_positions: Vec<Vec<String>>) -> (usize, String) {
    let mut index = 0;
    for rules in rules_for_positions {
        if rules.len() == 1 {
            return (index, rules[0].to_string());
        }
        index += 1;
    }

    return (0, "".to_string());
}

fn remove_rule_from_positions(rule: String, rules_for_positions: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut result = rules_for_positions.clone();
    let mut position_index = 0;
    for position_rules in rules_for_positions {
        if position_rules.contains(&rule) {
            let mut rule_index = 0;
            for rule_name in position_rules {
                if rule_name == rule {
                    result[position_index].remove(rule_index);
                    break;
                }
                rule_index += 1;
            }
        }
        position_index += 1;
    }

    return result;
}

fn parse_ticket_information(ticket_information: Vec<String>) -> (Vec<String>, Vec<usize>, Vec<Vec<usize>>) {
    let mut result = (vec![], vec![], vec![vec![]]);
    let mut rules = vec![];
    let mut own_ticket = vec![];
    let mut other_tickets = vec![];
    let mut phase = "rules".to_string();
    for line in ticket_information {
        if phase == "rules" {
            if line.is_empty() {
                phase = "own_ticket".to_string();
                continue;
            }
            rules.push(line);
        } else if phase == "own_ticket" {
            if !line.starts_with("your") {
                let values = line.split(",").collect::<Vec<&str>>();
                for value in values {
                    own_ticket.push(value.parse::<usize>().unwrap());
                }
                phase = "other_tickets".to_string();
            }
        } else {
            if !line.is_empty() && !line.starts_with("nearby") {
                let values = line.split(",").collect::<Vec<&str>>();
                let mut ticket = vec![];
                for value in values {
                    ticket.push(value.parse::<usize>().unwrap());
                }
                other_tickets.push(ticket);
            }
        }
    }

    if !rules.is_empty() {
        result.0 = rules;
    }
    if !own_ticket.is_empty() {
        result.1 = own_ticket;
    }
    if !other_tickets.is_empty() {
        result.2 = other_tickets;
    }
    return result;
}

fn evaluate_rules_for_invalid_values(rules: Vec<String>, other_tickets: Vec<Vec<usize>>) -> Vec<usize> {
    let parsed_rules = parse_rules(rules);

    let mut invalids = vec![];
    for ticket in other_tickets {
        'value_loop: for value in ticket {
            for rule in &parsed_rules {
                if (value >= rule.1.0.0 && value <= rule.1.0.1) || (value >= rule.1.1.0 && value <= rule.1.1.1) {
                    continue 'value_loop;
                }
            }
            invalids.push(value);
        }
    }

    return invalids;
}

fn evaluate_rules_for_valid_only_tickets(rules: Vec<String>, other_tickets: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let parsed_rules = parse_rules(rules);

    let mut invalids = vec![];
    'ticket_loop: for ticket in other_tickets {
        for value in ticket.clone() {
            let matches_rule = parsed_rules.clone().into_iter().any(|r| (value >= r.1.0.0 && value <= r.1.0.1) || (value >= r.1.1.0 && value <= r.1.1.1));
            if !matches_rule {
                continue 'ticket_loop;
            }
        }
        invalids.push(ticket);
    }

    return invalids;
}

fn parse_rules(rules: Vec<String>) -> Vec<(String, ((usize,usize),(usize,usize)))> {
    let mut parsed_rules = vec![];
    for rule in rules {
        let parts = rule.split(" ").collect::<Vec<&str>>();
        let first_range = parts[parts.len() - 3].split("-").collect::<Vec<&str>>();
        let second_range = parts[parts.len() - 1].split("-").collect::<Vec<&str>>();
        let name = rule.split(":").collect::<Vec<&str>>()[0].to_string();
        let ranges = ((first_range[0].parse::<usize>().unwrap(), first_range[1].parse::<usize>().unwrap()),(second_range[0].parse::<usize>().unwrap(), second_range[1].parse::<usize>().unwrap()));
        parsed_rules.push((name, ranges));
    }

    return parsed_rules;
}

#[cfg(test)]
#[path = "tests/day_16_tests.rs"]
mod day_16_tests;