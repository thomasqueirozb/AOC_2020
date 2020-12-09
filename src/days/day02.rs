pub fn day02(ifp: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut p = ifp.to_path_buf();
    p.push("day02.txt");
    let data = std::fs::read_to_string(p)?;
    let lines: Vec<_> = data
        .split('\n')
        .filter_map(|line| {
            let mut it = line.split(" ");

            let constraint = it.next()?;
            let mut constraint_bound_it = constraint.split("-");
            let lower_bound = constraint_bound_it.next()?;
            let upper_bound = constraint_bound_it.next()?;
            if constraint_bound_it.next().is_some() {
                return None;
            }
            let lower_bound = lower_bound.parse::<u32>().ok()?;
            let upper_bound = upper_bound.parse::<u32>().ok()?;

            let mut constraint_char_it = it.next()?.chars();
            let constraint_char = constraint_char_it.next()?;

            if constraint_char_it.next() != Some(':') || constraint_char_it.next().is_some() {
                return None;
            }

            let password = it.next()?;

            if it.next().is_some() {
                return None;
            }

            Some((lower_bound, upper_bound, constraint_char, password))
        })
        .collect();

    println!("Part 1");
    part1(&lines);
    println!("\nPart 2");
    part2(&lines);
    Ok(())
}

fn part1(lines: &Vec<(u32, u32, char, &str)>) {
    let mut valids = 0;
    for (lb, ub, c, pass) in lines {
        let mut count = 0;
        for ch in pass.chars() {
            if ch == *c {
                count += 1;
            }
        }
        if !(count < *lb || count > *ub) {
            valids += 1;
        }
    }

    println!("Valid passwords: {}", valids);
}

fn part2(lines: &Vec<(u32, u32, char, &str)>) {
    let mut valids = 0;

    for (lb, ub, c, pass) in lines {
        if let Some(i0) = pass.chars().nth((*lb - 1) as usize) {
            let c0 = i0 == *c;
            if let Some(i1) = pass.chars().nth((*ub - 1) as usize) {
                let c1 = i1 == *c;

                if !(c0 && c1 || !c0 && !c1) {
                    valids += 1
                }
            }
        }
    }
    println!("Valid passwords: {}", valids);
}
