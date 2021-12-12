use std::collections::{HashMap, HashSet};

use crate::{Solution, SolutionResult};

pub(crate) struct Day12();

impl Solution for Day12 {
    fn solve(self, input_lines: impl Iterator<Item = String>) -> SolutionResult {
        let mut edges = HashMap::<&str, Vec<&str>>::new();
        let mut big_caves = HashSet::<&str>::new();
        let graph = input_lines
            .map(|f| f.split_once('-').map(|(f, t)| (f.to_owned(), t.to_owned())))
            .collect::<Option<Vec<_>>>()
            .ok_or("Parsing failed")?;
        for (from, to) in &graph {
            edges.entry(from).or_default().push(to);
            edges.entry(to).or_default().push(from);

            if from.to_uppercase() == *from {
                big_caves.insert(from);
            }
            if to.to_uppercase() == *to {
                big_caves.insert(to);
            }
        }

        let paths = get_all_paths("start", &edges, &big_caves, &HashSet::new(), true);
        println!("Paths without revisit: {}", paths);
        let paths = get_all_paths("start", &edges, &big_caves, &HashSet::new(), false);
        println!("Paths with revisit: {}", paths);

        Ok(())
    }

    fn file_name(&self) -> &'static str {
        "day12"
    }
}

fn get_all_paths(
    cur: &str,
    edges: &HashMap<&str, Vec<&str>>,
    big_caves: &HashSet<&str>,
    path: &HashSet<String>,
    small_cave_revisited: bool,
) -> usize {
    let mut paths = 0;
    for &cave in &edges[cur] {
        if cave == "start" {
            continue;
        }
        let mut small_cave_revisited = small_cave_revisited;
        if !big_caves.contains(cave) && path.contains(cave) {
            if small_cave_revisited {
                continue;
            } else {
                small_cave_revisited = true;
            }
        }

        let mut path = path.clone();
        path.insert(cur.to_owned());

        if cave == "end" {
            paths += 1;
        } else {
            paths += get_all_paths(&cave, edges, big_caves, &path, small_cave_revisited);
        }
    }
    paths
}
