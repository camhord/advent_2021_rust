use std::fs;

fn main() {
    let mut input: Vec<i8> = fs::read_to_string("input/day6")
        .expect("Something went wrong reading the file")
        .trim()
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i8>().unwrap())
        .collect();

        println!("{:?}", input);

    let mut days: [FishDay; 256] = [FishDay{six_fish:0, eight_fish:0}; 256];
    let mut startfish: [usize; 7] = [0; 7];

    for fish in input.iter(){
        startfish[*fish as usize] += 1;
    }

    for i in 0..startfish.len(){
        days[i].six_fish += startfish[i] as i64;
        days[i].eight_fish += startfish[i] as i64;
    }

    for i in 0..days.len(){
        if i + 2 < days.len(){
            days[i + 2].six_fish += days[i].eight_fish;
        }

        if i + 7 < days.len(){
            days[i + 7].six_fish += days[i].six_fish;
            days[i + 7].eight_fish += days[i].six_fish;
        }
    }

    let mut count: i64 = 0;
    for fishday in days{
        count += fishday.eight_fish;
    }

    println!("added {}, total {}", count, count + input.len() as i64);
}

#[derive(Copy, Clone)]
struct FishDay{
    six_fish: i64,
    eight_fish: i64
}
