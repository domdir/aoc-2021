use crate::{Solution, SolutionResult};

pub(crate) struct Day17();

impl Solution for Day17 {
    fn solve(self, _input_lines: impl Iterator<Item = String>) -> SolutionResult {
        let target_x = 201..=230;
        let target_y = -99..=-65;
        let mut hit_count = 0;
        let mut highest_height = 0;

        for x in 0..1000 {
            for y in -100..1000 {
                let mut max_y = 0;
                let mut pos = (0, 0);
                let mut vel = (x, y);

                loop {
                    pos.0 += vel.0;
                    pos.1 += vel.1;

                    max_y = max_y.max(pos.1);
                    if vel.0 > 0 {
                        vel.0 -= 1;
                    }
                    vel.1 -= 1;

                    let hit = target_x.contains(&pos.0) && target_y.contains(&pos.1);
                    if hit {
                        hit_count += 1;
                        highest_height = highest_height.max(max_y);
                        break;
                    }
                    if pos.0 > target_x.clone().rev().next().unwrap() {
                        break;
                    }
                    if vel.0 == 0 && pos.0 < target_x.clone().next().unwrap() {
                        break;
                    }
                    if pos.1 < target_y.clone().next().unwrap() {
                        break;
                    }
                }
            }
        }

        println!("Max height: {}", highest_height);
        println!("Hit count: {}", hit_count);

        Ok(())
    }

    fn file_name(&self) -> &'static str {
        "day17.txt"
    }
}
