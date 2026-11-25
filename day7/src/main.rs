use std::fs;
use std::collections::{HashMap, HashSet};

// const FILE_PATH: &str = "data/temp.txt";
const FILE_PATH: &str = "data/org.txt";

fn main() {
    part1();
    part2();
}

fn part1() {
    match fs::read_to_string(FILE_PATH) {
        Ok(text) => {
            let mut bag_counts: HashMap<String, HashMap<String, u64>> = HashMap::new();

            // parent_bag and related child_bags
            let mut bags: HashMap<String, HashSet<String>> = HashMap::new();

            let lines = text.split('\n').collect::<Vec<&str>>();
            for &line in &lines {
                if line == "" {
                    continue;
                }

                let parts = line.split(", ").collect::<Vec<&str>>();

                // seperate parent and first child
                let front_parts = parts[0]
                    .split(" contain ")
                    .collect::<Vec<&str>>();

                let parent_bag = front_parts[0]
                    .split(" bags")
                    .collect::<Vec<&str>>()[0];

                // first child
                let first_child_and_count = front_parts[1]
                    .split(" bag")
                    .collect::<Vec<&str>>()[0]
                    .split(' ')
                    .collect::<Vec<&str>>(); 

                if first_child_and_count[first_child_and_count.len()-2] == "no" && first_child_and_count[first_child_and_count.len()-1] == "other" {
                    bags.entry(parent_bag.to_string())
                        .or_insert(HashSet::new());
                    continue;
                }

                let child_count: u64 = first_child_and_count[0]
                    .parse()
                    .expect("child_count should be a number");

                let child_bag = first_child_and_count[1..]
                    .join(" ");

                bags.entry(parent_bag.to_string())
                    .and_modify(|x| {
                        x.insert(child_bag.clone());
                    })
                    .or_insert(HashSet::from([child_bag.clone()]));

                bag_counts.entry(parent_bag.to_string())
                    .and_modify(|x| {
                        x.insert(
                            child_bag.clone(),
                            child_count,
                        );
                    })
                    .or_insert(
                        HashMap::from([
                            (child_bag.clone(), child_count),
                        ])
                    );

                // other children
                for part in &parts[1..] {
                    let child_and_count = part
                        .split(" bag")
                        .collect::<Vec<&str>>()[0]
                        .split(' ')
                        .collect::<Vec<&str>>(); 

                    let child_count: u64 = child_and_count[0]
                        .parse()
                        .expect("child_count should be a number");

                    let child_bag = child_and_count[1..]
                        .join(" ");

                    bags.entry(parent_bag.to_string())
                        .and_modify(|x| {
                            x.insert(child_bag.clone());
                        })
                        .or_insert(HashSet::from([child_bag.clone()]));

                    bag_counts.entry(parent_bag.to_string())
                        .and_modify(|x| {
                            x.insert(
                                child_bag.clone(),
                                child_count,
                            );
                        })
                        .or_insert(
                            HashMap::from([
                                (child_bag.clone(), child_count),
                            ])
                        );
                }
            }

            let shiny_gold_bag = String::from("shiny gold");

            if let Some(parent_bags) = get_parents(&bags, &shiny_gold_bag) {
                println!("[PART 1] {}", parent_bags.len());
            }
        },
        Err(err) => panic!("{}", err),
    }
}

fn get_parents(bags: &HashMap<String, HashSet<String>>, child_bag: &String) -> Option<HashSet<String>> {
    let mut parents: HashSet<String> = HashSet::new();

    for (parent_bag, child_bags) in bags {
        for bag in child_bags {
            if bag == child_bag {
                parents.insert(parent_bag.clone());

                match get_parents(bags, parent_bag) {
                    Some(parent_bags) => {
                        for p in parent_bags {
                            parents.insert(p.clone());
                        }
                    },
                    None => {},
                };

                break;
            }
        }
    }

    if parents.len() == 0 {
        None
    } else {
        Some(parents)
    }
}

fn part2() {
    match fs::read_to_string(FILE_PATH) {
        Ok(text) => {
            let mut bag_counts: HashMap<String, HashMap<String, u64>> = HashMap::new();

            // parent_bag and related child_bags
            let mut bags: HashMap<String, HashSet<String>> = HashMap::new();

            let lines = text.split('\n').collect::<Vec<&str>>();
            for &line in &lines {
                if line == "" {
                    continue;
                }

                let parts = line.split(", ").collect::<Vec<&str>>();

                // seperate parent and first child
                let front_parts = parts[0]
                    .split(" contain ")
                    .collect::<Vec<&str>>();

                let parent_bag = front_parts[0]
                    .split(" bags")
                    .collect::<Vec<&str>>()[0];

                // first child
                let first_child_and_count = front_parts[1]
                    .split(" bag")
                    .collect::<Vec<&str>>()[0]
                    .split(' ')
                    .collect::<Vec<&str>>(); 

                if first_child_and_count[first_child_and_count.len()-2] == "no" && first_child_and_count[first_child_and_count.len()-1] == "other" {
                    bags.entry(parent_bag.to_string())
                        .or_insert(HashSet::new());
                    continue;
                }

                let child_count: u64 = first_child_and_count[0]
                    .parse()
                    .expect("child_count should be a number");

                let child_bag = first_child_and_count[1..]
                    .join(" ");

                bags.entry(parent_bag.to_string())
                    .and_modify(|x| {
                        x.insert(child_bag.clone());
                    })
                    .or_insert(HashSet::from([child_bag.clone()]));

                bag_counts.entry(parent_bag.to_string())
                    .and_modify(|x| {
                        x.insert(
                            child_bag.clone(),
                            child_count,
                        );
                    })
                    .or_insert(
                        HashMap::from([
                            (child_bag.clone(), child_count),
                        ])
                    );

                // other children
                for part in &parts[1..] {
                    let child_and_count = part
                        .split(" bag")
                        .collect::<Vec<&str>>()[0]
                        .split(' ')
                        .collect::<Vec<&str>>(); 

                    let child_count: u64 = child_and_count[0]
                        .parse()
                        .expect("child_count should be a number");

                    let child_bag = child_and_count[1..]
                        .join(" ");

                    bags.entry(parent_bag.to_string())
                        .and_modify(|x| {
                            x.insert(child_bag.clone());
                        })
                        .or_insert(HashSet::from([child_bag.clone()]));

                    bag_counts.entry(parent_bag.to_string())
                        .and_modify(|x| {
                            x.insert(
                                child_bag.clone(),
                                child_count,
                            );
                        })
                        .or_insert(
                            HashMap::from([
                                (child_bag.clone(), child_count),
                            ])
                        );
                }
            }

            let shiny_gold_bag = String::from("shiny gold");
            println!("[PART 2] {}", count_children(&bag_counts, &shiny_gold_bag));
        },
        Err(err) => panic!("{}", err),
    }
}

fn count_children(bags: &HashMap<String, HashMap<String, u64>>, child_bag: &String) -> u64 {
    let mut count: u64 = 0;

    for (parent_bag, children_and_counts) in bags {
        if parent_bag == child_bag {
            for (child, c) in children_and_counts {
                count += c;
                count += c * count_children(bags, child);
            }
        }
    }

    return count;
}
