use crate::aoc_error::AOCError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Seat {
    EMPTY = 0,
    OCCUPIED = 1,
    FLOOR,
}

pub fn day11(ifp: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut slen = None;

    let seats: Vec<Vec<Seat>> = {
        let mut p = ifp.to_path_buf();
        p.push("day11.txt");
        let data = std::fs::read_to_string(p)?;
        data.split('\n')
            .filter_map(|line| {
                let sv = line
                    .chars()
                    .filter_map(|c| match c {
                        '.' => Some(Seat::FLOOR),
                        'L' => Some(Seat::EMPTY),
                        '#' => Some(Seat::OCCUPIED),
                        _ => None,
                    })
                    .collect::<Vec<Seat>>();
                if sv.len() == 0 {
                    None
                } else if let Some(l) = slen {
                    if l == sv.len() {
                        Some(sv)
                    } else {
                        eprintln!("Error line length day 11");
                        // break 's AOCError::new("Day 11 input does not have fixed line length");
                        // FIXME
                        None
                    }
                } else {
                    slen = Some(sv.len());
                    Some(sv)
                }
            })
            .collect()
    };
    let slen = slen.ok_or::<AOCError>("Cannot find valid lines in day11.txt".into())?;

    println!("Part 1");
    part1(seats.clone(), slen);
    println!("\nPart 2");
    part2(seats, slen);
    Ok(())
}

#[allow(dead_code)]
fn print_seats(seats: &Vec<Vec<Seat>>, slen: usize) {
    for row in 0..seats.len() {
        for col in 0..slen {
            print!(
                "{}",
                match seats[row][col] {
                    Seat::FLOOR => ".",
                    Seat::OCCUPIED => "#",
                    Seat::EMPTY => "L",
                }
            );
        }
        println!();
    }
}

fn part1(mut seats: Vec<Vec<Seat>>, slen: usize) {
    let rows = seats.len();
    // let new_seats = vec![vec![Seat::FLOOR; slen]; seats.len()];
    let mut rounds = 0;
    let mut changed = true;
    while changed {
        changed = false;
        let mut new_seats = seats.clone();
        for i in 0..rows {
            for j in 0..slen {
                let adjacent_seats = {
                    if i > 0 && i < rows - 1 && j > 0 && j < slen - 1 {
                        /* O O O O
                         * O X X O
                         * O X X O
                         * O O O O */
                        vec![
                            seats[i + 1][j],
                            seats[i - 1][j],
                            seats[i][j + 1],
                            seats[i][j - 1],
                            seats[i + 1][j + 1],
                            seats[i - 1][j + 1],
                            seats[i + 1][j - 1],
                            seats[i - 1][j - 1],
                        ]
                    } else if i == 0 && j > 0 && j < slen - 1 {
                        /* O X X O
                         * O O O O
                         * O O O O
                         * O O O O */
                        vec![
                            seats[i + 1][j],
                            seats[i][j + 1],
                            seats[i][j - 1],
                            seats[i + 1][j + 1],
                            seats[i + 1][j - 1],
                        ]
                    } else if i == rows - 1 && j > 0 && j < slen - 1 {
                        /* O O O O
                         * O O O O
                         * O O O O
                         * O X X O */
                        vec![
                            seats[i - 1][j],
                            seats[i][j + 1],
                            seats[i][j - 1],
                            seats[i - 1][j + 1],
                            seats[i - 1][j - 1],
                        ]
                    } else if j == 0 && i > 0 && i < rows - 1 {
                        /* O O O O
                         * X O O O
                         * X O O O
                         * O O O O */
                        vec![
                            seats[i + 1][j],
                            seats[i - 1][j],
                            seats[i][j + 1],
                            seats[i + 1][j + 1],
                            seats[i - 1][j + 1],
                        ]
                    } else if j == slen - 1 && i > 0 && i < rows - 1 {
                        /* O O O O
                         * O O O X
                         * O O O X
                         * O O O O */
                        vec![
                            seats[i + 1][j],
                            seats[i - 1][j],
                            seats[i][j - 1],
                            seats[i + 1][j - 1],
                            seats[i - 1][j - 1],
                        ]
                    } else if j == slen - 1 && i == rows - 1 {
                        /* O O O O
                         * O O O O
                         * O O O O
                         * O O O X */
                        vec![seats[i - 1][j], seats[i][j - 1], seats[i - 1][j - 1]]
                    } else if j == 0 && i == rows - 1 {
                        /* O O O O
                         * O O O O
                         * O O O O
                         * X O O O */
                        vec![seats[i - 1][j], seats[i][j + 1], seats[i - 1][j + 1]]
                    } else if j == 0 && i == 0 {
                        /* X O O O
                         * O O O O
                         * O O O O
                         * O O O O */
                        vec![seats[i + 1][j], seats[i][j + 1], seats[i + 1][j + 1]]
                    } else {
                        /* O O O X
                         * O O O O
                         * O O O O
                         * O O O O */
                        vec![seats[i + 1][j], seats[i][j - 1], seats[i + 1][j - 1]]
                    }

                    /*
                    let mut v = Vec::with_capacity(8); // Probably faster to allocate with 8 since its small and it wont get bigger
                    for k in [-1isize, 0, 1].iter().copied() {
                        for l in [-1isize, 0, 1].iter().copied() {
                            let ik = i as isize + k;
                            let jl = j as isize + l;
                            if ik < 0 || jl < 0 {
                                continue;
                            }
                            let ik = ik as usize;
                            let jl = jl as usize;
                            if !(ik >= rows || jl >= slen) && !(k == 0 && l == 0) {
                                v.push(seats[ik][jl]);
                            }
                        }
                    }
                    v
                    */
                };

                changed |= match seats[i][j] {
                    Seat::EMPTY => {
                        let mut cond = false;
                        for seat in &adjacent_seats {
                            if *seat == Seat::OCCUPIED {
                                cond = true;
                                break;
                            }
                        }
                        if !cond {
                            new_seats[i][j] = Seat::OCCUPIED;
                            true
                        } else {
                            false
                        }
                    }
                    Seat::OCCUPIED => {
                        let mut c = false;
                        let mut occupied_count = 0;
                        for seat in &adjacent_seats {
                            if *seat == Seat::OCCUPIED {
                                occupied_count += 1;
                                if occupied_count == 4 {
                                    new_seats[i][j] = Seat::EMPTY;
                                    c = true;
                                    break;
                                }
                            }
                        }
                        c
                    }
                    Seat::FLOOR => false,
                };
            }
        }
        // print_seats(&seats, slen);
        // println!();

        seats = new_seats;
        rounds += 1;
    }

    let mut occupied_seats = 0;
    for i in 0..rows {
        for j in 0..slen {
            if seats[i][j] == Seat::OCCUPIED {
                occupied_seats += 1;
            }
        }
    }
    println!(
        "{} seats were occupied after {} rounds",
        occupied_seats, rounds
    );
}

