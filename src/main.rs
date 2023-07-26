extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fmt;
use std::env;
use std::process;
use std::str::FromStr;
// use std::str::FromStr;
use std::error::Error;
use pest::Parser;
// use pest::iterators::Pair;
// use pest::iterators::Pairs;

#[derive(Parser)]
#[grammar = r"adi.pest"]
pub struct AdiParser;

// use std::collections::HashMap;
use std::fs;

#[allow(dead_code)]
#[derive(Debug)]
struct Reg1testHeader<'a> {
    name: &'a str,
    tdate: &'a str,
    pcall: &'a str,
    pwwlo: &'a str,
    pband: String,
    psect: String,
    pclub: String,
}

impl Default for Reg1testHeader<'_> {
    fn default() -> Self {
        Reg1testHeader {
            name: "REG1TEST;1",
            tdate: "",
            pcall: "",
            pwwlo: "",
            pband: "".to_string(),
            psect: "".to_string(),
            pclub: "".to_string(),
        }
    }
}

impl fmt::Display for Reg1testHeader<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]\n", self.name)?;
        write!(f, "TDate={}\n", self.tdate)?;
        write!(f, "PCall={}\n", self.pcall)?;
        write!(f, "PWWLo={}\n", self.pwwlo)?;
        write!(f, "PBand={}", self.pband)?;
        if self.psect != "" {write!(f, "\nPSect={}", self.psect)?;}
        if self.pclub != "" {write!(f, "\nPClub={}", self.pclub)?;}
        Ok(())
    }
}

impl Reg1testHeader<'_> {
    fn get_band (band: &str) -> String {
        match band.trim() {
            "6m" => return "50 MHz".to_string(),
            "4m" => return "70 MHz".to_string(),
            "2m" => return "144 MHz".to_string(),
            "70cm" => return "432 MHz".to_string(),
            "23cm" => return "1,3 GHz".to_string(),
            "13cm" => return "2,3 GHz".to_string(),
            "9cm" => return "3,4 GHz".to_string(),
            "6cm" => return "5,7 GHz".to_string(),
            "3cm" => return "10 GHz".to_string(),
            "1.25cm" => return "24 GHz".to_string(),
            "6mm" => return "47 GHz".to_string(),
            "4mm" => return "76 GHz".to_string(),
            "2.5mm" => return "120 GHz".to_string(),
            "2mm" => return "144 GHz".to_string(),
            "1mm" => return "248 GHz".to_string(),
            _ => "".to_string(),
        }
    }
}


#[allow(dead_code)]
#[derive(Debug)]
struct Reg1testRemarks<'a> {
    name: &'a str,
    multi_line: Vec<String>,
}

impl Default for Reg1testRemarks<'_> {
    fn default() -> Self {
        Reg1testRemarks {
            name: "Remarks",
            multi_line: Vec::new(),
        }
    }
}

impl fmt::Display for Reg1testRemarks<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.name)?;
        if self.multi_line.len() != 0 {
            for text_line in self.multi_line.iter() {
                write!(f, "\n{}", text_line)?;
            }
        }
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Reg1testQSOs<'a> {
    name: &'a str,
    count: u32,
    qso_records: Vec<Reg1testQSORecord<'a>>,
}

impl Default for Reg1testQSOs<'_> {
    fn default() -> Self {
        Reg1testQSOs { 
            name: "QSORecords",
            count: 0,
            qso_records: Vec::new(),
        }
    }
}

impl fmt::Display for Reg1testQSOs<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{};{}]\n", self.name, self.count)?;
        for qso in self.qso_records.iter() {
            write!(f, "{}", qso)?;
        }
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Reg1testQSORecord<'a> {
    date: &'a str, // YYMMDD, 6 characters, 6
    time: &'a str, // UTC, 4 characters, with leading zeros, 4
    call: &'a str, // 3 to 14 characters, 14
    mode_code: char, // 0 or 1 character, 1
    sent_rst: &'a str, // 0 or 2 or 3 characters, 3
    sent_qso_number: &'a str, // 0 or 3 or 4 characters, with leading zeros, 4
    received_rst: &'a str, // 0 or 2 or 3 characters, 3
    received_qso_number: &'a str, // 0 or 3 or 4 characters, with leading zeros, 4
    received_exchange: &'a str, // 0 or 1 to 6 characters (see also PExch), 6
    received_wwl: &'a str, // 0 or 4 or 6 characters, World Wide Locator, 6
    qso_points: &'a str, // 1 to 6 characters, including bandmultiplier, 6
    new_exchange: &'a str, // 0 or 1 character, "N" if QSO is a new exchange, 1
    new_wwl: &'a str, // 0 or 1 character, "N" if QSO is a new WWL, 1
    new_dxcc: &'a str, // 0 or 1 character, "N" if QSO is a new DXCCL, 1
    duplicate_qso: &'a str, // 0 or 1 character, "D" if contact is a duplicate QSO, 1
}

