use std::io;

fn main() {
    
    println!("Please enter the number of rows: ");
    
    let mut rows = String::new();
    
    io::stdin()
        .read_line(&mut rows)
        .expect("Failed to read line");
    
    let rows: u32 = rows.trim().parse().expect("Please type a number!");
    
    for number in 0..rows {
        
        let mut num_stars:u32 = 0;
        let mut num_spaces:u32 = 0;
        let row_stars = 2 * number + 1;
        
        let mut text=String::from("");
        
        while num_spaces < rows - number {
            
            text.push_str("  ");
            num_spaces += 1;
        }
        while num_stars < row_stars {
            
            text.push_str("* ");
            num_stars += 1;
        }
        println!("{}",text);
    }
}
