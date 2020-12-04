use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt, bytes::complete::tag, bytes::complete::take_till, character::complete::space1,
    multi::separated_list0, number::complete::hex_u32, sequence::separated_pair, IResult,
};

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<HashMap<String, String>> {
    // let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n\nhcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm\n\nhcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in";
    // let input = "eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\niyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946\n\nhcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\nhgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007";
    // let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f\n\neyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\nhcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022\n\niyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    let fixed_input = input
        .replace("\n\n", "\t")
        .replace("\n", " ")
        .replace("\t", "\n");

    let parsed = fixed_input
        .lines()
        .map(|line| {
            let res: IResult<_, _> = separated_list0(
                space1,
                alt((
                    separated_pair(tag("byr"), tag(":"), take_till(|a| a == ' ' || a == '\n')),
                    separated_pair(tag("iyr"), tag(":"), take_till(|a| a == ' ' || a == '\n')),
                    separated_pair(tag("eyr"), tag(":"), take_till(|a| a == ' ' || a == '\n')),
                    separated_pair(tag("hgt"), tag(":"), take_till(|a| a == ' ' || a == '\n')),
                    separated_pair(tag("hcl"), tag(":"), take_till(|a| a == ' ' || a == '\n')),
                    separated_pair(tag("ecl"), tag(":"), take_till(|a| a == ' ' || a == '\n')),
                    separated_pair(tag("pid"), tag(":"), take_till(|a| a == ' ' || a == '\n')),
                    separated_pair(tag("cid"), tag(":"), take_till(|a| a == ' ' || a == '\n')),
                )),
            )(line);
            res.unwrap()
                .1
                .iter()
                .map(|(a, b)| ((*a).into(), (*b).into()))
                .collect::<HashMap<_, _>>()
        })
        .collect::<Vec<_>>();

    parsed
}

#[aoc(day4, part1)]
pub fn solve_p1(input: &Vec<HashMap<String, String>>) -> u64 {
    input
        .iter()
        .map(|passport| if validate_fields(passport) { 1 } else { 0 })
        .sum()
}

fn validate_fields(input: &HashMap<String, String>) -> bool {
    input.contains_key("byr")
        && input.contains_key("iyr")
        && input.contains_key("eyr")
        && input.contains_key("hgt")
        && input.contains_key("hcl")
        && input.contains_key("ecl")
        && input.contains_key("pid")
    // && input.contains_key("cid")
}

#[aoc(day4, part2)]
pub fn solve_p2(input: &Vec<HashMap<String, String>>) -> u64 {
    input
        .iter()
        .map(|passport| {
            let res = if validate_fields2(passport) { 1 } else { 0 };
            // println!("{} {:?}", res, passport);
            res
        })
        .sum()
}

fn validate_fields2(input: &HashMap<String, String>) -> bool {
    check_byr(input.get_key_value("byr"))
        && check_iyr(input.get_key_value("iyr"))
        && check_eyr(input.get_key_value("eyr"))
        && check_hgt(input.get_key_value("hgt"))
        && check_hcl(input.get_key_value("hcl"))
        && check_ecl(input.get_key_value("ecl"))
        && check_pid(input.get_key_value("pid"))
    // && input.contains_key("cid")
}

fn check_byr(entry: Option<(&String, &String)>) -> bool {
    if let Some((_, v)) = entry {
        let res = v.parse::<i32>().unwrap() >= 1920 && v.parse::<i32>().unwrap() <= 2002;
        // println!("\tbyr: {}, {}", res, v);
        res
    } else {
        false
    }
}

fn check_iyr(entry: Option<(&String, &String)>) -> bool {
    if let Some((_, v)) = entry {
        let res = v.parse::<i32>().unwrap() >= 2010 && v.parse::<i32>().unwrap() <= 2020;
        // println!("\tiyr: {}, {}", res, v);
        res
    } else {
        false
    }
}

fn check_eyr(entry: Option<(&String, &String)>) -> bool {
    if let Some((_, v)) = entry {
        let res = v.parse::<i32>().unwrap() >= 2020 && v.parse::<i32>().unwrap() <= 2030;
        // println!("\teyr: {}, {}", res, v);
        res
    } else {
        false
    }
}

fn check_hgt(entry: Option<(&String, &String)>) -> bool {
    if let Some((_, v)) = entry {
        let res = if v.ends_with("cm") {
            if let Some(Ok(val)) = v.strip_suffix("cm").and_then(|i| i.parse::<i32>().into()) {
                val >= 150 && val <= 193
            } else {
                false
            }
        } else if v.ends_with("in") {
            if let Some(Ok(val)) = v.strip_suffix("in").and_then(|i| i.parse::<i32>().into()) {
                val >= 59 && val <= 76
            } else {
                false
            }
        } else {
            false
        };
        // println!("\thgt: {}, {}", res, v);
        res
    } else {
        false
    }
}
fn check_hcl(entry: Option<(&String, &String)>) -> bool {
    if let Some((_, v)) = entry {
        let res = if v.len() == 7 && v.starts_with("#") {
            let num = v.strip_prefix("#").unwrap();
            let res: IResult<&[u8], u32> = hex_u32(num.as_bytes());
            if let Ok(_) = res {
                true
            } else {
                false
            }
        } else {
            false
        };
        // println!("\thcl: {} {}", res, v);
        res
    } else {
        false
    }
}

fn check_ecl(entry: Option<(&String, &String)>) -> bool {
    let opts = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    if let Some((_, v)) = entry {
        let res = opts.contains(&(*v).as_str());
        // println!("\tecl: {} {}", res, v);
        res
    } else {
        false
    }
}

fn check_pid(entry: Option<(&String, &String)>) -> bool {
    if let Some((_, v)) = entry {
        let res = v.len() == 9 && v.parse::<u32>().is_ok();
        // println!("\tpid: {} {}", res, v);
        res
    } else {
        false
    }
}

/*

byr (Birth Year)
iyr (Issue Year)
eyr (Expiration Year)
hgt (Height)
hcl (Hair Color)
ecl (Eye Color)
pid (Passport ID)
cid (Country ID)
*/
