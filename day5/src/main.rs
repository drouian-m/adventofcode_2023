use std::collections::HashMap;
use std::io::{BufRead, Error};

use utils::read_input_file;

type Map = Vec<Vec<i64>>;
type MapCollection = HashMap<String, Map>;

fn parse_map(input: &str) -> Map {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

fn convert_number(number: i64, map: &Map) -> i64 {
    for line in map {
        let dest_start = line[0];
        let source_start = line[1];
        let range_length = line[2];

        if number >= source_start && number < source_start + range_length {
            return dest_start + (number - source_start);
        }
    }

    number
}

fn find_lowest_location(seeds: Vec<i64>, maps: &MapCollection) -> i64 {
    let mut locations = Vec::<i64>::new();
    let seed_to_soil = maps.get("seed-to-soil").unwrap();
    let soil_to_fertilizer = maps.get("soil-to-fertilizer").unwrap();
    let fertilizer_to_water = maps.get("fertilizer-to-water").unwrap();
    let water_to_light = maps.get("water-to-light").unwrap();
    let light_to_temperature = maps.get("light-to-temperature").unwrap();
    let temperature_to_humidity = maps.get("temperature-to-humidity").unwrap();
    let humidity_to_location = maps.get("humidity-to-location").unwrap();
    for seed in seeds {
        let soil = convert_number(seed, seed_to_soil);
        let fertilizer = convert_number(soil, soil_to_fertilizer);
        let water = convert_number(fertilizer, fertilizer_to_water);
        let light = convert_number(water, water_to_light);
        let temperature = convert_number(light, light_to_temperature);
        let humidity = convert_number(temperature, temperature_to_humidity);
        let location = convert_number(humidity, humidity_to_location);
        locations.push(location);
    }

    let min = locations.iter().min().unwrap();
    *min
}

fn process(input: String) -> Result<i64, Error> {
    let reader = read_input_file(input)?;

    let mut maps: MapCollection = HashMap::new();
    let mut seeds: Vec<i64> = Vec::new();
    let mut current_map_name = String::new();
    let mut current_map_lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            if !current_map_name.is_empty() && !current_map_lines.is_empty() {
                let map = parse_map(&current_map_lines.join("\n"));
                maps.insert(current_map_name.clone(), map);
                current_map_lines.clear();
            }
        } else if line.contains("seeds:") {
            seeds = line
                .trim_start_matches("seeds:")
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
        } else if line.ends_with(" map:") {
            current_map_name = line.trim_end_matches(" map:").to_string();
        } else {
            current_map_lines.push(line);
        }
    }

    if !current_map_name.is_empty() && !current_map_lines.is_empty() {
        let map = parse_map(&current_map_lines.join("\n"));
        maps.insert(current_map_name.clone(), map);
    }

    let lowest_location = find_lowest_location(seeds, &maps);

    Ok(lowest_location)
}

fn main() -> Result<(), Error> {
    println!("Hello advent of code day 5 !");

    let res = process("day5/src/resources/input.txt".to_owned())?;
    println!("The result is {}", res);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::process;

    #[test]
    fn shoud_get_lowest_location() -> Result<(), String> {
        let result = process("src/resources/test-input.txt".to_owned())
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        assert_eq!(result, 35);
        Ok(())
    }
}