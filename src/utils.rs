pub struct FoundCaptured {
    pub found: bool,
    pub captured: bool,
    pub length: usize,
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
    pub fn set_found(&mut self) {
        self.found = true;
    }

    pub fn set_length(&mut self, len: usize) {
        if self.found && !self.captured {
            self.length = len;
        }
    }

    pub fn is_found(&mut self) -> bool {
        if self.found && !self.captured {
            self.captured = true;
            return true;
        }
        false
    }
}