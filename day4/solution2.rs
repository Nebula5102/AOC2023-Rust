use std::env;
use std::fs;

/*
 * Takes file counts matches and returns 
 * two vectors of winning and player vals
 *
 */
fn handle_input(file_path: &String) -> Vec<(i32,Vec<i32>,Vec<i32>,i32)> {
    let mut lotto_draws = Vec::new();
    for line in fs::read_to_string(file_path).unwrap().lines(){
        // split by spaces to get nums
        let comps = line.split(" ");
        let mut winning = Vec::new();
        let mut mine = Vec::new();
        let mut check_flag = 0;
        for num in comps {
            let string = num.to_string();
            // check to switch to player nums
            if string == "|"{
                check_flag = 1;
            }
            if string.parse::<i32>().is_ok() {
                if check_flag == 0 {
                    winning.push(string.parse::<i32>().unwrap());
                } else if check_flag == 1 {
                    mine.push(string.parse::<i32>().unwrap());
                }
            }
        }
        let mut count = 0;
        // count matches
        for val in &mine {
            for num in &winning {
                if val == num {
                    count+=1;
                    break;
                }
            }
        } 
        // set card tuple
        // matches, winning nums, player nums, repeats
        let cards = (count,winning,mine,1);
        lotto_draws.push(cards);
    }
    return lotto_draws
}

/*
 * Checks for repeats in draws
 * adds the repeats up to get
 * total number of all
 * cards that are won
 *
 *
 */
fn check_draws(mut draws: Vec<(i32,Vec<i32>,Vec<i32>,i32)>) -> i32 {
    let mut i = 0;
    let mut sum = 0;
    // loop over draws
    while i < draws.len() {
        let card = draws[i].clone();
        let matches = card.0;
        let repeat = card.3;
        let mut j = 0;
        // loop in range of matches
        // to add repeats to future vals
        while j <= matches {
            let future = &mut draws[(i as usize)+(j as usize)];
            // add repeats as that is the number of 
            // cards that are won
            future.3 += repeat;
            j+=1;
        }
        i += 1;
        sum += repeat;
    }
    return sum;
}

fn main(){
    let args: Vec<String> = env::args().collect();
    let draws = handle_input(&args[1]);
    let sum = check_draws(draws.clone());
    println!{"{}",sum};
}
