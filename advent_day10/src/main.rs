use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let path = "input/day10";
    let input = fs::read_to_string(path)
        .unwrap();

    let mut score_map: HashMap<char, i32> = HashMap::from(
        [(')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137)]);

    let mut completion_score_map: HashMap<char, i32> = HashMap::from(
        [(')', 1),
        (']', 2),
        ('}', 3),
        ('>', 4)]);

    let opens = HashSet::from(['(', '[', '{', '<']);
    let closes = HashSet::from([')', ']', '}', '>']);
    let pair_map = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<'), 
    ('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    let mut sum = 0;
    let mut scores: Vec<i128> = Vec::new();
    
    'outer:for line in input.trim().split("\n"){
        let mut linestack: Vec<char> = Vec::new();
        for bracket in line.chars(){
            if opens.contains(&bracket){
                linestack.push(bracket);
            } 
            else if *linestack.last().unwrap() == pair_map[&bracket]{
                linestack.pop();
            } 
            else if closes.contains(&bracket){
                sum += score_map[&bracket];
                continue 'outer;
            } else {
                linestack.push(bracket);
            }
        }

        if linestack.len() > 0{
            // incomplete!
            scores.push(0);
            while linestack.len() > 0{
                let popped = linestack.pop().unwrap();
                let bracket = pair_map[&popped];
                let mut score = scores.pop().unwrap();
                score *= 5;
                score += completion_score_map[&bracket] as i128;
                scores.push(score);
            }
        }
    }

    scores.sort();
    let middle = scores[(scores.len() - 1) / 2];

    println!("middle incomplete score: {}", middle);
    //println!("error score: {}", sum);
}