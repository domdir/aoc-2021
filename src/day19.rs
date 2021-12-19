use std::ops::Sub;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    str::FromStr,
};

use crate::{Solution, SolutionResult};

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Point { x, y, z }
    }

    fn get_rots(self) -> Vec<Point> {
        let angles = [
            0f32.to_radians(),
            90f32.to_radians(),
            180f32.to_radians(),
            270f32.to_radians(),
        ];
        let mut points = vec![];
        // general rotation matrix
        for alpha in angles {
            for beta in angles {
                for gamma in angles {
                    let new_x = alpha.cos() * beta.cos() * self.x as f32
                        + (alpha.cos() * beta.sin() * gamma.sin() - alpha.sin() * gamma.cos())
                            * self.y as f32
                        + (alpha.cos() * beta.sin() * gamma.cos() + alpha.sin() * gamma.sin())
                            * self.z as f32;
                    let new_y = alpha.sin() * beta.cos() * self.x as f32
                        + (alpha.sin() * beta.sin() * gamma.sin() + alpha.cos() * gamma.cos())
                            * self.y as f32
                        + (alpha.sin() * beta.sin() * gamma.cos() - alpha.cos() * gamma.sin())
                            * self.z as f32;
                    let new_z = -beta.sin() * self.x as f32
                        + beta.cos() * gamma.sin() * self.y as f32
                        + beta.cos() * gamma.cos() * self.z as f32;

                    let new_x_int = new_x.round() as isize;
                    let new_y_int = new_y.round() as isize;
                    let new_z_int = new_z.round() as isize;

                    points.push(Point::new(new_x_int, new_y_int, new_z_int));
                }
            }
        }
        points
    }

    fn dis(self, other: Self) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl FromStr for Point {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(",");
        Ok(Point {
            x: parts.next().unwrap().parse()?,
            y: parts.next().unwrap().parse()?,
            z: parts.next().unwrap().parse()?,
        })
    }
}

struct Scanner {
    beacons: HashSet<Point>,
    rel_pos: HashMap<Point, Point>,
    rot_beacons: HashMap<usize, HashSet<Point>>,
    rot_rel_pos: HashMap<usize, HashMap<Point, Point>>,
}

impl Scanner {
    fn new() -> Self {
        Scanner {
            beacons: HashSet::new(),
            rel_pos: HashMap::new(),
            rot_beacons: HashMap::new(),
            rot_rel_pos: HashMap::new(),
        }
    }

    fn set_rot(&mut self) {
        self.rot_beacons = HashMap::new();
        for beacon in self.beacons.clone() {
            for (i, p) in beacon.get_rots().into_iter().enumerate() {
                self.rot_beacons.entry(i).or_default().insert(p);
                self.rot_rel_pos = self
                    .rot_beacons
                    .iter()
                    .map(|(k, v)| (*k, rel_pos(v)))
                    .collect();
            }
        }
    }
}

fn rel_pos(pos: &HashSet<Point>) -> HashMap<Point, Point> {
    let mut h = HashMap::new();
    for i in pos.iter() {
        for j in pos.iter() {
            if i != j {
                h.insert(*i - *j, *i);
            }
        }
    }
    h
}

fn fits(scanner: &Scanner, known: &Scanner, req: i32) -> Option<(HashSet<Point>, Point, usize)> {
    for (i, s) in &scanner.rot_rel_pos {
        let mut hit_count = HashMap::new();
        for (rel_point, og_point) in s {
            if known.rel_pos.contains_key(rel_point) {
                let offset = known.rel_pos[rel_point] - *og_point;
                *hit_count.entry(offset).or_insert(0) += 1;
            }
        }
        let t = hit_count
            .iter()
            .filter(|(_, v)| **v >= req)
            .collect::<HashMap<_, _>>();

        let mut m = None;
        let mut val = None;
        for (v, x) in t.iter() {
            match m {
                Some(h) => {
                    if h < x {
                        m = Some(x);
                        val = Some(v);
                    }
                }
                None => {
                    m = Some(x);
                    val = Some(v);
                }
            }
        }

        if let Some(offset) = val {
            return Some((scanner.rot_beacons[i].clone(), **offset, *i));
        }
    }
    None
}

pub(crate) struct Day19();

impl Solution for Day19 {
    fn solve(self, mut input_lines: impl Iterator<Item = String>) -> SolutionResult {
        let mut scanners = vec![];
        let mut s = Scanner::new();
        input_lines.next();
        while let Some(line) = input_lines.next() {
            if line.trim().is_empty() {
                continue;
            }
            if line.starts_with("---") {
                s.rel_pos = rel_pos(&s.beacons);
                s.set_rot();
                scanners.push(s);
                s = Scanner::new();
                continue;
            }
            s.beacons.insert(line.parse()?);
        }
        s.rel_pos = rel_pos(&s.beacons);
        s.set_rot();
        scanners.push(s);
        println!("ok done");

        let mut beacons = scanners[0].beacons.clone();
        let mut known_scanners = vec![(scanners.remove(0), Point { x: 0, y: 0, z: 0 })];
        let mut scanner_pos = vec![];
        let mut req = 12;
        'run: while !scanners.is_empty() {
            let l = scanners.len();
            for i in 0..l {
                println!("go: {}/{}", known_scanners.len(), l);
                for (known_scanner, _offset) in &known_scanners {
                    if let Some((bps, inner_offset, _)) = fits(&scanners[i], known_scanner, req) {
                        scanner_pos.push(inner_offset);

                        let mut count = 0;
                        let bps: HashSet<Point> = bps
                            .into_iter()
                            .map(|p| Point {
                                x: p.x + inner_offset.x,
                                y: p.y + inner_offset.y,
                                z: p.z + inner_offset.z,
                            })
                            .collect();
                        let mut t_beacons = beacons.clone();
                        for beacon in bps.clone() {
                            if !t_beacons.insert(beacon) {
                                count += 1;
                            }
                        }
                        println!("count: {}", count);
                        if count < req {
                            println!("Fake hit");
                            continue;
                        }
                        beacons = t_beacons;
                        let mut m = scanners.remove(i);
                        m.beacons = bps;
                        m.rel_pos = rel_pos(&m.beacons);
                        m.set_rot();
                        known_scanners.push((m, Point::new(0, 0, 0)));
                        continue 'run;
                    }
                }
            }
            println!("Count: {}", beacons.len());
            req -= 1;
            if req == 0 {
                break;
            }
            println!("lowering: {}", req);
            println!("weird");
        }

        println!("Final req: {}", req);
        println!("Count: {}", beacons.len());

        let mut max = 0;
        for i in 0..scanner_pos.len() {
            for j in 0..scanner_pos.len() {
                if i != j {
                    max = max.max(scanner_pos[i].dis(scanner_pos[j]));
                }
            }
        }
        println!("Max: {}", max);

        Ok(())
    }

    fn file_name(&self) -> &'static str {
        "day19.txt"
    }
}
