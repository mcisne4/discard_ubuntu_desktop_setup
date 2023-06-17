fn main() {
    //
    match rs_logger::log_details("0003F3") {
        Ok(details) => println!("{:?}", details),
        Err(e) => println!("ERROR:\n  {}", e.to_string()),
    }
}
