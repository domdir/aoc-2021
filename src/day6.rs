use crate::Solution;

pub(crate) struct Day6();

impl Solution for Day6 {
    fn solve(self, mut input_lines: impl Iterator<Item = String>) {
        let mut days = [0usize; 9];
        input_lines
            .next()
            .unwrap()
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .for_each(|f| days[f] += 1);

        for day in 0..256 {
            if day == 80 {
                println!("Fish after day 80: {}", days.iter().sum::<usize>());
            }
            days.rotate_left(1);
            days[6] += days[8];
        }
        println!("Fish after day 256: {}", days.iter().sum::<usize>());
    }

    fn file_name(&self) -> &'static str {
        "day6"
    }
}
