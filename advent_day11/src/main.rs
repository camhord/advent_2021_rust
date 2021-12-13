use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/day11").unwrap();
    let lines: Vec<&str> = input.trim().split("\n").collect();

    let mut octopi: [[i32; 10]; 10] = [[0; 10]; 10];

    for i in 0..lines.len(){
        let line: Vec<i32> = lines[i].trim().chars().map(|o| o.to_string().parse::<i32>().unwrap()).collect();
        for j in 0..line.len(){
            octopi[i][j] = line[j];
        }
    }

    let mut step = 1;
    while true{
        let flash_count = do_step(&mut octopi);
        if flash_count == 100{
            println!("all flash!: {}", step);
            break;
        } else{
            println!("{}: {}", step, flash_count);
        }
        step += 1;
    }
}

fn do_step(octopi: &mut [[i32; 10]; 10]) -> i32 {
    let mut flashed_set: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..10{
        for j in 0..10{
            octopi[i][j] += 1;
        }
    }

    let mut flashed = true;
    while flashed {
        flashed = false;

        for i in 0..10{
            for j in 0..10{
                if octopi[i][j] > 9 && !flashed_set.contains(&(i, j)){
                    flashed = true;
                    flashed_set.insert((i, j));
                    for point in get_neighbors((i, j), 10, 10){
                        octopi[point.0][point.1] += 1;
                    }
                }
            }
        }
    }

    for point in flashed_set.iter(){
        octopi[point.0][point.1] = 0;
    }

    return flashed_set.len() as i32;
}

fn get_neighbors(point: (usize, usize), max_r: usize, max_c: usize) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();

    for row in point.0 as i32 - 1..=point.0 as i32 + 1{
        if row < 0 || row >= max_r as i32{
            continue;
        }
        for col in point.1 as i32 - 1..=point.1 as i32 + 1{
            if col < 0 || col >= max_c as i32 || (row == point.0 as i32 && col == point.1 as i32){
                continue;
            }
            neighbors.insert(0, (row as usize, col as usize));
        }
    }

    return neighbors;
}
