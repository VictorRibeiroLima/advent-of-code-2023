use serde::{Deserialize, Serialize};
use yew_agent::prelude::*;

macro_rules! day {
    ($lib:ident,$part:expr,$input:expr ) => {
        match $part.as_str() {
            "part1" => $lib::part_1::process($input).to_string(),
            "part2" => $lib::part_2::process($input).to_string(),
            _ => "Not implemented".to_string(),
        }
    };
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Input {
    pub input: String,
    pub part: String,
    pub day: String,
}

#[oneshot]
pub fn ProcessInput(input: Input) -> String {
    let part = input.part;
    let day = input.day;
    let input = &input.input;

    match day.as_str() {
        "day1" => day!(day_1_lib, part, input),
        "day2" => day!(day_2_lib, part, input),
        "day3" => day!(day_3_lib, part, input),
        "day4" => day!(day_4_lib, part, input),
        "day5" => day!(day_5_lib, part, input),
        "day6" => day!(day_6_lib, part, input),
        "day7" => day!(day_7_lib, part, input),
        "day8" => day!(day_8_lib, part, input),
        "day9" => day!(day_9_lib, part, input),
        "day10" => day!(day_10_lib, part, input),
        "day11" => day!(day_11_lib, part, input),
        "day12" => day!(day_12_lib, part, input),
        "day13" => day!(day_13_lib, part, input),
        "day14" => day!(day_14_lib, part, input),
        "day15" => day!(day_15_lib, part, input),
        "day16" => day!(day_16_lib, part, input),
        "day17" => day!(day_17_lib, part, input),
        "day18" => day!(day_18_lib, part, input),
        "day19" => day!(day_19_lib, part, input),
        "day20" => day!(day_20_lib, part, input),
        "day21" => day!(day_21_lib, part, input),
        "day22" => day!(day_22_lib, part, input),
        "day23" => day!(day_23_lib, part, input),
        "day24" => day_24(part, input),
        "day25" => day_25(part, input),
        _ => "Not implemented".to_string(),
    }
}

fn day_24(part: String, _: &str) -> String {
    match part.as_str() {
        "part1" => "i64 needed".to_string(),
        _ => "Not implemented".to_string(),
    }
}

fn day_25(part: String, input: &str) -> String {
    match part.as_str() {
        "part1" => day_25_lib::part_1::process(input).to_string(),
        _ => "Not implemented".to_string(),
    }
}
