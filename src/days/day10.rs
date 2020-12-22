// use crate::aoc_error::AOCError;

pub fn day10(ifp: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut p = ifp.to_path_buf();
    p.push("day10.txt");
    let data = std::fs::read_to_string(p)?;

    let jolts = {
        let mut jolts: Vec<_> = std::iter::once(0u64)
            .chain(data.split('\n').filter_map(|line| line.parse::<u64>().ok()))
            .collect();
        jolts.sort();
        jolts
    };

    // println!("{:#?}", jolts);
    println!("Part 1");
    part1(&jolts);
    println!("\nPart 2");
    part2(&jolts);
    Ok(())
}

fn part1(jolts: &Vec<u64>) {
    let mut count1 = 0;
    let mut count3 = 1; // Last one is always 3
    for i in 0..jolts.len() - 1 {
        match jolts[i + 1] - jolts[i] {
            1 => count1 += 1,
            3 => count3 += 1,
            _ => (),
        }
    }

    // // We start at 0
    // match jolts[0] - 0 {
    //     1 => count1 += 1,
    //     3 => count3 += 1,
    //     _ => (),
    // }
    println!("{} * {} = {}", count1, count3, count1 * count3);
}

fn part2(jolts: &Vec<u64>) {
    let jl = jolts.len();
    let mut cache: Vec<u64> = vec![0; jl]; // Could use Option<i64> but will use 0 as default value
    cache[jl - 1] = 1; // This line is critical - it is assumed that values for jolts[>i] are already calculated

    // println!("cache {:#?}", cache);

    /*
    jolts = [0, 1, 2, 3, 5, 7];
    Possibilities:
        0, 1, 2, 3, 5, 7
        0, 1,    3, 5, 7
        0, 1, 2,    5, 7
        0,    2, 3, 5, 7
        0,    2,    5, 7
        0,       3, 5, 7

    cache[5] = 1; // Starting condition

    i = 4; jolts[i] = 5;
    counter = 1;
    cache[4] = cache[5] = 1

    i = 3; jolts[i] = 3;
    counter = 1;
    cache[3] = cache[4] = 1

    i = 2; jolts[i] = 2;
    counter = 2;
    cache[2] = cache[3] + cache[4] = 2

    i = 1; jolts[i] = 1;
    counter = 2;
    cache[1] = cache[2] + cache[3] = 3

    i = 0; jolts[i] = 0;
    counter = 3;
    cache[0] = cache[1] + cache[2] + cache[3] = 6
    */

    for i in (0..jl - 1).rev() {
        let mut counter: u64 = 0;
        let mut j = i + 1; // Always valid since maximum value of i is jl - 1
        loop {
            let diff = jolts[j] - jolts[i];
            if diff > 3 {
                break;
            }

            counter += cache[j];

            j += 1;
            if j >= jl {
                break;
            }
        }

        cache[i] = counter;
    }
    println!("cache[0] = {}", cache[0]);
}
