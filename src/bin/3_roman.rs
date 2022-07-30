// use std::collections::HashMap;
// let mut roman_map =  HashMap::new();

struct Solution;

fn get_roman_val(roman: &char)  -> i32{
    let number: i32 = match roman {
        'I' =>  1,
        'V' =>  5,
        'X' =>  10,
        'L' =>  50,
        'C' =>  100,
        'D' =>  500,
        'M' =>  1000,
        _   =>  0
    };
    return  number;
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let roman_vec: Vec<char> = s.chars().collect();
        let mut result : i32 = 0; 
        for (ind,val) in roman_vec.iter().enumerate(){
            if ind == 0{
                result+=get_roman_val(val);
                println!("Result  : {}",result);
            }
            else{
                let curr_val = get_roman_val(val);
                let prev_val = get_roman_val(&roman_vec[ind-1] ) ;
    
                if curr_val > prev_val{
                     result-=prev_val ;
                     result+=curr_val - prev_val;
                }
                else {
                    result+=curr_val;
                }
                println!("Result  : {}",result);
            }
        }
        result
    }
}

fn main(){
    println!("{:?}",Solution::roman_to_int( "MIXII".to_string()) );
}