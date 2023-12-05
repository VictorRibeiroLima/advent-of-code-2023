use std::fmt::Display;

mod inputs;

#[derive(Debug)]
pub struct MapRange {
    pub start: u32,
    pub end: u32,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Map {
    pub destination: MapRange,
    pub source: MapRange,
    range: u32,
}

impl Map {
    pub fn new(input: &str) -> Self {
        let input = input.trim();
        let mut destination = 0;
        let mut source = 0;
        let mut range = 0;
        for (i, number) in input.split_whitespace().enumerate() {
            let number = number.parse::<u32>().unwrap();
            if i == 0 {
                destination = number;
            } else if i == 1 {
                source = number;
            } else {
                range = number - 1;
            }
        }
        let map = Map {
            destination: MapRange {
                start: destination,
                end: destination + range,
            },
            source: MapRange {
                start: source,
                end: source + range,
            },
            range: range + 1,
        };
        return map;
    }
}

#[derive(Debug)]
pub struct Maps {
    pub soils: Vec<Map>,
    pub fertilizers: Vec<Map>,
    pub waters: Vec<Map>,
    pub lights: Vec<Map>,
    pub temperatures: Vec<Map>,
    pub humidities: Vec<Map>,
    pub locations: Vec<Map>,
}

impl Maps {
    pub fn new(input: &str) -> Maps {
        let inputs = inputs::Inputs::new(input);
        let mut soils = vec![];
        let mut fertilizers = vec![];
        let mut waters = vec![];
        let mut lights = vec![];
        let mut temperatures = vec![];
        let mut humidities = vec![];
        let mut locations = vec![];

        for line in inputs.soil.lines() {
            let map = Map::new(line);
            soils.push(map);
        }

        for line in inputs.fertilizer.lines() {
            let map = Map::new(line);
            fertilizers.push(map);
        }

        for line in inputs.water.lines() {
            let map = Map::new(line);
            waters.push(map);
        }

        for line in inputs.light.lines() {
            let map = Map::new(line);
            lights.push(map);
        }

        for line in inputs.temperature.lines() {
            let map = Map::new(line);
            temperatures.push(map);
        }

        for line in inputs.humidity.lines() {
            let map = Map::new(line);
            humidities.push(map);
        }

        for line in inputs.location.lines() {
            let map = Map::new(line);
            locations.push(map);
        }

        locations.sort_by(|a, b| a.destination.start.cmp(&b.destination.start));

        return Maps {
            soils,
            fertilizers,
            waters,
            lights,
            temperatures,
            humidities,
            locations,
        };
    }

    pub fn nest_lower_bound(&self, start: u32) -> u32 {
        let mut nest = None;
        for map in &self.locations {
            if start >= map.destination.start {
                continue;
            }
            nest = Some(map.destination.start);
            break;
        }
        if nest.is_some() {
            return nest.unwrap();
        }

        for i in (0..self.locations.len()).rev() {
            let map = &self.locations[i];
            if start >= map.destination.end {
                continue;
            }
            nest = Some(map.destination.start);
            break;
        }

        if nest.is_some() {
            return nest.unwrap();
        }

        // No match found,arbitrary nest
        return start + 10_000;
    }

    pub fn location_to_seed(&self, location: u32) -> u32 {
        let humidity = Maps::destination_to_source(location, &self.locations);
        let temperature = Maps::destination_to_source(humidity, &self.humidities);
        let light = Maps::destination_to_source(temperature, &self.temperatures);
        let water = Maps::destination_to_source(light, &self.lights);
        let fertilizer = Maps::destination_to_source(water, &self.waters);
        let soil = Maps::destination_to_source(fertilizer, &self.fertilizers);
        let seed = Maps::destination_to_source(soil, &self.soils);
        return seed;
    }

    fn destination_to_source(destination: u32, sources: &Vec<Map>) -> u32 {
        let mut source = None;
        for map in sources {
            if destination < map.destination.start {
                continue;
            }
            if destination > map.destination.end {
                continue;
            }

            let offset = destination - map.destination.start;
            source = Some(map.source.start + offset);
            break;
        }

        // No match found
        if source.is_none() {
            source = Some(destination);
        }
        return source.unwrap();
    }
}

impl Display for Maps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "seed-to-soil map:")?;
        for soil in &self.soils {
            write!(f, "{} ", soil.destination.start)?;
            write!(f, "{} ", soil.source.start)?;
            writeln!(f, "{}", soil.range)?;
        }
        writeln!(f, "")?;
        writeln!(f, "soil-to-fertilizer map:")?;

        for fertilizer in &self.fertilizers {
            write!(f, "{} ", fertilizer.destination.start)?;
            write!(f, "{} ", fertilizer.source.start)?;
            writeln!(f, "{}", fertilizer.range)?;
        }
        writeln!(f, "")?;
        writeln!(f, "fertilizer-to-water map:")?;

        for water in &self.waters {
            write!(f, "{} ", water.destination.start)?;
            write!(f, "{} ", water.source.start)?;
            writeln!(f, "{}", water.range)?;
        }
        writeln!(f, "")?;
        writeln!(f, "water-to-light map:")?;

        for light in &self.lights {
            write!(f, "{} ", light.destination.start)?;
            write!(f, "{} ", light.source.start)?;
            writeln!(f, "{}", light.range)?;
        }
        writeln!(f, "")?;
        writeln!(f, "light-to-temperature map:")?;

        for temperature in &self.temperatures {
            write!(f, "{} ", temperature.destination.start)?;
            write!(f, "{} ", temperature.source.start)?;
            writeln!(f, "{}", temperature.range)?;
        }
        writeln!(f, "")?;
        writeln!(f, "temperature-to-humidity map:")?;
        for humidity in &self.humidities {
            write!(f, "{} ", humidity.destination.start)?;
            write!(f, "{} ", humidity.source.start)?;
            writeln!(f, "{}", humidity.range)?;
        }
        writeln!(f, "")?;
        writeln!(f, "humidity-to-location map:")?;
        for location in &self.locations {
            write!(f, "{} ", location.destination.start)?;
            write!(f, "{} ", location.source.start)?;
            writeln!(f, "{}", location.range)?;
        }

        return Ok(());
    }
}
