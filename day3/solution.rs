use std::fs;
use std::env;

/*
 * Gets the file
 * puts file into string form
 * with no line breaks
 * Returns that String which is a grid that looks like this:
 *   467..114..  
 *   ...*......     15 16 17 18 19
 *   ..35..633.     25|26 27 28|29
 *   ......#...     35 36 37 38 39
 *
 */
fn setup_grid(file_path: &String) -> (String,i32){
    let mut grid = String::new();
    let mut lines = 0;
    //loop over string
    for line in fs::read_to_string(file_path).unwrap().lines(){
        lines+=1;
        //loop over chars
        for character in line.chars() {
            grid.push(character)
        }
    }
    //return Grid:String and the number of lines
    //for calculations
    let grid_item = (grid,lines);
    return grid_item;
}

/*
 * Finds the beginning of a number
 * in the grid
 * Returns pointer to that number in grid
 */
fn find_beginning(grid: String,cur: i32) -> i32 {
    let mut position = cur;
    // loop until beginning of number
    while (grid.chars().nth(position as usize).unwrap()).is_numeric() && position-1>=0{
        position-=1;
    }
    if position-1 <0 {
        return position;
    }
    else {
        return position + 1;
    }
}

/*
 * Gets the whole number from
 * pointer in grid
 * Returns number value
 */
fn get_number(grid: String,position:i32) -> i32 {
    let mut j = 0;
    let mut num = String::new();
    //loop over numbers in grid until no more numbers
    while (grid.chars().nth((position + j) as usize).unwrap()).is_numeric() {
        num.push(grid.chars().nth((position + j) as usize).unwrap());
        j+=1
    }
    return num.parse::<i32>().unwrap();
}

/*
 * Sums all part numbers
 * part numbers are defined by
 * a number touching a symbol in the 
 * grid
 * Returns sum of part numbers
 */
fn sum_parts(grid: (String,i32)) -> i32 {
    // initialization
    let mut sum = 0;
    let height = grid.1;
    let matrix = grid.0;
    let size = matrix.len() as i32;
    let mut positions: [i32; 10000] = [-1; 10000];
    let mut pointer = 0;
    let width: i32 = size / height;
    let mut position = 0 as i32;
    let mut values = Vec::new();
    // loop over grid
    while position < matrix.len() as i32 {
        let cur = matrix.chars().nth(position as usize).unwrap();
        // if a number check around
        if cur.is_numeric() {
            let spot = position % width;
            let row = position / width;
            let up_row = (row - 1) * width;
            let low_row = (row + 1) * width;

            let mut llc = position;
            let mut lrc = position;
            let mut lm = position;
            
            let mut ulc = position;
            let mut urc = position;
            let mut um = position;
            
            let mut left = position;
            let mut right = position;
            
            // calulations to find area around number
            if (row - 1) >= 0 && (row + 1) < height {
                llc = spot - 1 + low_row;
                lrc = spot + 1 + low_row;
                lm = spot + low_row;
                ulc = spot - 1 + up_row;
                urc = spot + 1 + up_row;
                um = spot + up_row;
                if spot + 1 <= width && spot - 1 >= 0{ 
                    left = position - 1;
                    right = position + 1;
                } else if spot + 1 > width {
                    left = position - 1;
                } else if spot - 1 < 0 {
                    right = position + 1;
                }
            } else if row - 1 < 0 {
                llc = spot - 1 + low_row;
                lrc = spot + 1 + low_row;
                lm = spot + low_row;
                if spot + 1 <= width && spot - 1 >= 0{ 
                    left = position - 1;
                    right = position + 1;
                } else if spot + 1 > width {
                    left = position - 1;
                } else if spot - 1 < 0 {
                    right = position + 1;
                }
            } else if row + 1 >= height {
                ulc = spot - 1 + up_row;
                urc = spot + 1 + up_row;
                um = spot + up_row;
                if spot + 1 <= width && spot - 1 >= 0{ 
                    left = position - 1;
                    right = position + 1;
                } else if spot + 1 > width {
                    left = position - 1;
                } else if spot - 1 < 0 {
                    right = position + 1;
                }
            }
            //prints pointers
            //println!("{},{},{},\n{},{},{},\n{},{},{}\n",ulc,um,urc,left,position,right,llc,lm,lrc);
            let up_left_corner = matrix.chars().nth(ulc as usize).unwrap();
            let up_right_corner = matrix.chars().nth(urc as usize).unwrap();
            let up_middle = matrix.chars().nth(um as usize).unwrap();
            let low_left_corner = matrix.chars().nth(llc as usize).unwrap();
            let low_right_corner = matrix.chars().nth(lrc as usize).unwrap();
            let low_middle = matrix.chars().nth(lm as usize).unwrap();
            let left_c = matrix.chars().nth(left as usize).unwrap();
            let right_c = matrix.chars().nth(right as usize).unwrap();
            // prints what number surrounding looks like
            // println!("{}{}{}\n{}/{}\n{}{}{}\n",up_left_corner,up_middle,up_right_corner,left_c,right_c,low_left_corner,low_middle,low_right_corner);
            // check if number is touching symbol and save position if is
            if !up_left_corner.is_numeric() && up_left_corner != '.' && pointer < positions.len(){
                positions[pointer] = position;
                pointer += 1;
            } else if !up_right_corner.is_numeric() && up_right_corner != '.' && pointer < positions.len(){
                positions[pointer] = position; 
                pointer += 1;
            } else if !up_middle.is_numeric() && up_middle != '.' && pointer < positions.len(){
                positions[pointer] = position; 
                pointer += 1;
            } else if !low_left_corner.is_numeric() && low_left_corner != '.' && pointer < positions.len(){
                positions[pointer] = position; 
                pointer += 1;
            } else if !low_right_corner.is_numeric() && low_right_corner != '.' && pointer < positions.len(){
                positions[pointer] = position; 
                pointer += 1;
            } else if !low_middle.is_numeric() && low_middle != '.' && pointer < positions.len(){
                positions[pointer] = position; 
                pointer += 1;
            } else if !left_c.is_numeric() && left_c != '.' && pointer < positions.len(){
                positions[pointer] = position; 
                pointer += 1;
            } else if !right_c.is_numeric() && right_c != '.' && pointer < positions.len(){
                positions[pointer] = position; 
                pointer += 1;
            } 
        }
        position+=1;
    }
    // final positions as in pointing to 
    // beginning of number
    let mut final_positions = Vec::new();
    for pos in positions {
        if pos == -1 {
            break;
        }
        // get beginning of number
        let beginning = find_beginning(matrix.clone(),pos.clone());   
        final_positions.push(beginning);
        let mut count = 0;
        // pop like numbers as in same number touching symbol
        while count+1 < final_positions.len() {
            if final_positions[count] == final_positions[count+1] {
                final_positions.pop();
            }
            count+=1;
        }
    }
    let mut count = 0;
    // loop over all numbers and sum them together
    while count < final_positions.len() {
        let num = get_number(matrix.clone(),final_positions[count]);
        values.push(num);
        count+=1
    }
    let mut i = 0;
    while i < values.len() {
        sum += values[i];
        i+=1
    }
    return sum;
}



fn main() { 
    let args: Vec<String> = env::args().collect();
    let grid = setup_grid(&args[1]);
    let sum = sum_parts(grid.clone());
    println!("{}",sum);
}
