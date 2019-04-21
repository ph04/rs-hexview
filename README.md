# rs-hexview
A CLI hexviewer written in Rust.

In `src/main.rs` you can find the source of the program, which uses **StructOpt** to parse line arguments, and **ansi_term** to color the output with the `-c` flag.

Example of usage:
`hexview.exe --input [INPUT FILE PATH]`

![alt text](https://drive.google.com/open?id=1lkLMVvDGNipIQp3RW1cv5BR3WpctaV8r)

Optional `-c` to color the output as follows:

**GREY**: null byte;

**ORANGE**: non-printable characters;

**LIGHT GREEN**: printable characters;

**PURPLE**: non-printable characters in the extended ASCII range.

**DARK GREEN**: printable in the extended ASCII range;
