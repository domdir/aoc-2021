use std::collections::HashMap;

use crate::{Solution, SolutionResult};

pub(crate) struct Day13();

impl Solution for Day13 {
    fn solve(self, input_lines: impl Iterator<Item = String>) -> SolutionResult {
        let mut dots: HashMap<usize, HashMap<usize, usize>> = HashMap::new();
        let map = input_lines
            .filter_map(|f| {
                f.split_once(',')
                    .map(|(f, t)| Some((f.parse::<usize>().ok()?, t.parse::<usize>().ok()?)))?
            })
            .collect::<Vec<(usize, usize)>>();
        for (x, y) in &map {
            let v = dots.entry(*x).or_default().entry(*y).or_default();
            *v = 1;
        }

        for (fold_dir, fold) in vec![
            ("x", 655),
            ("y", 447),
            ("x", 327),
            ("y", 223),
            ("x", 163),
            ("y", 111),
            ("x", 81),
            ("y", 55),
            ("x", 40),
            ("y", 27),
            ("y", 13),
            ("y", 6),
        ] {
            dots = fold_paper(dots, fold_dir, fold);
        }

        for y in 0..6 {
            for x in 0..39 {
                match dots.get(&x) {
                    Some(row) => match row.get(&y) {
                        Some(v) => {
                            if v == &1 {
                                print!("#")
                            } else {
                                print!(".")
                            }
                        }
                        None => print!("."),
                    },
                    None => print!("."),
                }
            }
            println!()
        }

        Ok(())
    }

    fn file_name(&self) -> &'static str {
        "day13"
    }
}

fn fold_paper(
    dots: HashMap<usize, HashMap<usize, usize>>,
    fold_dir: &str,
    fold: usize,
) -> HashMap<usize, HashMap<usize, usize>> {
    let mut t = dots.clone();
    for (x, row) in &dots {
        for (y, v) in row {
            if fold_dir == "y" {
                if *v == 1 && y >= &fold {
                    let ty = fold - (y - fold);
                    if ty > fold {
                        panic!("not allowed")
                    }
                    let nv = t.entry(*x).or_default().entry(ty).or_default();
                    *nv = 1;
                    let nv = t.entry(*x).or_default().entry(*y).or_default();
                    *nv = 0;
                }
            } else {
                if *v == 1 && x >= &fold {
                    let tx = fold - (x - fold);
                    if tx > fold {
                        panic!("not allowed")
                    }
                    let nv = t.entry(tx).or_default().entry(*y).or_default();
                    *nv = 1;
                    let nv = t.entry(*x).or_default().entry(*y).or_default();
                    *nv = 0;
                }
            }
        }
    }
    t
}
