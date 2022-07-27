fn is_found(nums: Vec<i32>, start:usize, num: i32) -> Option<usize>{
    return nums[start..].iter().position(|&x| x == num)
}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for n in nums.clone() {
            println!("{}",n);
            match is_found(nums.clone(), 0,  target-n) {
                None => println!("Not Found match for {}",n),
                Some(val) => {
                    println!("Found match for {} => {}",n,val as i32);
                    return vec![4, val as i32]
                },
            }
            // println!("{\t  }", is_found(nums.clone(), 0,  target-n));
        }
        return nums;
    }
    
        
}