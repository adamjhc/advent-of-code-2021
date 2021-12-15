use std::collections::HashMap;

use utils;

fn main() {
    println!("{}", part_one())
}

fn part_one() -> usize {
    let map = parse_input();

    let paths = find_paths_to_end(&map, "start", vec!["start".to_string()]);

    paths.len()
}

fn find_paths_to_end(
    map: &HashMap<String, Vec<String>>,
    from: &str,
    current_path: Vec<String>,
) -> Vec<Vec<String>> {
    if from == "end" {
        return Vec::new();
    }

    map.get(from)
        .unwrap()
        .iter()
        .fold(Vec::new(), |mut paths, to| {
            if current_path.contains(to) && to.to_lowercase().eq(to) {
                return paths;
            }

            let mut new_path = current_path.clone();
            new_path.push(to.to_string());

            let new_paths = find_paths_to_end(map, from, new_path);

            new_paths
                .iter()
                .filter(|path| path.last().unwrap() == "end")
                .collect();

            paths
        })
}

fn parse_input() -> HashMap<String, Vec<String>> {
    utils::read_string("./input.txt")
        .lines()
        .fold(HashMap::new(), |mut map, line| {
            let split: Vec<&str> = line.split("-").collect();
            let from = split[0].to_string();
            let to = split[1].to_string();

            (*map.entry(from).or_insert(Vec::new())).push(to);

            map
        })
}
