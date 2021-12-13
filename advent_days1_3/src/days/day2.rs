//part 1
    /*
    let mut pos: i32 = 0;
    let mut depth: i32 = 0;

    for a_command in parsed_dirs.split('\n'){
        let atoms: Vec<&str> = a_command.split_whitespace().collect();
        if atoms.len() != 2{
            break;
        }
        println!("{:?}", atoms);
        let magnitude = atoms[1].trim().parse::<i32>().unwrap();
        match atoms[0]{
            "forward" => pos = pos + magnitude,
            "down" => depth = depth + magnitude,
            "up" => depth = depth - magnitude,
            _ => println!("What?! {} -> {} {}", a_command, atoms[0], atoms[1])
        }
    }

    println!("{} x {} = {}", pos, depth, pos  * depth);
    */

    let mut pos: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    for a_command in parsed_dirs.split('\n'){
        let atoms: Vec<&str> = a_command.split_whitespace().collect();
        if atoms.len() != 2{
            break;
        }
        println!("{:?}", atoms);
        let magnitude = atoms[1].trim().parse::<i32>().unwrap();
        match atoms[0]{
            "forward" => {
                pos = pos + magnitude;
                depth = depth + magnitude * aim;
            },
            "down" => aim = aim + magnitude,
            "up" => aim = aim - magnitude,
            _ => println!("What?! {} -> {} {}", a_command, atoms[0], atoms[1])
        }
    }

    println!("{} x {} = {}", pos, depth, pos  * depth);

    pub fn parse_file_to_vector(path: &Path) -> Vec<i32> {
        let whole_file = fs::read_to_string(path).expect("shit");
        let mut parsed: Vec<i32> = Vec::new();
    
        for a_line in whole_file.split('\n'){
            let parsed_num = match a_line.trim().parse() {
                Err(why) => {
                    eprintln!("CHRIST'S BALLS, can't parse {} | {}", a_line, why);
                    -1
                },
                Ok(num) => num
            };
    
            if parsed_num >= 0 {
                parsed.push(parsed_num);
            }
        }
    
        return parsed;
    }