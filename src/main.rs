extern crate structopt;

use std::path::{Path, PathBuf};
use structopt::StructOpt;
use std::{io, fs};
use ansi_term::Colour::RGB;

#[derive(Debug)]
struct Hex {
    content: Vec<u8>,
    color_flag: bool,
}

impl Hex {
    fn new<T: AsRef<Path>>(file: T, color_flag: bool) -> io::Result<Self> { //check documentation for reference
        let content = fs::read(file)?;
        
        Ok(Self {
            content,
            color_flag,
        })
    }

    fn match_color(&self, byte: u8, src: &str) {
        if self.color_flag {
            let color = match byte {
                0 => RGB(120, 120, 120),
                1..=31 => RGB(255, 150, 0),
                32..=126 => RGB(0, 200, 100),
                127..=160 => RGB(100, 0, 100),
                _ => RGB(50, 100, 100),
            };

            print!("{}", color.paint(src));
        } else {
            print!("{}", src);
        }
    }

    fn show(&self) {
        println!("╔══════════╦═════════════════════════════════════════════════╦══════════════════╗");
        println!("║ Offset   ║ 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F ║ Content          ║");
        println!("╠══════════╬═════════════════════════════════════════════════╬══════════════════╣");
        
        let mut offset = 0;

        self.content.chunks(16).for_each(|chunk| {
            print!("║ {:08X} ║ ", offset);
            
            let padder = 16 - chunk.len();

            chunk.iter().for_each(|b| {
                let byte = format!("{:02X}", b);

                self.match_color(*b, &byte);

                print!(" ");
            });

            print!("{:p$}║ ", "", p = padder * 3); //padding

            chunk.iter().for_each(|b| {
                let byte = match b {
                    0..=31 | 127..=160 => ".".to_string(),
                    _ => (*b as char).to_string(),
                };

                self.match_color(*b, &byte);
            });

            println!("{:p$} ║", "", p = padder); //padding

            offset += 0x10;
        });

        print!("╚══════════╩═════════════════════════════════════════════════╩══════════════════╝")
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "hexview", about = "A hex viewer.")]
struct Opt {
    ///Colors the console output.
    #[structopt(short = "c", long = "color")]
    color: bool,

    ///Specify the input file's path.
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    let _ = ansi_term::enable_ansi_support();

    let opt = Opt::from_args();

    let hex = Hex::new(&opt.input, opt.color);

    match hex {
        Ok(c) => c.show(),
        _ => eprintln!("{} Couldn't find {:?}.", RGB(255, 0, 0).paint("error:"), opt.input),
    }
}
