use std::cmp::max;
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::process;
use unicode_width::UnicodeWidthStr;

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let records: Vec<_> = rdr.records().map(|r| r.unwrap()).collect();
    let mut len_map = HashMap::new();
    for record in &records {
        record.iter().enumerate().for_each(|(i, field)| {
            let actual_len = UnicodeWidthStr::width_cjk(field);
            let old_max = len_map.get(&i).unwrap_or(&0);
            let new_max = max(*old_max, actual_len);
            len_map.insert(i, new_max);
        })
    }
    for record in &records {
        print!("|");
        record.iter().enumerate().for_each(|(i, field)| {
            let actual_len = UnicodeWidthStr::width_cjk(field);
            let max_len = len_map.get(&i).unwrap_or(&0);
            print!(" {0}{1:<2$} |", field, "", max_len - actual_len)
        });
        println!()
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
