use std::fs;

fn main() {
    let mut input: Vec<i32> = fs::read_to_string("input/day7")
        .expect("Something went wrong reading the file")
        .trim()
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    input.sort();

    let midpoint = input[input.len()/2];
    let mut fuel_count = 0;

    for crab in input.iter(){
        fuel_count += (crab - midpoint).abs();
    }

    println!("{}", fuel_count);

    let mut min = *input.iter().min().unwrap();
    let mut max = *input.iter().max().unwrap();
    let mut result = 2;
    let mut check = (max - min) / 2;
    let mut fuel_count = 0;

    while result != 0{
        let out = CheckValue(&input, check);
        result = out.0;
        fuel_count = out.1;

        if result < 0{
            max = check;
            check = (check - min) / 2 + min;
        } else if result > 0{
            min = check;
            check = (max - check) / 2 + check;
        }

        println!("min {} | max {}", min, max);
    }

    println!("{} {}", check, fuel_count);
}

pub fn CheckValue(crabs: &Vec<i32>, value: i32) -> (i32, i32){
    let mut fuel_count = 0;
    let mut low_fuel_count = 0;
    let mut high_fuel_count = 0;

    for crab in crabs.iter(){
        let steps: f32 = (crab - value).abs() as f32;
        fuel_count += ((steps + 1.0) * (steps / 2.0)) as i32;

        let steps: f32 = (crab - (value - 1)).abs() as f32;
        low_fuel_count += ((steps + 1.0) * (steps / 2.0)) as i32;

        let steps: f32 = (crab - (value + 1)).abs() as f32;
        high_fuel_count += ((steps + 1.0) * (steps / 2.0)) as i32;
    }

    if low_fuel_count < fuel_count{
        return (-1, low_fuel_count);
    }
    else if high_fuel_count < fuel_count{
        return (1, high_fuel_count);
    }

    return (0, fuel_count);
}