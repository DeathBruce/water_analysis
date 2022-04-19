//! to be done
//! 
use std::error::Error;
use std::fs::File;
use std::fs;
use std::io::Write;


pub fn qe2xdatcar(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(input)?;
    let mut o = fs::File::create(&output).unwrap();
    let (mut lx, mut ly, mut lz) = (0.0, 0.0, 0.0);
    let mut cell: [f64;3] = [0.0;3];
    let mut configuration = 1;
    for (i,line) in contents.lines().enumerate() {
        if i < 2 {
            o.write((line.to_owned()+"\n").as_bytes()).expect("Write failed!");
        } else if i == 2 {
            o.write((line.to_owned()+"\n").as_bytes()).expect("Write failed!");
            let mut tmp: Vec<&str> = line.split_whitespace().collect();
            lx = tmp[0].parse::<f64>().unwrap();
        } else if i == 3 {
            o.write((line.to_owned()+"\n").as_bytes()).expect("Write failed!");
            let mut tmp: Vec<&str> = line.split_whitespace().collect();
            ly = tmp[1].parse::<f64>().unwrap();
        } else if i == 4 {
            o.write((line.to_owned()+"\n").as_bytes()).expect("Write failed!");
            let mut tmp: Vec<&str> = line.split_whitespace().collect();
            lz = tmp[2].parse::<f64>().unwrap();
            cell = [lx, ly, lz];
        } else if i == 5 || i == 6 {
            o.write((line.to_owned()+"\n").as_bytes()).expect("Write failed!");
        } else if i >=7 {
            if line.split_whitespace().collect::<Vec<&str>>().len() == 2 {
                println!("processing configuration : {}", configuration);
                o.write( (format!("Direct  configuration= {}\n", configuration))
                        .as_bytes() ).expect("Write failed!");
                configuration += 1; 
                continue
            } else if line.split_whitespace().collect::<Vec<&str>>().len() == 3 {
                let tmp: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
                let x = tmp[0].parse::<f64>().unwrap()*0.529;
                let y = tmp[1].parse::<f64>().unwrap()*0.529;
                let z = tmp[2].parse::<f64>().unwrap()*0.529;
                let mut xyz = [x,y,z];
                for j in 0..3 {
                    while xyz[j] < 0.0 { xyz[j] += cell[j] }
                    while xyz[j] >= cell[j] { xyz[j] -= cell[j] }
                }
                o.write( (format!("  {:.8}  {:.8}  {:.8}\n", 
                            xyz[0]/lx, xyz[1]/ly, xyz[2]/lz))
                            .as_bytes() ).expect("Write failed!");
            }
        }
    }

    

    //o.write((format!("{:.4}  {:.8}", r[i], gr[i]) + "\n").as_bytes())
    //.expect("write rdf to file failed");
    Ok(())
}