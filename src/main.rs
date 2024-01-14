extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use clap::Parser as clapParser;

//use core::num;
use std::env;
use std::error::Error;
use std::fmt;
use std::process;
use std::str::FromStr;
use strip_bom::*;

#[derive(Parser)]
#[grammar = r"adi.pest"]
pub struct AdiParser;


#[derive(clapParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Name of input file (ADI)
    #[arg(short, long)]
    adi_file: String,
    
    // Name of output file (EDI)
    #[arg(short, long)]
    edi_file: String,
    
    // No remarks in EDI file
    #[arg(short, long)]
    noremarks: bool,
}

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
        if self.psect != "" {
            write!(f, "\nPSect={}", self.psect)?;
        }
        if self.pclub != "" {
            write!(f, "\nPClub={}", self.pclub)?;
        }
        Ok(())
    }
}

impl Reg1testHeader<'_> {
    fn get_band(band: &str) -> String {
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
    #[allow(dead_code)]
    fn get_band_from_freq(f: f64) -> String {
        if f > 50.0 && f < 53.0 {return "50 MHz".to_string();}
        if f > 70.0 && f < 71.0 {return "70 MHz".to_string();}
        if f > 144.0 && f < 148.0 {return "144 MHz".to_string();}
        if f > 420.0 && f < 450.0 {return "432 MHz".to_string();}
        if f > 1240.0 && f < 1300.0 {return "1,3 GHz".to_string();}
        if f > 2300.0 && f < 2450.0 {return "2,3 GHz".to_string();}
        if f > 3300.0 && f < 3500.0 {return "3,4 GHz".to_string();}
        if f > 5650.0 && f < 5925.0 {return "5,7 GHz".to_string();}
        if f > 10000.0 && f < 10500.0 {return "10 GHz".to_string();}
        if f > 24000.0 && f < 24250.0 {return "24 GHz".to_string();}
        if f > 47000.0 && f < 47200.0 {return "47 GHz".to_string();}
        if f > 75500.0 && f < 81000.0 {return "76 GHz".to_string();}
        if f > 119980.0 && f < 12300.0 {return "120 GHz".to_string();}
        if f > 134000.0 && f < 14900.0 {return "144 GHz".to_string();}
        if f > 241000.0 && f < 25000.0 {return "248 GHz".to_string();}
        return "".to_string();
    }
}

#[allow(dead_code)]
// #[derive(Debug)]
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
        Ok(())
    }
}


impl fmt::Debug for Reg1testRemarks<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.name)?;
        if self.multi_line.len() != 0 {
            // If there are remark lines
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
    date: &'a str,                // YYMMDD, 6 characters, 6
    time: &'a str,                // UTC, 4 characters, with leading zeros, 4
    call: &'a str,                // 3 to 14 characters, 14
    mode_code: char,              // 0 or 1 character, 1
    sent_rst: &'a str,            // 0 or 2 or 3 characters, 3
    sent_qso_number: u16,         // 0 or 3 or 4 characters, with leading zeros, 4
    received_rst: &'a str,        // 0 or 2 or 3 characters, 3
    received_qso_number: u16,     // 0 or 3 or 4 characters, with leading zeros, 4
    received_exchange: &'a str,   // 0 or 1 to 6 characters (see also PExch), 6
    received_wwl: &'a str,        // 0 or 4 or 6 characters, World Wide Locator, 6
    qso_points: &'a str,          // 1 to 6 characters, including bandmultiplier, 6
    new_exchange: &'a str,        // 0 or 1 character, "N" if QSO is a new exchange, 1
    new_wwl: &'a str,             // 0 or 1 character, "N" if QSO is a new WWL, 1
    new_dxcc: &'a str,            // 0 or 1 character, "N" if QSO is a new DXCCL, 1
    duplicate_qso: &'a str,       // 0 or 1 character, "D" if contact is a duplicate QSO, 1
}

