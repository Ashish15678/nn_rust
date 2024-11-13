use data::Data;
pub mod data;
pub mod nn;

fn main() {
    let data = Data::new(vec![vec![1, 3, 4, 5], vec![1, 2, 3, 4, 5]], vec![5, 2]);
    println!("{}", data); // Use {} instead of {:?}
}
