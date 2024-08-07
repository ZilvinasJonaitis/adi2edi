extern crate pest;
#[macro_use]
extern crate pest_derive;

mod cli; // bring cli.rs module into scope
use crate::cli::CliArgs;
use crate::cli::PathBuf;
use crate::cli::clapParser;

mod reg1test; // bring reg1test.rs module into scope
mod utils; // bring utils.rs module into scope
mod converter; // bring converter.rs module into scope
use crate::converter::convert_to_reg1test;

//use clap::builder::OsStr;
use pest::Parser;
use std::process;
use std::str::FromStr;
use strip_bom::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::fs;
use lazy_static::lazy_static;
use regex::Regex;

// Global bool variable of Atomic type shared between main() and convert_to_reg1test()
static SKIP_REMARKS: AtomicBool = AtomicBool::new(false);

lazy_static! {
    static ref RE_PBAND: Regex = Regex::new(r"PBand=(\d+,*\d+\s\w+)").unwrap();
}


#[derive(Parser)]
#[grammar = r"adi.pest"]
pub struct AdiParser;


// fn change_file_name(path: impl AsRef<Path>, name: &str) -> PathBuf {
//     let path = path.as_ref();
//     let mut result = path.to_owned();
//     result.set_file_name(name);
//     if let Some(ext) = path.extension() {
//         result.set_extension(ext);
//     }
//     result
// }

fn main() -> std::io::Result<()> {
    let args = CliArgs::parse();
    let adi_file; //: PathBuf = Default::default();
    let mut edi_file: PathBuf = Default::default();
    let save_to_file;

    // Validate name of ADI file
    if let Some(s) = args.infile {
        if !s.is_file() {
            eprintln!("ERROR: input file not found");
            process::exit(0);
        }
        if let Some(e) = s.extension() {
            if e != "adi" {
                eprintln!("ERROR: input file extension is incorrect");
                process::exit(0);    
            }
        } else {
            eprintln!("ERROR: input file without .adi extension");
            process::exit(0);
        }

        // Input file name is correct
        adi_file = s.clone();
    } else {
        eprintln!("ERROR: input file not specified");
        eprintln!("\nUsage: adi2edi.exe [OPTIONS]");
        eprintln!("\nFor more information, try '--help'.");
    process::exit(0);
    }
    
    // Validate name of EDI file
    if let Some(s) = args.outfile
    {
        if let Some(e) = s.extension() {
            if e != "edi" {
                eprintln!("ERROR: output file extension is incorrect");
                process::exit(0);
            }
        } else {
            eprintln!("ERROR: output file without .edi extension");
            process::exit(0);
        }

        // Output file name is correct       
       edi_file = s.clone();
       // Save results to file
       save_to_file = true;
    }
    else // output file not specified
    {
        if args.to_file {
            // Create output file from input file with extension .edi
            edi_file.clone_from(&adi_file);
            edi_file.set_extension("edi");
            // Save results to file
            save_to_file = true;
        }
        else {            
            // Neither output file nor -f flag is specified; output results to terminal
            save_to_file = false;
        }
        
    }
    
    // println!("Input filename: {:?}", adi_file.to_str().unwrap());
    // println!("Output filename: {:?}", edi_file.to_str().unwrap());
    // println!("Include remarks: {}", !args.skip_remarks);

    SKIP_REMARKS.store(args.skip_remarks, Ordering::Relaxed);

    let unparsed_string = fs::read_to_string(adi_file.to_str().unwrap()).unwrap_or_else(|err| {
        eprintln!("ERROR: cannot open adi file: {}", err);
        process::exit(0);
    });

    // Run ADI parser and if successful collect in 'parse_result'
    match AdiParser::parse(Rule::adi, unparsed_string.strip_bom()) {
        Err(parse_error) => {
            eprintln!("ERROR: cannot parse adi file: {}", parse_error);
            process::exit(0);
        }
        Ok(parse_result) => {
            // Run ADI to Reg1test (EDI) converter and save results in string 'reg1test_output'
            let reg1test_output = convert_to_reg1test(parse_result).unwrap();
            if save_to_file {
                // Split results into band sections
                let band_section: Vec<&str> = reg1test_output.trim().split("\n\n").collect();
                if band_section.len() > 1 {
                    // Get stem of EDI file
                    let stem = edi_file.file_stem().unwrap();
                    band_section.iter().for_each(|r| {
                        // Parse PBand value from section string using Regex expression
                        match RE_PBAND.captures(&r) {
                            Some(cap) => {
                                // Replace comma with underscore and remove spaces
                                let suffix = cap.get(1).unwrap().as_str().replace(",", "_").replace(" ", "");
                                // Create new file name from the stem of EDI file stem and append suffix string
                                let filename = String::from_str(stem.to_str().unwrap()).unwrap() + "_" + suffix.as_str();
                                let mut file = edi_file.clone();
                                file.set_file_name(filename);
                                file.set_extension(edi_file.extension().unwrap());
                                fs::write(file.to_str().unwrap(), r).unwrap();
                                println!("Results successfully saved to: {}", file.to_str().unwrap());
                            },
                            None => {
                                eprintln!("ERROR: parsing file suffix from 'PBand' failed.");
                                process::exit(0);    
                            }
                        }
                    }); // for_each
                } else {
                    fs::write(edi_file.to_str().unwrap(), reg1test_output)?;
                    println!("Results successfully saved to: {}", edi_file.to_str().unwrap());
                }
            } else {
                println!("{}", reg1test_output);
            }
        }
    }
    Ok(())
}
