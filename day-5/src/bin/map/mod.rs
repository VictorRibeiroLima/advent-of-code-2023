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
    destination: MapRange,
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
    fertilizers: Vec<Map>,
    waters: Vec<Map>,
    lights: Vec<Map>,
    temperatures: Vec<Map>,
    humidities: Vec<Map>,
    locations: Vec<Map>,
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

        soils.sort_by(|a, b| a.source.start.cmp(&b.source.start));

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

    /*
        Seed: 459278395 Soil: 0
    Seed: 459278395 Fertilizer: 148429321
    Seed: 459278395 Water: 446822259
    Seed: 459278395 Light: 1028118976
    Seed: 459278395 Temperature: 291856735
    Seed: 459278395 Humidity: 597432307
    Seed: 459278395 Location: 260579843
    Seed: 459278395 Soil: 0
    Seed: 459278395 Fertilizer: 148429321
    Seed: 459278395 Water: 446822259
    Seed: 459278395 Light: 1028118976
    Seed: 459278395 Temperature: 291856735
    Seed: 459278395 Humidity: 597432307
    Seed: 459278395 Location: 260579843 */
    pub fn seed_to_location(&self, seed: u32) -> u32 {
        let soil = Maps::source_to_destination(seed, &self.soils);

        let fertilizer = Maps::source_to_destination(soil, &self.fertilizers);

        let water = Maps::source_to_destination(fertilizer, &self.waters);

        let light = Maps::source_to_destination(water, &self.lights);

        let temperature = Maps::source_to_destination(light, &self.temperatures);

        let humidity = Maps::source_to_destination(temperature, &self.humidities);

        let location = Maps::source_to_destination(humidity, &self.locations);

        return location;
    }

    fn source_to_destination(source: u32, destinations: &Vec<Map>) -> u32 {
        let mut destination = None;
        for map in destinations {
            if source < map.source.start {
                continue;
            }
            if source > map.source.end {
                continue;
            }

            let offset = source - map.source.start;
            destination = Some(map.destination.start + offset);
            break;
        }

        // No match found
        if destination.is_none() {
            destination = Some(source);
        }
        return destination.unwrap();
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
