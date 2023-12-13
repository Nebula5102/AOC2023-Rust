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
    //loop over lines in file to put in string
    for line in fs::read_to_string(file_path).unwrap().lines(){
        lines+=1;
        // loop over chars
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
    //loop until beginning of number
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
 * Checks gear is connected to two numbers
 * if so returns true as it can find ratios
 *
 */
fn check_gear(around: [(char,i32); 8]) -> bool {
    let mut is_gear = false;
    let ulc= around[0].0;
    let um= around[1].0;
    let urc= around[2].0;
    let l= around[3].0;
    let r= around[4].0;
    let llc= around[5].0;
    let lm= around[6].0;
    let lrc= around[7].0;
    // Checks numbers are not touching
    // and that there is two connected
    if (ulc.is_numeric() || um.is_numeric() || urc.is_numeric()) && (l.is_numeric() || r.is_numeric() || llc.is_numeric() || lm.is_numeric() || lrc.is_numeric()) {
        is_gear = true;
    } else if (l.is_numeric() || r.is_numeric()) && (ulc.is_numeric() || um.is_numeric() || urc.is_numeric() || llc.is_numeric() || lm.is_numeric() || lrc.is_numeric()) {
        is_gear = true;
    } else if l.is_numeric() && r.is_numeric() {
        is_gear = true;
    } else if ulc.is_numeric() && urc.is_numeric() && !um.is_numeric() {
        is_gear = true;
    } else if llc.is_numeric() && lrc.is_numeric() && !lm.is_numeric() {
        is_gear = true;
    } 
    return is_gear;
}

/*
 * Does all calculations for positions
 * returns the surrounding values
 * and pointers to surrounding values
 *
 */
fn calculations(position:i32,width:i32,height:i32,matrix:String) -> [(char,i32);8] {
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
    let mut right =position;

    // calculations also will not calculate if 
    // too high or too low
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
    //println!("{},{},{},\n{},{},{},\n{},{},{}\n",ulc,um,urc,left,gear,right,llc,lm,lrc);
    let up_left_corner = matrix.chars().nth(ulc as usize).unwrap();
    let up_right_corner = matrix.chars().nth(urc as usize).unwrap();
    let up_middle = matrix.chars().nth(um as usize).unwrap();
    let low_left_corner = matrix.chars().nth(llc as usize).unwrap();
    let low_right_corner = matrix.chars().nth(lrc as usize).unwrap();
    let low_middle = matrix.chars().nth(lm as usize).unwrap();
    let left_c = matrix.chars().nth(left as usize).unwrap();
    let right_c = matrix.chars().nth(right as usize).unwrap();
    let surrounding: [(char, i32); 8] = [(up_left_corner,ulc),(up_middle,um),(up_right_corner,urc),(left_c,left),(right_c,right),(low_left_corner,llc),(low_middle,lm),(low_right_corner,lrc)];
    return surrounding;
}

/*
 * Find the ratios of the numbers connected to
 * gears and return that value
 * Ratio is the two numbers connected to the gear
 * multipied together
 *
 */
fn find_ratio(matrix: String,gear:i32,width:i32,height:i32) -> i32{
    let surrounding = calculations(gear,width,height,matrix.clone());
    //calculations and inits
    let up_left_corner = surrounding[0].0;
    let up_right_corner =surrounding[2].0;
    let up_middle =surrounding[1].0;
    let low_left_corner =surrounding[5].0;
    let low_right_corner =surrounding[7].0;
    let low_middle =surrounding[6].0;
    let left_c =surrounding[3].0;
    let right_c =surrounding[4].0;
    let llc =surrounding[5].1;
    let lrc =surrounding[7].1;
    let lm =surrounding[6].1;
    let ulc =surrounding[0].1;
    let urc =surrounding[2].1;
    let um =surrounding[1].1;    
    let left =surrounding[3].1;
    let right =surrounding[4].1;
    
    let mut val = 0;
    let mut val2 = 0;

    //logic to check find where the two numbers are
    //then those two numbers are found and saved
    if up_left_corner.is_numeric() && left_c.is_numeric() {
        let s1 = find_beginning(matrix.clone(),left);
        val = get_number(matrix.clone(),s1);
        let s2 = find_beginning(matrix.clone(),ulc);
        val2 = get_number(matrix.clone(),s2); 
    } else if up_left_corner.is_numeric() && right_c.is_numeric() {
        let s1 = find_beginning(matrix.clone(),right);
        val = get_number(matrix.clone(),s1);
        let s2 = find_beginning(matrix.clone(),ulc);
        val2 = get_number(matrix.clone(),s2); 
    } else if up_left_corner.is_numeric() && low_left_corner.is_numeric() {
        let s1 = find_beginning(matrix.clone(),llc);
        val = get_number(matrix.clone(),s1);
        let s2 = find_beginning(matrix.clone(),ulc);
        val2 = get_number(matrix.clone(),s2);
    } else if up_left_corner.is_numeric() && low_middle.is_numeric() {
        let s1 = find_beginning(matrix.clone(),lm);
        val = get_number(matrix.clone(),s1);
        let s2 = find_beginning(matrix.clone(),ulc);
        val2 = get_number(matrix.clone(),s2);
    } else if up_left_corner.is_numeric() && low_right_corner.is_numeric() {
        let s1 = find_beginning(matrix.clone(),lrc);
        val = get_number(matrix.clone(),s1);
        let s2 = find_beginning(matrix.clone(),ulc);
        val2 = get_number(matrix.clone(),s2);
    } else if up_middle.is_numeric() && left_c.is_numeric() {
        let s1 = find_beginning(matrix.clone(),left);
        val = get_number(matrix.clone(),s1);
        let s2 = find_beginning(matrix.clone(),um);
        val2 = get_number(matrix.clone(),s2); 
    } else if up_middle.is_numeric() && right_c.is_numeric() {
        let s1 = find_beginning(matrix.clone(),right);
        val = get_number(matrix.clone(),s1);
        let s2 = find_beginning(matrix.clone(),um);
        val2 = get_number(matrix.clone(),s2); 
    } else if up_middle.is_numeric() && low_left_corner.is_numeric() {
        let s1 = find_beginning(matrix.clone(),llc);
        val = get_number(matrix.clone(),s1);
        let s2 = find_beginning(matrix.clone(),um);
        val2 = get_number(matrix.clone(),s2);
    } else if up_middle.is_numeric() && low_middle.is_numeric() {
        let s1 = find_beginning(matrix.clone(),lm);
        val = get_number(matrix.clone(),s1);
        let s2 = find_beginning(matrix.clone(),um);
        val2 = get_number(matrix.clone(),s2);
    } else if up_middle.is_numeric() && low_right_corner.is_numeric() {
        let s1 = find_beginning(matrix.clone(),lrc);
        val = get_number(matrix.clone(),s1);
        let s2 = find_beginning(matrix.clone(),um);
        val2 = get_number(matrix.clone(),s2);
    } else if up_right_corner.is_numeric() && left_c.is_numeric() {
        let s1 = find_beginning(matrix.clone(),left);
        val = get_number(matrix.clone(),s1);
        let s2 = find_beginning(matrix.clone(),urc);
        val2 = get_number(matrix.clone(),s2); 
    } else if up_right_corner.is_numeric() && right_c.is_numeric() {
        let s1 = find_beginning(matrix.clone(),right);
        val = get_number(matrix.clone(),s1);
        let s2 = find_beginning(matrix.clone(),urc);
        val2 = get_number(matrix.clone(),s2); 
    } else if up_right_corner.is_numeric() && low_left_corner.is_numeric() {
        let s1 = find_beginning(matrix.clone(),llc);
        val = get_number(matrix.clone(),s1);
        let s2 = find_beginning(matrix.clone(),urc);
        val2 = get_number(matrix.clone(),s2);
    } else if up_right_corner.is_numeric() && low_middle.is_numeric() {
        let s1 = find_beginning(matrix.clone(),lm);
        val = get_number(matrix.clone(),s1);
        let s2 = find_beginning(matrix.clone(),urc);
        val2 = get_number(matrix.clone(),s2);
    } else if up_right_corner.is_numeric() && low_right_corner.is_numeric() {
        let s1 = find_beginning(matrix.clone(),lrc);
        val = get_number(matrix.clone(),s1);
        let s2 = find_beginning(matrix.clone(),urc);
        val2 = get_number(matrix.clone(),s2);
    } else if up_left_corner.is_numeric() && up_right_corner.is_numeric() {
        let s1 = find_beginning(matrix.clone(),ulc);
        val = get_number(matrix.clone(),s1);
        let s2 = find_beginning(matrix.clone(),urc);
        val2 = get_number(matrix.clone(),s2);
    } else if left_c.is_numeric() && right_c.is_numeric() {
        let s1 = find_beginning(matrix.clone(),left);
        val = get_number(matrix.clone(),s1);
        let s2 = find_beginning(matrix.clone(),right);
        val2 = get_number(matrix.clone(),s2);
    } else if left_c.is_numeric() && low_left_corner.is_numeric() {
        let s1 = find_beginning(matrix.clone(),left);
        val = get_number(matrix.clone(),s1);
        let s2 = find_beginning(matrix.clone(),llc);
        val2 = get_number(matrix.clone(),s2);
    } else if left_c.is_numeric() && low_right_corner.is_numeric() {
        let s1 = find_beginning(matrix.clone(),left);
        val = get_number(matrix.clone(),s1);
        let s2 = find_beginning(matrix.clone(),lrc);
        val2 = get_number(matrix.clone(),s2);
    } else if left_c.is_numeric() && low_middle.is_numeric() {
        let s1 = find_beginning(matrix.clone(),left);
        val = get_number(matrix.clone(),s1);
        let s2 = find_beginning(matrix.clone(),lm);
        val2 = get_number(matrix.clone(),s2);
    } else if low_left_corner.is_numeric() && low_right_corner.is_numeric() {
        let s1 = find_beginning(matrix.clone(),llc);
        val = get_number(matrix.clone(),s1);
        let s2 = find_beginning(matrix.clone(),lrc);
        val2 = get_number(matrix.clone(),s2);
    } else if right_c.is_numeric() && low_right_corner.is_numeric() {
        let s1 = find_beginning(matrix.clone(),lrc);
        val = get_number(matrix.clone(),s1);
        let s2 = find_beginning(matrix.clone(),right);
        val2 = get_number(matrix.clone(),s2);
    } else if right_c.is_numeric() && low_left_corner.is_numeric() {
        let s1 = find_beginning(matrix.clone(),llc);
        val = get_number(matrix.clone(),s1);
        let s2 = find_beginning(matrix.clone(),right);
        val2 = get_number(matrix.clone(),s2);
    } else if right_c.is_numeric() && low_middle.is_numeric() {
        let s1 = find_beginning(matrix.clone(),lm);
        val = get_number(matrix.clone(),s1);
        let s2 = find_beginning(matrix.clone(),right);
        val2 = get_number(matrix.clone(),s2);
    }

    // Multiplies two numbers together
    return val * val2;
}

/*
 * Sums the ratios of the gears
 * returns the sum of all ratios
 *
 */
fn sum_parts(grid: (String,i32)) -> i32 {
    // value inits
    let mut sum = 0;
    let height = grid.1;
    let matrix = grid.0;
    let size = matrix.len() as i32;
    let mut positions: [i32; 1250] = [-1; 1250];
    let mut pointer = 0;
    let width: i32 = size / height;
    let mut position = 0 as i32;

    // loop over all values in grid
    while position < matrix.len() as i32 {
        let cur = matrix.chars().nth(position as usize).unwrap();
        // find all gears in grid
        if cur == '*' {
            let surrounding = calculations(position,width,height,matrix.clone());
            //check gear can be used for ratio calcs
            let ratio:bool = check_gear(surrounding.clone());
            // adds ratio gear pointer to list
            if ratio {
                positions[pointer] = position;
                pointer +=1;
            }
        }
        position+=1;
    }

    pointer = 0;
    // calculates ratios of all gears that are valid
    while positions[pointer] != -1 {
        let gear = positions[pointer];
        let gear_ratio = find_ratio(matrix.clone(),gear,width,height);
        sum += gear_ratio;
        pointer += 1;
    }
    return sum;
}



fn main() { 
    let args: Vec<String> = env::args().collect();
    let grid = setup_grid(&args[1]);
    let sum = sum_parts(grid.clone());
    println!("{}",sum);
}
