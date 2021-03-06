use std::fmt::Display;

pub const CSI_START: &str = "\x1b[";
pub const SET_FG_COLOR: &str = "\x1b[38;5;";
pub const SET_BG_COLOR: &str = "\x1b[48;5;";
pub const RESET: &str = "\x1b[0m";

// Select Graphic Rendition Parameters
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Sgr {
    Reset = 0,
    Bold,
    Faint,
    Italic,
    Underline,
    SlowBlink,
    RapidBlink,
    SwapFgAndBg,
    Hide,
    CrossedOut,
    DefaultFont,
    AlterFont11,
    AlterFont12,
    AlterFont13,
    AlterFont14,
    AlterFont15,
    AlterFont16,
    AlterFont17,
    AlterFont18,
    AlterFont19,
    Gothic,
    DoublyUnderline,
    NormalIntensity,
    NeitherItalic,
    NotUnderline,
    NotBlink,
    ProportionalSpacing,
    NotReversed,
    Reveal,
    NotCrossedOut,
    SetForegroundColor30,
    SetForegroundColor31,
    SetForegroundColor32,
    SetForegroundColor33,
    SetForegroundColor34,
    SetForegroundColor35,
    SetForegroundColor36,
    SetForegroundColor37,
    SetForegroundColor,
    DefaultForegroundColor,
    SetBackgroundColor40,
    SetBackgroundColor41,
    SetBackgroundColor42,
    SetBackgroundColor43,
    SetBackgroundColor44,
    SetBackgroundColor45,
    SetBackgroundColor46,
    SetBackgroundColor47,
    SetBackgroundColor,
    DefaultBackgroundColor,
    DisableProportionalSpacing,
    Framed,
    Encircled,
    Overlined,
    NeitherFramedNorEncircled,
    NotOverlined,
    SetUnderlineColor,
    DefaultUnderlineColor,
    IdeogramUnderlineOrRightSideLine,
    IdeogramDoubleUnderlineOrDoubleLineOnTheRightSide,
    IdeogramOverlineOrLeftSideLine,
    IdeogramDoubleOverlineOrDoubleLineOnTheLeftSide,
    IdeogramStressMarking,
    NoIdeogramAttributes,
    Superscript,
    Subscript,
    NeitherSuperscriptNorSubscript,
    SetBrightForegroundColor,
    SetBrightBackgroundColor,
}

impl Display for Sgr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}
