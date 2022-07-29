struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // let num: String = x.to_string();
        let num_vec: Vec<char> = x.to_string().chars().collect();
        println!("{:?}",num_vec);

        let mid_index = num_vec.len()/2 ;

        println!("Mid_index : {}",mid_index );
        for i in 0..mid_index{
            println!("{} == {}",num_vec[i],num_vec[num_vec.len()-i-1]);
            if num_vec[i] != num_vec[num_vec.len()-i-1]{
                return false
            }
        }
        return true
    }
}

fn main(){
    println!("{:?}",Solution::is_palindrome(-98789) );
}