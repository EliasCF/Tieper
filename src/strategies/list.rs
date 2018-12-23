pub struct ListStrategy {}

impl ListStrategy {
    pub fn handle (active: bool, inactive: bool, name: Option<String>) {
        println!("{}, {}, {}", active, inactive, name.unwrap());
    }
}