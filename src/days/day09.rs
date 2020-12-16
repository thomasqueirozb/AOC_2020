use crate::aoc_error::AOCError;
use std::collections::VecDeque;

pub fn day09(ifp: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut p = ifp.to_path_buf();
    p.push("day09.txt");
    let data = std::fs::read_to_string(p)?;
    let nums: Vec<_> = data
        .split('\n')
        .filter_map(|line| line.parse::<u64>().ok())
        .collect();

    if nums.len() < 25 {
        return Err(
            AOCError::new("Input supplied for day 09 has less than 25 valid entries").into(),
        );
    }

    println!("Part 1");
    let invalid_number = part1(&nums)
        .ok_or::<AOCError>(AOCError::new("Could not find invalid number for day 09"))?;
    println!("\nPart 2");
    part2(&nums, invalid_number);
    Ok(())
}

fn part1(nums: &Vec<u64>) -> Option<u64> {
    let mut queue: VecDeque<_> = nums.iter().take(25).copied().collect();
    for num in nums.iter().skip(25).copied() {
        let c = 'l: loop {
            for i in 0..25 {
                for j in i..25 {
                    if queue[i] + queue[j] == num {
                        break 'l true;
                    }
                }
            }
            break 'l false;
        };
        if !c {
            println!(
                "Number {} is not a sum of any of two of the 25 previous numbers",
                num
            );
            return Some(num);
        }
        queue.pop_front();
        queue.push_back(num);
    }
    None
}

fn part2(nums: &Vec<u64>, invalid_number: u64) {
    for i in 0..(nums.len() - 1) {
        if nums[i] == invalid_number {
            continue;
        }

        let mut s = nums[i];
        for j in (i + 1)..(nums.len()) {
            s += nums[j];
            if s >= invalid_number {
                if s == invalid_number {
                    let it = nums.iter().skip(i).take(j - i);
                    println!(
                        "Sum of minimum and maximum item in sequence: {}",
                        it.clone().min().unwrap() + it.max().unwrap()
                    );
                }
            }
        }
    }
}
