use std::fmt::Display;
use crate::{csi, colors::{Color, EightBitColors}};

pub trait StringMarker {
    fn to_string(&self) -> String;
    fn get_fg(&self) -> Option<EightBitColors> { None }
    fn get_bg(&self) -> Option<EightBitColors> { None }
}

impl StringMarker for String {
    fn to_string(&self) -> String {
        self.clone()
    }
}

impl<'a> StringMarker for &'a str {
    fn to_string(&self) -> String {
        String::from(*self)
    }
}

#[derive(Clone)]
pub struct DString {
    raw_text: String,
    foreground_color: Option<EightBitColors>,
    background_color: Option<EightBitColors>,
}

impl DString {
    pub fn from_string<S: StringMarker>(s: S) -> DString {
        DString {
            raw_text: s.to_string(),
            foreground_color: None,
            background_color: None,
        }
    }
    pub fn from_string_and_fg<S: StringMarker>(s: S, c: EightBitColors) -> DString {
        DString {
            raw_text: s.to_string(),
            foreground_color: Some(c),
            background_color: s.get_bg(),
        }
    }
    pub fn from_string_and_bg<S: StringMarker>(s: S, c: EightBitColors) -> DString {
        DString {
            raw_text: s.to_string(),
            foreground_color: s.get_fg(),
            background_color: Some(c),
        }
    }
}

impl StringMarker for DString {
    fn to_string(&self) -> String {
        self.clone().raw_text
    }

    fn get_fg(&self) -> Option<EightBitColors> {
        self.foreground_color
    }

    fn get_bg(&self) -> Option<EightBitColors> {
        self.background_color
    }
}

impl Display for DString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.foreground_color.is_none() && self.background_color.is_none() {
            write!(f, "{}", self.raw_text)?;
            return Ok(())
        }

        if let Some(c) = &self.foreground_color {
            write!(f,"{}{}m", csi::SET_FG_COLOR, c.to_color_number())?;
        }
        if let Some(c) = &self.background_color {
            write!(f, "{}{}m", csi::SET_BG_COLOR, c.to_color_number())?;
        }
        
        write!(f, "{}{}", self.raw_text, csi::RESET)?;
        
        Ok(())
    }
}
