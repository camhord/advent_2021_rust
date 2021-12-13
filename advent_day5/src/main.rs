use std::env;
use std::fs;
use std::collections::HashMap;
use std::cmp;

fn main() {
    let input = fs::read_to_string("input/day5")
        .expect("Something went wrong reading the file");

    let mut points: HashMap<(i32, i32), i32> = HashMap::new();
    let mut count = 0;

    for line in input.split("\n"){
        if line.len() < 1{
            break;
        }

        let coords: Vec<(i32, i32)> = line.split(" -> ").collect::<Vec<&str>>().into_iter().map(|x| {let numbers: Vec<&str> = x.split(",").collect(); (numbers[0].parse::<i32>().unwrap(), numbers[1].parse::<i32>().unwrap())}).collect();

        let len = cmp::max((coords[0].0 - coords[1].0).abs(), (coords[0].1 - coords[1].1).abs());

        print!("{} {}: ", line, len);

        for i in 0..len + 1{
            let mut x = coords[0].0;
            let mut y = coords[0].1;

            if x > coords[1].0{
                x -= i;
            } else if x < coords[1].0 {
                x += i;
            }

            if y > coords[1].1{
                y -= i;
            } else if y < coords[1].1 {
                y += i;
            }

            let new_coord = (x, y);

            if points.contains_key(&new_coord){
                *points.get_mut(&new_coord).unwrap() += 1;
            } else {
                points.insert(new_coord, 1);
            }

            if points[&new_coord] == 2{
                count = count + 1;
            }

            print!("{:?} ", new_coord);
        }
        println!();
    }
    
    println!("greater than 1 count: {}", count);
}
