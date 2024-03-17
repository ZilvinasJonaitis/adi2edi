use std::fmt;


#[allow(dead_code)]
#[derive(Debug)]
pub struct Reg1testHeader<'a> {
    pub name: &'a str,
    pub tdate: &'a str,
    pub pcall: &'a str,
    pub pwwlo: &'a str,
    pub pband: String,
    pub psect: String,
    pub pclub: String,
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
    pub fn get_band(band: &str) -> String {
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
    pub fn get_band_from_freq(f: f64) -> String {
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
pub struct Reg1testRemarks<'a> {
    pub name: &'a str,
    pub multi_line: Vec<String>,
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
pub struct Reg1testQSOs<'a> {
    pub name: &'a str,
    pub count: u32,
    pub qso_records: Vec<Reg1testQSORecord<'a>>,
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
pub struct Reg1testQSORecord<'a> {
    pub date: &'a str,                // YYMMDD, 6 characters, 6
    pub time: &'a str,                // UTC, 4 characters, with leading zeros, 4
    pub call: &'a str,                // 3 to 14 characters, 14
    pub mode_code: char,              // 0 or 1 character, 1
    pub sent_rst: &'a str,            // 0 or 2 or 3 characters, 3
    pub sent_qso_number: u16,         // 0 or 3 or 4 characters, with leading zeros, 4
    pub received_rst: &'a str,        // 0 or 2 or 3 characters, 3
    pub received_qso_number: u16,     // 0 or 3 or 4 characters, with leading zeros, 4
    pub received_exchange: &'a str,   // 0 or 1 to 6 characters (see also PExch), 6
    pub received_wwl: &'a str,        // 0 or 4 or 6 characters, World Wide Locator, 6
    pub qso_points: &'a str,          // 1 to 6 characters, including bandmultiplier, 6
    pub new_exchange: &'a str,        // 0 or 1 character, "N" if QSO is a new exchange, 1
    pub new_wwl: &'a str,             // 0 or 1 character, "N" if QSO is a new WWL, 1
    pub new_dxcc: &'a str,            // 0 or 1 character, "N" if QSO is a new DXCCL, 1
    pub duplicate_qso: &'a str,       // 0 or 1 character, "D" if contact is a duplicate QSO, 1
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
    pub fn get_mode(mode_string: &str) -> char {
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
