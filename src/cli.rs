pub use clap::Parser as clapParser;
pub use std::path::PathBuf;


#[derive(clapParser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    // Input file (ADI)
    #[arg(index = 1, required = true, help = "ADI file" /*, short, long, value_name = "ADI file"*/)]
    pub infile: Option<PathBuf>,
    
    // Output file (EDI)
    #[arg(index = 2, help = "EDI file (consider -f if not specified)" /*, short, long, value_name = "EDI file"*/)]
    pub outfile: Option<PathBuf>,

    // Default output to file
    #[arg(short = 'f', long = "to-file", help = "output to file(s)")]
    pub to_file: bool,

    // No remarks in EDI file
    #[arg(short = 's', long = "skip-remarks")]
    pub skip_remarks: bool,
}

/*
#[allow(dead_code)]
pub fn parse_args(args: &[String]) -> Result<&str, &str> {
    if args.len() < 2 {
        return Err("ADIF filename is missing");
    }
    let filename: &String = &args[1];
    if !filename
        .to_owned()
        .to_lowercase()
        .ends_with(".adi") {
        return Err("filename extension is not .adi");
    }
    Ok(filename)
}
*/