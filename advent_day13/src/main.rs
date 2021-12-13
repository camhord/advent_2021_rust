use std::fs;
use std::collections::HashSet;

fn main() {
    let path = "input/day13";
    let input = fs::read_to_string(path)
        .unwrap();
    let sections: Vec<&str> = input.trim().split("\n\n").collect();
    
    let points: Vec<(i32, i32)> = 
        sections[0]
            .trim()
            .split("\n")
            .filter(|x| x.len() > 1)
            .map(|p| {
                let point: Vec<&str> = p.trim().split(",").collect(); 
                (point[0].parse::<i32>().unwrap(), point[1].parse::<i32>().unwrap())
            })
            .collect();
    
    let folds: Vec<(i32, i32)> =
        sections[1]
            .trim()
            .split("\n")
            .map(|f| {
                let parts: Vec<&str> = f.trim().split("=").collect();
                let value = parts[1].parse::<i32>().unwrap();
                if parts[0].chars().last().unwrap() == 'x'{
                    return (value, 0)
                } else {
                    return (0, value)
                }
            })
            .collect();
    
    // do a fold on the point set, save the result back to the point set, repeat
    let mut folding_hashset: HashSet<(i32, i32)> = HashSet::from_iter(points.iter().cloned());
    for fold in folds{
        let folding_set =
            folding_hashset.iter()
                .map(|p| {
                    // don't fold points that don't fall after the fold
                    if !((p.0 > fold.0 && fold.0 > 0) || (p.1 > fold.1 && fold.1 > 0)){
                        return *p
                    }
                    if fold.0 > 0{
                        return (fold.0 - (p.0 - fold.0), p.1)
                    } else {
                        return (p.0, fold.1 - (p.1 - fold.1))
                    }
                })
                .collect::<Vec<(i32, i32)>>();

        folding_hashset = HashSet::from_iter(folding_set.iter().cloned());
    }

    // display output by constructing a plane of spaces and fill in points from folding_hashset with hash chars
    let x_vals = folding_hashset.iter().map(|p| p.0).collect::<Vec<i32>>();
    let y_vals = folding_hashset.iter().map(|p| p.1).collect::<Vec<i32>>();
    let max_x = x_vals.iter().max().unwrap();
    let max_y = y_vals.iter().max().unwrap();

    // this part looks transposed because tuple (x, y) maps to the vec at vec[y][x] since I treat the inner vec<char>s as rows in the output
    let mut display: Vec<Vec<char>> = vec![vec![' '; *max_x as usize + 1]; *max_y as usize + 1];
    for point in folding_hashset{
        display[point.1 as usize][point.0 as usize] = '#';
    }

    for line in display{
        println!("{}", line.iter().collect::<String>());
    }
}
