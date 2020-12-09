pub fn day03(ifp: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut p = ifp.to_path_buf();
    p.push("day03.txt");
    let data = std::fs::read_to_string(p)?;

    let mut ll = None;

    let lines: Vec<Vec<u32>> = data
        .split('\n')
        .filter_map(|line| {
            let c_ll = line.len();
            if ll.is_none() {
                if c_ll == 0 {
                    return None;
                }
                ll = Some(c_ll)
            } else {
                if c_ll != ll.unwrap() {
                    return None;
                    // TODO somehow error here
                }
            }
            let mut v: Vec<u32> = Vec::with_capacity(ll.unwrap());
            // TODO implement verficacion for line.len

            for ch in line.chars() {
                if ch == '.' {
                    v.push(0);
                } else {
                    v.push(1);
                    // TODO somehow error here if ch != #
                }
            }
            Some(v)
        })
        .collect();

    println!("Part 1");
    part1(&lines, ll.unwrap());
    println!("\nPart 2");
    part2(&lines, ll.unwrap());
    Ok(())
}

fn part1(lines: &Vec<Vec<u32>>, ll: usize) {
    let mut lines_it = lines.iter();
    lines_it.next();

    let mut pos = 3;
    let mut counter = 0;
    for line in lines_it {
        counter += line[pos % ll];
        pos += 3;
    }

    println!("Trees encountered {}", counter);
}

fn part2(lines: &Vec<Vec<u32>>, ll: usize) {
    let mut pcounter = 1;
    for p in &[1, 3, 5, 7] {
        let mut lines_it = lines.iter();
        lines_it.next();

        let mut pos = *p;
        let mut counter = 0;
        for line in lines_it {
            counter += line[pos % ll];
            pos += *p;
        }
        pcounter *= counter;
    }

    let mut lines_it = lines.iter();
    lines_it.next();
    lines_it.next();

    let mut pos = 1;
    let mut u = false;
    let mut counter = 0;
    for line in lines_it {
        if u {
            counter += line[pos % ll];
            pos += 1;
        }
        u = !u;
    }

    pcounter *= counter;

    println!("Trees encountered {}", pcounter);
}
