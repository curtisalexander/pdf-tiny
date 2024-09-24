# pdf-tiny
:crab: Rust wrapper around :ghost: GhostScript to reduce the size of PDFs

## Usage
```shell
pdf-tiny --path input_file.pdf
```

The output file (the smaller version of the PDF) is written to the same directory as the input file, appending `_tiny` to the filename.

## Requirements
Requires [GhostScript](https://ghostscript.com/releases/gsdnld.html) be installed.  Further requires GhostScript be installed within `C:\Program Files\gs` (using the default installation options).  If the installation path of GhostScript is something other than `C:\Program Files\gs` then [adjust](https://github.com/curtisalexander/pdf-tiny/blob/main/src/main.rs#L20) the Rust code accordingly.

## Installation
:warning: Only run the below if you trust this code to be compiled on your machine! :warning:

```shell
cargo install --git https://github.com/curtisalexander/pdf-tiny
```

## How Is This Built?
This is a :crab: Rust wrapper around a set of options to run :ghost: GhostScript from the command line.

The file path to GhostScript is assumed to start with `C:\Program Files\gs` and is [glob](https://docs.rs/glob/latest/glob/) matched to ignore the version number in order to identify the `gswin64c.exe` executable.  Then [xshell](https://docs.rs/xshell/latest/xshell/) is utilized to execute the program.

The GhostScript [incantation](https://github.com/curtisalexander/pdf-tiny/blob/main/src/main.rs#L34) is the following.  Note that the input file is taken from the command line and the output file has `_tiny` appended to the filename.
```shell
gswin64c.exe -sDEVICE=pdfwrite -dCompatibilityLevel=1.4 -dPDFSETTINGS=/screen -dNOPAUSE -dQUIET -dBATCH -sOutputFile=output_file.pdf input_file.pdf
```

## Compatibility
`pdf-tiny` only works on Windows due to [hard coding](https://github.com/curtisalexander/pdf-tiny/blob/main/src/main.rs#L20) the filepath for GhostScript.  It could easily be adapted for other operating systems.

## Why Build This?
- Opportunity to experiment with `xtask`
- I dislike large PDFs &mdash; they crash my printer

## Inspired By
[SmallerPDF](https://github.com/crissNb/SmallerPDF) by [crissNb](https://github.com/crissNb)