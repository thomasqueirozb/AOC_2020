use std::collections::HashMap;

pub fn day07(ifp: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut p = ifp.to_path_buf();
    p.push("day07.txt");
    let data = std::fs::read_to_string(p)?;

    let bags: HashMap<&str, Vec<(u32, &str)>> = data
        .split('\n')
        .filter_map(|line| {
            let bc = line.find(" bags contain ")?;
            let mbag = &line[..bc];
            let cbags = &line[bc + 14..line.len() - 1];
            let mut vbags = vec![];

            if cbags != "no other bags" {
                for cbag in cbags.split(", ") {
                    let space_idx = cbag.find(' ')?;
                    let bag_idx = cbag.find(" bag")?;
                    let n = &cbag[..space_idx];
                    let n = n.parse::<u32>().ok()?;
                    let bag = &cbag[space_idx + 1..bag_idx]; // Can panic
                    vbags.push((n, bag));
                }
            }
            // println!("cbags {}", cbags);
            Some((mbag, vbags))
        })
        .collect();

    println!("Part 1");
    part1(&bags);
    println!("\nPart 2");
    part2(&bags)?;
    Ok(())
}

fn contains_bag<'a>(
    b: &'a str,
    vb: &Vec<(u32, &'a str)>,
    shiny_gold_paths: &mut HashMap<&'a str, u32>,
    bags: &HashMap<&str, Vec<(u32, &'a str)>>,
) -> u32 {
    if let Some(c) = shiny_gold_paths.get(b) {
        *c
    } else {
        let mut counter = 0;
        for (n, bag) in vb {
            counter += n * contains_bag(bag, bags.get(bag).unwrap(), shiny_gold_paths, bags);
        }
        shiny_gold_paths.insert(b, counter);
        counter
    }
}

fn part1(bags: &HashMap<&str, Vec<(u32, &str)>>) {
    let mut shiny_gold_paths: HashMap<&str, u32> = HashMap::new();
    shiny_gold_paths.insert("shiny gold", 1);

    let mut c = 0;
    for (bag, vbags) in bags {
        if contains_bag(bag, vbags, &mut shiny_gold_paths, &bags) != 0 {
            c += 1;
        }
    }
    if c != 0 {
        c -= 1; // The shiny gold bag cannot hold itself
    }
    println!("Bags leading to shiny gold bag: {}", c);
}

fn contained_bag<'a>(
    bag: &'a str,
    contained_bags: &mut HashMap<&'a str, u32>,
    bags: &HashMap<&'a str, Vec<(u32, &'a str)>>,
) -> u32 {
    if let Some(c) = contained_bags.get(bag) {
        *c
    } else {
        let vbags = bags.get(bag).unwrap(); // FIXME
        if vbags.is_empty() {
            // println!("{} contains no bags", bag);
            contained_bags.insert(bag, 0);
            0
        } else {
            let mut c = 0;
            for (vbcount, vbag) in vbags {
                let res = contained_bag(vbag, contained_bags, bags);
                c += (res + 1) * vbcount;
            }
            // contained_bags.entry(bag).or_insert(c);
            contained_bags.insert(bag, c);
            // println!("{} contains {} bags", bag, c);
            c
        }
    }
}

fn part2(bags: &HashMap<&str, Vec<(u32, &str)>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut contained_bags: HashMap<&str, u32> = HashMap::new();

    println!(
        "Shiny gold contains {} bags",
        contained_bag("shiny gold", &mut contained_bags, &bags)
    );
    Ok(())
}
