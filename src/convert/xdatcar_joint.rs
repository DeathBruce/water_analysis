//! This module can joint two xdatcar into one xdatcar.
//! 
use std::error::Error;
use std::fs::File;
use std::fs;
use std::io::Write;

pub fn joint(input1: &str, input2: &Vec<&str>, output: &str) -> Result<(), Box<dyn Error>> {
    
    let contents1 = fs::read_to_string(input1)?;
    if input2.len() != 1 {panic!("xdatcar_joint only needs one arg, which is the filename to use")}
    let contents2 = fs::read_to_string(input2[0])?;
    let mut o = fs::File::create(&output).unwrap();
    let mut frame_idx: i32 = 1;

    for (i, line) in contents1.lines().enumerate() {
        if line.chars().next().unwrap() == 'D'{
            o.write( (format!("Direct  configuration= {}", frame_idx) +"\n").as_bytes() ).expect("Write failed!");
            frame_idx += 1;
        } else {
            o.write( (line.to_owned()+"\n").as_bytes() ).expect("Write failed!");
        }
    }

    for (i, line) in contents2.lines().enumerate() {
        if line.chars().next().unwrap() == 'D' && i > 6 {
            o.write( (format!("Direct  configuration= {}", frame_idx)+"\n").as_bytes()).expect("Write failed!");
            frame_idx += 1;
        } else if i > 6 {
            o.write((line.to_owned()+"\n").as_bytes()).expect("Write failed!");
        }
    }

    Ok(())
}