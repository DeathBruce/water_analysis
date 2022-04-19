//! # MSD
//! This module computes the mean square displacement.
//!
//! D = \lim_{t\rightarrow \infinity} \frac{1}{6Nt}
//!     \left\angle \sum_{j=1}^{N} \[ r_j(t)-r_j(0) \]^2 \right\angle
//!
//! Taskoption should be #element #startstep #stopstep #step of step.
//! 
//! An example command to run this process is 
//! ```
//! execfile ./XDATCAR vasp/xdatcar 1 2000 1 msd O xyz 1 2000 1 ./msd.out
//! ```

use crate::{Atom, Frame};
use crate::task::get_distance_pbc;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

/// Compute the average msd
/// 
/// msdopt = vec!["elementA", "direction_type", "stepstart", "d step", "max step"];
/// 
/// "direction_type" should be one of ['xyz','xy','xz','yz','x','y','z']
pub fn msd(system: &Vec<Frame>, msdopt: &Vec<String>, output: &String) -> Result<(), Box<dyn Error>> {
    // load task option
    let type_name: &String = &msdopt[0];                     // "elementA"
    let direction: Vec<usize> = match &msdopt[1] as &str {
        "xyz" => vec![0,1,2],
        "xy"  => vec![0,1],
        "xz"  => vec![0,2],
        "yz"  => vec![1,2],
        "x"   => vec![0],
        "y"   => vec![1],
        "z"   => vec![2],
        _     => panic!("Wrong direction_type, please make sure direction_type 
                         is in ['xyz','xy','xz','yz','x','y','z']!"),
    };
    let stepstart: i32 = msdopt[2].parse::<i32>().unwrap();  // start of frame_step
    let stepstop: i32 = msdopt[3].parse::<i32>().unwrap();   // end of frame_step
    let dstep: i32 = msdopt[4].parse::<i32>().unwrap();      // step of frame_step
    // load system information
    let cell: &Vec<f64> = &system[0].cell;                   // [Lx, Ly, Lz]
    let cutoff1: f64 = 0.7*cell[0]  ;                        // 
    let _cutoff2: f64 = 0.25*cell[0] ;                       // 
    let atom_type: &Vec<String> = &system[0].atom_type;      // ["elementA", "elementB"]
    let atom_numb: &Vec<i32> = &system[0].atom_numb;         // ["# of eleA", "# of eleB"]
    let natom: i32 = atom_numb.iter().sum::<i32>();          // number of atom in one frame
    let mut n: i32 = 0;                                      // number of elementA
    
    for i in 0..atom_type.len() {
        if &atom_type[i] == type_name {
            n = atom_numb[i];
        }
    }
    // It seems like for nvt only? because the average number is divided at the end.
    let mut o = fs::File::create(&output).unwrap();
    for dk in (stepstart..stepstop).filter(|x| ((x-stepstart)%dstep == 0) ) { // loop for frame length

        println!("Processing for dk = {}, please wait...", dk);
        let mut count:i32 = 0;                       // count the loop times
        let mut average_box: Vec< [f64;3] > = 
                    vec![ [0.0;3] ; n as usize];     // count average move box
        let mut total_sd: f64 = 0.0;                 // recoord total_sd in interval dk

        let frame_number: usize = system.len() - dk as usize;
        let mut drp: Vec< Vec< [f64;3] > > = 
                  vec![ vec![ [0.0; 3] ; n as usize ] ; frame_number ];
        let mut nbox: Vec< Vec< [f64;3] > > = 
                  vec![ vec![ [0.0; 3] ; n as usize ] ; frame_number ];
        //  nbox[k][i][j] , k for frame, i for atom, j for x, y, z

        for k in 0..(system.len()-dk as usize) {                      // loop for start frames

            let mut sd_oneframe: f64 = 0.0;

            for i in 0..natom as usize {                           // loop for atoms

                let mut dr: [f64;3] = [0.0;3];

                if &system[k].atom[i].type_name == type_name {
                    
                    for &j in direction.iter() {  // loop for x, y, z
                        let tmp_dr = &system[k+dk as usize].atom[i].coordination[j] 
                                   - &system[k].atom[i].coordination[j];
                        dr[j] = tmp_dr + nbox[k][i][j]*cell[j];
                        //  Adjust the parameter: nbox
                        if (dr[j]-drp[k][i][j]).abs() > cutoff1 {
                            if dr[j]-drp[k][i][j] > 0.0 {
                                nbox[k][i][j] -= 1.0;
                            } else {
                                nbox[k][i][j] += 1.0;
                            }
                            dr[j] = tmp_dr + nbox[k][i][j]*cell[j];
                        }
                        //  Situations that can go wrong
                        // if (dr[j]-drp[k][i][j]).abs() > cutoff2 {
                        //     eprintln!("dr-drp too large");
                        // }
                        average_box[i][j] += nbox[k][i][j];
                        drp[k][i][j] = dr[j];
                    }
                }
                sd_oneframe += dr[0].powi(2) + dr[1].powi(2) + dr[2].powi(2);
            }
            total_sd += sd_oneframe/n as f64;
            count += 1;
        }
        let msd = total_sd / count as f64;
        //println!("for dk = {}, total_sd = {}, count = {}, msd = {}", dk, total_sd, count, msd);
        o.write((format!("{:.4}  {:.8}", dk, msd)+"\n").as_bytes() )
            .expect("write msd to file failed");
    }
    Ok(())
}

