struct Solution;

fn get_num_vec(&num: &i32) -> Vec<i32>{
    let mut t_num = num;
    let mut num_vec = Vec::new();
    while t_num%10 != 0{
        println!("num : {}",num);
        num_vec.push( t_num%10);
        println!("t_num:{}, {:?}",t_num,num_vec);
        t_num = t_num/10 ;
    }
    return num_vec;
}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // let num: String = x.to_string();
        // let num_vec: Vec<char> = x.to_string().chars().collect();
        if x<0{
            return false;
        }
        let num_vec: Vec<i32> = get_num_vec(&x);
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
    println!("{:?}",Solution::is_palindrome(98789) );
    // println!("{:?}",get_num_vec(1234));
}