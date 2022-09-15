# Common Substring Search
This project contains a Rust library (and simple example binary) that can be used to find a common substring of length
k in two input substrings. It also contains benchmarks and analysis code to evaluate and plot the performance of 
different common substring implementations.

## Running the code
To run this project, you first need to install Rust and Cargo. If you also want to generate the plots from the
benchmarks, you will also need to install python and the packages mentioned in `analysis/requirements.txt`. You can
install the required packages by running the command `pip install -r analysis/requirements.txt` from this directory. I
highly recommend using a conda environment from the anaconda project (https://anaconda.org/).

### Running Tests
Once you have Rust and Cargo installed, you can run the tests by running `cargo test` from this directory.

### Running Benchmarks
Once you have Rust and Cargo installed, you can run the benchmarks by running `cargo bench` from this directory. The
benchmarks take several hours to run to completion on my machine.

### Running Simple Example Binary
Once you have Rust and Cargo installed, you can run the simple example binary using the command `cargo run`. It will
read in "War and Peace" and "Anna Karenina" and print out the first common substring found of length 20.

### Generating Plots from Benchmarks
Once you have python and the packages required, you can generate the plots by running `python plots.py` from the
analysis directory.
