struct Rectangle(i8,i8);
// struct Point(u8,u8,u8);

fn main(){
    let c1 = Rectangle(8,4);


    let area = calculate_area(&c1);
    print!("Rect : height ->{}, width -> {}",c1.0,c1.1);
    println!("Area of recatangle:{}",area);
    
}

fn calculate_area(rect:&Rectangle)-> i8{
    rect.0 * rect.1
}
