pub use clap::Parser as clapParser;
pub use std::path::PathBuf;


#[derive(clapParser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    // Name of input file (ADI)
    #[arg(short, long, value_name = "ADI file")]
    pub infile: Option<PathBuf>,
    
    // Name of output file (EDI)
    #[arg(short, long, value_name = "EDI file")]
    pub outfile: Option<PathBuf>,
    
    // No remarks in EDI file
    #[arg(long = "skip-remarks")]
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