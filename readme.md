# sizeplz
A simple asynchronous recursive directory summarisation tool written in rust.

### Usage
```
USAGE:
    sizeplz [FLAGS] [OPTIONS] [PATH]

ARGS:
    <PATH>    The path to the directory to scan

FLAGS:
    -e, --empty      Ignore empty directories / files
    -f, --files      Include the size of files within the output
    -h, --help       Prints help information
    -i, --invert     Invert sorted results
    -t, --tree       Whether the search should show all results.
    -V, --version    Prints version information

OPTIONS:
    -d, --depth <DEPTH>    The recessive depth to scan [default: 1]
    -r, --round <ROUND>    The number of figures to round too. [default: 0]
    -s, --sort <sort>      How the results should be sorted [possible values: size, name, created,
                           modified]
    -u, --unit <UNIT>      The unit of file size to output [possible values: b, kb, mb, gb, tb, pb, auto]
```

### Install
#### Download
Head over to the [releases](https://github.com/SamHDev/sizeplz/releases) page and download a binary for your system.

To install on unix: run this command (requires wget)
```
sudo wget https://github.com/SamHDev/sizeplz/releases/download/0.0.1/sizeplz -O /usr/bin/sizeplz
```

#### Build
Building from source will require the rust chain, which can be installed using [rustup](https://rustup.rs)
```
git clone https://github.com/samhdev/sizeplz.git
cd sizeplz
cargo build
```
