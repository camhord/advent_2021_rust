use std::fs;
use std::collections::HashMap;

fn main() {
    let path = "input/day14";
    let input = fs::read_to_string(path)
        .unwrap();
    let sections: Vec<&str> = input.trim().split("\n\n").collect();
    let poly = sections[0].to_string();
    let poly_dict: HashMap<(char, char), char> = 
        sections[1].trim().split("\n").collect::<Vec<&str>>().iter().map(|x| {
            let parts: Vec<&str> = x.split(" -> ").collect();
            let pair: Vec<char> = parts[0].chars().collect();
            ((pair[0], pair[1]), parts[1].chars().collect::<Vec<char>>()[0])
        }).collect::<HashMap<_, _>>();

    let step_count = 40;

    let mut step_counts: Vec<HashMap<(char, char), i64>> = vec![HashMap::new(); step_count + 1];

    for key in poly_dict.keys(){
        step_counts[0].insert(*key, 0);
    }

    let poly_chars = poly.chars().collect::<Vec<char>>();
    for i in 0..poly.len() - 1{
        let pair = (poly_chars[i], poly_chars[i+1]);

        if step_counts[0].contains_key(&pair){
            let count = step_counts[0].entry(pair).or_insert(0);
            *count += 1;
        }
    }

    for i in 0..step_count{
        for key in poly_dict.keys(){
            if !step_counts[i].contains_key(key){
                continue;
            }

            let curr_pair_count = step_counts[i][key];
            let char_new = poly_dict[key];
            let pair_a = (key.0, char_new);
            let pair_b = (char_new, key.1);

            if poly_dict.contains_key(&pair_a){
                let count = step_counts[i + 1].entry(pair_a).or_insert(0);
                *count += curr_pair_count;
            }

            if poly_dict.contains_key(&pair_b){
                let count = step_counts[i + 1].entry(pair_b).or_insert(0);
                *count += curr_pair_count;
            }
        }
    }

    // count letters by the first letter appearing in each pair
    let final_step = step_counts.last().unwrap();
    let mut final_counts: HashMap<char, i64> = HashMap::new();
    for key in final_step.keys(){
        let count = final_counts.entry(key.0).or_insert(0);
        *count += final_step[key];
    }
    // increment the last letter in the original poly since we don't count it otherwise (I think?)
    let count = final_counts.entry(poly.chars().last().unwrap()).or_insert(0);
    *count += 1;

    let max = final_counts.values().max().unwrap();
    let min = final_counts.values().min().unwrap();

    println!("{} - {} = {}", max, min, max - min);
}
