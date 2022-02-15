use clap::Clap;
use rriscv::Emulator;
use std::fs;

#[derive(Clap)]
#[clap(version = "0.1", author = "takuchalle <me@takuchalle.dev>")]
struct Opts {
    /// Input binary file
    input: String,
}

fn main() -> std::io::Result<()> {
    let opts = Opts::parse();

    println!("input file: {}", opts.input);

    let mut emu = {
        let bin = fs::read(opts.input)?;
        Emulator::new(bin)
    };

    emu.run();

    Ok(())
}
