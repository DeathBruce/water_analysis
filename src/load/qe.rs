//! This module contains functions that read files in qe format,
//! specificly for pure water for now.
//!
//! The full information of a qe traj is divided into both
//! .pos and .in.
//! To use the functions here, add an title like
//! 
//! 
//!     system
//! 
//!     1.00
//! 
//!       a 0 0
//! 
//!       0 b 0
//! 
//!       0 0 c
//! 
//!     A B C D
//! 
//!     1 2 3 4
//! 
//! 
//! into the qe.pos file (in VASP units Angstrom).

use std::error::Error;
use std::fs;
//use cgmath::Vector3;

use crate::{Atom, Frame};

/// read from qe/traj and return vec![Frame1, Frame2, ...]
/// 
/// The qe file should be added a title like XDATCAR (in Angstrom, not Bohr).
/// frameopt in the form of vec!["#start", "#stop", "#step"]
pub fn read_traj(filename: &str, frameopt: &Vec<&str>) -> Result<Vec<Frame>, Box<dyn Error>> {
    //  ------Collect the basic information of the frame---------
    let contents = fs::read_to_string(filename)?;
    let mut lines = vec![];
    for line in contents.lines() {
        lines.push(line);
    }

    let lx = lines[2]
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<f64>()
        .unwrap();
    let mut ly = lines[3].split_whitespace();
    ly.next();
    let ly = ly.next().unwrap().parse::<f64>().unwrap();
    let mut lz = lines[4].split_whitespace();
    lz.next();
    lz.next();
    let lz = lz.next().unwrap().parse::<f64>().unwrap();
    let cell: Vec<f64> = vec![lx, ly, lz];

    let mut atom_type: Vec<String> = vec![];
    for i in lines[5].split_whitespace() {
        atom_type.push(i.to_string())
    }

    let mut atom_numb: Vec<i32> = vec![];
    for i in lines[6].split_whitespace() {
        atom_numb.push(i.parse().expect("not valid poscar"))
    }

    //  let xyz_type = lines[7].chars().next().unwrap();
    //  println!("{:?}",xyz_type);

    let natom: i32 = atom_numb.iter().sum::<i32>();

    //  ----------Collect the coordination of the frame----------
    let start: i32 = frameopt[0].parse::<i32>().unwrap();
    let stop: i32 = frameopt[1].parse::<i32>().unwrap();
    let step: i32 = frameopt[2].parse::<i32>().unwrap();
    let mut i: i32 = 7 + (start - 1) * (natom + 1);
    let mut system: Vec<Frame> = vec![];
    let mut frame_idx: i32 = start - 1;
    while i < lines.len().try_into().unwrap() {
        //println!("i = {} ; frame_idx = {}",i, frame_idx);
        //println!("lines[i] = {:?}\n", lines[i as usize]);
        //let mut line = lines[(i as usize)].split_whitespace();
        //println!("{}",i);
        if (frame_idx - 1) * step + 1 > stop {
            break;
        }
        //println!("{:?}", line.next().unwrap());
        if (i - 7) % (natom + 1) == 0 {
            frame_idx += 1;
            i += 1;
        } else {
            let mut coord: Vec<Atom> = vec![];
            let mut atom_index: i32 = 0;
            let mut k = 0;
            for j in 0..natom {
                //println!("atom_index: {:?}, natom: {}", atom_index, natom);
                if atom_index >= atom_numb[k] {
                    atom_index -= atom_numb[k];
                    k = k + 1;
                }
                //println!("{:?}", k);
                let mut xyz = lines[((i + j) as usize)].split_whitespace();
                let mut atom = Atom {
                    index: j + 1,
                    type_name: atom_type[k].to_string(),
                    coordination: [
                        xyz.next().unwrap().parse::<f64>().unwrap() * 0.53,
                        xyz.next().unwrap().parse::<f64>().unwrap() * 0.53,
                        xyz.next().unwrap().parse::<f64>().unwrap() * 0.53,
                    ],
                };
                for i in 0..3 {
                    while atom.coordination[i] < 0.0 {
                        atom.coordination[i] += cell[i];
                    }
                    while atom.coordination[i] > cell[i] {
                        atom.coordination[i] -= cell[i]
                    }
                }

                //println!("{:?}", atom);
                coord.push(atom);
                atom_index += 1;
            }
            i += (natom + 1) * step - 1;
            system.push(Frame {
                frame_idx: (frame_idx - 1) * step + 1,
                cell: cell.clone(),
                atom_type: atom_type.clone(),
                atom_numb: atom_numb.clone(),
                natom: natom,
                atom: coord,
            });
        }
    }

    Ok(system)
}

