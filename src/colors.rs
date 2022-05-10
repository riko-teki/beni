pub enum EightBitColors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    LightGray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
    Grey0,
    NavyBlue,
    DarkBlue,
    Blue3a,
    Blue3b,
    Blue1,
    DarkGreen,
    DeepSkyBlue4a,
    DeepSkyBlue4b,
    DeepSkyBlue4c,
    DodgerBlue3,
    DodgerBlue2,
    Green4,
    SpringGreen4,
    Turquoise4,
    DeepSkyBlue3a,
    DeepSkyBlue3b,
    DodgerBlue1,
    Green3a,
    SpringGreen3a,
    DarkCyan,
    LightSeaGreen,
    DeepSkyBlue2,
    DeepSkyBlue1,
    Green3b,
    SpringGreen3b,
    SpringGreen2a,
    Cyan3,
    DarkTurquoise,
    Turquoise2,
    Green1,
    SpringGreen2b,
    SpringGreen1,
    MediumSpringGreen,
    Cyan2,
    Cyan1,
    DarkRed1,
    DeepPink4a,
    Purple4a,
    Purple4b,
    Purple3,
    BlueViolet,
    Orange4a,
    Grey37,
    MediumPurple4,
    SlateBlue3a,
    SlateBlue3b,
    RoyalBlue1,
    Chartreuse4,
    DarkSeaGreen4a,
    PaleTurquoise4,
    SteelBlue,
    SteelBlue3,
    CornflowerBlue,
    Chartreuse3a,
    DarkSeaGreen4b,
    CadetBlue2,
    CadetBlue1,
    SkyBlue3,
    SteelBlue1a,
    Chartreuse3b,
    PaleGreen3a,
    SeaGreen3,
    Aquamarine3,
    MediumTurquoise,
    SteelBlue1b,
    Chartreuse2a,
    SeaGreen2,
    SeaGreen1a,
    SeaGreen1b,
    Aquamarine1a,
    DarkSlateGray2,
    DarkRed2,
    DeepPink4b,
    DarkMagenta1,
    DarkMagenta2,
    DarkViolet1a,
    Purple1a,
    Orange4b,
    LightPink4,
    Plum4,
    MediumPurple3a,
    MediumPurple3b,
    SlateBlue1,
    Yellow4a,
    Wheat4,
    Grey53,
    LightSlateGrey,
    MediumPurple,
    LightSlateBlue,
    Yellow4b,
    DarkOliveGreen3a,
    DarkGreenSea,
    LightSkyBlue3a,
    LightSkyBlue3b,
    SkyBlue2,
    Chartreuse2b,
    DarkOliveGreen3b,
    PaleGreen3b,
    DarkSeaGreen3a,
    DarkSlateGray3,
    SkyBlue1,
    Chartreuse1,
    LightGreen2,
    LightGreen3,
    PaleGreen1a,
    Aquamarine1b,
    DarkSlateGray1,
    Red3a,
    DeepPink4c,
    MediumVioletRed,
    Magenta3a,
    DarkViolet1b,
    Purple1b,
    DarkOrange3a,
    IndianRed1a,
    HotPink3a,
    MediumOrchid3,
    MediumOrchid,
    MediumPurple2a,
    DarkGoldenrod,
    LightSalmon3a,
    RosyBrown,
    Grey63,
    MediumPurple2b,
    MediumPurple1,
    Gold3a,
    DarkKhaki,
    NavajoWhite3,
    Grey69,
    LightSteelBlue3,
    LightSteelBlue,
    Yellow3a,
    DarkOliveGreen3,
    DarkSeaGreen3b,
    DarkSeaGreen2,
    LightCyan3,
    LightSkyBlue1,
    GreenYellow,
    DarkOliveGreen2,
    PaleGreen1b,
    DarkSeaGreen5b,
    DarkSeaGreen5a,
    PaleTurquoise1,
    Red3b,
    DeepPink3a,
    DeepPink3b,
    Magenta3b,
    Magenta3c,
    Magenta2a,
    DarkOrange3b,
    IndianRed1b,
    HotPink3b,
    HotPink2,
    Orchid,
    MediumOrchid1a,
    Orange3,
    LightSalmon3b,
    LightPink3,
    Pink3,
    Plum3,
    Violet,
    Gold3b,
    LightGoldenrod3,
    Tan,
    MistyRose3,
    Thistle3,
    Plum2,
    Yellow3b,
    Khaki3,
    LightGoldenrod2a,
    LightYellow3,
    Grey84,
    LightSteelBlue1,
    Yellow2,
    DarkOliveGreen1a,
    DarkOliveGreen1b,
    DarkSeaGreen1,
    Honeydew2,
    LightCyan1,
    Red1,
    DeepPink2,
    DeepPink1a,
    DeepPink1b,
    Magenta2b,
    Magenta1,
    OrangeRed1,
    IndianRed1c,
    IndianRed1d,
    HotPink1a,
    HotPink1b,
    MediumOrchid1b,
    DarkOrange,
    Salmon1,
    LightCoral,
    PaleVioletRed1,
    Orchid2,
    Orchid1,
    Orange1,
    SandyBrown,
    LightSalmon1,
    LightPink1,
    Pink1,
    Plum1,
    Gold1,
    LightGoldenrod2b,
    LightGoldenrod2c,
    NavajoWhite1,
    MistyRose1,
    Thistle1,
    Yellow1,
    LightGoldenrod1,
    Khaki1,
    Wheat1,
    CornSilk1,
    Grey100,
    Grey3,
    Grey7,
    Grey11,
    Grey15,
    Grey19,
    Grey23,
    Grey27,
    Grey30,
    Grey35,
    Grey39,
    Grey42,
    Grey46,
    Grey50,
    Grey54,
    Grey58,
    Grey62,
    Grey66,
    Grey70,
    Grey74,
    Grey78,
    Grey82,
    Grey85,
    Grey89,
    Grey93,    
}

impl EightBitColors {
    fn to_color_string(&self) -> String {
        return (*self as u8).to_string(); 
    }
}
