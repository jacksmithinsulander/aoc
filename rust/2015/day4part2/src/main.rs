use md5;

fn main() {
    let mut num: u32 = 0;
    let key = "iwrupvqb";

    loop {
        num += 1;
        let full = format!("{key}{num}");
        let digest = md5::compute(full);    
        println!(
            "Testing: {}", 
            format!("{:x}",digest)
        );
        if &format!("{:x}", digest)[..6] == "000000"{
            println!("Found it at num: {}", num);
            break;
        }
    }
}
