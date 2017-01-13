use std::io;
use std::thread;
use std::time;
use clap;
use device::*;

pub struct Apa102 {
    /// 5-bit grayscale value to apply to all pixels.
    pub grayscale: u8,
}

impl Device for Apa102 {

    fn clock_phase(&self) -> u8 {
        0
    }

    fn clock_polarity(&self) -> u8 {
        0
    }

    fn first_bit(&self) -> FirstBit {
        FirstBit::MSB
    }

    fn write_frame(&self, writer: &mut io::Write, pixels: &[Pixel]) -> io::Result<()> {
        try!(writer.write_all(&[0x00; 4]));
        for pix in pixels {
            try!(writer.write_all(&[0b11100000 | self.grayscale, pix.r, pix.g, pix.b]));
        }
        thread::sleep(time::Duration::new(0, 500_000));
        Ok(())
    }

}

pub fn command<'a, 'b>() -> clap::App<'a, 'b> {
    clap::SubCommand::with_name("apa102")
        .arg(clap::Arg::with_name("grayscale")
             .short("g")
             .long("grayscale")
             .validator(validate_grayscale)
             .default_value("31")
             .help("Set the 5-bit grayscale for all pixels"))
}

pub fn from_command(args: &clap::ArgMatches) -> Box<Device> {
    let gs = args.value_of("grayscale").unwrap();
    Box::new(Apa102{ grayscale: gs.parse::<u8>().unwrap() })
}

fn validate_grayscale(v: String) -> Result<(), String> {
    match v.parse::<u8>() {
        Ok(i) => if i <= 31 {
            Ok(())
        } else {
            Err(format!("Grayscale value out of range: 0 <= {} <= 31", i))
        },
        Err(e) => Err(format!("{}", e)),
    }
}