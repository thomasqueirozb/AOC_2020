pub mod aoc_error;
pub mod days;

use crate::days::{
    day01::day01, day02::day02, day03::day03, day04::day04, day05::day05, day06::day06,
};
use aoc_error::AOCError;

use clap::{App, Arg};

use std::error::Error;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("AOC 2020")
        .version("0.1.0")
        .author("Thomas Queiroz <thomasqueirozb@gmail.com>")
        .about("Advent of Code 2020")
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .value_name("DAY")
                .help("Chooses day")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("input_folder")
                .short("i")
                .long("input")
                .value_name("INPUT_FOLDER")
                .help(
                    "Path to folder containing inputs with names dayXX.txt\n\
                The default path is the folder inputs/ and if this folder is \n\
                not found in the current directory it will be searched for in \n\
                the directory above.",
                )
                .next_line_help(true)
                .takes_value(true),
        )
        .get_matches();

    let ifp = {
        match matches.value_of("input_folder") {
            Some(input_folder) => {
                let ifp = Path::new(input_folder);
                if ifp.is_dir() {
                    Ok(ifp)
                } else {
                    Err(AOCError::new("Supplied input_folder is not a directory"))
                }
            }
            None => {
                let ifp = Path::new("inputs");
                if ifp.is_dir() {
                    Ok(ifp)
                } else {
                    let ifp = Path::new("../inputs");
                    if ifp.is_dir() {
                        Ok(ifp)
                    } else {
                        Err(AOCError::new(
                            "Could not find folder inputs/ in this directory or the directory above",
                        ))
                    }
                }
            }
        }
    }?;

    let day: i32 = match matches.value_of("day") {
        Some(day) => Ok(day.parse::<i32>()?),
        None => {
            let mdr: Result<Option<i32>, std::io::Error> = {
                let mut max_day = None;
                for entry in fs::read_dir(&ifp)? {
                    if let Ok(entry) = entry {
                        if let Ok(file_type) = entry.file_type() {
                            if file_type.is_file() {
                                if let Some(name) = entry.file_name().to_str() {
                                    // check if name is dayXX.txt
                                    if name.find("day") == Some(0)
                                        && name.find(".txt") == Some(5)
                                        && name.len() == 9
                                    {
                                        // Create string from name[3..4]
                                        let n_big = name.chars().nth(3).unwrap();
                                        let n_small = name.chars().nth(4).unwrap();
                                        let mut s = String::with_capacity(2);
                                        s.push(n_big);
                                        s.push(n_small);

                                        // Check if constructed string is a valid i32 number
                                        if let Ok(i) = s.parse::<i32>() {
                                            if let Some(m) = max_day {
                                                if i > m {
                                                    max_day = Some(i)
                                                }
                                            } else {
                                                max_day = Some(i)
                                            }
                                        }
                                    }
                                };
                            }
                        }
                    }
                }
                Ok(max_day)
            };
            mdr?.ok_or("No files matching dayXX.txt in inputs folder")
        }
    }?;

    if day <= 0 || day > 31 {
        Err(AOCError::new("Day not valid"))?;
    }

    match day {
        1 => day01(ifp),
        2 => day02(ifp),
        3 => day03(ifp),
        4 => day04(ifp),
        5 => day05(ifp),
        6 => day06(ifp),
        _ => todo!("Day {} not implemented", day),
    }?;

    Ok(())
}
