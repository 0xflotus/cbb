# cbb
A base converter

### Examples

`cbb 20 -t 8` -> `24` 

`cbb -b 3` -> `0+--`
## Usage

```console
$ cbb -h
A converter for numbers

USAGE:
    cbb [FLAGS] [OPTIONS] <number>

ARGS:
    <number>    Sets the number

FLAGS:
    -b, --balanced-ternary    Converts decimal to balanced ternary
    -h, --help                Prints help information
    -V, --version             Prints version information

OPTIONS:
    -t, --to <base>    Sets the target base.
```