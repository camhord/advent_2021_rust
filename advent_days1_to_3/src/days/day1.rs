use std::collections::VecDeque;
extern crate queues;
use queues::*;

pub struct DepthSet{
    set: Vec<i32>
}

impl DepthSet{
    fn sum(&self) -> i32 {
        let mut sum = 0;
        for a_value in &self.set{
            sum = sum + a_value;
        }
        return sum;
    }

    fn is_ready(&self) -> bool {
        return self.set.len() == 3;
    }

    fn add(&mut self, aValue: i32){
        if self.is_ready() {
            self.set.pop();
        }
        
        self.set.insert(0, aValue);
    }
}

// part 1
    /*
    let mut curr_depth = parsed_values[0];
    let mut down_count = 0;
    for a_depth in parsed_values {
        if a_depth > curr_depth {
            down_count = down_count + 1;
            println!("increase");
        }
        else {
            println!("decrease");
        }
        curr_depth = a_depth;
    }
    */

    // part 2
    let mut setA = DepthSet { set: Vec::new() };
    let mut setB = DepthSet { set: Vec::new() };
    let mut down_count = 0;

    setA.add(parsed_values[0]);

    for a_depth in parsed_values.iter().skip(1) {
        setB.add(*a_depth);

        if setA.is_ready() && setB.is_ready(){
            if setB.sum() > setA.sum(){
                down_count = down_count + 1;
            }
        } else {
            println!("A: {} | B: {}", setA.is_ready(), setB.is_ready());
        }

        setA.add(*a_depth);
    }

    println!("{}", down_count);