use serde::Deserialize; 
use wmi::{Variant, WMIConnection, WMIDateTime}; 

use std::collections::HashMap; 
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wmi_con = WMIConnection::new()?;

    let results: Vec<HashMap<String, Variant>> = wmi_con.raw_query("SELECT * FROM Win32_Processor")?;
    let mut types:Vec<u16> = Vec::new();
    let mut speeds:Vec<u32> = Vec::new();
    let mut names:Vec<String> = Vec::new();
    let mut number_core:Vec<u32> = Vec::new();
    for os in results {
        match os.get("ProcessorType"){
            Some(Variant::UI2(type_cpu)) =>  {println!("kiểu cpu: {}", type_cpu); types.push(*type_cpu);},
            Some(other) => println!("ProcessorType (kiểu khác): {:?}", other),
            None => println!("Không có dữ liệu."),
        };
        match os.get("MaxClockSpeed") {
            Some(Variant::UI4(speed)) =>  {println!("speed cpu: {}", speed); speeds.push(*speed);},
            Some(other) => println!("số  (speed khác): {:?}", other),
            None => println!("Không có dữ liệu."),
        }
        match os.get("Name") {
            Some(Variant::String(name)) =>  {println!("kiểu cpu: {}", name); names.push(name.clone());},
            Some(other) => println!("tên khác: {:?}", other),
            None => println!("Không có dữ liệu."),
        }
        match os.get("NumberOfCores") {
            Some(Variant::UI4(number)) =>  {println!("số core cpu: {}", number); number_core.push(*number);},
            Some(other) => println!("số cpu khác: {:?}", other),
            None => println!("Không có dữ liệu."),
        }
    }
    Ok(())
}
