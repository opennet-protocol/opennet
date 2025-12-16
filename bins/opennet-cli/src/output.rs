//! Output formatting.

use serde::Serialize;

pub fn print_json<T: Serialize>(value: &T) {
    println!("{}", serde_json::to_string_pretty(value).unwrap());
}

pub fn print_table(headers: &[&str], rows: &[Vec<String>]) {
    // Print headers
    println!("{}", headers.join("\t"));
    println!("{}", "-".repeat(40));
    
    // Print rows
    for row in rows {
        println!("{}", row.join("\t"));
    }
}
