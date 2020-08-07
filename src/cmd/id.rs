use crate::id;

pub fn run(name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    match name {
        Some(n) => {
            println!("/id {} = {}", n, id::get(n));
        }
        None => {
            println!("{}", id::new());
        }
    };

    Ok(())
}
