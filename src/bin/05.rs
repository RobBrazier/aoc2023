use rayon::prelude::*;

advent_of_code::solution!(5);

#[derive(Debug)]
struct Mapping {
    source_start: u32,
    destination_start: u32,
    range_length: u32,
}

#[derive(Debug)]
struct Seed {
    #[allow(dead_code)]
    id: u32,
    #[allow(dead_code)]
    soil: u32,
    #[allow(dead_code)]
    fertilizer: u32,
    #[allow(dead_code)]
    water: u32,
    #[allow(dead_code)]
    light: u32,
    #[allow(dead_code)]
    temperature: u32,
    #[allow(dead_code)]
    humidity: u32,
    location: u32,
}

fn expand(data: Vec<u32>) -> Mapping {
    // println!("expanding {:?}", data);

    let dest_range_start = data[0];
    let src_range_start = data[1];
    let range_length = data[2];
    // println!(
    //     "dest_range_start: {}, range_length: {}",
    //     dest_range_start, range_length
    // );
    Mapping {
        source_start: src_range_start,
        destination_start: dest_range_start,
        range_length,
    }
}

fn get_value(value: &u32, data: &[Mapping]) -> u32 {
    for mapping in data {
        let source_start = mapping.source_start as u64;
        let source_end = (mapping.source_start as u64) + (mapping.range_length as u64);
        let destination_start = mapping.destination_start as u64;
        let value = *value as u64;
        if (source_start..source_end).contains(&value) {
            let dest_index = value - source_start;
            let result = destination_start + dest_index;
            // println!("{} -> {}", value, result);
            return result as u32;
        }
    }
    // println!("no value found for {}", value);
    *value
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let mut seeds: Vec<u32> = vec![];
    let mut seed_to_soil: Vec<Mapping> = vec![];
    let mut soil_to_fertilizer: Vec<Mapping> = vec![];
    let mut fertilizer_to_water: Vec<Mapping> = vec![];
    let mut water_to_light: Vec<Mapping> = vec![];
    let mut light_to_temperature: Vec<Mapping> = vec![];
    let mut temperature_to_humidity: Vec<Mapping> = vec![];
    let mut humidity_to_location: Vec<Mapping> = vec![];

    let mut header = "";

    for line in lines {
        if line.starts_with("seeds") {
            let seeds_str = line.split(": ").nth(1).unwrap();
            seeds = seeds_str
                .split_ascii_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            continue;
        }
        if line.starts_with("seed-to-soil") {
            header = "seed-to-soil";
            continue;
        } else if line.starts_with("soil-to-fertilizer") {
            header = "soil-to-fertilizer";
            continue;
        } else if line.starts_with("fertilizer-to-water") {
            header = "fertilizer-to-water";
            continue;
        } else if line.starts_with("water-to-light") {
            header = "water-to-light";
            continue;
        } else if line.starts_with("light-to-temperature") {
            header = "light-to-temperature";
            continue;
        } else if line.starts_with("temperature-to-humidity") {
            header = "temperature-to-humidity";
            continue;
        } else if line.starts_with("humidity-to-location") {
            header = "humidity-to-location";
            continue;
        }

        if line.trim().is_empty() {
            continue;
        }

        match header {
            "seed-to-soil" => {
                // println!("expanding seed-to-soil");
                seed_to_soil.push(expand(
                    line.split_ascii_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>(),
                ));
            }
            "soil-to-fertilizer" => {
                // println!("expanding soil-to-fertilizer");
                soil_to_fertilizer.push(expand(
                    line.split_ascii_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>(),
                ));
            }
            "fertilizer-to-water" => {
                // println!("expanding fertilizer-to-water");
                fertilizer_to_water.push(expand(
                    line.split_ascii_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>(),
                ));
            }
            "water-to-light" => {
                // println!("expanding water-to-light");
                water_to_light.push(expand(
                    line.split_ascii_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>(),
                ));
            }
            "light-to-temperature" => {
                // println!("expanding light-to-temperature");
                light_to_temperature.push(expand(
                    line.split_ascii_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>(),
                ));
            }
            "temperature-to-humidity" => {
                // println!("expanding temperature-to-humidity");
                temperature_to_humidity.push(expand(
                    line.split_ascii_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>(),
                ));
            }
            "humidity-to-location" => {
                // println!("expanding humidity-to-location");
                humidity_to_location.push(expand(
                    line.split_ascii_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>(),
                ));
            }
            _ => {}
        }
    }
    let full_seeds: Vec<Seed> = seeds
        .iter()
        .map(|seed| {
            let soil = get_value(seed, &seed_to_soil);
            let fertilizer = get_value(&soil, &soil_to_fertilizer);
            let water = get_value(&fertilizer, &fertilizer_to_water);
            let light = get_value(&water, &water_to_light);
            let temperature = get_value(&light, &light_to_temperature);
            let humidity = get_value(&temperature, &temperature_to_humidity);
            let location = get_value(&humidity, &humidity_to_location);
            Seed {
                id: *seed,
                soil,
                fertilizer,
                water,
                light,
                temperature,
                humidity,
                location,
            }
        })
        .collect();

    let lowest_location = full_seeds
        .iter()
        .min_by(|a, b| a.location.partial_cmp(&b.location).unwrap())
        .unwrap();
    Some(lowest_location.location)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let mut seeds: Vec<(u32, u32)> = vec![];
    let mut seed_to_soil: Vec<Mapping> = vec![];
    let mut soil_to_fertilizer: Vec<Mapping> = vec![];
    let mut fertilizer_to_water: Vec<Mapping> = vec![];
    let mut water_to_light: Vec<Mapping> = vec![];
    let mut light_to_temperature: Vec<Mapping> = vec![];
    let mut temperature_to_humidity: Vec<Mapping> = vec![];
    let mut humidity_to_location: Vec<Mapping> = vec![];

    let mut header = "";

    for line in lines {
        if line.starts_with("seeds") {
            let seeds_str = line.split(": ").nth(1).unwrap();
            let raw_seeds: Vec<u32> = seeds_str
                .split_ascii_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            seeds = raw_seeds.chunks(2).map(|x| (x[0], x[1])).collect();
            continue;
        }
        if line.starts_with("seed-to-soil") {
            header = "seed-to-soil";
            continue;
        } else if line.starts_with("soil-to-fertilizer") {
            header = "soil-to-fertilizer";
            continue;
        } else if line.starts_with("fertilizer-to-water") {
            header = "fertilizer-to-water";
            continue;
        } else if line.starts_with("water-to-light") {
            header = "water-to-light";
            continue;
        } else if line.starts_with("light-to-temperature") {
            header = "light-to-temperature";
            continue;
        } else if line.starts_with("temperature-to-humidity") {
            header = "temperature-to-humidity";
            continue;
        } else if line.starts_with("humidity-to-location") {
            header = "humidity-to-location";
            continue;
        }

        if line.trim().is_empty() {
            continue;
        }

        match header {
            "seed-to-soil" => {
                println!("expanding seed-to-soil");
                seed_to_soil.push(expand(
                    line.split_ascii_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>(),
                ));
            }
            "soil-to-fertilizer" => {
                println!("expanding soil-to-fertilizer");
                soil_to_fertilizer.push(expand(
                    line.split_ascii_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>(),
                ));
            }
            "fertilizer-to-water" => {
                println!("expanding fertilizer-to-water");
                fertilizer_to_water.push(expand(
                    line.split_ascii_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>(),
                ));
            }
            "water-to-light" => {
                println!("expanding water-to-light");
                water_to_light.push(expand(
                    line.split_ascii_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>(),
                ));
            }
            "light-to-temperature" => {
                println!("expanding light-to-temperature");
                light_to_temperature.push(expand(
                    line.split_ascii_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>(),
                ));
            }
            "temperature-to-humidity" => {
                println!("expanding temperature-to-humidity");
                temperature_to_humidity.push(expand(
                    line.split_ascii_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>(),
                ));
            }
            "humidity-to-location" => {
                println!("expanding humidity-to-location");
                humidity_to_location.push(expand(
                    line.split_ascii_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>(),
                ));
            }
            _ => {}
        }
    }
    let mut lowest_locations: Vec<Seed> = vec![];

    for seed in &seeds {
        println!("seed: {:?}", seed);
        let seeds = (seed.0..(seed.0 + seed.1)).collect::<Vec<u32>>();

        lowest_locations.push(
            seeds
                .into_par_iter()
                .map(|id| {
                    let soil = get_value(&id, &seed_to_soil);
                    let fertilizer = get_value(&soil, &soil_to_fertilizer);
                    let water = get_value(&fertilizer, &fertilizer_to_water);
                    let light = get_value(&water, &water_to_light);
                    let temperature = get_value(&light, &light_to_temperature);
                    let humidity = get_value(&temperature, &temperature_to_humidity);
                    let location = get_value(&humidity, &humidity_to_location);
                    Seed {
                        id,
                        soil,
                        fertilizer,
                        water,
                        light,
                        temperature,
                        humidity,
                        location,
                    }
                })
                .min_by(|a, b| a.location.partial_cmp(&b.location).unwrap())
                .unwrap(),
        );
        println!("lowest_locations: {:?}", lowest_locations);
    }

    let lowest_location = lowest_locations
        .iter()
        .min_by(|a, b| a.location.partial_cmp(&b.location).unwrap())
        .unwrap();
    Some(lowest_location.location)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
