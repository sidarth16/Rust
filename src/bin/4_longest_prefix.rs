struct Solution;

fn update_common_substring(s: String, sub: &String)  -> String{
    let s_vec: Vec<char> = s.chars().collect();
    let sub_vec: Vec<char> = sub.chars().collect();
    // println!("subvec len{}",sub_vec.len());
    for (ind,_val) in s_vec.iter().enumerate(){

        if s_vec[ind] != sub_vec[ind]{
            println!("==> {} != {}",s_vec[ind],sub_vec[ind]);
            if ind==0 {return "".to_string(); }
            return String::from_iter(&sub_vec[0..ind]);
        }

        if ind == sub_vec.len()-1 {
            return sub.to_string();
        }
    } 
    return  String::from_iter(&sub_vec);
}

impl Solution {
    pub fn longest_common_prefix(str1: Vec<String>) -> String {
        let mut strs = str1;
        strs.sort();
        let mut sub = strs[0].to_string();
        if sub == ""{return sub;}

        println!("{:?}",strs);
        for val in strs[1..].iter(){
            println!("\nstr : {}", val);
            // println!("sub : {}",update_common_substring(val.to_string(), &sub));
            sub = update_common_substring(val.to_string(), &sub);
            if sub == ""{return sub;}
            println!("sub : {}",sub);
        } 
        sub    
    }
}

fn main(){
    let  strs =  vec!["flower".to_string(), "flow".into(), "flight".into()];
    // let  strs =  vec!["a".to_string(), "ab".into(), "a".into(), "".into()];
    println!("{:?}",Solution::longest_common_prefix( strs ));
}