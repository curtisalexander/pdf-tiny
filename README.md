# pdf-tiny
:crab: Rust wrapper around :ghost: GhostScript to reduce the size of PDFs

## Usage
```shell
pdf-tiny --path input_file.pdf
```

The output file is written to same directory as the input file, appending `_tiny` to the filename.

## Requirements
Requires [GhostScript](https://ghostscript.com/releases/gsdnld.html) to be installed.  Further assumes that GhostScript is installed within `C:\Program Files` using the default installation options.  [Adjust](https://github.com/curtisalexander/pdf-tiny/blob/main/src/main.rs#L20) as needed.

## Installation
:warning: Only run the below if you trust this code to be compiled on your machine!

```shell
cargo install --git https://github.com/curtisalexander/pdf-tiny
```

## How Is This Built?
This is a :crab: Rust wrapper around a set of options to run GhostScript from the command line.

The file path to GhostScript is assumed to start with `C:\Program Files\gs` and is [glob](https://docs.rs/glob/latest/glob/) matched to ignore the version number in order to identify the `gswin64c.exe` executable.  Then [xshell](https://docs.rs/xshell/latest/xshell/) is utilized to execute the program.

The GhostScript [incantation](https://github.com/curtisalexander/pdf-tiny/blob/main/src/main.rs#L34) is the following.  Note that the input file is taken from the command line and the output file has `_tiny` appended to the filename.
```shell
gswin64c.exe -sDEVICE=pdfwrite -dCompatibilityLevel=1.4 -dPDFSETTINGS=/screen -dNOPAUSE -dQUIET -dBATCH -sOutputFile=output_file.pdf input_file.pdf
```

## Compatibility
Only works on Windows due to [hard coding](https://github.com/curtisalexander/pdf-tiny/blob/main/src/main.rs#L20) the filepath for GhostScript.  Could very easily be adapted for other operating systems.

## Why Build This?
- Opportunity to experiment with `xtask`
- I dislike large PDFs &mdash; they crash my printer

## Inspired By
[SmallerPDF](https://github.com/crissNb/SmallerPDF) by [crissNb](https://github.com/crissNb)