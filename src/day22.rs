use std::{
    collections::HashMap,
    ops::{Range, RangeInclusive},
};

use crate::{Solution, SolutionResult};

pub(crate) struct Day22();

impl Solution for Day22 {
    fn solve(self, mut input_lines: impl Iterator<Item = String>) -> SolutionResult {
        let ins = input_lines
            .map(|line| {
                let (on, coords) = line.split_once(" ").unwrap();
                let c = coords
                    .split(",")
                    .map(|l| &l[2..])
                    .map(|c| c.split_once("..").unwrap())
                    .map(|(a, b)| (a.to_string(), b.to_string()))
                    .map(to_range)
                    .collect::<Vec<_>>();
                let o = on == "on";
                (o, c)
            })
            .collect::<Vec<_>>();

        let mut map = HashMap::new();
        let mut ranges: Vec<((isize, isize), (isize, isize), (isize, isize))> = vec![];
        for (on, ins) in ins {
            println!("{:?}", ins);
            let mut it = ins.into_iter();
            let x_range = it.next().unwrap();
            let y_range = it.next().unwrap();
            let z_range = it.next().unwrap();
            assert!(it.next().is_none());
            // for x in x_range.clone() {
            //     for y in y_range.clone() {
            //         for z in z_range.clone() {
            //             *map.entry((x, y, z)).or_default() = on;
            //         }
            //     }
            // }
            let mut new_ranges = vec![];
            for (cur_x_range, cur_y_range, cur_z_range) in ranges {
                if ((x_range.0 >= cur_x_range.0 && x_range.0 <= cur_x_range.1)
                    && (x_range.1 >= cur_x_range.0 && x_range.1 <= cur_x_range.1))
                    && ((y_range.0 >= cur_y_range.0 && y_range.0 <= cur_y_range.1)
                        && (y_range.1 >= cur_y_range.0 && y_range.1 <= cur_y_range.1))
                    && ((z_range.0 >= cur_z_range.0 && z_range.0 <= cur_z_range.1)
                        && (z_range.1 >= cur_z_range.0 && z_range.1 <= cur_z_range.1))
                {
                    if cur_x_range.0 != x_range.0
                        && cur_y_range.0 != y_range.0
                        && cur_z_range.0 != z_range.0
                    {
                        new_ranges.push((
                            (cur_x_range.0, x_range.0 - 1),
                            (cur_y_range.0, y_range.0 - 1),
                            (cur_z_range.0, z_range.0 - 1),
                        ));
                    }
                    if cur_x_range.1 != x_range.1
                        && cur_y_range.1 != y_range.1
                        && cur_z_range.1 != z_range.1
                    {
                        new_ranges.push((
                            (x_range.1 + 1, cur_x_range.1),
                            (y_range.1 + 1, cur_y_range.1),
                            (z_range.1 + 1, cur_z_range.1),
                        ));
                    }
                } else if ((x_range.0 >= cur_x_range.0 && x_range.0 <= cur_x_range.1)
                    || (x_range.1 >= cur_x_range.0 && x_range.1 <= cur_x_range.1))
                    && ((y_range.0 >= cur_y_range.0 && y_range.0 <= cur_y_range.1)
                        || (y_range.1 >= cur_y_range.0 && y_range.1 <= cur_y_range.1))
                    && ((z_range.0 >= cur_z_range.0 && z_range.0 <= cur_z_range.1)
                        || (z_range.1 >= cur_z_range.0 && z_range.1 <= cur_z_range.1))
                {
                } else {
                    new_ranges.push((cur_x_range, cur_y_range, cur_z_range));
                }
            }
            if on {
                new_ranges.push((x_range, y_range, z_range));
            }
            ranges = new_ranges;
        }

        let count = ranges
            .iter()
            .filter(|(x, y, z)| x.0 >= x.1 && y.0 >= y.1 && z.0 >= z.1)
            .map(|(x, y, z)| (x.1 - x.0 + 1) * (y.1 - y.0 + 1) * (z.1 - z.0 + 1))
            .sum::<isize>();
        println!("c:{}", count);

        Ok(())
    }

    fn file_name(&self) -> &'static str {
        "day22.txt"
    }
}

fn to_range((a, b): (String, String)) -> (isize, isize) {
    let a = a.parse::<isize>().unwrap();
    let b = b.parse::<isize>().unwrap();
    let start = a.min(b);
    let end = a.max(b);
    // if end < -50 || start > 50 {
    //     0..=-1
    // } else {
    //     start.max(-50)..=end.min(50)
    // }
    (start, end)
}
