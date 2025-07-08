struct Rectangle(i8,i8);
// struct Point(u8,u8,u8);

fn main(){
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    
}

fn calculate_area(rect:&Rectangle)-> i8{
    rect.0 * rect.1
}
