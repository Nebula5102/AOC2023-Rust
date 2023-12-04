use std::env;
use std::fs;


/* 
 * Takes substring of line and 
 * traverses it to find word 
 * form of number
 * Returns char
 */
fn number_detector(mut substring: String) -> char {
    //number arrays
    let numbers3 = ["one","two","six"];
    let numbers4 = ["four","five","nine"];
    let numbers5 = ["three","seven","eight"];
    //clone substring to traverse backwards
    let mut temp = substring.clone();
    //init final val
    let mut final_val = '0';
    //loop through temp and substring until both are size 2
    //substring removes first character
    //temp removes last character
    while temp.len() > 2 {
        let size = temp.len();
        let mut found = 0;
        let mut count = 0;
        //check size to pick array to traverse
        if size == 3 {
            for number in numbers3 {
                count += 1;
                //check to find word is equal to a number
                if number == temp || number == substring{
                    found = count; 
                }
            }
            //set number to correct char
            if found == 1 {
                final_val = '1';
            } else if found == 2 {
                final_val = '2';
            } else if found == 3{
                final_val = '6';
            }
        } else if size == 4 {
            for number in numbers4 {
                count += 1;
                if number == temp || number == substring{
                    found = count; 
                }
            }
            if found == 1 {
                final_val = '4';
            } else if found == 2 {
                final_val = '5';
            } else if found == 3{
                final_val = '9';
            }
        } else {
            for number in numbers5 {
                count += 1;
                if number == temp || number == substring{
                    found = count; 
                }
            }
            if found == 1 {
                final_val = '3';
            } else if found == 2 {
                final_val = '7';
            } else if found == 3{
                final_val = '8';
            }
        }
        //removal actions
        temp.pop();
        substring.remove(0);
    }
    return final_val;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut count = 0;
    
    //read file line by line
    //add up all two digit integers found
    for line in fs::read_to_string(&args[1]).unwrap().lines() {
         let result = &line.to_string();
         let mut number = String::new();
         let mut word = String::new();
         for c in result.chars() {
             if c.is_numeric() {
                //push number to create two digit number
                number.push(c);
                break;
             } else {
                //create word to be checked by number detector
                word.push(c);
                //start checking words at size 3 as that is 
                //min size
                if word.len() >= 3 {
                    let val = number_detector(word.clone());
                    if val != '0' {
                        number.push(val);
                        break;
                    }
                //remove first character as 5 is max size
                } else if word.len() > 5{
                    word.remove(0);
                    let val = number_detector(word.clone());
                    if val != '0' {
                        number.push(val);
                        break;
                    }
                }
             }
         }
         //reset and reverse string
         word = String::new();
         let reversed: String = result.chars().rev().collect();
         //repeat same as above but in reverse
         for c in reversed.chars() {
             if c.is_numeric() {
                number.push(c);
                break;
             } else {
                //push letter to start instead of back
                //since words are read left to right
                word.insert(0,c);
                if word.len() >= 3 {
                    let val = number_detector(word.clone());
                    if val != '0' {
                        number.push(val);
                        break;
                    }
                } else if word.len() > 5{
                    word.remove(0);
                    let val = number_detector(word.clone());
                    if val != '0' {
                        number.push(val);
                        break;
                    }
                }
             }
         }
         //cast string to unsigned integer
         let cur_num: u32 = number.parse().unwrap();
         //sum all numbers
         count += cur_num
    }

    println!("{}",count);
}

