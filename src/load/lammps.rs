//! This module contains functions that read files in lammps format,
//! including lammpstrj.


use std::error::Error;
use std::fs;
use crate::{Atom, Frame};

/// read from lammpstrj and return vec![Frame1, Frame2, ...]
/// For now, this function can only load for nvt.
pub fn read_lammpstrj(filename: &str, frameopt: &Vec<&str>) -> Result<Vec<Frame>, Box<dyn Error>> {
    //  read from POSCAR and return vec![Frame]

    //  ------Collect the basic information of the frame---------
    //  Every frame has 9+natom lines.
    //  ITEM: TIMESTEP
    //  ....
    //  1 1 x y z
    //  ITEM: TIMESTEP ...
    let contents = fs::read_to_string(filename)?;
    let mut lines = vec![];
    for line in contents.lines() {
        lines.push(line);
    }
    let natom: i32 = lines[3].parse::<i32>().unwrap();
    let lx: f64 = lines[5].split_whitespace()
                    .collect::<Vec<&str>>()[1].parse::<f64>().unwrap();
    let ly: f64 = lines[6].split_whitespace()
                    .collect::<Vec<&str>>()[1].parse::<f64>().unwrap();
    let lz: f64 = lines[7].split_whitespace()
                    .collect::<Vec<&str>>()[1].parse::<f64>().unwrap();
    let cell: Vec<f64> = vec![lx, ly, lz];
    let mut atom_type: Vec<String> = vec![];
    let mut atom_numb: Vec<i32> = vec![];

    //  --------Get atom_type and atom_numb from frame 1------
    // println!("Getting atom_type and atom_numb from frame 1.");
    let mut i: i32 = 9;
    while lines[i as usize] != "ITEM: TIMESTEP" {
        let line = lines[i as usize].split_whitespace().collect::<Vec<&str>>();
        if atom_type.contains(&line[1].to_string()) {
            for (i, tmp_type) in atom_type.iter().enumerate() {
                if &line[1] == tmp_type {
                    atom_numb[i] += 1;
                }
            }
        } else {
            atom_type.push(line[1].to_string());
            atom_numb.push(1)
        }
        i += 1;
    }
    
    if atom_numb.iter().sum::<i32>() != natom {
        panic!("Atom number of frame 1 is not equal to natom!");
    }
    // println!("basic information is collected!");

    //  ----------Collect the coordination of the frame----------
    let start : i32 = frameopt[0].parse::<i32>().unwrap();
    let stop  : i32 = frameopt[1].parse::<i32>().unwrap();
    let step  : i32 = frameopt[2].parse::<i32>().unwrap();
    let mut i: i32 = 0 + (start-1)*(9+natom);
    let mut system: Vec<Frame> = vec![];
    let mut frame_idx: i32 = start;
    while i < lines.len().try_into().unwrap() {
        let mut line: &str = lines[(i as usize)];
        //println!("{}", i);
        if frame_idx > stop {
            break;
        }

        if line == "ITEM: TIMESTEP" {  //  seems like unnecessary
            //println!("i is {}, ITEM: TIMESTEP", i);
            //frame_idx += 1;
            i += 9;
        } else {
            let mut coord: Vec<Atom> = vec![];

            for j in 0..natom {
                let mut xyz = lines[((i+j) as usize)]
                            .split_whitespace().collect::<Vec<&str>>();
                let atom: Atom = Atom {
                    index: xyz[0].parse::<i32>().unwrap(),
                    type_name: xyz[1].to_string(),
                    coordination: [ xyz[2].parse::<f64>().unwrap(),
                                   xyz[3].parse::<f64>().unwrap(),
                                   xyz[4].parse::<f64>().unwrap() ],
                };
                coord.push(atom);
            }
            system.push(Frame {
                frame_idx: frame_idx,
                cell: cell.clone(),
                atom_type: atom_type.clone(),
                atom_numb: atom_numb.clone(),
                natom: natom,
                atom: coord,
            });
            //println!("end of a frame {}", i);
            i += (natom + 9) * step - 9 ;
            frame_idx += step;
            //println!("jumped to {}", i);
        }
    }
    // println!("{:?}", system);
    Ok(system)
}