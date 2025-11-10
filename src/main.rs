use serde::Deserialize; 
use wmi::{Variant, WMIConnection, WMIDateTime}; 

use std::collections::HashMap; 
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wmi_con = WMIConnection::new()?;

    let results: Vec<HashMap<String, Variant>> = wmi_con.raw_query("SELECT * FROM Win32_Processor")?;
    let mut types:Vec<u16> = Vec::new();
    for os in results {
        match os.get("ProcessorType"){
            Some(Variant::UI2(type_cpu)) =>  {println!("Lõi: {}", type_cpu); types.push(*type_cpu);},
            Some(other) => println!("ProcessorType (kiểu khác): {:?}", other),
            None => println!("Không có dữ liệu."),
        };
        // println!("{:#?}", os.get("ProcessorType"));
    }
    Ok(())
}
