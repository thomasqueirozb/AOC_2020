use std::collections::{HashMap, HashSet};

pub fn day06(ifp: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut p = ifp.to_path_buf();
    p.push("day06.txt");
    let data = std::fs::read_to_string(p)?;

    let lines: Vec<&str> = data.split('\n').collect();

    println!("Part 1");
    part1(&lines);
    println!("\nPart 2");
    part2(&lines);
    Ok(())
}

fn part1(lines: &Vec<&str>) {
    let mut group = String::new();
    let mut counts: usize = 0;
    for line in lines {
        if line == &"" {
            let mut questions = HashSet::new();
            for ch in group.chars() {
                questions.insert(ch);
            }
            counts += questions.len();
            group.clear();
        } else {
            group.push_str(line);
        }
    }
    println!("Sum of counts: {}", counts);
}

fn part2(lines: &Vec<&str>) {
    let mut group: Vec<&str> = vec![];
    let mut counts: usize = 0;
    for line in lines {
        if line == &"" {
            let mut group_questions = HashMap::new();
            let group_len = group.len();

            for person in &group {
                let mut questions = HashSet::new();
                for ch in person.chars() {
                    questions.insert(ch);
                }

                for question in questions {
                    let c = group_questions.entry(question).or_insert(0);
                    *c += 1;
                }
            }

            for val in group_questions.values() {
                if *val == group_len {
                    counts += 1;
                }
            }
            group.clear();
        } else {
            group.push(line);
        }
    }
    println!("Sum of counts: {}", counts);
}
