use csv; //csv crate in rust
use std::error::Error; //to print out std error message

fn read_from_csv(path: &str) -> Result<(), Box<dyn Error>> {
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
