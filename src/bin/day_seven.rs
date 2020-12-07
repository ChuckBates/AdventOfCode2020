use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use petgraph::graph::NodeIndex;
use petgraph::prelude::*;
use petgraph::*;

fn main() {
    let input_file_path = "src/bin/inputs/day_seven_input.txt";

    // Part One
    let part_one_result = execute_part_one(input_file_path);
    println!("bags containing shiny gold: {}", part_one_result);

    // Part Two
    let part_two_result = execute_part_two(input_file_path);
    println!("bags contained inside shiny gold: {}", part_two_result);
}

fn execute_part_one(file_path: &str) -> usize {
    let lines = parse_input(file_path);
    let parsed_lines = parse_lines(lines);

    let mut counter = 0;
    let mut graph = Graph::<String,()>::new();
    for (parent, children) in parsed_lines.iter() {
        let parent_option = graph.node_indices().find(|i| graph[*i] == parent.to_string());
        let mut _parent_node = NodeIndex::new(0);
        if  parent_option.is_none() {
            _parent_node = graph.add_node(parent.to_string());
        } else {
            _parent_node = parent_option.unwrap();
        }
        for (child, _) in children {
            let child_option = graph.node_indices().find(|i| graph[*i] == child.to_string());
            let mut _child_node = NodeIndex::new(0);
            if  child_option.is_none() {
                _child_node = graph.add_node(child.to_string());
            } else {
                _child_node = child_option.unwrap();
            }
            graph.add_edge(_parent_node, _child_node, ());
        }        
    }

    let shiny_gold_node = graph.node_indices().find(|i| graph[*i] == "shiny gold").unwrap();
    for start in graph.node_indices() {   
        if start == shiny_gold_node {
            continue;
        }   
        let dijkstra =  algo::dijkstra(&graph, start, None, |_| 1);
        if dijkstra.contains_key(&shiny_gold_node) {
            counter = counter + 1;
        }
    }
    return counter;
}

fn execute_part_two(file_path: &str) -> usize {
    let lines = parse_input(file_path);
    let parsed_lines = parse_lines(lines);
        
    return get_child_bag_count(parsed_lines, "shiny gold".to_string());
}

fn get_child_bag_count(map: HashMap<String, HashMap<String, usize>>, color: String) -> usize {
    let child = map.get(&color).unwrap();
    let mut counter = 0;
    for (next, count) in child {
        let next_children = get_child_bag_count(map.clone(), next.to_string());
        if next_children != 0 {
            counter = counter + (count * next_children);
        }
        counter = counter + count;
    }
    return counter;
}   

fn parse_input(file_path:&str) -> Vec<String> {
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file"));
    let mut result = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        result.push(line);
    }

    return result;
}

fn parse_lines(lines: Vec<String>) -> HashMap<String, HashMap<String, usize>> {
    let mut result = HashMap::new();
    for line in lines {
        let parts = line.split("bags contain").collect::<Vec<&str>>();
        let children = parse_children_bags(&parts[1]);
        result.insert(parts[0].trim().to_string(), children);
    }

    return result;
}

fn parse_children_bags(line_part: &str) -> HashMap<String, usize> {
    if line_part.trim().starts_with("no other bags") {
        return HashMap::new();
    }

    let mut result = HashMap::new();
    let bags = line_part.split(",").collect::<Vec<&str>>();
    for bag in bags {
        let parts = bag.split("bag").collect::<Vec<&str>>();
        let count_and_color = &parts[0].trim();
        let count: usize = count_and_color[..1].parse().unwrap();
        let color = count_and_color[2..].trim().to_string();
        result.insert(color, count);
    }
    return result
}

#[cfg(test)]
#[path = "tests/day_seven_tests.rs"]
mod day_seven_tests;