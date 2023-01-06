# rreplace

This program is inspired by sed, which I've had to use a lot of times. Unlike sed, however, this program is a lot easier to use and the regular expressions are in the format every modern programmer is familiar with.

## Usage

``` sh
git clone https://github.com/wisepythagoras/rreplace && cd rreplace
cargo build --release
```

It only takes 4 arguments or 2 with the input file/string passed via stdin and the output is sent to stdout:

```
rreplace - Change occurences of a string in a file to another
Usage: rreplace <target> <replacement> <INPUT> <OUTPUT>
Usage: cat INPUT | rreplace <target> <replacement> > OUTPUT
```
