use std::env;
use std::fs;

/*
 * Takes input and returns 
 * a tuple containing
 * a vector of winning nums
 * and a vector of the nums
 * you have
 *
 */
fn handle_input(file_path: &String) -> Vec<(Vec<i32>,Vec<i32>)> {
    let mut lotto_draws = Vec::new();
    // read file
    for line in fs::read_to_string(file_path).unwrap().lines(){
        // split by spaces
        let comps = line.split(" ");
        let mut winning = Vec::new();
        let mut mine = Vec::new();
        let mut check_flag = 0;
        for num in comps {
            let string = num.to_string();
            // check to switch to players card
            if string == "|"{
                check_flag = 1;
            }
            // check if val is numeric
            if string.parse::<i32>().is_ok() {
                if check_flag == 0 {
                    winning.push(string.parse::<i32>().unwrap());
                } else if check_flag == 1 {
                    mine.push(string.parse::<i32>().unwrap());
                }
            }
        }
        // push vector of winning nums and player nums
        lotto_draws.push((winning,mine))
    }
    return lotto_draws
}



fn main(){
    let args: Vec<String> = env::args().collect();
    let draws = handle_input(&args[1]);
    let mut total = 0;
    // loop over draws and count matches
    for draw in draws {
        let mut count = 0;
        let winning = &draw.0;
        let mine = &draw.1;
        for num in mine {
            for val in winning {
                if num == val {
                    count += 1;
                    break;
                }
            }
        }
        if count == 1 {
            total +=1;
        } else if count > 1 {
            let base: i32 = 2;
            // calc points gained
            // 1 for first card then doubles every time
            total += base.pow(count-1);
        }    
    }
    println!{"{}",total};
}
