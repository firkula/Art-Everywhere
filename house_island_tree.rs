use std::io;

fn main() {
    println!("Let's create some art!\n"); 
    
    let mut canvas_width = String::new(); 
    println!("Please enter the desired width of the canvas: "); 
    io::stdin().read_line(&mut canvas_width)
        .expect("Failed to read line");
     
    let mut canvas_height = String::new(); 
    println!("Please enter the desired height of the canvas: "); 
    io::stdin().read_line(&mut canvas_height)
        .expect("Failed to read line"); 
 
    let mut canvas_char = String::new(); 
    println!("Please enter the desired character for the canvas: "); 
    io::stdin().read_line(&mut canvas_char)
        .expect("Failed to read line"); 
 
    let mut canvas_shape = String::new(); 
    println!("Please enter the desired shape of the canvas: "); 
    io::stdin().read_line(&mut canvas_shape)
        .expect("Failed to read line"); 
 
    let cw: u8 = match canvas_width.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    }; 
 
    let ch: u8 = match canvas_height.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    }; 
 
    let cs: char = canvas_shape.trim().parse().expect("Please provide a valid character"); 
 
    let cchar: char = canvas_char.trim().parse().expect("Please provide a valid character"); 
 
    let mut canvas: Vec<Vec<char>> = Vec::with_capacity(ch as usize);
 
    for _ in 0..ch {
        let mut cols: Vec<char> = Vec::with_capacity(cw as usize); 
        for _ in 0 .. cw {
            cols.push(cs);
        }
        canvas.push(cols); 
    }
 
    //Draw the shape
    let center_x = cw / 2; 
    let center_y = ch / 2; 
    let diameter_x = cw as f32 * 0.4; 
    let diameter_y = ch as f32 * 0.4;
 
    for (y, row) in canvas.iter_mut().enumerate() {
        for (x, col) in row.iter_mut().enumerate() {
            let dist_x = (x as f32 - center_x as f32).abs(); 
            let dist_y = (y as f32 - center_y as f32).abs(); 
 
            if (dist_x * dist_x) / (diameter_x * diameter_x) + (dist_y * dist_y) / (diameter_y * diameter_y) <= 1.0 {
                *col = cchar; 
            }
        }
    } 
 
    //Print the canvas
    println!(); 
    for row in canvas.iter() {
        for col in row.iter() {
            print!("{}", col); 
        }
        println!(); 
    }
}