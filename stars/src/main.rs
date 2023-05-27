use std::env;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    
    let mut rows: u32 = 10;
    
    if args.len() >= 2 {
        
        rows = match args[1].trim().parse() {
            Ok(num) => num,
            Err(_) =>  { println!("Invalid value '{}' for rows. Defaulting to 10 rows", args[1].trim());
                10 },
                
        };
    }
    
    for number in 0..rows {
    
       print_row(rows, number)
    }
    
    for number in (0..rows-1).rev() {
        
        print_row(rows, number)
       
    }
}

fn print_row(max_rows: u32, row_number: u32) {
    
    let mut num_stars:u32 = 0;
    let mut num_spaces:u32 = 0;
    let row_stars = 2 * row_number + 1;
    
    let mut text=String::from("");
    
    while num_spaces < max_rows - row_number {
        
        text.push_str("  ");
        num_spaces += 1;
    }
    while num_stars < row_stars {
        
        text.push_str("* ");
        num_stars += 1;
    }
    println!("{}",text);
    
}
