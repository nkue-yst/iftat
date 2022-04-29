# iftat
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
