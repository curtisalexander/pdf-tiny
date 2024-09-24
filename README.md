# pdf-tiny
:crab: Rust wrapper around :ghost: GhostScript to reduce the size of PDFs

## Usage
```shell
pdf-tiny input_file.pdf
```

Writes output file to same directory as input file, appending `_tiny` to the filename.

## Requirements
Requires [GhostScript](https://ghostscript.com/releases/gsdnld.html) to be installed.  Further assumes that GhostScript is installed within `C:\Program Files` using the default installation options.  [Adjust]() as needed.

## Installation

## How Is It Built?
This is simply a :crab: Rust wrapper around a set of options to run GhostScript from the command line.

The file path to GhostScript is assumed to start with `C:\Program Files\gs` and is [glob](https://docs.rs/glob/latest/glob/) matched to ignore the version number in order to identify the `gswin64c.exe` executable.  Then [xshell](https://docs.rs/xshell/latest/xshell/) is utilized to execute the program.

## Why Build This?
- Opportunity to mess around with `xtask`
- I dislike large PDFs &mdash; they crash my printer

## Inspired By
[SmallerPDF](https://github.com/crissNb/SmallerPDF) by [crissNb](https://github.com/crissNb)