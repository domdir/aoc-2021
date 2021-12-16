use crate::{Solution, SolutionResult};

pub(crate) struct Day16();

impl Solution for Day16 {
    fn solve(self, mut input_lines: impl Iterator<Item = String>) -> SolutionResult {
        let mut binary = vec![];
        for c in input_lines.next().unwrap().chars() {
            match c {
                '0' => {
                    binary.push('0');
                    binary.push('0');
                    binary.push('0');
                    binary.push('0')
                }
                '1' => {
                    binary.push('0');
                    binary.push('0');
                    binary.push('0');
                    binary.push('1')
                }
                '2' => {
                    binary.push('0');
                    binary.push('0');
                    binary.push('1');
                    binary.push('0')
                }
                '3' => {
                    binary.push('0');
                    binary.push('0');
                    binary.push('1');
                    binary.push('1')
                }
                '4' => {
                    binary.push('0');
                    binary.push('1');
                    binary.push('0');
                    binary.push('0')
                }
                '5' => {
                    binary.push('0');
                    binary.push('1');
                    binary.push('0');
                    binary.push('1')
                }
                '6' => {
                    binary.push('0');
                    binary.push('1');
                    binary.push('1');
                    binary.push('0')
                }
                '7' => {
                    binary.push('0');
                    binary.push('1');
                    binary.push('1');
                    binary.push('1')
                }
                '8' => {
                    binary.push('1');
                    binary.push('0');
                    binary.push('0');
                    binary.push('0')
                }
                '9' => {
                    binary.push('1');
                    binary.push('0');
                    binary.push('0');
                    binary.push('1')
                }
                'A' => {
                    binary.push('1');
                    binary.push('0');
                    binary.push('1');
                    binary.push('0')
                }
                'B' => {
                    binary.push('1');
                    binary.push('0');
                    binary.push('1');
                    binary.push('1')
                }
                'C' => {
                    binary.push('1');
                    binary.push('1');
                    binary.push('0');
                    binary.push('0')
                }
                'D' => {
                    binary.push('1');
                    binary.push('1');
                    binary.push('0');
                    binary.push('1')
                }
                'E' => {
                    binary.push('1');
                    binary.push('1');
                    binary.push('1');
                    binary.push('0')
                }
                'F' => {
                    binary.push('1');
                    binary.push('1');
                    binary.push('1');
                    binary.push('1')
                }
                _ => panic!("Invalid"),
            }
        }

        let (_, nr) = all_packets(&mut binary.into_iter());
        println!("Result: {}", nr);

        Ok(())
    }

    fn file_name(&self) -> &'static str {
        "day16.txt"
    }
}

fn all_packets(binary: &mut impl Iterator<Item = char>) -> ((usize, usize), usize) {
    let version = read_3(binary);
    let type_nr = read_3(binary);
    if type_nr == 4 {
        let (len_read, nr) = literal(binary);
        ((version, len_read), nr)
    } else {
        let ((versions, len_read), nr) = operator(binary, type_nr);
        ((versions + version, len_read), nr)
    }
}

fn read_3(binary: &mut impl Iterator<Item = char>) -> usize {
    let v = binary.take(3).collect::<String>();
    usize::from_str_radix(&v, 2).unwrap()
}

fn literal(binary: &mut impl Iterator<Item = char>) -> (usize, usize) {
    let mut len_read = 6;
    let mut total = String::new();
    loop {
        len_read += 5;
        let last = binary.next().unwrap() == '0';
        total.push_str(&binary.take(4).collect::<String>());

        if last {
            break;
        }
    }
    let nr = usize::from_str_radix(&total, 2).unwrap();
    (len_read, nr)
}

fn operator(binary: &mut impl Iterator<Item = char>, op: usize) -> ((usize, usize), usize) {
    let length_type = binary.next().unwrap();
    let mut total_version = 0;
    let mut len_read = 0;
    let mut numbers = Vec::new();
    if length_type == '0' {
        let length = usize::from_str_radix(&binary.take(15).collect::<String>(), 2).unwrap();

        while len_read < length {
            let ((v, l), nr) = all_packets(binary);
            total_version += v;
            len_read += l;
            numbers.push(nr);
        }
        len_read += 15;
    } else {
        let length = usize::from_str_radix(&binary.take(11).collect::<String>(), 2).unwrap();
        len_read += 11;

        for _ in 0..length {
            let ((v, l), nr) = all_packets(binary);
            total_version += v;
            len_read += l;
            numbers.push(nr);
        }
    }
    let final_nr: usize = match op {
        0 => numbers.iter().sum(),
        1 => numbers.iter().product(),
        2 => *numbers.iter().min().unwrap(),
        3 => *numbers.iter().max().unwrap(),
        5 => {
            if numbers[0] > numbers[1] {
                1
            } else {
                0
            }
        }
        6 => {
            if numbers[0] < numbers[1] {
                1
            } else {
                0
            }
        }
        7 => {
            if numbers[0] == numbers[1] {
                1
            } else {
                0
            }
        }
        _ => panic!(),
    };

    len_read += 7;
    ((total_version, len_read), final_nr)
}
