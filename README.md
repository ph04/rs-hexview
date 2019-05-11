# rs-hexview
A CLI hexviewer written in Rust.

In `src/main.rs` you can find the source of the program, which uses **structopt** to parse line arguments, and **ansi_term** to color the output with the `-c` flag.

Example of usage:

`hexview.exe --input [INPUT FILE PATH]`

![alt text](https://raw.githubusercontent.com/ph04/rs-hexview/master/example.jpg)

Optional `-c` flag to color the output as follows:

**GREY**: null byte;

**ORANGE**: non-printable characters;

**LIGHT GREEN**: printable characters;

**PURPLE**: non-printable characters in the extended ASCII range;

**DARK GREEN**: printable in the extended ASCII range.

# License
This project is under the GNU GPLv3 license.
