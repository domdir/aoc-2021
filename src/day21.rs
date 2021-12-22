use std::collections::HashMap;

use crate::{Solution, SolutionResult};

pub(crate) struct Day21();

impl Solution for Day21 {
    fn solve(self, mut _input_lines: impl Iterator<Item = String>) -> SolutionResult {
        let mut pos_1 = 8;
        // let pos_1 =
        // let mut score_1 = 0;
        let mut pos_2 = 5;
        // let pos_2 = 8;
        // let mut score_2 = 0;
        // let mut turn = true;
        // let mut die = (1..=100).cycle();
        let mut chance = HashMap::new();
        for i in 1..=3 {
            for j in 1..=3 {
                for k in 1..=3 {
                    *chance.entry(i + j + k).or_default() += 1;
                }
            }
        }
        play(&chance, pos_1, pos_2);
        // let l_score = if score_1 > score_2 { score_2 } else { score_1 };
        Ok(())
    }

    fn file_name(&self) -> &'static str {
        "day21.txt"
    }
}

fn play(chance: &HashMap<usize, usize>, pos_1: usize, pos_2: usize) {
    let mut turn = true;
    let mut wins_1 = 0;
    let mut wins_2 = 0;
    let mut scores = HashMap::new();
    scores.insert(((0, pos_1), (0, pos_2)), 1);
    loop {
        println!("turn");
        let mut turn_scores: HashMap<((usize, usize), (usize, usize)), usize> = HashMap::new();
        for (roll, count) in chance {
            if turn {
                for (((score, pos), (score_2, pos_2)), score_count) in &scores {
                    let mut pos_1 = *pos;
                    let mut score_1 = *score;
                    pos_1 += *roll;
                    while pos_1 > 10 {
                        pos_1 -= 10;
                    }
                    score_1 += pos_1;
                    if score_1 >= 21 {
                        wins_1 += score_count * count;
                    } else {
                        *turn_scores
                            .entry(((score_1, pos_1), (*score_2, *pos_2)))
                            .or_default() += score_count * count;
                    }
                }
            } else {
                for (((score_1, pos_1), (score, pos)), score_count) in &scores {
                    let mut pos_2 = *pos;
                    let mut score_2 = *score;
                    pos_2 += *roll;
                    while pos_2 > 10 {
                        pos_2 -= 10;
                    }
                    score_2 += pos_2;
                    if score_2 >= 21 {
                        wins_2 += score_count * count;
                    } else {
                        *turn_scores
                            .entry(((*score_1, *pos_1), (score_2, pos_2)))
                            .or_default() += score_count * count;
                    }
                }
            }
        }
        if turn_scores.is_empty() {
            break;
        }
        scores = turn_scores;
        turn = !turn;
    }
    println!("{}", wins_1);
    println!("{}", wins_2);
    if wins_1 > wins_2 {
    } else {
    }
}
// fn solve(self, mut input_lines: impl Iterator<Item = String>) -> SolutionResult {
//     let mut pos_1 = 8;
//     // let mut pos_1 = 4;
//     let mut score_1 = 0;
//     let mut pos_2 = 5;
//     // let mut pos_2 = 8;
//     let mut score_2 = 0;
//     let mut turn = true;
//     let mut die = (1..=100).cycle();
//     let mut count = 0;
//     while score_1 < 1000 && score_2 < 1000 {
//         let roll = die.next().unwrap() + die.next().unwrap() + die.next().unwrap();
//         count += 3;
//         if turn {
//             pos_1 += roll;
//             while pos_1 > 10 {
//                 pos_1 -= 10;
//             }
//             score_1 += pos_1;
//         } else {
//             pos_2 += roll;
//             while pos_2 > 10 {
//                 pos_2 -= 10;
//             }
//             score_2 += pos_2;
//         }
//         turn = !turn;
//         println!("{},{},{}:{}", score_1, score_2, pos_1, pos_2);
//     }
//     let l_score = if score_1 > score_2 { score_2 } else { score_1 };
//     println!("{},{},{}", l_score, count, l_score * count);
//     Ok(())
// }