impl Default for Reg1testQSORecord<'_> {
    fn default() -> Self {
        Reg1testQSORecord {
            date: "",
            time: "",
            call: "",
            mode_code: '0',
            sent_rst: "",
            sent_qso_number: 0,
            received_rst: "",
            received_qso_number: 0,
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
        let mut sent_qso_num = String::from(""); 
        let mut received_qso_num = String::from("");

        if self.sent_qso_number > 0 {
            if self.sent_qso_number > 999 {
                sent_qso_num = format!("{:04}", self.sent_qso_number);
            }
            else {
                sent_qso_num = format!("{:03}", self.sent_qso_number);
            }
        }
        
        if self.received_qso_number > 0 {
            if self.received_qso_number > 999 {
                received_qso_num = format!("{:04}", self.received_qso_number);
            } else {
                received_qso_num = format!("{:03}", self.received_qso_number);
            }
        }

        write!(
            f,
            "{};{};{};{};{};{};{};{};{};{};{};{};{};{};{}\n",
            self.date,
            self.time,
            self.call,
            self.mode_code,
            self.sent_rst,
            sent_qso_num,
            self.received_rst,
            received_qso_num,
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
    fn get_mode(mode_string: &str) -> char {
        match mode_string.trim() {
            "SSB" => {
                return '1';
            }
            "CW" => {
                return '2';
            }
            "AM" => {
                return '5';
            }
            "FM" => {
                return '6';
            }
            "RTTY" => {
                return '7';
            }
            "SSTV" => {
                return '8';
            }
            "ATV" => {
                return '9';
            }
            _ => {
                return '0';
            }
        }
    }
}

struct FoundCaptured {
    found: bool,
    captured: bool,
    length: usize,
}

impl Default for FoundCaptured {
    fn default() -> Self {
        FoundCaptured {
            found: false,
            captured: false,
            length: 0,
        }
    }
}

impl FoundCaptured {
    fn set_found(&mut self) {
        self.found = true;
    }
    fn set_length(&mut self, len: usize) {
        if self.found && !self.captured {
            self.length = len;
        }
    }
    fn is_found(&mut self) -> bool {
        if self.found && !self.captured {
            self.captured = true;
            return true;
        }
        false
    }
}

fn parse_args(args: &[String]) -> Result<&str, &str> {
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

fn adi_to_reg1test(
    mut parse_result: pest::iterators::Pairs<'_, Rule>,
) -> Result<String, Box<dyn Error>> {
    let mut r1t_header: Reg1testHeader = Reg1testHeader::default();
    let mut r1t_remarks = Reg1testRemarks::default();
    let mut r1t_qso_records = Reg1testQSOs::default();
    let mut r1t_record = Reg1testQSORecord::default();

    let mut station_callsign = FoundCaptured::default();
    let mut my_square = FoundCaptured::default();
    let mut band = FoundCaptured::default();

    let mut min_date: u32 = 99991231; // extremely large date as number
    let mut min_date_str: &str = "";
    let mut max_date: u32 = 0; // extremly small date as number
    let mut max_date_str: &str = "";

    r1t_remarks
        .multi_line
        .push("Converted from ADIF to REG1TEST. ADIF header below:".to_owned());

    let parsed_adi_rules = parse_result.next().unwrap();

    for inner_pair in parsed_adi_rules.into_inner() {
        //        println!("{:?}", inner_pair.as_str().trim());
        match inner_pair.as_rule() {
            Rule::header => {
                // println!("{:?}", inner_pair.as_rule());
                for inner_pair1 in inner_pair.into_inner() {
                    match inner_pair1.as_rule() {
                        Rule::adt_string => {
                            // println!("{:?} {:?}", inner_pair1.as_rule(), inner_pair1.as_str());
                            r1t_remarks.multi_line.push(inner_pair1.as_str().to_owned());
                        }
                        Rule::adt_multi_string => {
                            // println!("{:?} {:?}", inner_pair1.as_rule(), inner_pair1.as_str());
                            for inner_pair2 in inner_pair1.into_inner() {
                                match inner_pair2.as_rule() {
                                    Rule::adt_string => {
                                        // println!("{:?} {:?}", inner_pair2.as_rule(), inner_pair2.as_str());
                                        r1t_remarks
                                            .multi_line
                                            .push(inner_pair2.as_str().to_owned());
                                    }
                                    _ => continue,
                                }
                            }
                        }
                        Rule::data_specifier => {
                            // println!("{:?} {:?}", inner_pair1.as_rule(), inner_pair1.as_str());
                            let mut adif_version = FoundCaptured::default();
                            let mut created_timestamp = FoundCaptured::default();
                            let mut program_id = FoundCaptured::default();
                            let mut program_version = FoundCaptured::default();

                            for inner_pair2 in inner_pair1.into_inner() {
                                match inner_pair2.as_rule() {
                                    Rule::field => {
                                        // println!("{:?} {:?}", inner_pair2.as_rule(), inner_pair2.as_str());
                                        for inner_pair3 in inner_pair2.into_inner() {
                                            match inner_pair3.as_rule() {
                                                Rule::field_name => {
                                                    // println!("{:?} {:?}", inner_pair3.as_rule(), inner_pair3.as_str());
                                                    match inner_pair3
                                                        .as_str()
                                                        .to_uppercase()
                                                        .as_str()
                                                    {
                                                        "ADIF_VER" => {
                                                            adif_version.set_found();
                                                        }
                                                        "CREATED_TIMESTAMP" => {
                                                            created_timestamp.set_found();
                                                        }
                                                        "PROGRAMID" => {
                                                            program_id.set_found();
                                                        }
                                                        "PROGRAMVERSION" => {
                                                            program_version.set_found();
                                                        }
                                                        _ => continue,
                                                    }
                                                }
                                                Rule::data_length => {
                                                    let len: usize =
                                                        inner_pair3.as_str().parse().unwrap();
                                                    // println!("{:?} {:?} {}", inner_pair3.as_rule(), inner_pair3.as_str(), len);
                                                    adif_version.set_length(len);
                                                    created_timestamp.set_length(len);
                                                    program_id.set_length(len);
                                                    program_version.set_length(len);
                                                }
                                                _ => continue,
                                            }
                                        }
                                    }
                                    Rule::data => {
                                        let data_as_string =
                                            inner_pair2.as_str().trim().to_string();
                                        // println!("{:?} {:?} {}", inner_pair2.as_rule(), inner_pair2.as_str(), data_as_string);
                                        if adif_version.is_found() {
                                            let text_line = format!("ADIF_VER={}", data_as_string);
                                            r1t_remarks.multi_line.push(text_line);
                                        }
                                        if created_timestamp.is_found() {
                                            let text_line =
                                                format!("CREATED_TIMESTAMP={}", data_as_string);
                                            r1t_remarks.multi_line.push(text_line);
                                        }
                                        if program_id.is_found() {
                                            let text_line = format!("PROGRAMID={}", data_as_string);
                                            r1t_remarks.multi_line.push(text_line);
                                        }
                                        if program_version.is_found() {
                                            let text_line =
                                                format!("PROGRAMVERSION={}", data_as_string);
                                            r1t_remarks.multi_line.push(text_line);
                                        }
                                    }
                                    _ => continue,
                                }
                            }
                        }
                        _ => continue,
                    }
                }
            }

            Rule::record => {
                // println!("{:?}", inner_pair.as_str().trim());
                let mut call = FoundCaptured::default();
                let mut qso_date = FoundCaptured::default();
                let mut time = FoundCaptured::default();
                let mut mode = FoundCaptured::default();
                let mut rst_sent = FoundCaptured::default();
                let mut stx = FoundCaptured::default();
                let mut rst_rcvd = FoundCaptured::default();
                let mut srx = FoundCaptured::default();
                let mut gridsquare = FoundCaptured::default();

                r1t_qso_records.count += 1;

                for inner_pair1 in inner_pair.into_inner() {
                    // println!("{:?} {:?}", inner_pair1.as_rule(), inner_pair1.as_str());
                    for inner_pair2 in inner_pair1.into_inner() {
                        // println!("{:?} {:?}", inner_pair2.as_rule(), inner_pair2.as_str());
                        match inner_pair2.as_rule() {
                            Rule::field => {
                                for inner_pair3 in inner_pair2.into_inner() {
                                    // println!("{:?} {:?}", inner_pair3.as_rule(), inner_pair3.as_str());
                                    match inner_pair3.as_rule() {
                                        Rule::field_name => {
                                            match inner_pair3.as_str().to_uppercase().as_str() {
                                                "STATION_CALLSIGN" => {
                                                    station_callsign.set_found();
                                                }
                                                "QSO_DATE" => {
                                                    qso_date.set_found();
                                                }
                                                "TIME_ON" => {
                                                    time.set_found();
                                                }
                                                "BAND" => {
                                                    band.set_found();
                                                }
                                                "CALL" => {
                                                    call.set_found();
                                                }
                                                "MODE" => {
                                                    mode.set_found();
                                                }
                                                "RST_SENT" => {
                                                    rst_sent.set_found();
                                                }
                                                "STX" => {
                                                    stx.set_found();
                                                }
                                                "RST_RCVD" => {
                                                    rst_rcvd.set_found();
                                                }
                                                "SRX" => {
                                                    srx.set_found();
                                                }
                                                "GRIDSQUARE" => {
                                                    gridsquare.set_found();
                                                }
                                                "MY_GRIDSQUARE" => {
                                                    my_square.set_found();
                                                }
                                                _ => continue,
                                            }
                                        }
                                        Rule::data_length => {
                                            let len: usize = inner_pair3.as_str().parse().unwrap();
                                            // println!("{:?} {:?} {}", inner_pair3.as_rule(), inner_pair3.as_str(), len);
                                            station_callsign.set_length(len);
                                            qso_date.set_length(len);
                                            time.set_length(len);
                                            band.set_length(len);
                                            call.set_length(len);
                                            mode.set_length(len);
                                            rst_sent.set_length(len);
                                            stx.set_length(len);
                                            rst_rcvd.set_length(len);
                                            srx.set_length(len);
                                            gridsquare.set_length(len);
                                            my_square.set_length(len);
                                        }
                                        _ => continue,
                                    }
                                }
                            }
                            Rule::data => {
                                let data_as_str = inner_pair2.as_str().trim();
                                // println!("{:?} {:?} {}", inner_pair2.as_rule(), inner_pair2.as_str(), data_as_str);
                                if station_callsign.is_found() {
                                    r1t_header.pcall = data_as_str;
                                }
                                if qso_date.is_found() {
                                    let date_as_number: u32 = data_as_str.parse().unwrap(); // string to number
                                    if date_as_number < min_date {
                                        // set MIN qso date
                                        min_date = date_as_number;
                                        min_date_str = data_as_str;
                                    }
                                    if date_as_number > max_date {
                                        // set MAX qso date
                                        max_date = date_as_number;
                                        max_date_str = data_as_str;
                                    }
                                    r1t_record.date = &data_as_str[2..];
                                }
                                if time.is_found() {
                                    if data_as_str.len() > 4 {
                                        r1t_record.time = &data_as_str[0..4];
                                    } else {
                                        r1t_record.time = data_as_str;
                                    }
                                }
                                if band.is_found() {
                                    r1t_header.pband = Reg1testHeader::get_band(data_as_str);
                                }
                                if call.is_found() {
                                    r1t_record.call = data_as_str.into();
                                }
                                if mode.is_found() {
                                    // Regex string for RST: r"([12345][123456789])([123456789asm])*"
                                    r1t_record.mode_code =
                                        Reg1testQSORecord::get_mode(inner_pair2.as_str());
                                }
                                if rst_sent.is_found() {
                                    r1t_record.sent_rst = data_as_str.into();
                                }
                                if stx.is_found() {
                                    let number: u16 = data_as_str.trim_start_matches('0').parse().unwrap(); // string to number
                                    r1t_record.sent_qso_number = number;
                                }
                                if rst_rcvd.is_found() {
                                    r1t_record.received_rst = data_as_str.into();
                                }
                                if srx.is_found() {
                                    let number: u16 = data_as_str.trim_start_matches('0').parse().unwrap(); // string to number
                                    r1t_record.received_qso_number = number;
                                }
                                if gridsquare.is_found() {
                                    r1t_record.received_wwl = data_as_str;
                                }
                                if my_square.is_found() {
                                    r1t_header.pwwlo = data_as_str;
                                }
                            }
                            _ => continue,
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

    let reg1test_result =
        String::from_str(format!("{}\n{:?}\n{}", r1t_header, r1t_remarks, r1t_qso_records).as_ref())?;

    return Ok(reg1test_result);
}

fn main() {
/*
    let args: Vec<String> = env::args().collect();

    let adi_file_name = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("Error while parsing arguments: {}", err);
        process::exit(1);
    });

    // let reg_file_name = adi_file_name.to_ascii_uppercase().replace(".ADI", ".edi");
    // println!("\nOutput file: {}", reg_file_name);

*/

    let args = Args::parse();

    println!("{}", args.adi_file);
    println!("{}", args.edi_file);
    println!("{}", args.noremarks);

    let unparsed_string = fs::read_to_string(args.adi_file).unwrap_or_else(|err| {
        eprintln!("Error while reading file: {}", err);
        process::exit(1);
    });

    match AdiParser::parse(Rule::adi, unparsed_string.strip_bom()) {
        Err(parse_error) => {
            eprintln!("Error while parsing file: {}", parse_error);
            process::exit(1);
        }
        Ok(parse_result) => {
            let reg1test_output = adi_to_reg1test(parse_result).unwrap();
            println!("{}", reg1test_output);
        }
    }
}