impl Default for Reg1testQSORecord<'_> {
    fn default() -> Self {
        Reg1testQSORecord {
            date: "",
            time: "",
            call: "",
            mode_code: '0',
            sent_rst: "",
            sent_qso_number: "",
            received_rst: "",
            received_qso_number: "",
            received_exchange: "",
            received_wwl: "",
            qso_points: "",
            new_exchange: "",
            new_wwl: "",
            new_dxcc: "",
            duplicate_qso: "",
        }
    }
}

impl fmt::Display for Reg1testQSORecord<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{};{};{};{};{};{};{};{};{};{};{};{};{};{};{}\n",
            self.date,
            self.time,
            self.call,
            self.mode_code,
            self.sent_rst,
            self.sent_qso_number,
            self.received_rst,
            self.received_qso_number,
            self.received_exchange,
            self.received_wwl,
            self.qso_points,
            self.new_exchange,
            self.new_wwl,
            self.new_dxcc,
            self.duplicate_qso,
        )
    }
}

impl Reg1testQSORecord<'_> {
    fn get_mode (mode_string: &str) -> char {
        match mode_string.trim() {
            "SSB" => {return '1';},
            "LSB" => {return '1';}, // non-standard ADIF MODE values
            "USB" => {return '1';}, // non-standard ADIF MODE values
            "CW" => {return '2';},
            "AM" => {return '5';},
            "FM" => {return '6';},
            "RTTY" => {return '7';},
            "SSTV" => {return '8';},
            "ATV" => {return '9';},
            _ => {return '0';}, 
        }
    }
}

struct FoundCaptured {
    found: bool,
    captured: bool,
}

impl Default for FoundCaptured {
    fn default() -> Self {
        return FoundCaptured { found: false, captured: false }
    }
}

impl FoundCaptured {
    fn set_found (&mut self) {
        self.found = true;
    }
}

impl FoundCaptured {
    fn is_found (&mut self) -> bool {
        if self.found && !self.captured {
            self.captured = true;
            return true
        }
        return false
    }
}

fn parse_args (args: &[String]) -> Result<&str, &str> {
    if args.len() < 2 {
        return Err("filename is missing");
    }
    let filename: &String = &args[1];
    Ok(filename)
}


