use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/day8")
        .expect("Something went wrong reading the file");
    let lines = input
        .trim()
        .split("\n")
        .collect::<Vec<&str>>();

        let mut sevensegs: Vec<HashSet<char>> = vec![HashSet::new(); 10];

        sevensegs[0] = ['a', 'b', 'c', 'e', 'f', 'g'].into_iter().collect();
        sevensegs[1] = ['c','f'].into_iter().collect();
        sevensegs[2] = ['a', 'c', 'd', 'e', 'g'].into_iter().collect();
        sevensegs[3] = ['a', 'c', 'd', 'f', 'g'].into_iter().collect();
        sevensegs[4] = ['b', 'c', 'd', 'f'].into_iter().collect();
        sevensegs[5] = ['a', 'b', 'd', 'f', 'g'].into_iter().collect();
        sevensegs[6] = ['a', 'b', 'd', 'e', 'f', 'g'].into_iter().collect();
        sevensegs[7] = ['a', 'c', 'f'].into_iter().collect();
        sevensegs[8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'].into_iter().collect();
        sevensegs[9] = ['a', 'b', 'c', 'd', 'f', 'g'].into_iter().collect();

    let mut unique_count = 0;

    for line in lines.iter(){
        let output: Vec<&str> = line.split(" | ").collect::<Vec<&str>>()[1].split_whitespace().collect();

        for value in output{
            if [2, 4, 3, 7].contains(&(value.len() as i32)){
                unique_count += 1;
            } 
        }
    }

    let mut sum = 0;

    for line in lines.iter(){
        let mut local: Vec<HashSet<char>> = vec![HashSet::new(); 10];

        let split = line.split(" | ").collect::<Vec<&str>>();
        let input: Vec<&str> = split[0].split_whitespace().collect();
        let output: Vec<&str> = split[1].split_whitespace().collect();

        for signal in input.iter(){
            let chars: HashSet<char> = signal.chars().collect();
            match signal.len(){
                2 => local[1] = chars,
                3 => local[7] = chars,
                4 => local[4] = chars,
                7 => local[8] = chars,
                _ => local[0] = HashSet::new(),
            }
        }

        let local_a: char = *local[7].difference(&local[1]).collect::<Vec<&char>>()[0];


        // len: 6 = 0, 9, 6 | len 5 = 2, 3, 5
        // 9: 4 & local_a && len = 6
        // 6: 8 ^ 7 && len = 6
        // 0: 
        // 2: 
        // 3: interset 7 == 7 && len = 5
        // 5: 

        for signal in input.iter(){
            let chars: HashSet<char> = signal.chars().collect();
            if chars.len() == 5 {
                if chars.intersection(&local[7]).collect::<Vec<&char>>().len() == 3{
                    local[3] = chars;
                }
                else if chars.intersection(&local[4]).collect::<Vec<&char>>().len() == 3{
                    local[5] = chars;
                }
                else {
                    local[2] = chars;
                }
            } else if chars.len() == 6 {
                if chars.contains(&local_a) && chars.intersection(&local[4]).collect::<Vec<&char>>().len() == local[4].len() {
                    local[9] = chars;
                } 
                else if chars.intersection(&local[1]).collect::<Vec<&char>>().len() == 1 {
                    local[6] = chars;
                }
                else {
                    local[0] = chars;
                }
            }
        }

        for i in 0..10{
            print!("{}: {:?}  ", i, local[i]);
        }
        println!("");

        let mut out_val = 0;
        for i in 0..4{
            let chars: HashSet<char> = output[i].chars().collect();
            for j in 0..10{
                if local[j].len() == chars.len() && local[j].difference(&chars).collect::<Vec<&char>>().len() == 0{
                    out_val += (j as i32) * i32::pow(10, 3 - i as u32);
                    break;
                }
            }
        }

        sum += out_val;
        println!("{:?}: {}", output, out_val);
    }

    println!("{}", sum);
}
