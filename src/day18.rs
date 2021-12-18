use std::ops::Add;

use crate::{Solution, SolutionResult};

#[derive(Debug)]
enum Snr {
    S(Box<Snr>, Box<Snr>, Option<*mut Snr>),
    Num(usize, Option<*mut Snr>),
}

impl Snr {
    fn patch_pointer(&mut self, pointer: *mut Snr) {
        match self {
            Snr::S(_, _, p) => {
                *p = Some(pointer);
            }
            Snr::Num(_, p) => {
                *p = Some(pointer);
            }
        }
    }
}

impl Add for Box<Snr> {
    type Output = Box<Snr>;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        let mut new = Box::new(Snr::Num(0, None));
        let pointer = &mut *new as *mut Snr;
        self.patch_pointer(pointer);
        rhs.patch_pointer(pointer);
        *&mut *new = Snr::S(self, rhs, None);
        reduce(&mut new);
        new
    }
}

fn explode_left_down(n: usize, s: &mut Snr) {
    match s {
        Snr::S(_, r, _) => explode_left_down(n, &mut *r),
        Snr::Num(v, _) => *v += n,
    }
}

fn explode_left_up(n: usize, p: &mut Snr, og: *const Snr) {
    match p {
        Snr::S(l, _, pp) => {
            if &**l as *const Snr == og {
                if pp.is_some() {
                    return explode_left_up(n, unsafe { &mut *pp.unwrap() }, p as *const _);
                } else {
                    return;
                }
            }
            explode_left_down(n, l);
        }
        _ => panic!("cannot happen"),
    }
}

fn explode_right_down(n: usize, s: &mut Snr) {
    match s {
        Snr::S(l, _, _) => explode_right_down(n, &mut *l),
        Snr::Num(v, _) => *v += n,
    }
}

fn explode_right_up(n: usize, p: &mut Snr, og: *const Snr) {
    match p {
        Snr::S(_, r, pp) => {
            if &**r as *const Snr == og {
                if pp.is_some() {
                    return explode_right_up(n, unsafe { &mut *pp.unwrap() }, p as *const _);
                } else {
                    return;
                }
            }

            explode_right_down(n, r);
        }
        _ => panic!("cannot happen"),
    }
}

fn explode(s: &mut Box<Snr>, count: usize) -> (bool, bool) {
    let og = &**s as *const _;
    let og_mut = &mut **s as *mut _;
    match &mut **s {
        Snr::S(l, r, p) => {
            if count == 0 {
                let l_num = match **l {
                    Snr::S(_, _, _) => todo!(),
                    Snr::Num(x, _) => x,
                };
                explode_left_up(l_num, unsafe { &mut *p.unwrap() }, og);

                let r_num = match **r {
                    Snr::S(_, _, _) => todo!(),
                    Snr::Num(x, _) => x,
                };
                explode_right_up(r_num, unsafe { &mut *p.unwrap() }, og);

                return (true, true);
            } else {
                let (found, direct) = explode(l, count - 1);
                if direct {
                    *l = Box::new(Snr::Num(0, Some(og_mut)));
                    return (true, false);
                }
                if found {
                    return (true, false);
                }

                let (found, direct) = explode(r, count - 1);
                if direct {
                    *r = Box::new(Snr::Num(0, Some(og_mut)));
                    return (true, false);
                }
                if found {
                    return (true, false);
                }
            }
        }
        _ => {}
    }
    (false, false)
}

fn split(s: &mut Box<Snr>) -> bool {
    let mut l = 0;
    let mut r = 0;
    let mut need_split = false;
    let mut p = Some(0 as *mut Snr);
    match &mut **s {
        Snr::S(l, r, _) => {
            let found = split(l);
            if found {
                return true;
            }
            return split(r);
        }
        Snr::Num(x, parent) => {
            if *x >= 10 {
                l = *x / 2;
                r = *x - l;
                p = parent.to_owned();
                need_split = true;
            }
        }
    }
    if need_split {
        let bp = &mut **s as *mut Snr;

        *&mut **s = Snr::S(
            Box::new(Snr::Num(l, Some(bp))),
            Box::new(Snr::Num(r, Some(bp))),
            p,
        );
    }
    need_split
}

fn reduce(s: &mut Box<Snr>) {
    while explode(s, 4).0 || split(s) {}
}

fn parse(s: &str) -> Box<Snr> {
    if s.chars().next().unwrap() == '[' {
        let inner = &s[1..s.len() - 1];
        let mut depth = 0;
        let mut split_point = None;
        for (i, c) in inner.chars().enumerate() {
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                ',' => {
                    if depth == 0 {
                        split_point = Some(i);
                    }
                }
                _ => {}
            }
        }
        let split_point = split_point.unwrap();
        let mut l = parse(&inner[0..split_point]);
        let mut r = parse(&inner[split_point + 1..]);
        let mut new = Box::new(Snr::Num(0, None));
        let pointer = &mut *new as *mut Snr;
        l.patch_pointer(pointer);
        r.patch_pointer(pointer);
        *&mut *new = Snr::S(l, r, None);
        new
    } else {
        Box::new(Snr::Num(
            s.chars().next().unwrap().to_string().parse().unwrap(),
            None,
        ))
    }
}

pub(crate) struct Day18();

impl Solution for Day18 {
    fn solve(self, input_lines: impl Iterator<Item = String>) -> SolutionResult {
        let numbers = input_lines.collect::<Vec<_>>();

        let mut it = numbers.iter();
        let mut start = parse(it.next().unwrap());
        for n in it {
            start = start + parse(n);
        }
        println!("Result:");
        print_num(&start);
        println!("Magnitude: {}", magnitude(&start));

        let mut m = 0;
        for i in 0..numbers.len() {
            for j in 0..numbers.len() {
                if i != j {
                    m = m.max(magnitude(
                        &(parse(&numbers[i].clone()) + parse(&numbers[j].clone())),
                    ));
                }
            }
        }
        println!("Max magnitude: {}", m);
        Ok(())
    }

    fn file_name(&self) -> &'static str {
        "day18.txt"
    }
}

fn magnitude(s: &Snr) -> usize {
    match s {
        Snr::S(l, r, _) => 3 * magnitude(l) + 2 * magnitude(r),
        Snr::Num(x, _) => *x,
    }
}

fn print_num(s: &Snr) {
    print_inner(s, true, None);
}

fn print_inner(s: &Snr, f: bool, p: Option<*mut Snr>) {
    let x = &*s as *const _ as *mut _;
    let og = Some(x);
    match s {
        Snr::S(l, r, pp) => {
            print!("[");
            print_inner(l, false, og);
            print!(",");
            print_inner(r, false, og);
            print!("]");
            if !f {
                assert_eq!(p, *pp);
            }
        }
        Snr::Num(x, pp) => {
            print!("{}", x);
            if !f {
                assert_eq!(p, *pp);
            }
        }
    }
    if f {
        println!("");
    }
}
