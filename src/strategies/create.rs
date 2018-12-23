pub struct CreateStrategy {}

impl CreateStrategy {
    pub fn handle (inactive: bool, name: String)  {
        println!("{}, {}", inactive, name);
    }
}