use garden::vegetables::Asparagus;

mod garden;

fn main() {
    garden::hello_from_garden();
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
