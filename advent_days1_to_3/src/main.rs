use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2{
        eprintln!("Shit, that's not enough args. {:?}", args);
        return;
    }

    let flag = &args[1];
    let input = Path::new(&args[2]);

    /*
    let parsed_file = fs::read_to_string(input).unwrap();
    let parsed_lines: Vec<&str> = parsed_file.split('\n').collect();

    let mut count = 0;
    let mut sum_vec: Vec<i32> = Vec::new();

    for a_num in parsed_lines{
        if a_num.len() < 1{
            break;
        }

        let vec: Vec<i32> = a_num.chars().map(|x| if x == '1' {1} else {0}).collect();
        if sum_vec.len() == 0{
            sum_vec = vec;
        }
        else {
            for i in 0..sum_vec.len(){
                sum_vec[i] = sum_vec[i] + vec[i];
            }
        }

        count = count+ 1;
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    let len = sum_vec.len();
    
    for i in 0..len{
        if sum_vec[i] >= count / 2{
            gamma = gamma + (1 << (len - i - 1));
        } else {
            epsilon = epsilon + (1 << (len - i - 1));
        }
    }

    println!("gamma: {} epsilon: {} product: {}", gamma, epsilon, gamma * epsilon);
    */

    //let mut parsed_ints: Vec<i32> = parse_binary_file_to_vector(input);

    let parsed_file = fs::read_to_string(input).unwrap();
    let parsed_lines: Vec<&str> = parsed_file.split('\n').collect();

    let o2: i32 = calculate(parsed_lines.clone(), true);

    println!("O2: {}", o2);

    let co2: i32 = calculate(parsed_lines.clone(), false);

    println!("CO2: {}", co2);

    println!("product: {}", o2 * co2);
}

pub fn calculate(input: Vec<&str>, most: bool) -> i32 {
    let mut vec = input.clone();

    for i in 0..input[0].len(){
        if vec.len() == 1{
            break
        }

        let clone = vec.clone();
        let winner = common(clone.clone(), i, most);
        let w_char: char = if winner == 1 {'1'} else {'0'};
        vec = clone.into_iter().filter(|num| !num.is_empty() && num.trim().chars().nth(i).unwrap() == w_char).collect();
    }

    return isize::from_str_radix(vec[0].trim(), 2).unwrap().try_into().unwrap();
}

pub fn common(input: Vec<&str>, index: usize, most: bool) -> i32{
    let mut sum: i32 = 0;
    let len: i32 = input.len().try_into().unwrap();

    for a_num in input.into_iter(){
        if a_num.len() < 1{
            break;
        }

        let vec: Vec<i32> = a_num.trim().chars().map(|x| if x == '1' {1} else {0}).collect();
        sum = sum + vec[index];
    }

    let mut check = sum >= len - sum;
    if !most {
        check = !check;
    }

    return if check {1} else {0};
}

pub fn parse_binary_file_to_vector(path: &Path) -> Vec<i32> {
    let whole_file = fs::read_to_string(path).expect("shit");
    let mut parsed: Vec<i32> = Vec::new();

    for a_line in whole_file.split('\n'){
        let parsed_num: i32 = isize::from_str_radix(a_line, 2).unwrap().try_into().unwrap();

        if parsed_num >= 0 {
            parsed.push(parsed_num);
        }
    }

    return parsed;
}
