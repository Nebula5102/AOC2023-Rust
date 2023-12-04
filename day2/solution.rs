use std::fs;
use std::env;

/*
 * Formats file in the form of 
 * ID xrxgxb...
 * ID xrxgxb...
 * ID xrxgxb...
 * returns a in above format
 */
fn format_input(file_path: &String) -> String{
    //init final output
    let mut pre_str = String::new();
    //init flags
    let mut id_flag: u32;
    let mut colour_flag = 0;
    //loop over characters in strinng and put them in specified format
    for line in fs::read_to_string(file_path).unwrap().lines(){
        id_flag = 0;
        for character in line.chars() {
            if character == ':' && id_flag == 0 {
                id_flag = 1;
                pre_str.push(' ');
            }
            if id_flag == 0 && character.is_numeric() {
                pre_str.push(character);
            } else {
                if character.is_numeric() {
                    pre_str.push(character);
                    colour_flag = 0;
                } else if colour_flag == 0 && (character == 'b' || character == 'r' || character == 'g'){
                    colour_flag = 1;
                    pre_str.push(character);
                }
            }
        }
        pre_str.push_str("\n");
    }
    return pre_str
}

/*
 * checks if game is possible
 * format for possible games:
 * r<=12 g<=13 b<=14
 * if possible adds sums all IDs
 * of possible games
 */ 
fn possible(formatted: &String) -> u32{
    let mut sum = 0;
    // loop through formatted string
    for val in formatted.lines() {
        let mut final_bool = true;
        let mut id_flag = 0;
        let mut id = String::new();
        let mut cur_val = String::new();
        for c in val.chars() {
            // get id and skip to values
            if id_flag == 0 && c == ' ' {
                id_flag = 1;
            } else if id_flag == 0 {
                id.push(c);
            } else {
                if c.is_numeric() {
                    cur_val.push(c);
                } else {
                    //check to see if value is within game parameters
                    let check = cur_val.parse::<u32>().unwrap();
                    if check > 12 && c == 'r' {
                        final_bool = false; 
                    } else if check > 13 && c == 'g' {
                        final_bool = false;
                    } else if check > 14 && c == 'b' {
                        final_bool = false;
                    }
                    cur_val.clear();
                }
            }
        }
        //sum all values together
        if final_bool == true {
            let add = id.parse::<u32>().unwrap();
            sum += add;
        }
    }
    return sum;
}

/*
 * finds largest values of
 * r g and b then multiplies
 * them together to get the
 * power
 * returns sum of powers
 */
fn power(formatted: &String) -> u32 {
    let mut sum = 0;
    //loops to find largest rgb of each line
    for val in formatted.lines() {
        let mut id_flag = 0;
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        let mut cur_val = String::new();
        for c in val.chars() {
            //skip over id
            if id_flag == 0 && c == ' ' {
                id_flag = 1;
            } 
            if id_flag == 1 && c != ' '{
                if c.is_numeric() {
                    cur_val.push(c);
                } else {
                    //check for largest value
                    let check = cur_val.parse::<u32>().unwrap();
                    if c == 'r' && check > r {
                        r = check;
                    } else if c == 'g' && check > g {
                        g = check;
                    } else if c == 'b' && check > b {
                        b = check;
                    }
                    cur_val.clear();
                }
            }
        }
        //finds power of line and adds to sum
        let power = r*g*b;
        sum += power;
    }
    return sum;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = format_input(&args[1]);
    let sum = possible(&input);
    let power = power(&input);
    println!("{}",sum);
    println!("{}",power);
}
