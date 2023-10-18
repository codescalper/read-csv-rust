use csv; //csv crate in rust
use std::error::Error; //to print out std error message

fn read_from_csv(path: &str) -> Result<(), Box<dyn Error>> {
    //* dyn mean the error can be of any type  */
    // * it represents the use of a dynamic dispatch for the Error trait in Rust.
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
fn main() {
    if let Err(e) = read_from_csv("./Sample.csv") {
        eprintln!("{}", e); //* eprintln! -> only for error and progress messages. */
    }
}
