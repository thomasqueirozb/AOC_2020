pub fn day05(ifp: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut p = ifp.to_path_buf();
    p.push("day05.txt");
    let data = std::fs::read_to_string(p)?;

    let seats: Vec<isize> = data
        .split('\n')
        .filter_map(|line| {
            let mut line_it = line.chars().into_iter().peekable();
            if line_it.peek().is_none() {
                return None;
            }
            let row: String = line_it
                .clone()
                .take(7)
                .map(|ch| match ch {
                    'F' => '0',
                    'B' => '1',
                    _ => {
                        panic!("Input not F or B");
                    }
                })
                .collect();
            assert_eq!(row.len(), 7);
            let col: String = line_it
                .skip(7)
                .take(3)
                .map(|ch| match ch {
                    'L' => '0',
                    'R' => '1',
                    _ => {
                        panic!("Input not L or R");
                    }
                })
                .collect();
            assert_eq!(col.len(), 3);

            let row = isize::from_str_radix(row.as_ref(), 2).unwrap();
            let col = isize::from_str_radix(col.as_ref(), 2).unwrap();

            Some(row * 8 + col)
        })
        .collect();

    println!("Part 1");
    part1(&seats);
    println!("\nPart 2");
    part2(seats);
    Ok(())
}

fn part1(seats: &Vec<isize>) {
    println!("Highest seat ID {}", seats.iter().max().unwrap());
}

fn part2(mut seats: Vec<isize>) {
    seats.sort_unstable();
    for i in 1..seats.len() {
        let diff = seats[i] - seats[i - 1];
        if diff != 1 {
            println!("My seat ID {}", seats[i] - 1);
            break;
        }
    }
}
