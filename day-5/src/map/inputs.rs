pub struct Inputs<'a> {
    pub soil: &'a str,
    pub fertilizer: &'a str,
    pub water: &'a str,
    pub light: &'a str,
    pub temperature: &'a str,
    pub humidity: &'a str,
    pub location: &'a str,
}

impl<'a> Inputs<'a> {
    pub fn new(input: &'a str) -> Inputs<'a> {
        let (_, soil_input) = input.split_at(input.find("seed-to-soil map:").unwrap());
        let (soil_input, fertilizer_input) =
            soil_input.split_at(soil_input.find("soil-to-fertilizer map:").unwrap());
        let (fertilizer_input, water_input) =
            fertilizer_input.split_at(fertilizer_input.find("fertilizer-to-water map:").unwrap());
        let (water_input, light_input) =
            water_input.split_at(water_input.find("water-to-light map:").unwrap());
        let (light_input, temperature_input) =
            light_input.split_at(light_input.find("light-to-temperature map:").unwrap());
        let (temperature_input, humidity_input) = temperature_input.split_at(
            temperature_input
                .find("temperature-to-humidity map:")
                .unwrap(),
        );
        let (humidity_input, location_input) =
            humidity_input.split_at(humidity_input.find("humidity-to-location map:").unwrap());

        let (_, soil_input) = soil_input.split_at(soil_input.find('\n').unwrap());
        let (_, fertilizer_input) = fertilizer_input.split_at(fertilizer_input.find('\n').unwrap());
        let (_, water_input) = water_input.split_at(water_input.find('\n').unwrap());
        let (_, light_input) = light_input.split_at(light_input.find('\n').unwrap());
        let (_, temperature_input) =
            temperature_input.split_at(temperature_input.find('\n').unwrap());
        let (_, humidity_input) = humidity_input.split_at(humidity_input.find('\n').unwrap());
        let (_, location_input) = location_input.split_at(location_input.find('\n').unwrap());

        let soil_input = soil_input.trim();
        let fertilizer_input = fertilizer_input.trim();
        let water_input = water_input.trim();
        let light_input = light_input.trim();
        let temperature_input = temperature_input.trim();
        let humidity_input = humidity_input.trim();
        let location_input = location_input.trim();

        return Inputs {
            soil: soil_input,
            fertilizer: fertilizer_input,
            water: water_input,
            light: light_input,
            temperature: temperature_input,
            humidity: humidity_input,
            location: location_input,
        };
    }
}
