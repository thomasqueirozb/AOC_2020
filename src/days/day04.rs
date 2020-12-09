use std::collections::{HashMap, HashSet};

pub fn day04(ifp: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut p = ifp.to_path_buf();
    p.push("day04.txt");
    let data = std::fs::read_to_string(p)?;

    let lines: Vec<&str> = data.split('\n').collect();

    println!("Part 1");
    let passports = part1(&lines);
    println!("\nPart 2");
    part2(passports);
    Ok(())
}

fn part1<'a>(lines: &'a Vec<&str>) -> Vec<HashMap<&'a str, &'a str>> {
    let mut valid_passports = 0;
    let mut n_keys = 0;
    let mut cid_missing = true;
    let mut invalid = false;
    let mut passport = HashMap::new();
    let mut passports: Vec<HashMap<&str, &str>> = vec![];
    let valid_keys: HashSet<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]
        .iter()
        .cloned()
        .collect();
    for line in lines {
        if line == &"" {
            if n_keys != 0 {
                if !invalid && (n_keys == 8 || (n_keys == 7 && cid_missing)) {
                    valid_passports += 1;
                    passports.push(passport.clone());
                }
                n_keys = 0;
                invalid = false;
                cid_missing = true;

                passport.clear();
            }
        } else {
            for token in line.split_whitespace() {
                let t: Vec<_> = token.split(":").collect();
                assert_eq!(t.len(), 2);
                let key = t[0];
                n_keys += 1;
                if valid_keys.contains(key) {
                    if key == "cid" {
                        cid_missing = false;
                    }
                    let val = t[1];
                    passport.insert(key.clone(), val.clone());
                } else {
                    println!("key {}", key);
                    invalid = true;
                }
            }
        }
    }
    if !invalid && (n_keys == 8 || (n_keys == 7 && cid_missing)) {
        valid_passports += 1;
        passports.push(passport.clone());
    }

    println!("Valid passports {}", valid_passports);
    passports
}

fn part2(passports: Vec<HashMap<&str, &str>>) {
    let mut valid_passports = 0;

    let valid_ecls: HashSet<&str> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .cloned()
        .collect();
    for passport in passports {
        let mut valid = true;
        for (key, val) in passport {
            let v = match key {
                "byr" => {
                    if let Ok(byr) = val.parse::<i32>() {
                        byr >= 1920 && byr <= 2002
                    } else {
                        false
                    }
                }
                "iyr" => {
                    if let Ok(iyr) = val.parse::<i32>() {
                        iyr >= 2010 && iyr <= 2020
                    } else {
                        false
                    }
                }
                "eyr" => {
                    if let Ok(eyr) = val.parse::<i32>() {
                        eyr >= 2020 && eyr <= 2030
                    } else {
                        false
                    }
                }
                "hgt" => {
                    if let Some(cm_idx) = val.find("cm") {
                        if let Ok(cm) = val[..cm_idx].parse::<i32>() {
                            cm >= 150 && cm <= 193
                        } else {
                            false
                        }
                    } else if let Some(in_idx) = val.find("in") {
                        if let Ok(inc) = val[..in_idx].parse::<i32>() {
                            inc >= 59 && inc <= 76
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                "hcl" => {
                    let mut vc = val.chars();
                    if let Some(ch) = vc.next() {
                        if ch == '#' {
                            let mut v = true;
                            for ch in vc {
                                if !ch.is_alphanumeric() {
                                    v = false;
                                    break;
                                }
                            }
                            v
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                "ecl" => valid_ecls.contains(val),
                "pid" => {
                    let mut v = true;
                    for ch in val.chars() {
                        if !ch.is_ascii_digit() {
                            v = false;
                            break;
                        }
                    }
                    v && val.len() == 9
                }
                _ => true,
            };
            if !v {
                valid = false;
                break;
            }
        }
        valid_passports += valid as i32;
    }
    println!("Valid passports {}", valid_passports);
}
