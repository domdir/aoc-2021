use crate::Solution;

pub(crate) const MEASUREMENT_LENGTH: usize = 12;

pub(crate) struct Day3<const L: usize>();

impl<const L: usize> Day3<L> {
    fn calc_base_stats<'a>(input_lines: impl Iterator<Item = &'a String>) -> (usize, usize) {
        let mut additional_ones = [0; L];
        Self::count_0s_and_1s(input_lines, &mut additional_ones);
        let mut gamma = 0;
        let mut epsilon = 0;
        for i in 0..L {
            gamma *= 2;
            epsilon *= 2;
            if additional_ones[i] > 0 {
                gamma += 1;
            } else {
                epsilon += 1;
            }
        }
        (gamma, epsilon)
    }

    fn calc_advanced_stats(mut input_lines: Vec<String>, keep_more: bool) -> usize {
        for i in 0..L {
            let mut additional_ones = [0; L];
            Self::count_0s_and_1s(input_lines.iter(), &mut additional_ones);

            let char_of_more = if additional_ones[i] >= 0 { '1' } else { '0' };
            input_lines = input_lines
                .into_iter()
                .filter(|v| {
                    let c = v.chars().nth(i).unwrap();
                    if keep_more {
                        c == char_of_more
                    } else {
                        c != char_of_more
                    }
                })
                .collect::<Vec<_>>();

            if input_lines.len() == 1 {
                return usize::from_str_radix(&input_lines[0], 2).unwrap();
            }
        }
        panic!("No solution found for advanced stats")
    }

    fn count_0s_and_1s<'a>(
        input_lines: impl Iterator<Item = &'a String>,
        additional_ones: &mut [isize; L],
    ) {
        for bin in input_lines {
            for (i, char) in bin.chars().enumerate() {
                match char {
                    '0' => additional_ones[i] -= 1,
                    '1' => additional_ones[i] += 1,
                    _ => panic!("Invalid char. Expected 0 or 1, got '{}'", char),
                }
            }
        }
    }
}

impl<const L: usize> Solution for Day3<L> {
    fn solve(self, input_lines: impl Iterator<Item = String>) {
        let input_lines = input_lines.collect::<Vec<_>>();
        let (gamma, epsilon) = Self::calc_base_stats(input_lines.iter());
        println!(
            "Gamma ({}) * Epsilon ({}) = power consumption ({})",
            gamma,
            epsilon,
            gamma * epsilon
        );

        let oxygen = Self::calc_advanced_stats(input_lines.clone(), true);
        let co2 = Self::calc_advanced_stats(input_lines, false);
        println!(
            "Oxygen generator rating ({}) * CO2 scrubber rating ({}) = life support rating ({})",
            oxygen,
            co2,
            oxygen * co2
        );
    }

    fn file_name(&self) -> &'static str {
        "day3"
    }
}
