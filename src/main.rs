use std::{env, process::exit};

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let mut zero_place=  "0";
    let mut one_place= "1";

    if args.len() == 1 {
        println!("Usage: rbin base_10_number [options 0 placeholder] [options 1 placeholder]");
        exit(0);
    }

    if args.len() >=3 {
        zero_place = args[2].trim();
    }
    
    if args.len() ==4 {
        one_place = args[3].trim();
    }
    


    let digit: i64 = args[1].trim().parse().expect("Can't parse digit");
    
    let digit:String = format!("{:b}", digit);

    let mug:usize = digit.len() % 4;
    for (i, c) in digit.chars().enumerate() {
        if i % 4 == mug && i != 0 {
            print!(" ");
        }

        if c == '1' {
            //print!("█");
            print!("{}",one_place);
        } else {
            print!("{}",zero_place);
            
            //print!("░");
            //print!("0")
        }
    }
    println!();
}
