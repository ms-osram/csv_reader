# csv_reader

## Statistics (my machine)

| Language   | User Time | Maximum resident set size |
|:-----------|----------:|--------------------------:|
| Python     |    19.41s |                 940696 kb |
| JavaScript |    12.84s |                 906312 kb |
| Rust       |     2.23s |                 102052 kB |

User time represents the amount of time the CPU runs the program.

Maximum resident set size represents the maximum amount of RAM required to run the program.

For both user time and maximum resident set size, lower is better.

## Reproducibility

### Prerequisites

Use a package manager to install these (e.g. `sudo apt install [pkg]`):
- python
- nodejs
- time

Install Rust using the instructions here: http://rustup.rs/

### Generating inputs

Generate the inputs by running `generator.py > input.txt`. You should get a CSV file that is 10 million lines long.

### Running

Run these:

- Python: `/usr/bin/time -v python csv_reader.py`
- JavaScript: `/usr/bin/time -v node csv_reader.js`
- Rust: `rustc csv_reader.rs -O; /usr/bin/time -v ./csv_reader`

Read the lines containing `User time (seconds)` and `Maximum resident set size` to get the total CPU time used and maximum RAM used, respectively.