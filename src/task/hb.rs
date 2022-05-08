//! This module contains the main function of computing hydrogen bonds.

use crate::task::cov::find_cov_oneatom;
use crate::task::{get_angle, get_distance_pbc};
use crate::{Atom, Frame};
use std::f64::consts::PI;
use std::error::Error;
use std::fs;
use std::io::Write;

/// This function computes the average HBs on each water molecule in one frame.
pub fn compute_HBs_oneframe(frame: &Frame) -> Result<f64, Box<dyn Error>> {
    //  ------Collect the information of O and H in this frame------
    let cell: &Vec<f64> = &frame.cell;
    let mut coord_O: Vec<&Atom> = vec![];
    let mut coord_H: Vec<&Atom> = vec![];
    //let numb_O = frame.atom_numb[0] as usize;
    //let numb_H = frame.atom_numb[1] as usize;
    for i in frame.atom.iter() {
        if i.type_name == "O" || i.type_name == "1" {
            coord_O.push(i)
        }
        if i.type_name == "H" || i.type_name == "2" {
            coord_H.push(i)
        }
    }

    // ------Main loop to compute #HBs------
    let mut total_numb_HB: i32 = 0;

    for atom_o1 in coord_O.iter() {
        //  let mut numb_accept: i32 = 0;
        //  let mut numb_donate: i32 = 0;
        let mut numb_HB: i32 = 0;
        for atom_o2 in coord_O.iter() {
            if atom_o1.index == atom_o2.index {
                continue;
            }
            //  println!("atom1: {:?} \n atom2: {:?}", atom_o1, atom_o2);
            if get_distance_pbc(atom_o1.coordination, atom_o2.coordination, cell) < 3.5 {
                //  compute #HB_accept
                let neighbour_o2: Vec<&Atom> = find_cov_oneatom(atom_o2, &coord_H, &cell)?;
                for atom_h in neighbour_o2.iter() {
                    if get_angle(
                        atom_o1.coordination,
                        atom_o2.coordination,
                        atom_h.coordination,
                        &cell,
                    ) < (PI / 6.0)
                    {
                        //  numb_accept += 1;
                        numb_HB += 1;
                    }
                }
                //  compute #HB_donate
                let neighbour_o1: Vec<&Atom> = find_cov_oneatom(atom_o1, &coord_H, &cell)?;
                for atom_h in neighbour_o1.iter() {
                    if get_angle(
                        atom_o2.coordination,
                        atom_o1.coordination,
                        atom_h.coordination,
                        &cell,
                    ) < (PI / 6.0)
                    {
                        //  numb_donate += 1;
                        numb_HB += 1;
                    }
                }
            }
            //if numb_HB >= 4 { continue }
        }

        //if numb_accept >= 4 || numb_donate >= 4 {
        //    panic!("#HB_accept is {} or #HB_donate is {} for atom {:?}", numb_accept, numb_donate, atom_o1);
        //}
        //println!("{:?}", numb_HB);

        total_numb_HB += numb_HB;
    }
    /*
    for atom_o1 in coord_O.iter() {
        //let mut numb_accept: i32 = 0;
        //let mut numb_donate: i32 = 0;
        let mut numb_HB    : i32 = 0;
        for atom_h in coord_H.iter() {
            let dist_cov: f64 = get_distance_pbc(atom_o1.coordination, atom_h.coordination, &cell);
            if dist_cov <= 1.3 {
                for atom_o2 in coord_O.iter() {
                    let dist_oo: f64 = get_distance_pbc(atom_o1.coordination, atom_o2.coordination, &cell);
                    if dist_oo < 3.5 && atom_o1.index != atom_o2.index {
                        if get_angle(atom_h.coordination, atom_o1.coordination, atom_o2.coordination, &cell) < 30.0 {
                            numb_HB += 1;
                        }
                    }
                }
            } else if dist_cov > 1.3 && dist_cov < 4.5 {
                for atom_o2 in coord_O.iter() {
                    if get_distance_pbc(atom_o2.coordination, atom_h.coordination, &cell) <= 1.3 {
                        if get_distance_pbc(atom_o1.coordination, atom_o2.coordination, &cell) < 3.5 {
                            if get_angle(atom_h.coordination, atom_o2.coordination, atom_o1.coordination, &cell) < 30.0 {
                                numb_HB += 1;
                            }
                        }
                    }
                }

            }
        }
        total_numb_HB += numb_HB;
    }*/

    let avg_numb_HB: f64 = total_numb_HB as f64 / coord_O.len() as f64;
    //println!("{:?}", avg_numb_HB);
    Ok(avg_numb_HB)
}

/// This function collects all the average HBs of each frame and give an answer.
pub fn compute_HBs(system: &Vec<Frame>, output: &str) -> Result<(), Box<dyn Error>> {
    let mut o = fs::File::create(&output).unwrap();
    let mut avg_numb_HB: f64 = 0.0;
    for i in system.iter() {
        let numb_HB_oneframe: f64 = compute_HBs_oneframe(i)?;
        println!("{}  {}", &i.frame_idx, &numb_HB_oneframe);
        o.write((format!("{:.4}  {:.8}", i.frame_idx, numb_HB_oneframe) + "\n").as_bytes())
            .expect("write hb to file failed");
        avg_numb_HB += numb_HB_oneframe;
    }
    avg_numb_HB = avg_numb_HB / system.len() as f64;
    println!("{:?}", avg_numb_HB);
    o.write( format!("#avg {:.8}", avg_numb_HB).as_bytes()  ).expect("write avg hb to file failed");
    Ok(())
}
