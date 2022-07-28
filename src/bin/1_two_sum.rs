struct Solution;

fn is_found(nums: Vec<i32>, num: i32) -> Option<usize>{
    // println!("\t==> {:?}",nums);
    return nums.iter().position(|&x| x == num)
}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // println!("{:?}",nums);
        for (ind, val) in nums.iter().enumerate() {
            // println!("=>{},{}",ind,val);
            match is_found(nums[ind+1..].to_vec(), target-val) {
                None => {},
                Some(found_ind) => {
                    // println!("Found match for {} => {}",val,nums[found_ind+ind+1 ] );
                    return vec![ind as i32, (found_ind+ind+1) as i32];
                }
            }
        }
        return vec![];
    }   
}

fn main(){
    println!("{:?}",Solution::two_sum( vec![2,7,1,5] , 9 ) );
}