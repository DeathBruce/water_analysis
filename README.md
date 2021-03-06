<span style="font-size:larger;">Water_analysis 0.2 Manual</span>
========
This program is mainly used for water analysis. For now, trajectories in LAMMPS, VASP and QE are supported.

# Installation
Make sure you have rust installed (see https://www.rust-lang.org/).

```bash
tar -zxvf water_analysis.tar
cd water_analysis
cargo build
```
If you are in an offline environment, to compile this program you can use the offline package at `/path/to/water_analysis/vendor`. To do this, one needs to create a file:
```bash
vi ~/.cargo/config
```
then write:
```rust
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "/path/to/water_analysis/vendor"
```
and build the program.

Then an temporary executable file will be created at  `/path/to/water_analysis/target/debug/water_analysis`. One could use `cargo build --release` to compile a release version at `/path/to/water_analysis/target/release/water_analysis`, which will greatly enhance the performance of the program.

# Usage
Input file can be:

| Software | format  | multi frames | format key     |
| ---      | ---     | ---          | ---            |
| lammps   | dump    |    True      | 'lammps/traj'  |
| vasp     | POSCAR  |    False     | 'vasp/poscar'  |
| vasp     | XDATCAR |    True      | 'vasp/xdatcar' |
| qe       | log     |    True      | 'qe/traj'      |

Note that element name is different for different format (for lammps usually is "1" "2").

For now, to use qe file, one needs to add an title like:
```
    system

    1.00

      a 0 0

      0 b 0

      0 0 c

    A B C D

    1 2 3 4
```
into the qe.pos file (in VASP units Angstrom).


## Task:
- [Normal task](#normal-task)
  - [covalence bond](#cov)
  - [average hydrogen bond](#hb)
  - [mean squared displayment](#msd)
  - [radial distribution function](#rdf)
  - [tetrahedral order parameter](#q)
- [Convert task](#convert-task)
  - [xdatcar_joint](#xdatcar-joint)
  - [qe2xdatcar](#qe2xdatcar)

One basic example of using this program should be like:
```bash
execfile --in <input file> --infmt <file fmt> --task <task name> --frameopt <FrameStart FrameStop FrameStep> --taskopt <taskopt1 taskopt2 ...> --out <output file>
```
or you can use:
```bash
execfile --help
```

to show some help information.

Note that *--frameopt* *--taskopt* are not always needed (such as cov and hb task).


# normal task

## cov
Compute covalence bond for water molecule. *--taskopt* is not needed.


## hb
Compute Hydrogen bonds (HBs) for each frame and the average HBs. *--taskopt* is not needed.

## msd
Compute mean squared displayment for specific element. An typical *--taskopt* looks like `"element type startframe stopframe step"`.*type* can be one of [*"xyz", "xy", "xz", "yz", "x", "y", "z"*]. An example looks like:
```bash
execfile --in ./a.xdatcar --infmt vasp/xdatcar --frameopt "1 10000 10" --task msd --taskopt "O xyz 1 5000 200" --out ./rdf.dat
```

## rdf
Compute radial distribution function (rdf) for specific elements. An typical *--taskopt* looks like `"elementA elementB cutoff num_of_bins"`. An example looks like:
```bash
execfile --in ./a.xdatcar --infmt vasp/xdatcar --frameopt "1 10000 10" --task rdf --taskopt "O O 6 240" --out ./rdf.dat
```
## q
Comput tetrahedral order parameter for pure water. An typical example looks like:
```bash
execfile --in ./a.xdatcar --infmt vasp/xdatcar --frameopt "1 10000 10" --task q --out ./rdf.dat
```


# Convert task


## xdatcar_joint
Joint two different `XDATCAR` file in one file. An example looks like:
```bash
execfile --in ./first.xdatcar --infmt vasp/xdatcar --task convert --taskopt ./second.xdatcar --out ./XDATCAR
```

## qe2xdatcar
Convert file in 'qe/traj' to 'vasp/xdatcar'. An example looks like:
```bash
execfile --in ./qe.pos --infmt qe/traj --task convert --taskopt qe2xdatcar --out ./some.xdatcar
```


# To be done

 * Dpgen will generate lots of lammps file at `01.model.devi`, how to load these files at one time like vmd?
