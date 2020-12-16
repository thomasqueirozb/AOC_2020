#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Op {
    ACC,
    JMP,
    NOP,
}

pub fn day08(ifp: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut p = ifp.to_path_buf();
    p.push("day08.txt");
    let data = std::fs::read_to_string(p)?;

    let instructions: Vec<(Op, i64)> = data
        .split('\n')
        .filter_map(|line| {
            let mut line_sp = line.split_whitespace();
            let op = line_sp.next()?;
            let n = line_sp.next()?;
            if line_sp.next().is_some() {
                return None;
            }
            let n: i64 = n.parse().ok()?;
            let op = match op {
                "acc" => Some(Op::ACC),
                "jmp" => Some(Op::JMP),
                "nop" => Some(Op::NOP),
                _ => None,
            }?;

            Some((op, n))
        })
        .collect();

    // println!("Instructions {:#?}", instructions);
    println!("Part 1");
    part1(&instructions);
    println!("\nPart 2");
    part2(instructions);
    Ok(())
}

fn part1(instructions: &Vec<(Op, i64)>) {
    let size = instructions.len();
    let mut visited = vec![false; size];

    let mut acc: i64 = 0;
    let mut pc: usize = 0;

    while pc < size && !visited[pc] {
        visited[pc] = true;

        let (op, n) = &instructions[pc];
        // println!("{:?} {} {}", op, n, pc);
        match op {
            Op::JMP => {
                pc = pc.wrapping_add(*n as isize as usize);
            }
            Op::ACC => {
                acc += n;
                pc += 1;
            }
            Op::NOP => pc += 1,
        };
    }
    println!("Accumulator: {}", acc);
}

fn completes(instructions: &Vec<(Op, i64)>) -> bool {
    let size = instructions.len();
    let mut visited = vec![false; size];

    let mut pc: usize = 0;

    while pc < size && !visited[pc] {
        visited[pc] = true;

        let (op, n) = &instructions[pc];
        match op {
            Op::JMP => {
                pc = pc.wrapping_add(*n as isize as usize);
            }
            Op::ACC => {
                pc += 1;
            }
            Op::NOP => pc += 1,
        };
    }
    pc >= size
}

fn part2(mut instructions: Vec<(Op, i64)>) {
    // for (op, _) in instructions.iter_mut() { // Doesn't work bc have to borrow as mutable twice
    for i in 0..instructions.len() {
        {
            let (op, _) = &mut instructions[i];

            if *op == Op::NOP {
                *op = Op::JMP;
            } else if *op == Op::JMP {
                *op = Op::NOP;
            }
        }

        if completes(&instructions) {
            part1(&instructions);
            break;
        }

        {
            let (op, _) = &mut instructions[i];

            if *op == Op::NOP {
                *op = Op::JMP;
            } else if *op == Op::JMP {
                *op = Op::NOP;
            }
        }
    }
}
