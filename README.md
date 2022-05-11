# iftat

<img src="https://img.shields.io/badge/-Rust-000000.svg?logo=rust&style=plastic"> <img src="https://img.shields.io/badge/-Linux-FCC624.svg?logo=linux&style=plastic"> <img src="https://img.shields.io/badge/-Windows-0078D6.svg?logo=windows&style=plastic">

[![build](https://github.com/nkue-yst/iftat/actions/workflows/build.yaml/badge.svg)](https://github.com/nkue-yst/iftat/actions/workflows/build.yaml) [![Coverage Status](https://coveralls.io/repos/github/nkue-yst/iftat/badge.svg?branch=main)](https://coveralls.io/github/nkue-yst/iftat?branch=main) [![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/nkue-yst/iftat)](https://rust-reportcard.xuri.me/report/github.com/nkue-yst/iftat)

Command line tool to convert image file format, get meta-data, etc...

## Description
### Functions
1. Get and output image file meta-data.
2. Convert input image file (png, jpeg, etc...) to another format image.

## Usage
```
USAGE:
    iftat [OPTIONS] <FILE>...

ARGS:
    <FILE>...    

OPTIONS:
    -d, --data       Print image file meta data (Default)
    -h, --help       Print help information
    -j, --jpeg       Convert input image file to jpeg file
    -p, --png        Convert input image file to png file
    -V, --version    Print version information
```

### Get and output meta-data
```
$ iftat -d Image.png
Format : PNG
Width  : 1920
Height : 1080
Size   : 42.3 MiB
Updated: 2022/04/20
```

### Convert input image file to another format image
```
$ ls
Image.png

$ iftat -j Image.png

$ ls
Image.jpeg Image.png
```
