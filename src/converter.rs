use crate::reg1test::Reg1testHeader;
use crate::reg1test::Reg1testQSORecord;
use crate::reg1test::Reg1testQSOs;
use crate::reg1test::Reg1testRemarks;

use crate::utils::FoundCaptured;
use crate::Rule;
use crate::SKIP_REMARKS;
use core::sync::atomic::Ordering;
use std::error::Error;
// use std::str::FromStr;

pub struct Band<'a> {
    pub header: Reg1testHeader<'a>,
    pub records: Reg1testQSOs<'a>, 
}

impl<'a> Band<'a> {
    pub fn add_qso(&mut self, qso: Reg1testQSORecord<'a>) {
        self.records.qso_records.push(qso);
        self.records.count += 1;
    }
}

pub fn convert_to_reg1test(
    mut parse_result: pest::iterators::Pairs<'_, Rule>,
) -> Result<String, Box<dyn Error>> {
    let mut band_array: Vec<Band> = Vec::new();

    let mut r1t_header: Reg1testHeader = Reg1testHeader::default();
    let mut r1t_remarks = Reg1testRemarks::default();
    let mut r1t_qso_records = Reg1testQSOs::default();
    let mut r1t_record = Reg1testQSORecord::default();

    let mut station_callsign = FoundCaptured::default();
    let mut my_square = FoundCaptured::default();

    let mut min_date: u32 = 99991231; // extremely large date as number
    let mut min_date_str: &str = "";
    let mut max_date: u32 = 0; // extremely small date as number
    let mut max_date_str: &str = "";

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
                let mut band = FoundCaptured::default();  // was missing?
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

                let mut band_found = false;
                for band in band_array.iter_mut() {
                    if band.header.pband == r1t_header.pband {
                        band.add_qso(r1t_record);
                        // band.records.qso_records.push(r1t_record);
                        // band.records.count += 1; 
                        band_found = true;
                        break
                    }
                }
                if band_found == false {
                    let mut new_band = Band {
                        header: Reg1testHeader::default(),
                        records: Reg1testQSOs::default()
                    };
                    new_band.header = r1t_header.clone();
                    new_band.add_qso(r1t_record);
                    // new_band.records.qso_records.push(r1t_record);
                    // new_band.records.count += 1;
                    band_array.push(new_band);
                }
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    let mut pdate: String = min_date_str.to_string();
    pdate.push_str(";");
    pdate.push_str(max_date_str);
    r1t_header.tdate = pdate.as_str();

    let mut reg1test_result = String::new();

    for band in band_array.iter_mut() {
        band.header.tdate = pdate.as_str();

        if SKIP_REMARKS.load(Ordering::Relaxed) {
            r1t_remarks.multi_line.clear()
        };

        reg1test_result.push_str(format!("{}\n{}\n{}\n", band.header, r1t_remarks, band.records).as_ref())

    }
    Ok(reg1test_result)
}