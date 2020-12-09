pub fn day01(ifp: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut p = ifp.to_path_buf();
    p.push("day01.txt");
    let data = std::fs::read_to_string(p)?;
    let nums: Vec<_> = data
        .split('\n')
        .filter_map(|line| line.parse::<i32>().ok())
        .collect();

    println!("Part 1");
    part1(&nums);
    println!("\nPart 2");
    part2(&nums);
    Ok(())
}

fn part1(nums: &Vec<i32>) {
    'p1: loop {
        for i in nums.iter() {
            for j in nums.iter() {
                if i + j == 2020 {
                    println!("{} + {} = 2020; {} * {} = {}", i, j, i, j, i * j);
                    break 'p1;
                }
            }
        }
        break 'p1;
    }
}

fn part2(nums: &Vec<i32>) {
    'p2: loop {
        for i in nums.iter() {
            for j in nums.iter() {
                for k in nums.iter() {
                    if i + j + k == 2020 {
                        println!(
                            "{} + {} + {} = 2020; {} * {} * {} = {}",
                            i,
                            j,
                            k,
                            i,
                            j,
                            k,
                            i * j * k
                        );
                        break 'p2;
                    }
                }
            }
        }
        break 'p2;
    }
}