fn part2(mut seats: Vec<Vec<Seat>>, slen: usize) {
    let rows = seats.len();
    let mut rounds = 0;
    let mut changed = true;
    while changed {
        changed = false;
        let mut new_seats = seats.clone();
        for i in 0..rows {
            for j in 0..slen {
                let seen = {
                    let mut seen = 0;

                    // Up
                    for x in (0..i).rev() {
                        if seats[x][j] == Seat::OCCUPIED {
                            seen += 1;
                            break;
                        } else if seats[x][j] == Seat::EMPTY {
                            break;
                        }
                    }
                    // println!("Up {}", seen);
                    // Down
                    for x in (i + 1)..rows {
                        if seats[x][j] == Seat::OCCUPIED {
                            seen += 1;
                            break;
                        } else if seats[x][j] == Seat::EMPTY {
                            break;
                        }
                    }
                    // println!("Down {}", seen);

                    // Left
                    for y in (0..j).rev() {
                        if seats[i][y] == Seat::OCCUPIED {
                            seen += 1;
                            break;
                        } else if seats[i][y] == Seat::EMPTY {
                            break;
                        }
                    }
                    // println!("Left {}", seen);
                    // Right
                    for y in j + 1..slen {
                        if seats[i][y] == Seat::OCCUPIED {
                            seen += 1;
                            break;
                        } else if seats[i][y] == Seat::EMPTY {
                            break;
                        }
                    }
                    // println!("Right {}", seen);

                    // Up Left
                    for (x, y) in (0..i).rev().zip((0..j).rev()) {
                        if seats[x][y] == Seat::OCCUPIED {
                            seen += 1;
                            break;
                        } else if seats[x][y] == Seat::EMPTY {
                            break;
                        }
                    }
                    // println!("Up left {}", seen);
                    // Up Right
                    for (x, y) in (0..i).rev().zip(j + 1..slen) {
                        if seats[x][y] == Seat::OCCUPIED {
                            seen += 1;
                            break;
                        } else if seats[x][y] == Seat::EMPTY {
                            break;
                        }
                    }
                    // println!("Up right {}", seen);

                    // Down Left
                    for (x, y) in ((i + 1)..rows).zip((0..j).rev()) {
                        if seats[x][y] == Seat::OCCUPIED {
                            seen += 1;
                            break;
                        } else if seats[x][y] == Seat::EMPTY {
                            break;
                        }
                    }
                    // println!("Down left {}", seen);

                    // Down Right
                    for (x, y) in ((i + 1)..rows).zip(j + 1..slen) {
                        if seats[x][y] == Seat::OCCUPIED {
                            seen += 1;
                            break;
                        } else if seats[x][y] == Seat::EMPTY {
                            break;
                        }
                    }
                    // println!("Down right {}", seen);
                    seen
                };

                changed |= match seats[i][j] {
                    Seat::EMPTY => {
                        // println!("Empty seat ({}, {}) sees {} occupied seats", i, j, seen);
                        if seen == 0 {
                            new_seats[i][j] = Seat::OCCUPIED;
                            true
                        } else {
                            false
                        }
                    }
                    Seat::OCCUPIED => {
                        // println!("Occupied seat ({}, {}) sees {} occupied seats", i, j, seen);
                        if seen >= 5 {
                            new_seats[i][j] = Seat::EMPTY;
                            true
                        } else {
                            false
                        }
                    }
                    Seat::FLOOR => false,
                };
            }
        }

        seats = new_seats;
        rounds += 1;
    }

    let occupied_seats = {
        let mut occupied_seats = 0;
        for i in 0..rows {
            for j in 0..slen {
                if seats[i][j] == Seat::OCCUPIED {
                    occupied_seats += 1;
                }
            }
        }
        occupied_seats
    };

    println!(
        "{} seats were occupied after {} rounds",
        occupied_seats, rounds
    );
}
