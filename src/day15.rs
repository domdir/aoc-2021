use pathfinding::{directed::astar::astar, prelude::absdiff};

use crate::{Solution, SolutionResult};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn distance(&self, other: &Pos) -> usize {
        (absdiff(self.0, other.0) + absdiff(self.1, other.1)) as usize
    }

    fn successors(&self, costs: &Vec<Vec<usize>>) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = self;
        let mut next = Vec::new();
        if x > 0 {
            next.push(Pos(x - 1, y));
        }
        if y > 0 {
            next.push(Pos(x, y - 1));
        }
        if x < costs.len() - 1 {
            next.push(Pos(x + 1, y));
        }
        if y < costs[x].len() - 1 {
            next.push(Pos(x, y + 1));
        }
        next.into_iter()
            .map(|p| (p.clone(), costs[p.0][p.1]))
            .collect()
    }
}

pub(crate) struct Day15();

impl Solution for Day15 {
    fn solve(self, input_lines: impl Iterator<Item = String>) -> SolutionResult {
        let mut grid = input_lines
            .filter_map(|l| {
                l.chars()
                    .map(|c| c.to_string().parse::<usize>())
                    .collect::<Result<Vec<_>, _>>()
                    .ok()
            })
            .collect::<Vec<_>>();

        let og_len = grid.len();
        for r in 0..4 {
            for i in 0..grid.len() {
                for j in og_len * r..og_len * (r + 1) {
                    let mut v = grid[i][j] + 1;
                    if v == 10 {
                        v = 1;
                    }
                    grid[i].push(v);
                }
            }
        }

        for r in 0..4 {
            for i in og_len * r..og_len * (r + 1) {
                grid.push(
                    grid[i]
                        .iter()
                        .map(|f| {
                            let mut v = f + 1;
                            if v == 10 {
                                v = 1;
                            }
                            v
                        })
                        .collect::<Vec<_>>(),
                );
            }
        }

        let goal = Pos(grid.len() - 1, grid.len() - 1);
        let result = astar(
            &Pos(0, 0),
            |p| p.successors(&grid),
            |p| p.distance(&goal),
            |p| *p == goal,
        );

        if let Some(cost) = result {
            println!("Total cost: {:?}", cost.1);
        } else {
            println!("No path to goal found");
        }

        Ok(())
    }

    fn file_name(&self) -> &'static str {
        "day15.txt"
    }
}
