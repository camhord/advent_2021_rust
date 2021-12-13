use std::fs;
use std::collections::HashSet;

fn main() {
    let path = "input/day9";
    let input = fs::read_to_string(path)
        .unwrap();
    let map: Vec<Vec<i32>> = input
        .trim()
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>().iter().map(|c| c.to_string().parse::<i32>().unwrap()).collect())
        .collect::<Vec<Vec<i32>>>();
    
    let mut basin_sizes: Vec<i32> = Vec::new();

    for i in 0..map.len(){
        for j in 0..map[i].len(){
            let current = map[i][j];
            if j > 0 && map[i][j - 1] <= current{
                continue;
            }
            if i > 0 && map[i - 1][j] <= current{
                continue;
            }
            if i < map.len() - 1 && map[i + 1][j] <= current{
                continue;
            }
            if j < map[i].len() - 1 && map[i][j + 1] <= current{
                continue;
            }
            let basinset: HashSet<(usize, usize)> = HashSet::new();
            let basin = FindBasin(&map, (i, j), basinset);
            println!("{:?}: {}", basin.1, basin.0);
            basin_sizes.insert(0, basin.0);
        }
    }

    basin_sizes.sort();
    basin_sizes.reverse();

    println!("{:?}", basin_sizes);
    println!("{}", basin_sizes[0] * basin_sizes[1] * basin_sizes[2]);
}

fn FindBasin(map: &Vec<Vec<i32>>, point: (usize, usize), mut current: HashSet<(usize, usize)>) -> (i32, HashSet<(usize, usize)>){
    if current.contains(&point){
        return (0, current);
    }

    let mut size = 1;

    current.insert(point);

    if point.1 > 0 && map[point.0][point.1 - 1] != 9{
        let result = FindBasin(map, (point.0, point.1 - 1), current);
        size += result.0;
        current = result.1;
    }
    if point.0 > 0 && map[point.0 - 1][point.1] != 9{
        let result = FindBasin(map, (point.0 - 1, point.1), current);
        size += result.0;
        current = result.1;
    }
    if point.0 < map.len() - 1 && map[point.0 + 1][point.1] != 9{
        let result = FindBasin(map, (point.0 + 1, point.1), current);
        size += result.0;
        current = result.1;
    }
    if point.1 < map[point.0].len() - 1 && map[point.0][point.1 + 1] != 9{
        let result = FindBasin(map, (point.0, point.1 + 1), current);
        size += result.0;
        current = result.1;
    }

    return (size, current);
}
