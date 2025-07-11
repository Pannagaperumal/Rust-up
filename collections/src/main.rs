use std::collections::HashMap;
fn main() {
 let mut scores = HashMap::new();
 scores.insert(String::from("red"),10);

print!("{:?}",scores.get(&String::from("red")));
}
