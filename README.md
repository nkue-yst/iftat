# iftat
Command line tool to convert image file format, get meta-data, etc...

## Description
### Functions
1. Get and output image file meta-data.
2. Convert input image file (png, jpeg, etc...) to another format image.

## Usage
```
Usage: iftat [OPTION]... [FILE]...

  [FILE]
  Path to image file
  Use all image file if FILE is directory.

  [OPTION]
  -d        Print image file meta-data
  -c        Convert input image file to another format image
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

$ iftat -c jpeg Image.png

$ ls
Image.jpeg Image.png
```