fn adi_to_reg1test (mut parse_result: pest::iterators::Pairs<'_, Rule>) -> Result<String, Box<dyn Error>> {

    let mut r1t_header: Reg1testHeader = Reg1testHeader::default();
    let mut r1t_remarks =Reg1testRemarks::default();
    let mut r1t_qso_records = Reg1testQSOs::default();
    let mut r1t_record = Reg1testQSORecord::default();

    let mut station_callsign = FoundCaptured::default();
    let mut my_square = FoundCaptured::default();
    let mut band = FoundCaptured::default();

    let mut min_date: u32 = 99991231; // extremely large date as number
    let mut min_date_str: &str = "";
    let mut max_date: u32 = 0; // extremly small date as number
    let mut max_date_str: &str = "";

    println!();

    let parsed_adi_rules = parse_result.next().unwrap();

    for line in parsed_adi_rules.into_inner() {

        match line.as_rule() {
            
            Rule::header => {
                for inner_rule in line.into_inner() {
                    println!("{:?}", inner_rule.as_str().trim().to_string());
                    let mut program_id = FoundCaptured::default();
                    let mut adif_version = FoundCaptured::default();
                    let header_line = inner_rule.as_str().trim();

                    if !header_line.contains("<EOH>") {
                        if header_line.starts_with("<") {
                            for inner_pair in inner_rule.into_inner() {
                                match inner_pair.as_rule() {
                                    Rule::field_name => {
                                        match inner_pair.as_str() {
                                            "PROGRAMID" => {program_id.set_found();},
                                            "ADIF_VER" => {adif_version.set_found();},
                                            _ => continue
                                        }
                                    },
                                    Rule::data => {
                                        if program_id.is_found() {
                                            let text_line = format!("Program_ID={}", inner_pair.as_str().trim().to_string());
                                            r1t_remarks.multi_line.push(text_line);
                                        }
                                        if adif_version.is_found() {
                                            let text_line = format!("ADIF_version={}", inner_pair.as_str().trim().to_string());
                                            r1t_remarks.multi_line.push(text_line);
                                        }
                                    },    
                                    _ => continue
                                }
                            }
                        } else {
                            r1t_remarks.multi_line.push("Converted from ADIF to REG1TEST".to_owned());
                            r1t_remarks.multi_line.push(header_line.to_string());
                        }
                    }
                }
            }
            
            Rule::record => {                
                println!("{:?}", line.as_str().trim());
                let mut call = FoundCaptured::default();
                let mut qso_date = FoundCaptured::default();
                let mut time = FoundCaptured::default();
                let mut mode = FoundCaptured::default();
                let mut rst_sent = FoundCaptured::default();
                let mut rst_rcvd = FoundCaptured::default();
                let mut gridsquare = FoundCaptured::default();
                r1t_qso_records.count += 1;

                for inner_rule in line.into_inner() {

                    for inner_pair in inner_rule.into_inner() {

                        match inner_pair.as_rule() {

                            Rule::field_name => {
                                match inner_pair.as_str() {
                                    "STATION_CALLSIGN" => {station_callsign.set_found();},
                                    "QSO_DATE" => {qso_date.set_found();},
                                    "TIME_ON" => {time.set_found()},
                                    "BAND" => {band.set_found();},
                                    "CALL" => {call.set_found();},
                                    "MODE" => {mode.set_found();},
                                    "RST_SENT" => {rst_sent.set_found();},
                                    "RST_RCVD" => {rst_rcvd.set_found();},
                                    "GRIDSQUARE" => {gridsquare.set_found();},
                                    "MY_GRIDSQUARE" => {my_square.set_found();},
                                    _ => continue
                                }
                            },

                            Rule::data => {
                                if station_callsign.is_found() {
                                    r1t_header.pcall = inner_pair.as_str().trim();
                                }
                                if qso_date.is_found() {
                                    let date_as_number: u32 = inner_pair.as_str().trim().parse().unwrap(); // string to number
                                    // set MIN qso date
                                    if date_as_number < min_date {
                                        min_date = date_as_number;
                                        min_date_str = inner_pair.as_str().trim();
                                    }
                                    // set MAX qso date
                                    if date_as_number > max_date {
                                        max_date = date_as_number;
                                        max_date_str = inner_pair.as_str().trim();
                                    }
                                    r1t_record.date = &inner_pair.as_str().trim()[2..];
                                }
                                if time.is_found() {
                                    r1t_record.time = inner_pair.as_str().trim();
                                }
                                if band.is_found() {
                                    r1t_header.pband = Reg1testHeader::get_band(inner_pair.as_str().trim());
                                }
                                if call.is_found() {
                                    r1t_record.call = inner_pair.as_str().trim().into();
                                }
                                if mode.is_found() {
                                    // Regex string for RST: r"([12345][123456789])([123456789asm])*"
                                    r1t_record.mode_code = Reg1testQSORecord::get_mode(inner_pair.as_str());
                                }
                                if rst_sent.is_found() {
                                    r1t_record.sent_rst = inner_pair.as_str().trim().into();
                                }
                                if rst_rcvd.is_found() {
                                    r1t_record.received_rst = inner_pair.as_str().trim().into();
                                }
                                if gridsquare.is_found() {
                                    r1t_record.received_wwl = inner_pair.as_str().trim();
                                }
                                if my_square.is_found() {
                                    r1t_header.pwwlo = inner_pair.as_str().trim();
                                }
                            },

                            _ => continue
                        }
                    }
                }
                r1t_qso_records.qso_records.push(r1t_record);
                // println!("{:?}", r1t_record);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    
    let mut pdate: String = min_date_str.to_string();
    pdate.push_str(";");
    pdate.push_str(max_date_str);
    r1t_header.tdate = pdate.as_str();

    let reg1test_result = String::from_str(format!("{}\n{}\n{}", r1t_header, r1t_remarks, r1t_qso_records).as_ref())?;

    return Ok(reg1test_result)
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = parse_args(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(0);
    });
    
    let unparsed_string = fs::read_to_string(filename).unwrap_or_else(|err| {
        println!("Problem reading file: {}", err);
        process::exit(0);
    });

    match AdiParser::parse(Rule::adi, &unparsed_string) {
        Err(parse_error) => {
            println!("Error while parsing file: {}", parse_error);
            process::exit(0);
        },
        Ok(parse_result) => {
            let reg1test_output = adi_to_reg1test(parse_result).unwrap();
            println!("\n{}", reg1test_output);
        },
    }
}
