use crate::{Solution, SolutionResult};

pub(crate) struct Day5();

impl Solution for Day5 {
    fn solve(self, input_lines: impl Iterator<Item = String>) -> SolutionResult {
        let mut map_without_diagonals = vec![vec![0; 1_000]; 1_000];
        let mut map_with_diagonals = vec![vec![0; 1_000]; 1_000];
        for ((from_x, from_y), (to_x, to_y)) in input_lines
            .map(|line| {
                let mut parts = line.split(" -> ").filter_map(|parts| {
                    let mut p = parts
                        .split(",")
                        .map(|parts| parts.parse::<usize>())
                        .collect::<Result<Vec<_>, _>>()
                        .ok()?
                        .into_iter();
                    Some((p.next()?, p.next()?))
                });
                Some((parts.next()?, parts.next()?))
            })
            .collect::<Option<Vec<_>>>()
            .ok_or("Not enough parts")?
        {
            if from_x == to_x {
                for i in if from_y < to_y {
                    from_y..=to_y
                } else {
                    to_y..=from_y
                } {
                    map_without_diagonals[from_x][i] += 1;
                    map_with_diagonals[from_x][i] += 1;
                }
            } else if from_y == to_y {
                for i in if from_x < to_x {
                    from_x..=to_x
                } else {
                    to_x..=from_x
                } {
                    map_without_diagonals[i][from_y] += 1;
                    map_with_diagonals[i][from_y] += 1;
                }
            } else {
                let start_x = usize::min(from_x, to_x);
                let end_x = usize::max(from_x, to_x);
                for i in 0..=end_x - start_x {
                    let x = if from_x > to_x {
                        from_x - i
                    } else {
                        from_x + i
                    };
                    let y = if from_y > to_y {
                        from_y - i
                    } else {
                        from_y + i
                    };
                    map_with_diagonals[x][y] += 1;
                }
            }
        }
        let count_without_diagonals = count_overlaps(&map_without_diagonals);
        println!("Count without diagonals: {}", count_without_diagonals);
        let count_with_diagonals = count_overlaps(&map_with_diagonals);
        println!("Count with diagonals: {}", count_with_diagonals);
        Ok(())
    }

    fn file_name(&self) -> &'static str {
        "day5"
    }
}

fn count_overlaps(map: &Vec<Vec<usize>>) -> usize {
    map.iter().flatten().filter(|&&v| v > 1).count()
}