pub fn read_traj_nopbc(filename: &str, frameopt: &Vec<&str>) -> Result<Vec<Frame>, Box<dyn Error>> {
    //  ------Collect the basic information of the frame---------
    let contents = fs::read_to_string(filename)?;
    let mut lines = vec![];
    for line in contents.lines() {
        lines.push(line);
    }

    let lx = lines[2]
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<f64>()
        .unwrap();
    let mut ly = lines[3].split_whitespace();
    ly.next();
    let ly = ly.next().unwrap().parse::<f64>().unwrap();
    let mut lz = lines[4].split_whitespace();
    lz.next();
    lz.next();
    let lz = lz.next().unwrap().parse::<f64>().unwrap();
    let cell: Vec<f64> = vec![lx, ly, lz];

    let mut atom_type: Vec<String> = vec![];
    for i in lines[5].split_whitespace() {
        atom_type.push(i.to_string())
    }

    let mut atom_numb: Vec<i32> = vec![];
    for i in lines[6].split_whitespace() {
        atom_numb.push(i.parse().expect("not valid poscar"))
    }

    //  let xyz_type = lines[7].chars().next().unwrap();
    //  println!("{:?}",xyz_type);

    let natom: i32 = atom_numb.iter().sum::<i32>();

    //  ----------Collect the coordination of the frame----------
    let start: i32 = frameopt[0].parse::<i32>().unwrap();
    let stop: i32 = frameopt[1].parse::<i32>().unwrap();
    let step: i32 = frameopt[2].parse::<i32>().unwrap();
    let mut i: i32 = 7 + (start - 1) * (natom + 1);
    let mut system: Vec<Frame> = vec![];
    let mut frame_idx: i32 = start - 1;
    while i < lines.len().try_into().unwrap() {
        //println!("i = {} ; frame_idx = {}",i, frame_idx);
        //println!("lines[i] = {:?}\n", lines[i as usize]);
        //let mut line = lines[(i as usize)].split_whitespace();
        //println!("{}",i);
        if (frame_idx - 1) * step + 1 > stop {
            break;
        }
        //println!("{:?}", line.next().unwrap());
        if (i - 7) % (natom + 1) == 0 {
            frame_idx += 1;
            i += 1;
        } else {
            let mut coord: Vec<Atom> = vec![];
            let mut atom_index: i32 = 0;
            let mut k = 0;
            for j in 0..natom {
                //println!("atom_index: {:?}, natom: {}", atom_index, natom);
                if atom_index >= atom_numb[k] {
                    atom_index -= atom_numb[k];
                    k = k + 1;
                }
                //println!("{:?}", k);
                let mut xyz = lines[((i + j) as usize)].split_whitespace();
                let atom = Atom {
                    index: j + 1,
                    type_name: atom_type[k].to_string(),
                    coordination: [
                        xyz.next().unwrap().parse::<f64>().unwrap() * 0.529,
                        xyz.next().unwrap().parse::<f64>().unwrap() * 0.529,
                        xyz.next().unwrap().parse::<f64>().unwrap() * 0.529,
                    ],
                };

                //println!("{:?}", atom);
                coord.push(atom);
                atom_index += 1;
            }
            i += (natom + 1) * step - 1;
            system.push(Frame {
                frame_idx: (frame_idx - 1) * step + 1,
                cell: cell.clone(),
                atom_type: atom_type.clone(),
                atom_numb: atom_numb.clone(),
                natom: natom,
                atom: coord,
            });
        }
    }

    Ok(system)
}