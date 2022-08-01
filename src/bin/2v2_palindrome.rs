struct Solution;

fn get_rev_num(num: &mut i32) -> i32{
    let mut rev_num:i32 = 0;
    while *num != 0{
        rev_num = rev_num*10 + *num%10 ;
        // println!("t_num:{}, {:?}",t_num,num_vec);
        *num = *num/10 ;
    }
    println!("rev_num : {:?}",rev_num);
    return rev_num;
}


impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut x = x;
        if x<0 {return false}
        if x == get_rev_num(&mut x) { return true}
        false
    }
}

fn main(){
    println!("{:?}",Solution::is_palindrome(1331) );
}