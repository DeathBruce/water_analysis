//! This module contains functions that read files in VASP format,
//! including POSCAR and XDATCAR.
//!
//! The functions are read_poscar() and read read_xdatcar(), respectively.

use std::error::Error;
use std::fs;
//use cgmath::Vector3;

use crate::{Atom, Frame};

/// read from POSCAR and return vec![Frame1, Frame2, ...]
pub fn read_poscar(filename: &str) -> Result<Vec<Frame>, Box<dyn Error>> {
    //  read from POSCAR and return vec![Frame]

    //  ------Collect the basic information of the frame---------
    //  Basic information of POSCAR is contained in tiile


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

    let xyz_type = lines[7].chars().next().unwrap(); // for VASP, 'D' or 'C'
                                                     //  println!("{:?}",xyz_type);

    lines = lines.split_off(8);

    let natom: i32 = atom_numb.iter().sum::<i32>();

    //  ----------Collect the coordination of the frame----------
    let mut coord: Vec<Atom> = vec![];
    let mut atom_index: i32 = 0;
    let mut j = 0;
    for i in 0..natom {
        if atom_index >= atom_numb[j] {
            atom_index -= atom_numb[j];
            j = j + 1
        }
        let mut xyz = lines[(i as usize)].split_whitespace();
        let atom: Atom;
        match xyz_type {
            //  Cartesian
            'C' => {
                atom = Atom {
                    index: i + 1,
                    type_name: atom_type[j].to_string(),
                    coordination: [
                        xyz.next().unwrap().parse::<f64>().unwrap(),
                        xyz.next().unwrap().parse::<f64>().unwrap(),
                        xyz.next().unwrap().parse::<f64>().unwrap(),
                    ],
                }
            }
            'D' => {
                atom = Atom {
                    index: i + 1,
                    type_name: atom_type[j].to_string(),
                    coordination: [
                        xyz.next().unwrap().parse::<f64>().unwrap() * cell[0],
                        xyz.next().unwrap().parse::<f64>().unwrap() * cell[1],
                        xyz.next().unwrap().parse::<f64>().unwrap() * cell[2],
                    ],
                }
            }
            _ => panic!("Cartesian or Direct? Please check your VASP file."),
        }
        coord.push(atom);
        atom_index += 1;
    }

    let system: Vec<Frame> = vec![Frame {
        frame_idx: 1,
        cell: cell,
        atom_type: atom_type,
        atom_numb: atom_numb,
        natom: natom,
        atom: coord,
    }];
    Ok(system)
}

/// read from XDATCAR and return vec![Frame1, Frame2, ...]
/// frameopt in the form of vec!["#start", "#stop", "#step"]
pub fn read_xdatcar(
    filename: &str,
    frameopt: &Vec<&str>,
) -> Result<Vec<Frame>, Box<dyn Error>> {
    //  read from XDATCAR and return vec![Frame1, Frame2, ...]
    //  frameopt in the form of vec!["#start", "#stop", "#step"]

    //  ------Collect the basic information of the frame---------
    //  Basic information of POSCAR is contained in tiile
    //  system
    //  ...
    //  Direct  configuration=  1
    //   x y z
    //  Direct  configuration=  2
    //   ...
    //   ...

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

    let xyz_type = lines[7].chars().next().unwrap(); // for VASP, 'D' or 'C'
                                                     //  println!("{:?}",xyz_type);

    let natom: i32 = atom_numb.iter().sum::<i32>();

    //  ----------Collect the coordination of the frame----------
    let start: i32 = frameopt[0].parse::<i32>().unwrap();
    let stop: i32 = frameopt[1].parse::<i32>().unwrap();
    let step: i32 = frameopt[2].parse::<i32>().unwrap();
    let mut i: i32 = 7 + (start - 1) * (natom + 1);
    let mut system: Vec<Frame> = vec![];
    let mut frame_idx: i32 = 0;
    while i < lines.len().try_into().unwrap() {
        let mut line = lines[(i as usize)].split_whitespace();

        //println!("{}",i);
        if frame_idx > stop {
            break;
        }
        if line.next().unwrap() == "Direct" {
            line.next();
            frame_idx = line.next().unwrap().parse::<i32>().unwrap();
            i += 1;
        } else {
            let mut coord: Vec<Atom> = vec![];
            let mut atom_index: i32 = 0;
            let mut k = 0;    //  index to recoord atom_type
            for j in 0..natom {
                //println!("atom_index: {:?}, natom: {}", atom_index, natom);
                if atom_index >= atom_numb[k] {
                    atom_index -= atom_numb[k];
                    k = k + 1;
                }
                //println!("{:?}", k);
                let mut xyz = lines[((i + j) as usize)].split_whitespace();
                let atom: Atom;
                match xyz_type {
                    'C' => {
                        atom = Atom {
                            index: j + 1, //  Cartesian
                            type_name: atom_type[k].to_string(),
                            coordination: [
                                xyz.next().unwrap().parse::<f64>().unwrap(),
                                xyz.next().unwrap().parse::<f64>().unwrap(),
                                xyz.next().unwrap().parse::<f64>().unwrap(),
                            ],
                        };
                    }
                    'D' => {
                        atom = Atom {
                            index: j + 1, //  Direct
                            type_name: atom_type[k].to_string(),
                            coordination: [
                                xyz.next().unwrap().parse::<f64>().unwrap() * cell[0],
                                xyz.next().unwrap().parse::<f64>().unwrap() * cell[1],
                                xyz.next().unwrap().parse::<f64>().unwrap() * cell[2],
                            ],
                        };
                    }
                    _ => panic!("Cartesian or Direct? Please check your VASP file."),
                }
                coord.push(atom);
                atom_index += 1;
            }
            i += (natom + 1) * step - 1;
            system.push(Frame {
                frame_idx: frame_idx,
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
