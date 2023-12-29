#[derive(Clone, Copy)]
struct SeedRange {
    seed_no: u64,
    range: u64,
}

impl SeedRange {
    fn new(seed_no: u64, range: u64) -> Self {
        Self { seed_no, range }
    }
}

struct Conversion {
    src: u64,
    dst: u64,
    range: u64,
}

impl Conversion {
    /// Expect a str "x x x"
    fn new(s: &str) -> Self {
        let mut v: Vec<u64> = s.split(' ').map(|n| n.parse::<u64>().unwrap()).collect();
        let range: u64 = v.pop().unwrap();
        let src: u64 = v.pop().unwrap();
        let dst: u64 = v.pop().unwrap();

        Self { src, dst, range }
    }

    fn get_conversion(&self, num: u64) -> u64 {
        if num >= self.src && num < self.src + self.range {
            return self.dst + (num - self.src);
        }

        num
    }

    /// Returns a tuple where the first element indicates if a match was found, and the second a vector containing 1 to 3 seed ranges
    fn get_conversion_range(&self, seed_range: &SeedRange) -> (bool, Vec<SeedRange>) {
        // Fist, check if the given range is outside of the map
        if seed_range.seed_no >= self.src + self.range
            || seed_range.seed_no + seed_range.range < self.src
        {
            return (false, vec![]);
        }

        // determine the intersection between both ranges.
        let seed_start = seed_range.seed_no;
        let seed_end = seed_range.seed_no + seed_range.range;
        let map_start = self.src;
        let map_end = self.src + self.range;
        let mut range_bounds = vec![seed_start, seed_end, map_start, map_end];

        range_bounds.sort();

        // seed range is inside of map range -> gets 1 range returned
        if *range_bounds.first().unwrap() == map_start && *range_bounds.last().unwrap() == map_end {
            return (
                true,
                vec![SeedRange::new(
                    self.get_conversion(seed_start),
                    seed_range.range,
                )],
            );
        }

        // map range is inside of seed range -> gets 3 ranges returned
        if *range_bounds.first().unwrap() == seed_start && *range_bounds.last().unwrap() == seed_end
        {
            let r1 = SeedRange::new(seed_start, map_start - seed_start);
            let r2 = SeedRange::new(self.get_conversion(map_start), map_end - map_start);
            let r3 = SeedRange::new(map_end, seed_end - map_end);

            return (true, vec![r1, r2, r3]);
        }
        // partial overlap -> gets 2 ranges returned
        // seed start is outside of map
        if *range_bounds.first().unwrap() == seed_start {
            let r1 = SeedRange::new(seed_start, map_start - seed_start);
            let r2 = SeedRange::new(self.get_conversion(map_start), seed_end - map_start);
            return (true, vec![r1, r2]);
        // seed start is inside of map
        } else {
            let r1 = SeedRange::new(self.get_conversion(seed_start), map_end - seed_start);
            let r2 = SeedRange::new(map_end, seed_end - map_end);
            return (true, vec![r1, r2]);
        }
    }
}

struct ConversionMap {
    // title: String,
    conversions: Vec<Conversion>,
}

impl ConversionMap {
    /// Expect a map including title. ie. everything between two blank lines
    fn new(s: &str) -> Self {
        let (_, map) = s.split_once('\n').unwrap();
        // let title = title.to_string();
        let conversions = map.split('\n').map(|l| Conversion::new(l)).collect();

        Self { conversions }
    }

    fn get_conversion(&self, num: u64) -> u64 {
        let mut conv = num;
        for m in &self.conversions {
            conv = m.get_conversion(num);
            if conv != num {
                return conv;
            }
        }

        conv
    }

    /// Consumes a SeedRange and returns a vec of SeedRange.
    /// If the range wasn't matched, the same range is returned, otherwise the vec will contain all new ranges of seeds
    fn get_conversion_range(&self, seed_range: SeedRange) -> Vec<SeedRange> {
        for m in &self.conversions {
            let ranges = m.get_conversion_range(&seed_range);
            if ranges.0 {
                return ranges.1;
            }
        }
        vec![seed_range]
    }
}

pub fn solve_q5_p1(s: &str) -> u64 {
    let (seeds, maps) = s.split_once("\n\n").unwrap();
    let seed_list: Vec<u64> = seeds
        .split_once(' ')
        .unwrap()
        .1
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let conv_maps: Vec<ConversionMap> = maps.split("\n\n").map(|m| ConversionMap::new(m)).collect();

    let mut locations: Vec<u64> = seed_list
        .iter()
        .map(|s| convert_seed(*s, &conv_maps))
        .collect();

    locations.sort();

    *locations.first().unwrap()
}

fn convert_seed(seed: u64, conv_maps: &Vec<ConversionMap>) -> u64 {
    let mut result = seed;
    for m in conv_maps {
        result = m.get_conversion(result);
    }

    result
}

pub fn solve_q5_p2(s: &str) -> u64 {
    let (seeds, maps) = s.split_once("\n\n").unwrap();
    let mut seed_list: Vec<u64> = seeds
        .split_once(' ')
        .unwrap()
        .1
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let mut ranges = vec![];
    // create seed ranges
    while seed_list.len() > 0 {
        let range = seed_list.pop().unwrap();
        let seed_no = seed_list.pop().unwrap();
        ranges.push(SeedRange::new(seed_no, range));
    }
    // create conversion maps
    let conv_maps: Vec<ConversionMap> = maps.split("\n\n").map(|m| ConversionMap::new(m)).collect();
    // run all seed ranges through conversion maps
    let mut locations: Vec<SeedRange> = ranges
        .iter()
        .flat_map(|r| convert_ranges(*r, &conv_maps))
        .collect();
    // sort seed ranges
    locations.sort_by(|a, b| a.seed_no.partial_cmp(&b.seed_no).unwrap());
    locations.first().unwrap().seed_no
}

fn convert_ranges(range: SeedRange, conv_maps: &Vec<ConversionMap>) -> Vec<SeedRange> {
    let mut result: Vec<SeedRange> = vec![range];
    for m in conv_maps {
        result = result
            .iter()
            .flat_map(|&r| m.get_conversion_range(r))
            .collect();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_q5p1() {
        let d5p1_test = include_str!("./input1_test.txt");
        assert_eq!(solve_q5_p1(d5p1_test), 35);
    }

    #[test]
    fn test_solve_q5p2() {
        let d5p2_test = include_str!("./input1_test.txt");
        assert_eq!(solve_q5_p2(d5p2_test), 46);
    }
}
