use colors::EightBitColors;
use colors::Color;

pub mod colors;
pub mod csi;
pub mod sys;

pub trait ColorString {
    fn fg_color<C: colors::Color>(self, color: C) -> String;
    fn bg_color<C: colors::Color>(self, color: C) -> String;
    fn fg_black(self) -> String;
    fn fg_red(self) -> String;
    fn fg_green(self) -> String;
    fn fg_yellow(self) -> String;
    fn fg_blue(self) -> String;
    fn fg_magenta(self) -> String;
    fn fg_cyan(self) -> String;
    fn fg_lightgray(self) -> String;
    fn fg_darkgray(self) -> String;
    fn fg_lightred(self) -> String;
    fn fg_lightgreen(self) -> String;
    fn fg_lightyellow(self) -> String;
    fn fg_lightblue(self) -> String;
    fn fg_lightmagenta(self) -> String;
    fn fg_lightcyan(self) -> String;
    fn fg_white(self) -> String;
    fn fg_grey0(self) -> String;
    fn fg_navyblue(self) -> String;
    fn fg_darkblue(self) -> String;
    fn fg_blue3a(self) -> String;
    fn fg_blue3b(self) -> String;
    fn fg_blue1(self) -> String;
    fn fg_darkgreen(self) -> String;
    fn fg_deepskyblue4a(self) -> String;
    fn fg_deepskyblue4b(self) -> String;
    fn fg_deepskyblue4c(self) -> String;
    fn fg_dodgerblue3(self) -> String;
    fn fg_dodgerblue2(self) -> String;
    fn fg_green4(self) -> String;
    fn fg_springgreen4(self) -> String;
    fn fg_turquoise4(self) -> String;
    fn fg_deepskyblue3a(self) -> String;
    fn fg_deepskyblue3b(self) -> String;
    fn fg_dodgerblue1(self) -> String;
    fn fg_green3a(self) -> String;
    fn fg_springgreen3a(self) -> String;
    fn fg_darkcyan(self) -> String;
    fn fg_lightseagreen(self) -> String;
    fn fg_deepskyblue2(self) -> String;
    fn fg_deepskyblue1(self) -> String;
    fn fg_green3b(self) -> String;
    fn fg_springgreen3b(self) -> String;
    fn fg_springgreen2a(self) -> String;
    fn fg_cyan3(self) -> String;
    fn fg_darkturquoise(self) -> String;
    fn fg_turquoise2(self) -> String;
    fn fg_green1(self) -> String;
    fn fg_springgreen2b(self) -> String;
    fn fg_springgreen1(self) -> String;
    fn fg_mediumspringgreen(self) -> String;
    fn fg_cyan2(self) -> String;
    fn fg_cyan1(self) -> String;
    fn fg_darkred1(self) -> String;
    fn fg_deeppink4a(self) -> String;
    fn fg_purple4a(self) -> String;
    fn fg_purple4b(self) -> String;
    fn fg_purple3(self) -> String;
    fn fg_blueviolet(self) -> String;
    fn fg_orange4a(self) -> String;
    fn fg_grey37(self) -> String;
    fn fg_mediumpurple4(self) -> String;
    fn fg_slateblue3a(self) -> String;
    fn fg_slateblue3b(self) -> String;
    fn fg_royalblue1(self) -> String;
    fn fg_chartreuse4(self) -> String;
    fn fg_darkseagreen4a(self) -> String;
    fn fg_paleturquoise4(self) -> String;
    fn fg_steelblue(self) -> String;
    fn fg_steelblue3(self) -> String;
    fn fg_cornflowerblue(self) -> String;
    fn fg_chartreuse3a(self) -> String;
    fn fg_darkseagreen4b(self) -> String;
    fn fg_cadetblue2(self) -> String;
    fn fg_cadetblue1(self) -> String;
    fn fg_skyblue3(self) -> String;
    fn fg_steelblue1a(self) -> String;
    fn fg_chartreuse3b(self) -> String;
    fn fg_palegreen3a(self) -> String;
    fn fg_seagreen3(self) -> String;
    fn fg_aquamarine3(self) -> String;
    fn fg_mediumturquoise(self) -> String;
    fn fg_steelblue1b(self) -> String;
    fn fg_chartreuse2a(self) -> String;
    fn fg_seagreen2(self) -> String;
    fn fg_seagreen1a(self) -> String;
    fn fg_seagreen1b(self) -> String;
    fn fg_aquamarine1a(self) -> String;
    fn fg_darkslategray2(self) -> String;
    fn fg_darkred2(self) -> String;
    fn fg_deeppink4b(self) -> String;
    fn fg_darkmagenta1(self) -> String;
    fn fg_darkmagenta2(self) -> String;
    fn fg_darkviolet1a(self) -> String;
    fn fg_purple1a(self) -> String;
    fn fg_orange4b(self) -> String;
    fn fg_lightpink4(self) -> String;
    fn fg_plum4(self) -> String;
    fn fg_mediumpurple3a(self) -> String;
    fn fg_mediumpurple3b(self) -> String;
    fn fg_slateblue1(self) -> String;
    fn fg_yellow4a(self) -> String;
    fn fg_wheat4(self) -> String;
    fn fg_grey53(self) -> String;
    fn fg_lightslategrey(self) -> String;
    fn fg_mediumpurple(self) -> String;
    fn fg_lightslateblue(self) -> String;
    fn fg_yellow4b(self) -> String;
    fn fg_darkolivegreen3a(self) -> String;
    fn fg_darkgreensea(self) -> String;
    fn fg_lightskyblue3a(self) -> String;
    fn fg_lightskyblue3b(self) -> String;
    fn fg_skyblue2(self) -> String;
    fn fg_chartreuse2b(self) -> String;
    fn fg_darkolivegreen3b(self) -> String;
    fn fg_palegreen3b(self) -> String;
    fn fg_darkseagreen3a(self) -> String;
    fn fg_darkslategray3(self) -> String;
    fn fg_skyblue1(self) -> String;
    fn fg_chartreuse1(self) -> String;
    fn fg_lightgreen2(self) -> String;
    fn fg_lightgreen3(self) -> String;
    fn fg_palegreen1a(self) -> String;
    fn fg_aquamarine1b(self) -> String;
    fn fg_darkslategray1(self) -> String;
    fn fg_red3a(self) -> String;
    fn fg_deeppink4c(self) -> String;
    fn fg_mediumvioletred(self) -> String;
    fn fg_magenta3a(self) -> String;
    fn fg_darkviolet1b(self) -> String;
    fn fg_purple1b(self) -> String;
    fn fg_darkorange3a(self) -> String;
    fn fg_indianred1a(self) -> String;
    fn fg_hotpink3a(self) -> String;
    fn fg_mediumorchid3(self) -> String;
    fn fg_mediumorchid(self) -> String;
    fn fg_mediumpurple2a(self) -> String;
    fn fg_darkgoldenrod(self) -> String;
    fn fg_lightsalmon3a(self) -> String;
    fn fg_rosybrown(self) -> String;
    fn fg_grey63(self) -> String;
    fn fg_mediumpurple2b(self) -> String;
    fn fg_mediumpurple1(self) -> String;
    fn fg_gold3a(self) -> String;
    fn fg_darkkhaki(self) -> String;
    fn fg_navajowhite3(self) -> String;
    fn fg_grey69(self) -> String;
    fn fg_lightsteelblue3(self) -> String;
    fn fg_lightsteelblue(self) -> String;
    fn fg_yellow3a(self) -> String;
    fn fg_darkolivegreen3(self) -> String;
    fn fg_darkseagreen3b(self) -> String;
    fn fg_darkseagreen2(self) -> String;
    fn fg_lightcyan3(self) -> String;
    fn fg_lightskyblue1(self) -> String;
    fn fg_greenyellow(self) -> String;
    fn fg_darkolivegreen2(self) -> String;
    fn fg_palegreen1b(self) -> String;
    fn fg_darkseagreen5b(self) -> String;
    fn fg_darkseagreen5a(self) -> String;
    fn fg_paleturquoise1(self) -> String;
    fn fg_red3b(self) -> String;
    fn fg_deeppink3a(self) -> String;
    fn fg_deeppink3b(self) -> String;
    fn fg_magenta3b(self) -> String;
    fn fg_magenta3c(self) -> String;
    fn fg_magenta2a(self) -> String;
    fn fg_darkorange3b(self) -> String;
    fn fg_indianred1b(self) -> String;
    fn fg_hotpink3b(self) -> String;
    fn fg_hotpink2(self) -> String;
    fn fg_orchid(self) -> String;
    fn fg_mediumorchid1a(self) -> String;
    fn fg_orange3(self) -> String;
    fn fg_lightsalmon3b(self) -> String;
    fn fg_lightpink3(self) -> String;
    fn fg_pink3(self) -> String;
    fn fg_plum3(self) -> String;
    fn fg_violet(self) -> String;
    fn fg_gold3b(self) -> String;
    fn fg_lightgoldenrod3(self) -> String;
    fn fg_tan(self) -> String;
    fn fg_mistyrose3(self) -> String;
    fn fg_thistle3(self) -> String;
    fn fg_plum2(self) -> String;
    fn fg_yellow3b(self) -> String;
    fn fg_khaki3(self) -> String;
    fn fg_lightgoldenrod2a(self) -> String;
    fn fg_lightyellow3(self) -> String;
    fn fg_grey84(self) -> String;
    fn fg_lightsteelblue1(self) -> String;
    fn fg_yellow2(self) -> String;
    fn fg_darkolivegreen1a(self) -> String;
    fn fg_darkolivegreen1b(self) -> String;
    fn fg_darkseagreen1(self) -> String;
    fn fg_honeydew2(self) -> String;
    fn fg_lightcyan1(self) -> String;
    fn fg_red1(self) -> String;
    fn fg_deeppink2(self) -> String;
    fn fg_deeppink1a(self) -> String;
    fn fg_deeppink1b(self) -> String;
    fn fg_magenta2b(self) -> String;
    fn fg_magenta1(self) -> String;
    fn fg_orangered1(self) -> String;
    fn fg_indianred1c(self) -> String;
    fn fg_indianred1d(self) -> String;
    fn fg_hotpink1a(self) -> String;
    fn fg_hotpink1b(self) -> String;
    fn fg_mediumorchid1b(self) -> String;
    fn fg_darkorange(self) -> String;
    fn fg_salmon1(self) -> String;
    fn fg_lightcoral(self) -> String;
    fn fg_palevioletred1(self) -> String;
    fn fg_orchid2(self) -> String;
    fn fg_orchid1(self) -> String;
    fn fg_orange1(self) -> String;
    fn fg_sandybrown(self) -> String;
    fn fg_lightsalmon1(self) -> String;
    fn fg_lightpink1(self) -> String;
    fn fg_pink1(self) -> String;
    fn fg_plum1(self) -> String;
    fn fg_gold1(self) -> String;
    fn fg_lightgoldenrod2b(self) -> String;
    fn fg_lightgoldenrod2c(self) -> String;
    fn fg_navajowhite1(self) -> String;
    fn fg_mistyrose1(self) -> String;
    fn fg_thistle1(self) -> String;
    fn fg_yellow1(self) -> String;
    fn fg_lightgoldenrod1(self) -> String;
    fn fg_khaki1(self) -> String;
    fn fg_wheat1(self) -> String;
    fn fg_cornsilk1(self) -> String;
    fn fg_grey100(self) -> String;
    fn fg_grey3(self) -> String;
    fn fg_grey7(self) -> String;
    fn fg_grey11(self) -> String;
    fn fg_grey15(self) -> String;
    fn fg_grey19(self) -> String;
    fn fg_grey23(self) -> String;
    fn fg_grey27(self) -> String;
    fn fg_grey30(self) -> String;
    fn fg_grey35(self) -> String;
    fn fg_grey39(self) -> String;
    fn fg_grey42(self) -> String;
    fn fg_grey46(self) -> String;
    fn fg_grey50(self) -> String;
    fn fg_grey54(self) -> String;
    fn fg_grey58(self) -> String;
    fn fg_grey62(self) -> String;
    fn fg_grey66(self) -> String;
    fn fg_grey70(self) -> String;
    fn fg_grey74(self) -> String;
    fn fg_grey78(self) -> String;
    fn fg_grey82(self) -> String;
    fn fg_grey85(self) -> String;
    fn fg_grey89(self) -> String;
    fn fg_grey93(self) -> String;

    // background
    fn bg_black(self) -> String;
    fn bg_red(self) -> String;
    fn bg_green(self) -> String;
    fn bg_yellow(self) -> String;
    fn bg_blue(self) -> String;
    fn bg_magenta(self) -> String;
    fn bg_cyan(self) -> String;
    fn bg_lightgray(self) -> String;
    fn bg_darkgray(self) -> String;
    fn bg_lightred(self) -> String;
    fn bg_lightgreen(self) -> String;
    fn bg_lightyellow(self) -> String;
    fn bg_lightblue(self) -> String;
    fn bg_lightmagenta(self) -> String;
    fn bg_lightcyan(self) -> String;
    fn bg_white(self) -> String;
    fn bg_grey0(self) -> String;
    fn bg_navyblue(self) -> String;
    fn bg_darkblue(self) -> String;
    fn bg_blue3a(self) -> String;
    fn bg_blue3b(self) -> String;
    fn bg_blue1(self) -> String;
    fn bg_darkgreen(self) -> String;
    fn bg_deepskyblue4a(self) -> String;
    fn bg_deepskyblue4b(self) -> String;
    fn bg_deepskyblue4c(self) -> String;
    fn bg_dodgerblue3(self) -> String;
    fn bg_dodgerblue2(self) -> String;
    fn bg_green4(self) -> String;
    fn bg_springgreen4(self) -> String;
    fn bg_turquoise4(self) -> String;
    fn bg_deepskyblue3a(self) -> String;
    fn bg_deepskyblue3b(self) -> String;
    fn bg_dodgerblue1(self) -> String;
    fn bg_green3a(self) -> String;
    fn bg_springgreen3a(self) -> String;
    fn bg_darkcyan(self) -> String;
    fn bg_lightseagreen(self) -> String;
    fn bg_deepskyblue2(self) -> String;
    fn bg_deepskyblue1(self) -> String;
    fn bg_green3b(self) -> String;
    fn bg_springgreen3b(self) -> String;
    fn bg_springgreen2a(self) -> String;
    fn bg_cyan3(self) -> String;
    fn bg_darkturquoise(self) -> String;
    fn bg_turquoise2(self) -> String;
    fn bg_green1(self) -> String;
    fn bg_springgreen2b(self) -> String;
    fn bg_springgreen1(self) -> String;
    fn bg_mediumspringgreen(self) -> String;
    fn bg_cyan2(self) -> String;
    fn bg_cyan1(self) -> String;
    fn bg_darkred1(self) -> String;
    fn bg_deeppink4a(self) -> String;
    fn bg_purple4a(self) -> String;
    fn bg_purple4b(self) -> String;
    fn bg_purple3(self) -> String;
    fn bg_blueviolet(self) -> String;
    fn bg_orange4a(self) -> String;
    fn bg_grey37(self) -> String;
    fn bg_mediumpurple4(self) -> String;
    fn bg_slateblue3a(self) -> String;
    fn bg_slateblue3b(self) -> String;
    fn bg_royalblue1(self) -> String;
    fn bg_chartreuse4(self) -> String;
    fn bg_darkseagreen4a(self) -> String;
    fn bg_paleturquoise4(self) -> String;
    fn bg_steelblue(self) -> String;
    fn bg_steelblue3(self) -> String;
    fn bg_cornflowerblue(self) -> String;
    fn bg_chartreuse3a(self) -> String;
    fn bg_darkseagreen4b(self) -> String;
    fn bg_cadetblue2(self) -> String;
    fn bg_cadetblue1(self) -> String;
    fn bg_skyblue3(self) -> String;
    fn bg_steelblue1a(self) -> String;
    fn bg_chartreuse3b(self) -> String;
    fn bg_palegreen3a(self) -> String;
    fn bg_seagreen3(self) -> String;
    fn bg_aquamarine3(self) -> String;
    fn bg_mediumturquoise(self) -> String;
    fn bg_steelblue1b(self) -> String;
    fn bg_chartreuse2a(self) -> String;
    fn bg_seagreen2(self) -> String;
    fn bg_seagreen1a(self) -> String;
    fn bg_seagreen1b(self) -> String;
    fn bg_aquamarine1a(self) -> String;
    fn bg_darkslategray2(self) -> String;
    fn bg_darkred2(self) -> String;
    fn bg_deeppink4b(self) -> String;
    fn bg_darkmagenta1(self) -> String;
    fn bg_darkmagenta2(self) -> String;
    fn bg_darkviolet1a(self) -> String;
    fn bg_purple1a(self) -> String;
    fn bg_orange4b(self) -> String;
    fn bg_lightpink4(self) -> String;
    fn bg_plum4(self) -> String;
    fn bg_mediumpurple3a(self) -> String;
    fn bg_mediumpurple3b(self) -> String;
    fn bg_slateblue1(self) -> String;
    fn bg_yellow4a(self) -> String;
    fn bg_wheat4(self) -> String;
    fn bg_grey53(self) -> String;
    fn bg_lightslategrey(self) -> String;
    fn bg_mediumpurple(self) -> String;
    fn bg_lightslateblue(self) -> String;
    fn bg_yellow4b(self) -> String;
    fn bg_darkolivegreen3a(self) -> String;
    fn bg_darkgreensea(self) -> String;
    fn bg_lightskyblue3a(self) -> String;
    fn bg_lightskyblue3b(self) -> String;
    fn bg_skyblue2(self) -> String;
    fn bg_chartreuse2b(self) -> String;
    fn bg_darkolivegreen3b(self) -> String;
    fn bg_palegreen3b(self) -> String;
    fn bg_darkseagreen3a(self) -> String;
    fn bg_darkslategray3(self) -> String;
    fn bg_skyblue1(self) -> String;
    fn bg_chartreuse1(self) -> String;
    fn bg_lightgreen2(self) -> String;
    fn bg_lightgreen3(self) -> String;
    fn bg_palegreen1a(self) -> String;
    fn bg_aquamarine1b(self) -> String;
    fn bg_darkslategray1(self) -> String;
    fn bg_red3a(self) -> String;
    fn bg_deeppink4c(self) -> String;
    fn bg_mediumvioletred(self) -> String;
    fn bg_magenta3a(self) -> String;
    fn bg_darkviolet1b(self) -> String;
    fn bg_purple1b(self) -> String;
    fn bg_darkorange3a(self) -> String;
    fn bg_indianred1a(self) -> String;
    fn bg_hotpink3a(self) -> String;
    fn bg_mediumorchid3(self) -> String;
    fn bg_mediumorchid(self) -> String;
    fn bg_mediumpurple2a(self) -> String;
    fn bg_darkgoldenrod(self) -> String;
    fn bg_lightsalmon3a(self) -> String;
    fn bg_rosybrown(self) -> String;
    fn bg_grey63(self) -> String;
    fn bg_mediumpurple2b(self) -> String;
    fn bg_mediumpurple1(self) -> String;
    fn bg_gold3a(self) -> String;
    fn bg_darkkhaki(self) -> String;
    fn bg_navajowhite3(self) -> String;
    fn bg_grey69(self) -> String;
    fn bg_lightsteelblue3(self) -> String;
    fn bg_lightsteelblue(self) -> String;
    fn bg_yellow3a(self) -> String;
    fn bg_darkolivegreen3(self) -> String;
    fn bg_darkseagreen3b(self) -> String;
    fn bg_darkseagreen2(self) -> String;
    fn bg_lightcyan3(self) -> String;
    fn bg_lightskyblue1(self) -> String;
    fn bg_greenyellow(self) -> String;
    fn bg_darkolivegreen2(self) -> String;
    fn bg_palegreen1b(self) -> String;
    fn bg_darkseagreen5b(self) -> String;
    fn bg_darkseagreen5a(self) -> String;
    fn bg_paleturquoise1(self) -> String;
    fn bg_red3b(self) -> String;
    fn bg_deeppink3a(self) -> String;
    fn bg_deeppink3b(self) -> String;
    fn bg_magenta3b(self) -> String;
    fn bg_magenta3c(self) -> String;
    fn bg_magenta2a(self) -> String;
    fn bg_darkorange3b(self) -> String;
    fn bg_indianred1b(self) -> String;
    fn bg_hotpink3b(self) -> String;
    fn bg_hotpink2(self) -> String;
    fn bg_orchid(self) -> String;
    fn bg_mediumorchid1a(self) -> String;
    fn bg_orange3(self) -> String;
    fn bg_lightsalmon3b(self) -> String;
    fn bg_lightpink3(self) -> String;
    fn bg_pink3(self) -> String;
    fn bg_plum3(self) -> String;
    fn bg_violet(self) -> String;
    fn bg_gold3b(self) -> String;
    fn bg_lightgoldenrod3(self) -> String;
    fn bg_tan(self) -> String;
    fn bg_mistyrose3(self) -> String;
    fn bg_thistle3(self) -> String;
    fn bg_plum2(self) -> String;
    fn bg_yellow3b(self) -> String;
    fn bg_khaki3(self) -> String;
    fn bg_lightgoldenrod2a(self) -> String;
    fn bg_lightyellow3(self) -> String;
    fn bg_grey84(self) -> String;
    fn bg_lightsteelblue1(self) -> String;
    fn bg_yellow2(self) -> String;
    fn bg_darkolivegreen1a(self) -> String;
    fn bg_darkolivegreen1b(self) -> String;
    fn bg_darkseagreen1(self) -> String;
    fn bg_honeydew2(self) -> String;
    fn bg_lightcyan1(self) -> String;
    fn bg_red1(self) -> String;
    fn bg_deeppink2(self) -> String;
    fn bg_deeppink1a(self) -> String;
    fn bg_deeppink1b(self) -> String;
    fn bg_magenta2b(self) -> String;
    fn bg_magenta1(self) -> String;
    fn bg_orangered1(self) -> String;
    fn bg_indianred1c(self) -> String;
    fn bg_indianred1d(self) -> String;
    fn bg_hotpink1a(self) -> String;
    fn bg_hotpink1b(self) -> String;
    fn bg_mediumorchid1b(self) -> String;
    fn bg_darkorange(self) -> String;
    fn bg_salmon1(self) -> String;
    fn bg_lightcoral(self) -> String;
    fn bg_palevioletred1(self) -> String;
    fn bg_orchid2(self) -> String;
    fn bg_orchid1(self) -> String;
    fn bg_orange1(self) -> String;
    fn bg_sandybrown(self) -> String;
    fn bg_lightsalmon1(self) -> String;
    fn bg_lightpink1(self) -> String;
    fn bg_pink1(self) -> String;
    fn bg_plum1(self) -> String;
    fn bg_gold1(self) -> String;
    fn bg_lightgoldenrod2b(self) -> String;
    fn bg_lightgoldenrod2c(self) -> String;
    fn bg_navajowhite1(self) -> String;
    fn bg_mistyrose1(self) -> String;
    fn bg_thistle1(self) -> String;
    fn bg_yellow1(self) -> String;
    fn bg_lightgoldenrod1(self) -> String;
    fn bg_khaki1(self) -> String;
    fn bg_wheat1(self) -> String;
    fn bg_cornsilk1(self) -> String;
    fn bg_grey100(self) -> String;
    fn bg_grey3(self) -> String;
    fn bg_grey7(self) -> String;
    fn bg_grey11(self) -> String;
    fn bg_grey15(self) -> String;
    fn bg_grey19(self) -> String;
    fn bg_grey23(self) -> String;
    fn bg_grey27(self) -> String;
    fn bg_grey30(self) -> String;
    fn bg_grey35(self) -> String;
    fn bg_grey39(self) -> String;
    fn bg_grey42(self) -> String;
    fn bg_grey46(self) -> String;
    fn bg_grey50(self) -> String;
    fn bg_grey54(self) -> String;
    fn bg_grey58(self) -> String;
    fn bg_grey62(self) -> String;
    fn bg_grey66(self) -> String;
    fn bg_grey70(self) -> String;
    fn bg_grey74(self) -> String;
    fn bg_grey78(self) -> String;
    fn bg_grey82(self) -> String;
    fn bg_grey85(self) -> String;
    fn bg_grey89(self) -> String;
    fn bg_grey93(self) -> String;
}

impl<T> ColorString for T where T: StringMarker {
    fn fg_color<C: colors::Color>(self, color: C) -> String {
        color.to_fg_color_string()
    }
    fn bg_color<C: colors::Color>(self, color: C) -> String {
        color.to_bg_color_string()
    }
    fn fg_black(self) -> String { EightBitColors::Black.to_fg_color_string() }
    fn fg_red(self) -> String { EightBitColors::Red.to_fg_color_string() }
    fn fg_green(self) -> String {EightBitColors::Green.to_fg_color_string()}
    fn fg_yellow(self) -> String {EightBitColors::Yellow.to_fg_color_string()}
    fn fg_blue(self) -> String {EightBitColors::Blue.to_fg_color_string()}
    fn fg_magenta(self) -> String {EightBitColors::Magenta.to_fg_color_string()}
    fn fg_cyan(self) -> String {EightBitColors::Cyan.to_fg_color_string()}
    fn fg_lightgray(self) -> String {EightBitColors::LightGray.to_fg_color_string()}
    fn fg_darkgray(self) -> String {EightBitColors::DarkGray.to_fg_color_string()}
    fn fg_lightred(self) -> String {EightBitColors::LightRed.to_fg_color_string()}
    fn fg_lightgreen(self) -> String {EightBitColors::LightGreen.to_fg_color_string()}
    fn fg_lightyellow(self) -> String {EightBitColors::LightYellow.to_fg_color_string()}
    fn fg_lightblue(self) -> String {EightBitColors::LightBlue.to_fg_color_string()}
    fn fg_lightmagenta(self) -> String {EightBitColors::LightMagenta.to_fg_color_string()}
    fn fg_lightcyan(self) -> String {EightBitColors::LightCyan.to_fg_color_string()}
    fn fg_white(self) -> String {EightBitColors::White.to_fg_color_string()}
    fn fg_grey0(self) -> String {EightBitColors::Grey0.to_fg_color_string()}
    fn fg_navyblue(self) -> String {EightBitColors::NavyBlue.to_fg_color_string()}
    fn fg_darkblue(self) -> String {EightBitColors::DarkBlue.to_fg_color_string()}
    fn fg_blue3a(self) -> String {EightBitColors::Blue3a.to_fg_color_string()}
    fn fg_blue3b(self) -> String {EightBitColors::Blue3b.to_fg_color_string()}
    fn fg_blue1(self) -> String {EightBitColors::Blue1.to_fg_color_string()}
    fn fg_darkgreen(self) -> String {EightBitColors::DarkGreen.to_fg_color_string()}
    fn fg_deepskyblue4a(self) -> String {EightBitColors::DeepSkyBlue4a.to_fg_color_string()}
    fn fg_deepskyblue4b(self) -> String {EightBitColors::DeepSkyBlue4b.to_fg_color_string()}
    fn fg_deepskyblue4c(self) -> String {EightBitColors::DeepSkyBlue4c.to_fg_color_string()}
    fn fg_dodgerblue3(self) -> String {EightBitColors::DodgerBlue3.to_fg_color_string()}
    fn fg_dodgerblue2(self) -> String {EightBitColors::DodgerBlue2.to_fg_color_string()}
    fn fg_green4(self) -> String {EightBitColors::Green4.to_fg_color_string()}
    fn fg_springgreen4(self) -> String {EightBitColors::SpringGreen4.to_fg_color_string()}
    fn fg_turquoise4(self) -> String {EightBitColors::Turquoise4.to_fg_color_string()}
    fn fg_deepskyblue3a(self) -> String {EightBitColors::DeepSkyBlue3a.to_fg_color_string()}
    fn fg_deepskyblue3b(self) -> String {EightBitColors::DeepSkyBlue3b.to_fg_color_string()}
    fn fg_dodgerblue1(self) -> String {EightBitColors::DodgerBlue1.to_fg_color_string()}
    fn fg_green3a(self) -> String {EightBitColors::Green3a.to_fg_color_string()}
    fn fg_springgreen3a(self) -> String {EightBitColors::SpringGreen3a.to_fg_color_string()}
    fn fg_darkcyan(self) -> String {EightBitColors::DarkCyan.to_fg_color_string()}
    fn fg_lightseagreen(self) -> String {EightBitColors::LightSeaGreen.to_fg_color_string()}
    fn fg_deepskyblue2(self) -> String {EightBitColors::DeepSkyBlue2.to_fg_color_string()}
    fn fg_deepskyblue1(self) -> String {EightBitColors::DeepSkyBlue1.to_fg_color_string()}
    fn fg_green3b(self) -> String {EightBitColors::Green3b.to_fg_color_string()}
    fn fg_springgreen3b(self) -> String {EightBitColors::SpringGreen3b.to_fg_color_string()}
    fn fg_springgreen2a(self) -> String {EightBitColors::SpringGreen2a.to_fg_color_string()}
    fn fg_cyan3(self) -> String {EightBitColors::Cyan3.to_fg_color_string()}
    fn fg_darkturquoise(self) -> String {EightBitColors::DarkTurquoise.to_fg_color_string()}
    fn fg_turquoise2(self) -> String {EightBitColors::Turquoise2.to_fg_color_string()}
    fn fg_green1(self) -> String {EightBitColors::Green1.to_fg_color_string()}
    fn fg_springgreen2b(self) -> String {EightBitColors::SpringGreen2b.to_fg_color_string()}
    fn fg_springgreen1(self) -> String {EightBitColors::SpringGreen1.to_fg_color_string()}
    fn fg_mediumspringgreen(self) -> String {EightBitColors::MediumSpringGreen.to_fg_color_string()}
    fn fg_cyan2(self) -> String {EightBitColors::Cyan2.to_fg_color_string()}
    fn fg_cyan1(self) -> String {EightBitColors::Cyan1.to_fg_color_string()}
    fn fg_darkred1(self) -> String {EightBitColors::DarkRed1.to_fg_color_string()}
    fn fg_deeppink4a(self) -> String {EightBitColors::DeepPink4a.to_fg_color_string()}
    fn fg_purple4a(self) -> String {EightBitColors::Purple4a.to_fg_color_string()}
    fn fg_purple4b(self) -> String {EightBitColors::Purple4b.to_fg_color_string()}
    fn fg_purple3(self) -> String {EightBitColors::Purple3.to_fg_color_string()}
    fn fg_blueviolet(self) -> String {EightBitColors::BlueViolet.to_fg_color_string()}
    fn fg_orange4a(self) -> String {EightBitColors::Orange4a.to_fg_color_string()}
    fn fg_grey37(self) -> String {EightBitColors::Grey37.to_fg_color_string()}
    fn fg_mediumpurple4(self) -> String {EightBitColors::MediumPurple4.to_fg_color_string()}
    fn fg_slateblue3a(self) -> String {EightBitColors::SlateBlue3a.to_fg_color_string()}
    fn fg_slateblue3b(self) -> String {EightBitColors::SlateBlue3b.to_fg_color_string()}
    fn fg_royalblue1(self) -> String {EightBitColors::RoyalBlue1.to_fg_color_string()}
    fn fg_chartreuse4(self) -> String {EightBitColors::Chartreuse4.to_fg_color_string()}
    fn fg_darkseagreen4a(self) -> String {EightBitColors::DarkSeaGreen4a.to_fg_color_string()}
    fn fg_paleturquoise4(self) -> String {EightBitColors::PaleTurquoise4.to_fg_color_string()}
    fn fg_steelblue(self) -> String {EightBitColors::SteelBlue.to_fg_color_string()}
    fn fg_steelblue3(self) -> String {EightBitColors::SteelBlue3.to_fg_color_string()}
    fn fg_cornflowerblue(self) -> String {EightBitColors::CornflowerBlue.to_fg_color_string()}
    fn fg_chartreuse3a(self) -> String {EightBitColors::Chartreuse3a.to_fg_color_string()}
    fn fg_darkseagreen4b(self) -> String {EightBitColors::DarkSeaGreen4b.to_fg_color_string()}
    fn fg_cadetblue2(self) -> String {EightBitColors::CadetBlue2.to_fg_color_string()}
    fn fg_cadetblue1(self) -> String {EightBitColors::CadetBlue1.to_fg_color_string()}
    fn fg_skyblue3(self) -> String {EightBitColors::SkyBlue3.to_fg_color_string()}
    fn fg_steelblue1a(self) -> String {EightBitColors::SteelBlue1a.to_fg_color_string()}
    fn fg_chartreuse3b(self) -> String {EightBitColors::Chartreuse3b.to_fg_color_string()}
    fn fg_palegreen3a(self) -> String {EightBitColors::PaleGreen3a.to_fg_color_string()}
    fn fg_seagreen3(self) -> String {EightBitColors::SeaGreen3.to_fg_color_string()}
    fn fg_aquamarine3(self) -> String {EightBitColors::Aquamarine3.to_fg_color_string()}
    fn fg_mediumturquoise(self) -> String {EightBitColors::MediumTurquoise.to_fg_color_string()}
    fn fg_steelblue1b(self) -> String {EightBitColors::SteelBlue1b.to_fg_color_string()}
    fn fg_chartreuse2a(self) -> String {EightBitColors::Chartreuse2a.to_fg_color_string()}
    fn fg_seagreen2(self) -> String {EightBitColors::SeaGreen2.to_fg_color_string()}
    fn fg_seagreen1a(self) -> String {EightBitColors::SeaGreen1a.to_fg_color_string()}
    fn fg_seagreen1b(self) -> String {EightBitColors::SeaGreen1b.to_fg_color_string()}
    fn fg_aquamarine1a(self) -> String {EightBitColors::Aquamarine1a.to_fg_color_string()}
    fn fg_darkslategray2(self) -> String {EightBitColors::DarkSlateGray2.to_fg_color_string()}
    fn fg_darkred2(self) -> String {EightBitColors::DarkRed2.to_fg_color_string()}
    fn fg_deeppink4b(self) -> String {EightBitColors::DeepPink4b.to_fg_color_string()}
    fn fg_darkmagenta1(self) -> String {EightBitColors::DarkMagenta1.to_fg_color_string()}
    fn fg_darkmagenta2(self) -> String {EightBitColors::DarkMagenta2.to_fg_color_string()}
    fn fg_darkviolet1a(self) -> String {EightBitColors::DarkViolet1a.to_fg_color_string()}
    fn fg_purple1a(self) -> String {EightBitColors::Purple1a.to_fg_color_string()}
    fn fg_orange4b(self) -> String {EightBitColors::Orange4b.to_fg_color_string()}
    fn fg_lightpink4(self) -> String {EightBitColors::LightPink4.to_fg_color_string()}
    fn fg_plum4(self) -> String {EightBitColors::Plum4.to_fg_color_string()}
    fn fg_mediumpurple3a(self) -> String {EightBitColors::MediumPurple3a.to_fg_color_string()}
    fn fg_mediumpurple3b(self) -> String {EightBitColors::MediumPurple3b.to_fg_color_string()}
    fn fg_slateblue1(self) -> String {EightBitColors::SlateBlue1.to_fg_color_string()}
    fn fg_yellow4a(self) -> String {EightBitColors::Yellow4a.to_fg_color_string()}
    fn fg_wheat4(self) -> String {EightBitColors::Wheat4.to_fg_color_string()}
    fn fg_grey53(self) -> String {EightBitColors::Grey53.to_fg_color_string()}
    fn fg_lightslategrey(self) -> String {EightBitColors::LightSlateGrey.to_fg_color_string()}
    fn fg_mediumpurple(self) -> String {EightBitColors::MediumPurple.to_fg_color_string()}
    fn fg_lightslateblue(self) -> String {EightBitColors::LightSlateBlue.to_fg_color_string()}
    fn fg_yellow4b(self) -> String {EightBitColors::Yellow4b.to_fg_color_string()}
    fn fg_darkolivegreen3a(self) -> String {EightBitColors::DarkOliveGreen3a.to_fg_color_string()}
    fn fg_darkgreensea(self) -> String {EightBitColors::DarkGreenSea.to_fg_color_string()}
    fn fg_lightskyblue3a(self) -> String {EightBitColors::LightSkyBlue3a.to_fg_color_string()}
    fn fg_lightskyblue3b(self) -> String {EightBitColors::LightSkyBlue3b.to_fg_color_string()}
    fn fg_skyblue2(self) -> String {EightBitColors::SkyBlue2.to_fg_color_string()}
    fn fg_chartreuse2b(self) -> String {EightBitColors::Chartreuse2b.to_fg_color_string()}
    fn fg_darkolivegreen3b(self) -> String {EightBitColors::DarkOliveGreen3b.to_fg_color_string()}
    fn fg_palegreen3b(self) -> String {EightBitColors::PaleGreen3b.to_fg_color_string()}
    fn fg_darkseagreen3a(self) -> String {EightBitColors::DarkSeaGreen3a.to_fg_color_string()}
    fn fg_darkslategray3(self) -> String {EightBitColors::DarkSlateGray3.to_fg_color_string()}
    fn fg_skyblue1(self) -> String {EightBitColors::SkyBlue1.to_fg_color_string()}
    fn fg_chartreuse1(self) -> String {EightBitColors::Chartreuse1.to_fg_color_string()}
    fn fg_lightgreen2(self) -> String {EightBitColors::LightGreen2.to_fg_color_string()}
    fn fg_lightgreen3(self) -> String {EightBitColors::LightGreen3.to_fg_color_string()}
    fn fg_palegreen1a(self) -> String {EightBitColors::PaleGreen1a.to_fg_color_string()}
    fn fg_aquamarine1b(self) -> String {EightBitColors::Aquamarine1b.to_fg_color_string()}
    fn fg_darkslategray1(self) -> String {EightBitColors::DarkSlateGray1.to_fg_color_string()}
    fn fg_red3a(self) -> String {EightBitColors::Red3a.to_fg_color_string()}
    fn fg_deeppink4c(self) -> String {EightBitColors::DeepPink4c.to_fg_color_string()}
    fn fg_mediumvioletred(self) -> String {EightBitColors::MediumVioletRed.to_fg_color_string()}
    fn fg_magenta3a(self) -> String {EightBitColors::Magenta3a.to_fg_color_string()}
    fn fg_darkviolet1b(self) -> String {EightBitColors::DarkViolet1b.to_fg_color_string()}
    fn fg_purple1b(self) -> String {EightBitColors::Purple1b.to_fg_color_string()}
    fn fg_darkorange3a(self) -> String {EightBitColors::DarkOrange3a.to_fg_color_string()}
    fn fg_indianred1a(self) -> String {EightBitColors::IndianRed1a.to_fg_color_string()}
    fn fg_hotpink3a(self) -> String {EightBitColors::HotPink3a.to_fg_color_string()}
    fn fg_mediumorchid3(self) -> String {EightBitColors::MediumOrchid3.to_fg_color_string()}
    fn fg_mediumorchid(self) -> String {EightBitColors::MediumOrchid.to_fg_color_string()}
    fn fg_mediumpurple2a(self) -> String {EightBitColors::MediumPurple2a.to_fg_color_string()}
    fn fg_darkgoldenrod(self) -> String {EightBitColors::DarkGoldenrod.to_fg_color_string()}
    fn fg_lightsalmon3a(self) -> String {EightBitColors::LightSalmon3a.to_fg_color_string()}
    fn fg_rosybrown(self) -> String {EightBitColors::RosyBrown.to_fg_color_string()}
    fn fg_grey63(self) -> String {EightBitColors::Grey63.to_fg_color_string()}
    fn fg_mediumpurple2b(self) -> String {EightBitColors::MediumPurple2b.to_fg_color_string()}
    fn fg_mediumpurple1(self) -> String {EightBitColors::MediumPurple1.to_fg_color_string()}
    fn fg_gold3a(self) -> String {EightBitColors::Gold3a.to_fg_color_string()}
    fn fg_darkkhaki(self) -> String {EightBitColors::DarkKhaki.to_fg_color_string()}
    fn fg_navajowhite3(self) -> String {EightBitColors::NavajoWhite3.to_fg_color_string()}
    fn fg_grey69(self) -> String {EightBitColors::Grey69.to_fg_color_string()}
    fn fg_lightsteelblue3(self) -> String {EightBitColors::LightSteelBlue3.to_fg_color_string()}
    fn fg_lightsteelblue(self) -> String {EightBitColors::LightSteelBlue.to_fg_color_string()}
    fn fg_yellow3a(self) -> String {EightBitColors::Yellow3a.to_fg_color_string()}
    fn fg_darkolivegreen3(self) -> String {EightBitColors::DarkOliveGreen3.to_fg_color_string()}
    fn fg_darkseagreen3b(self) -> String {EightBitColors::DarkSeaGreen3b.to_fg_color_string()}
    fn fg_darkseagreen2(self) -> String {EightBitColors::DarkSeaGreen2.to_fg_color_string()}
    fn fg_lightcyan3(self) -> String {EightBitColors::LightCyan3.to_fg_color_string()}
    fn fg_lightskyblue1(self) -> String {EightBitColors::LightSkyBlue1.to_fg_color_string()}
    fn fg_greenyellow(self) -> String {EightBitColors::GreenYellow.to_fg_color_string()}
    fn fg_darkolivegreen2(self) -> String {EightBitColors::DarkOliveGreen2.to_fg_color_string()}
    fn fg_palegreen1b(self) -> String {EightBitColors::PaleGreen1b.to_fg_color_string()}
    fn fg_darkseagreen5b(self) -> String {EightBitColors::DarkSeaGreen5b.to_fg_color_string()}
    fn fg_darkseagreen5a(self) -> String {EightBitColors::DarkSeaGreen5a.to_fg_color_string()}
    fn fg_paleturquoise1(self) -> String {EightBitColors::PaleTurquoise1.to_fg_color_string()}
    fn fg_red3b(self) -> String {EightBitColors::Red3b.to_fg_color_string()}
    fn fg_deeppink3a(self) -> String {EightBitColors::DeepPink3a.to_fg_color_string()}
    fn fg_deeppink3b(self) -> String {EightBitColors::DeepPink3b.to_fg_color_string()}
    fn fg_magenta3b(self) -> String {EightBitColors::Magenta3b.to_fg_color_string()}
    fn fg_magenta3c(self) -> String {EightBitColors::Magenta3c.to_fg_color_string()}
    fn fg_magenta2a(self) -> String {EightBitColors::Magenta2a.to_fg_color_string()}
    fn fg_darkorange3b(self) -> String {EightBitColors::DarkOrange3b.to_fg_color_string()}
    fn fg_indianred1b(self) -> String {EightBitColors::IndianRed1b.to_fg_color_string()}
    fn fg_hotpink3b(self) -> String {EightBitColors::HotPink3b.to_fg_color_string()}
    fn fg_hotpink2(self) -> String {EightBitColors::HotPink2.to_fg_color_string()}
    fn fg_orchid(self) -> String {EightBitColors::Orchid.to_fg_color_string()}
    fn fg_mediumorchid1a(self) -> String {EightBitColors::MediumOrchid1a.to_fg_color_string()}
    fn fg_orange3(self) -> String {EightBitColors::Orange3.to_fg_color_string()}
    fn fg_lightsalmon3b(self) -> String {EightBitColors::LightSalmon3b.to_fg_color_string()}
    fn fg_lightpink3(self) -> String {EightBitColors::LightPink3.to_fg_color_string()}
    fn fg_pink3(self) -> String {EightBitColors::Pink3.to_fg_color_string()}
    fn fg_plum3(self) -> String {EightBitColors::Plum3.to_fg_color_string()}
    fn fg_violet(self) -> String {EightBitColors::Violet.to_fg_color_string()}
    fn fg_gold3b(self) -> String {EightBitColors::Gold3b.to_fg_color_string()}
    fn fg_lightgoldenrod3(self) -> String {EightBitColors::LightGoldenrod3.to_fg_color_string()}
    fn fg_tan(self) -> String {EightBitColors::Tan.to_fg_color_string()}
    fn fg_mistyrose3(self) -> String {EightBitColors::MistyRose3.to_fg_color_string()}
    fn fg_thistle3(self) -> String {EightBitColors::Thistle3.to_fg_color_string()}
    fn fg_plum2(self) -> String {EightBitColors::Plum2.to_fg_color_string()}
    fn fg_yellow3b(self) -> String {EightBitColors::Yellow3b.to_fg_color_string()}
    fn fg_khaki3(self) -> String {EightBitColors::Khaki3.to_fg_color_string()}
    fn fg_lightgoldenrod2a(self) -> String {EightBitColors::LightGoldenrod2a.to_fg_color_string()}
    fn fg_lightyellow3(self) -> String {EightBitColors::LightYellow3.to_fg_color_string()}
    fn fg_grey84(self) -> String {EightBitColors::Grey84.to_fg_color_string()}
    fn fg_lightsteelblue1(self) -> String {EightBitColors::LightSteelBlue1.to_fg_color_string()}
    fn fg_yellow2(self) -> String {EightBitColors::Yellow2.to_fg_color_string()}
    fn fg_darkolivegreen1a(self) -> String {EightBitColors::DarkOliveGreen1a.to_fg_color_string()}
    fn fg_darkolivegreen1b(self) -> String {EightBitColors::DarkOliveGreen1b.to_fg_color_string()}
    fn fg_darkseagreen1(self) -> String {EightBitColors::DarkSeaGreen1.to_fg_color_string()}
    fn fg_honeydew2(self) -> String {EightBitColors::Honeydew2.to_fg_color_string()}
    fn fg_lightcyan1(self) -> String {EightBitColors::LightCyan1.to_fg_color_string()}
    fn fg_red1(self) -> String {EightBitColors::Red1.to_fg_color_string()}
    fn fg_deeppink2(self) -> String {EightBitColors::DeepPink2.to_fg_color_string()}
    fn fg_deeppink1a(self) -> String {EightBitColors::DeepPink1a.to_fg_color_string()}
    fn fg_deeppink1b(self) -> String {EightBitColors::DeepPink1b.to_fg_color_string()}
    fn fg_magenta2b(self) -> String {EightBitColors::Magenta2b.to_fg_color_string()}
    fn fg_magenta1(self) -> String {EightBitColors::Magenta1.to_fg_color_string()}
    fn fg_orangered1(self) -> String {EightBitColors::OrangeRed1.to_fg_color_string()}
    fn fg_indianred1c(self) -> String {EightBitColors::IndianRed1c.to_fg_color_string()}
    fn fg_indianred1d(self) -> String {EightBitColors::IndianRed1d.to_fg_color_string()}
    fn fg_hotpink1a(self) -> String {EightBitColors::HotPink1a.to_fg_color_string()}
    fn fg_hotpink1b(self) -> String {EightBitColors::HotPink1b.to_fg_color_string()}
    fn fg_mediumorchid1b(self) -> String {EightBitColors::MediumOrchid1b.to_fg_color_string()}
    fn fg_darkorange(self) -> String {EightBitColors::DarkOrange.to_fg_color_string()}
    fn fg_salmon1(self) -> String {EightBitColors::Salmon1.to_fg_color_string()}
    fn fg_lightcoral(self) -> String {EightBitColors::LightCoral.to_fg_color_string()}
    fn fg_palevioletred1(self) -> String {EightBitColors::PaleVioletRed1.to_fg_color_string()}
    fn fg_orchid2(self) -> String {EightBitColors::Orchid2.to_fg_color_string()}
    fn fg_orchid1(self) -> String {EightBitColors::Orchid1.to_fg_color_string()}
    fn fg_orange1(self) -> String {EightBitColors::Orange1.to_fg_color_string()}
    fn fg_sandybrown(self) -> String {EightBitColors::SandyBrown.to_fg_color_string()}
    fn fg_lightsalmon1(self) -> String {EightBitColors::LightSalmon1.to_fg_color_string()}
    fn fg_lightpink1(self) -> String {EightBitColors::LightPink1.to_fg_color_string()}
    fn fg_pink1(self) -> String {EightBitColors::Pink1.to_fg_color_string()}
    fn fg_plum1(self) -> String {EightBitColors::Plum1.to_fg_color_string()}
    fn fg_gold1(self) -> String {EightBitColors::Gold1.to_fg_color_string()}
    fn fg_lightgoldenrod2b(self) -> String {EightBitColors::LightGoldenrod2b.to_fg_color_string()}
    fn fg_lightgoldenrod2c(self) -> String {EightBitColors::LightGoldenrod2c.to_fg_color_string()}
    fn fg_navajowhite1(self) -> String {EightBitColors::NavajoWhite1.to_fg_color_string()}
    fn fg_mistyrose1(self) -> String {EightBitColors::MistyRose1.to_fg_color_string()}
    fn fg_thistle1(self) -> String {EightBitColors::Thistle1.to_fg_color_string()}
    fn fg_yellow1(self) -> String {EightBitColors::Yellow1.to_fg_color_string()}
    fn fg_lightgoldenrod1(self) -> String {EightBitColors::LightGoldenrod1.to_fg_color_string()}
    fn fg_khaki1(self) -> String {EightBitColors::Khaki1.to_fg_color_string()}
    fn fg_wheat1(self) -> String {EightBitColors::Wheat1.to_fg_color_string()}
    fn fg_cornsilk1(self) -> String {EightBitColors::CornSilk1.to_fg_color_string()}
    fn fg_grey100(self) -> String {EightBitColors::Grey100.to_fg_color_string()}
    fn fg_grey3(self) -> String {EightBitColors::Grey3.to_fg_color_string()}
    fn fg_grey7(self) -> String {EightBitColors::Grey7.to_fg_color_string()}
    fn fg_grey11(self) -> String {EightBitColors::Grey11.to_fg_color_string()}
    fn fg_grey15(self) -> String {EightBitColors::Grey15.to_fg_color_string()}
    fn fg_grey19(self) -> String {EightBitColors::Grey19.to_fg_color_string()}
    fn fg_grey23(self) -> String {EightBitColors::Grey23.to_fg_color_string()}
    fn fg_grey27(self) -> String {EightBitColors::Grey27.to_fg_color_string()}
    fn fg_grey30(self) -> String {EightBitColors::Grey30.to_fg_color_string()}
    fn fg_grey35(self) -> String {EightBitColors::Grey35.to_fg_color_string()}
    fn fg_grey39(self) -> String {EightBitColors::Grey39.to_fg_color_string()}
    fn fg_grey42(self) -> String {EightBitColors::Grey42.to_fg_color_string()}
    fn fg_grey46(self) -> String {EightBitColors::Grey46.to_fg_color_string()}
    fn fg_grey50(self) -> String {EightBitColors::Grey50.to_fg_color_string()}
    fn fg_grey54(self) -> String {EightBitColors::Grey54.to_fg_color_string()}
    fn fg_grey58(self) -> String {EightBitColors::Grey58.to_fg_color_string()}
    fn fg_grey62(self) -> String {EightBitColors::Grey62.to_fg_color_string()}
    fn fg_grey66(self) -> String {EightBitColors::Grey66.to_fg_color_string()}
    fn fg_grey70(self) -> String {EightBitColors::Grey70.to_fg_color_string()}
    fn fg_grey74(self) -> String {EightBitColors::Grey74.to_fg_color_string()}
    fn fg_grey78(self) -> String {EightBitColors::Grey78.to_fg_color_string()}
    fn fg_grey82(self) -> String {EightBitColors::Grey82.to_fg_color_string()}
    fn fg_grey85(self) -> String {EightBitColors::Grey85.to_fg_color_string()}
    fn fg_grey89(self) -> String {EightBitColors::Grey89.to_fg_color_string()}
    fn fg_grey93(self) -> String {EightBitColors::Grey93.to_fg_color_string()}

    fn bg_black(self) -> String { EightBitColors::Black.to_bg_color_string() }
    fn bg_red(self) -> String { EightBitColors::Red.to_bg_color_string() }
    fn bg_green(self) -> String {EightBitColors::Green.to_bg_color_string()}
    fn bg_yellow(self) -> String {EightBitColors::Yellow.to_bg_color_string()}
    fn bg_blue(self) -> String {EightBitColors::Blue.to_bg_color_string()}
    fn bg_magenta(self) -> String {EightBitColors::Magenta.to_bg_color_string()}
    fn bg_cyan(self) -> String {EightBitColors::Cyan.to_bg_color_string()}
    fn bg_lightgray(self) -> String {EightBitColors::LightGray.to_bg_color_string()}
    fn bg_darkgray(self) -> String {EightBitColors::DarkGray.to_bg_color_string()}
    fn bg_lightred(self) -> String {EightBitColors::LightRed.to_bg_color_string()}
    fn bg_lightgreen(self) -> String {EightBitColors::LightGreen.to_bg_color_string()}
    fn bg_lightyellow(self) -> String {EightBitColors::LightYellow.to_bg_color_string()}
    fn bg_lightblue(self) -> String {EightBitColors::LightBlue.to_bg_color_string()}
    fn bg_lightmagenta(self) -> String {EightBitColors::LightMagenta.to_bg_color_string()}
    fn bg_lightcyan(self) -> String {EightBitColors::LightCyan.to_bg_color_string()}
    fn bg_white(self) -> String {EightBitColors::White.to_bg_color_string()}
    fn bg_grey0(self) -> String {EightBitColors::Grey0.to_bg_color_string()}
    fn bg_navyblue(self) -> String {EightBitColors::NavyBlue.to_bg_color_string()}
    fn bg_darkblue(self) -> String {EightBitColors::DarkBlue.to_bg_color_string()}
    fn bg_blue3a(self) -> String {EightBitColors::Blue3a.to_bg_color_string()}
    fn bg_blue3b(self) -> String {EightBitColors::Blue3b.to_bg_color_string()}
    fn bg_blue1(self) -> String {EightBitColors::Blue1.to_bg_color_string()}
    fn bg_darkgreen(self) -> String {EightBitColors::DarkGreen.to_bg_color_string()}
    fn bg_deepskyblue4a(self) -> String {EightBitColors::DeepSkyBlue4a.to_bg_color_string()}
    fn bg_deepskyblue4b(self) -> String {EightBitColors::DeepSkyBlue4b.to_bg_color_string()}
    fn bg_deepskyblue4c(self) -> String {EightBitColors::DeepSkyBlue4c.to_bg_color_string()}
    fn bg_dodgerblue3(self) -> String {EightBitColors::DodgerBlue3.to_bg_color_string()}
    fn bg_dodgerblue2(self) -> String {EightBitColors::DodgerBlue2.to_bg_color_string()}
    fn bg_green4(self) -> String {EightBitColors::Green4.to_bg_color_string()}
    fn bg_springgreen4(self) -> String {EightBitColors::SpringGreen4.to_bg_color_string()}
    fn bg_turquoise4(self) -> String {EightBitColors::Turquoise4.to_bg_color_string()}
    fn bg_deepskyblue3a(self) -> String {EightBitColors::DeepSkyBlue3a.to_bg_color_string()}
    fn bg_deepskyblue3b(self) -> String {EightBitColors::DeepSkyBlue3b.to_bg_color_string()}
    fn bg_dodgerblue1(self) -> String {EightBitColors::DodgerBlue1.to_bg_color_string()}
    fn bg_green3a(self) -> String {EightBitColors::Green3a.to_bg_color_string()}
    fn bg_springgreen3a(self) -> String {EightBitColors::SpringGreen3a.to_bg_color_string()}
    fn bg_darkcyan(self) -> String {EightBitColors::DarkCyan.to_bg_color_string()}
    fn bg_lightseagreen(self) -> String {EightBitColors::LightSeaGreen.to_bg_color_string()}
    fn bg_deepskyblue2(self) -> String {EightBitColors::DeepSkyBlue2.to_bg_color_string()}
    fn bg_deepskyblue1(self) -> String {EightBitColors::DeepSkyBlue1.to_bg_color_string()}
    fn bg_green3b(self) -> String {EightBitColors::Green3b.to_bg_color_string()}
    fn bg_springgreen3b(self) -> String {EightBitColors::SpringGreen3b.to_bg_color_string()}
    fn bg_springgreen2a(self) -> String {EightBitColors::SpringGreen2a.to_bg_color_string()}
    fn bg_cyan3(self) -> String {EightBitColors::Cyan3.to_bg_color_string()}
    fn bg_darkturquoise(self) -> String {EightBitColors::DarkTurquoise.to_bg_color_string()}
    fn bg_turquoise2(self) -> String {EightBitColors::Turquoise2.to_bg_color_string()}
    fn bg_green1(self) -> String {EightBitColors::Green1.to_bg_color_string()}
    fn bg_springgreen2b(self) -> String {EightBitColors::SpringGreen2b.to_bg_color_string()}
    fn bg_springgreen1(self) -> String {EightBitColors::SpringGreen1.to_bg_color_string()}
    fn bg_mediumspringgreen(self) -> String {EightBitColors::MediumSpringGreen.to_bg_color_string()}
    fn bg_cyan2(self) -> String {EightBitColors::Cyan2.to_bg_color_string()}
    fn bg_cyan1(self) -> String {EightBitColors::Cyan1.to_bg_color_string()}
    fn bg_darkred1(self) -> String {EightBitColors::DarkRed1.to_bg_color_string()}
    fn bg_deeppink4a(self) -> String {EightBitColors::DeepPink4a.to_bg_color_string()}
    fn bg_purple4a(self) -> String {EightBitColors::Purple4a.to_bg_color_string()}
    fn bg_purple4b(self) -> String {EightBitColors::Purple4b.to_bg_color_string()}
    fn bg_purple3(self) -> String {EightBitColors::Purple3.to_bg_color_string()}
    fn bg_blueviolet(self) -> String {EightBitColors::BlueViolet.to_bg_color_string()}
    fn bg_orange4a(self) -> String {EightBitColors::Orange4a.to_bg_color_string()}
    fn bg_grey37(self) -> String {EightBitColors::Grey37.to_bg_color_string()}
    fn bg_mediumpurple4(self) -> String {EightBitColors::MediumPurple4.to_bg_color_string()}
    fn bg_slateblue3a(self) -> String {EightBitColors::SlateBlue3a.to_bg_color_string()}
    fn bg_slateblue3b(self) -> String {EightBitColors::SlateBlue3b.to_bg_color_string()}
    fn bg_royalblue1(self) -> String {EightBitColors::RoyalBlue1.to_bg_color_string()}
    fn bg_chartreuse4(self) -> String {EightBitColors::Chartreuse4.to_bg_color_string()}
    fn bg_darkseagreen4a(self) -> String {EightBitColors::DarkSeaGreen4a.to_bg_color_string()}
    fn bg_paleturquoise4(self) -> String {EightBitColors::PaleTurquoise4.to_bg_color_string()}
    fn bg_steelblue(self) -> String {EightBitColors::SteelBlue.to_bg_color_string()}
    fn bg_steelblue3(self) -> String {EightBitColors::SteelBlue3.to_bg_color_string()}
    fn bg_cornflowerblue(self) -> String {EightBitColors::CornflowerBlue.to_bg_color_string()}
    fn bg_chartreuse3a(self) -> String {EightBitColors::Chartreuse3a.to_bg_color_string()}
    fn bg_darkseagreen4b(self) -> String {EightBitColors::DarkSeaGreen4b.to_bg_color_string()}
    fn bg_cadetblue2(self) -> String {EightBitColors::CadetBlue2.to_bg_color_string()}
    fn bg_cadetblue1(self) -> String {EightBitColors::CadetBlue1.to_bg_color_string()}
    fn bg_skyblue3(self) -> String {EightBitColors::SkyBlue3.to_bg_color_string()}
    fn bg_steelblue1a(self) -> String {EightBitColors::SteelBlue1a.to_bg_color_string()}
    fn bg_chartreuse3b(self) -> String {EightBitColors::Chartreuse3b.to_bg_color_string()}
    fn bg_palegreen3a(self) -> String {EightBitColors::PaleGreen3a.to_bg_color_string()}
    fn bg_seagreen3(self) -> String {EightBitColors::SeaGreen3.to_bg_color_string()}
    fn bg_aquamarine3(self) -> String {EightBitColors::Aquamarine3.to_bg_color_string()}
    fn bg_mediumturquoise(self) -> String {EightBitColors::MediumTurquoise.to_bg_color_string()}
    fn bg_steelblue1b(self) -> String {EightBitColors::SteelBlue1b.to_bg_color_string()}
    fn bg_chartreuse2a(self) -> String {EightBitColors::Chartreuse2a.to_bg_color_string()}
    fn bg_seagreen2(self) -> String {EightBitColors::SeaGreen2.to_bg_color_string()}
    fn bg_seagreen1a(self) -> String {EightBitColors::SeaGreen1a.to_bg_color_string()}
    fn bg_seagreen1b(self) -> String {EightBitColors::SeaGreen1b.to_bg_color_string()}
    fn bg_aquamarine1a(self) -> String {EightBitColors::Aquamarine1a.to_bg_color_string()}
    fn bg_darkslategray2(self) -> String {EightBitColors::DarkSlateGray2.to_bg_color_string()}
    fn bg_darkred2(self) -> String {EightBitColors::DarkRed2.to_bg_color_string()}
    fn bg_deeppink4b(self) -> String {EightBitColors::DeepPink4b.to_bg_color_string()}
    fn bg_darkmagenta1(self) -> String {EightBitColors::DarkMagenta1.to_bg_color_string()}
    fn bg_darkmagenta2(self) -> String {EightBitColors::DarkMagenta2.to_bg_color_string()}
    fn bg_darkviolet1a(self) -> String {EightBitColors::DarkViolet1a.to_bg_color_string()}
    fn bg_purple1a(self) -> String {EightBitColors::Purple1a.to_bg_color_string()}
    fn bg_orange4b(self) -> String {EightBitColors::Orange4b.to_bg_color_string()}
    fn bg_lightpink4(self) -> String {EightBitColors::LightPink4.to_bg_color_string()}
    fn bg_plum4(self) -> String {EightBitColors::Plum4.to_bg_color_string()}
    fn bg_mediumpurple3a(self) -> String {EightBitColors::MediumPurple3a.to_bg_color_string()}
    fn bg_mediumpurple3b(self) -> String {EightBitColors::MediumPurple3b.to_bg_color_string()}
    fn bg_slateblue1(self) -> String {EightBitColors::SlateBlue1.to_bg_color_string()}
    fn bg_yellow4a(self) -> String {EightBitColors::Yellow4a.to_bg_color_string()}
    fn bg_wheat4(self) -> String {EightBitColors::Wheat4.to_bg_color_string()}
    fn bg_grey53(self) -> String {EightBitColors::Grey53.to_bg_color_string()}
    fn bg_lightslategrey(self) -> String {EightBitColors::LightSlateGrey.to_bg_color_string()}
    fn bg_mediumpurple(self) -> String {EightBitColors::MediumPurple.to_bg_color_string()}
    fn bg_lightslateblue(self) -> String {EightBitColors::LightSlateBlue.to_bg_color_string()}
    fn bg_yellow4b(self) -> String {EightBitColors::Yellow4b.to_bg_color_string()}
    fn bg_darkolivegreen3a(self) -> String {EightBitColors::DarkOliveGreen3a.to_bg_color_string()}
    fn bg_darkgreensea(self) -> String {EightBitColors::DarkGreenSea.to_bg_color_string()}
    fn bg_lightskyblue3a(self) -> String {EightBitColors::LightSkyBlue3a.to_bg_color_string()}
    fn bg_lightskyblue3b(self) -> String {EightBitColors::LightSkyBlue3b.to_bg_color_string()}
    fn bg_skyblue2(self) -> String {EightBitColors::SkyBlue2.to_bg_color_string()}
    fn bg_chartreuse2b(self) -> String {EightBitColors::Chartreuse2b.to_bg_color_string()}
    fn bg_darkolivegreen3b(self) -> String {EightBitColors::DarkOliveGreen3b.to_bg_color_string()}
    fn bg_palegreen3b(self) -> String {EightBitColors::PaleGreen3b.to_bg_color_string()}
    fn bg_darkseagreen3a(self) -> String {EightBitColors::DarkSeaGreen3a.to_bg_color_string()}
    fn bg_darkslategray3(self) -> String {EightBitColors::DarkSlateGray3.to_bg_color_string()}
    fn bg_skyblue1(self) -> String {EightBitColors::SkyBlue1.to_bg_color_string()}
    fn bg_chartreuse1(self) -> String {EightBitColors::Chartreuse1.to_bg_color_string()}
    fn bg_lightgreen2(self) -> String {EightBitColors::LightGreen2.to_bg_color_string()}
    fn bg_lightgreen3(self) -> String {EightBitColors::LightGreen3.to_bg_color_string()}
    fn bg_palegreen1a(self) -> String {EightBitColors::PaleGreen1a.to_bg_color_string()}
    fn bg_aquamarine1b(self) -> String {EightBitColors::Aquamarine1b.to_bg_color_string()}
    fn bg_darkslategray1(self) -> String {EightBitColors::DarkSlateGray1.to_bg_color_string()}
    fn bg_red3a(self) -> String {EightBitColors::Red3a.to_bg_color_string()}
    fn bg_deeppink4c(self) -> String {EightBitColors::DeepPink4c.to_bg_color_string()}
    fn bg_mediumvioletred(self) -> String {EightBitColors::MediumVioletRed.to_bg_color_string()}
    fn bg_magenta3a(self) -> String {EightBitColors::Magenta3a.to_bg_color_string()}
    fn bg_darkviolet1b(self) -> String {EightBitColors::DarkViolet1b.to_bg_color_string()}
    fn bg_purple1b(self) -> String {EightBitColors::Purple1b.to_bg_color_string()}
    fn bg_darkorange3a(self) -> String {EightBitColors::DarkOrange3a.to_bg_color_string()}
    fn bg_indianred1a(self) -> String {EightBitColors::IndianRed1a.to_bg_color_string()}
    fn bg_hotpink3a(self) -> String {EightBitColors::HotPink3a.to_bg_color_string()}
    fn bg_mediumorchid3(self) -> String {EightBitColors::MediumOrchid3.to_bg_color_string()}
    fn bg_mediumorchid(self) -> String {EightBitColors::MediumOrchid.to_bg_color_string()}
    fn bg_mediumpurple2a(self) -> String {EightBitColors::MediumPurple2a.to_bg_color_string()}
    fn bg_darkgoldenrod(self) -> String {EightBitColors::DarkGoldenrod.to_bg_color_string()}
    fn bg_lightsalmon3a(self) -> String {EightBitColors::LightSalmon3a.to_bg_color_string()}
    fn bg_rosybrown(self) -> String {EightBitColors::RosyBrown.to_bg_color_string()}
    fn bg_grey63(self) -> String {EightBitColors::Grey63.to_bg_color_string()}
    fn bg_mediumpurple2b(self) -> String {EightBitColors::MediumPurple2b.to_bg_color_string()}
    fn bg_mediumpurple1(self) -> String {EightBitColors::MediumPurple1.to_bg_color_string()}
    fn bg_gold3a(self) -> String {EightBitColors::Gold3a.to_bg_color_string()}
    fn bg_darkkhaki(self) -> String {EightBitColors::DarkKhaki.to_bg_color_string()}
    fn bg_navajowhite3(self) -> String {EightBitColors::NavajoWhite3.to_bg_color_string()}
    fn bg_grey69(self) -> String {EightBitColors::Grey69.to_bg_color_string()}
    fn bg_lightsteelblue3(self) -> String {EightBitColors::LightSteelBlue3.to_bg_color_string()}
    fn bg_lightsteelblue(self) -> String {EightBitColors::LightSteelBlue.to_bg_color_string()}
    fn bg_yellow3a(self) -> String {EightBitColors::Yellow3a.to_bg_color_string()}
    fn bg_darkolivegreen3(self) -> String {EightBitColors::DarkOliveGreen3.to_bg_color_string()}
    fn bg_darkseagreen3b(self) -> String {EightBitColors::DarkSeaGreen3b.to_bg_color_string()}
    fn bg_darkseagreen2(self) -> String {EightBitColors::DarkSeaGreen2.to_bg_color_string()}
    fn bg_lightcyan3(self) -> String {EightBitColors::LightCyan3.to_bg_color_string()}
    fn bg_lightskyblue1(self) -> String {EightBitColors::LightSkyBlue1.to_bg_color_string()}
    fn bg_greenyellow(self) -> String {EightBitColors::GreenYellow.to_bg_color_string()}
    fn bg_darkolivegreen2(self) -> String {EightBitColors::DarkOliveGreen2.to_bg_color_string()}
    fn bg_palegreen1b(self) -> String {EightBitColors::PaleGreen1b.to_bg_color_string()}
    fn bg_darkseagreen5b(self) -> String {EightBitColors::DarkSeaGreen5b.to_bg_color_string()}
    fn bg_darkseagreen5a(self) -> String {EightBitColors::DarkSeaGreen5a.to_bg_color_string()}
    fn bg_paleturquoise1(self) -> String {EightBitColors::PaleTurquoise1.to_bg_color_string()}
    fn bg_red3b(self) -> String {EightBitColors::Red3b.to_bg_color_string()}
    fn bg_deeppink3a(self) -> String {EightBitColors::DeepPink3a.to_bg_color_string()}
    fn bg_deeppink3b(self) -> String {EightBitColors::DeepPink3b.to_bg_color_string()}
    fn bg_magenta3b(self) -> String {EightBitColors::Magenta3b.to_bg_color_string()}
    fn bg_magenta3c(self) -> String {EightBitColors::Magenta3c.to_bg_color_string()}
    fn bg_magenta2a(self) -> String {EightBitColors::Magenta2a.to_bg_color_string()}
    fn bg_darkorange3b(self) -> String {EightBitColors::DarkOrange3b.to_bg_color_string()}
    fn bg_indianred1b(self) -> String {EightBitColors::IndianRed1b.to_bg_color_string()}
    fn bg_hotpink3b(self) -> String {EightBitColors::HotPink3b.to_bg_color_string()}
    fn bg_hotpink2(self) -> String {EightBitColors::HotPink2.to_bg_color_string()}
    fn bg_orchid(self) -> String {EightBitColors::Orchid.to_bg_color_string()}
    fn bg_mediumorchid1a(self) -> String {EightBitColors::MediumOrchid1a.to_bg_color_string()}
    fn bg_orange3(self) -> String {EightBitColors::Orange3.to_bg_color_string()}
    fn bg_lightsalmon3b(self) -> String {EightBitColors::LightSalmon3b.to_bg_color_string()}
    fn bg_lightpink3(self) -> String {EightBitColors::LightPink3.to_bg_color_string()}
    fn bg_pink3(self) -> String {EightBitColors::Pink3.to_bg_color_string()}
    fn bg_plum3(self) -> String {EightBitColors::Plum3.to_bg_color_string()}
    fn bg_violet(self) -> String {EightBitColors::Violet.to_bg_color_string()}
    fn bg_gold3b(self) -> String {EightBitColors::Gold3b.to_bg_color_string()}
    fn bg_lightgoldenrod3(self) -> String {EightBitColors::LightGoldenrod3.to_bg_color_string()}
    fn bg_tan(self) -> String {EightBitColors::Tan.to_bg_color_string()}
    fn bg_mistyrose3(self) -> String {EightBitColors::MistyRose3.to_bg_color_string()}
    fn bg_thistle3(self) -> String {EightBitColors::Thistle3.to_bg_color_string()}
    fn bg_plum2(self) -> String {EightBitColors::Plum2.to_bg_color_string()}
    fn bg_yellow3b(self) -> String {EightBitColors::Yellow3b.to_bg_color_string()}
    fn bg_khaki3(self) -> String {EightBitColors::Khaki3.to_bg_color_string()}
    fn bg_lightgoldenrod2a(self) -> String {EightBitColors::LightGoldenrod2a.to_bg_color_string()}
    fn bg_lightyellow3(self) -> String {EightBitColors::LightYellow3.to_bg_color_string()}
    fn bg_grey84(self) -> String {EightBitColors::Grey84.to_bg_color_string()}
    fn bg_lightsteelblue1(self) -> String {EightBitColors::LightSteelBlue1.to_bg_color_string()}
    fn bg_yellow2(self) -> String {EightBitColors::Yellow2.to_bg_color_string()}
    fn bg_darkolivegreen1a(self) -> String {EightBitColors::DarkOliveGreen1a.to_bg_color_string()}
    fn bg_darkolivegreen1b(self) -> String {EightBitColors::DarkOliveGreen1b.to_bg_color_string()}
    fn bg_darkseagreen1(self) -> String {EightBitColors::DarkSeaGreen1.to_bg_color_string()}
    fn bg_honeydew2(self) -> String {EightBitColors::Honeydew2.to_bg_color_string()}
    fn bg_lightcyan1(self) -> String {EightBitColors::LightCyan1.to_bg_color_string()}
    fn bg_red1(self) -> String {EightBitColors::Red1.to_bg_color_string()}
    fn bg_deeppink2(self) -> String {EightBitColors::DeepPink2.to_bg_color_string()}
    fn bg_deeppink1a(self) -> String {EightBitColors::DeepPink1a.to_bg_color_string()}
    fn bg_deeppink1b(self) -> String {EightBitColors::DeepPink1b.to_bg_color_string()}
    fn bg_magenta2b(self) -> String {EightBitColors::Magenta2b.to_bg_color_string()}
    fn bg_magenta1(self) -> String {EightBitColors::Magenta1.to_bg_color_string()}
    fn bg_orangered1(self) -> String {EightBitColors::OrangeRed1.to_bg_color_string()}
    fn bg_indianred1c(self) -> String {EightBitColors::IndianRed1c.to_bg_color_string()}
    fn bg_indianred1d(self) -> String {EightBitColors::IndianRed1d.to_bg_color_string()}
    fn bg_hotpink1a(self) -> String {EightBitColors::HotPink1a.to_bg_color_string()}
    fn bg_hotpink1b(self) -> String {EightBitColors::HotPink1b.to_bg_color_string()}
    fn bg_mediumorchid1b(self) -> String {EightBitColors::MediumOrchid1b.to_bg_color_string()}
    fn bg_darkorange(self) -> String {EightBitColors::DarkOrange.to_bg_color_string()}
    fn bg_salmon1(self) -> String {EightBitColors::Salmon1.to_bg_color_string()}
    fn bg_lightcoral(self) -> String {EightBitColors::LightCoral.to_bg_color_string()}
    fn bg_palevioletred1(self) -> String {EightBitColors::PaleVioletRed1.to_bg_color_string()}
    fn bg_orchid2(self) -> String {EightBitColors::Orchid2.to_bg_color_string()}
    fn bg_orchid1(self) -> String {EightBitColors::Orchid1.to_bg_color_string()}
    fn bg_orange1(self) -> String {EightBitColors::Orange1.to_bg_color_string()}
    fn bg_sandybrown(self) -> String {EightBitColors::SandyBrown.to_bg_color_string()}
    fn bg_lightsalmon1(self) -> String {EightBitColors::LightSalmon1.to_bg_color_string()}
    fn bg_lightpink1(self) -> String {EightBitColors::LightPink1.to_bg_color_string()}
    fn bg_pink1(self) -> String {EightBitColors::Pink1.to_bg_color_string()}
    fn bg_plum1(self) -> String {EightBitColors::Plum1.to_bg_color_string()}
    fn bg_gold1(self) -> String {EightBitColors::Gold1.to_bg_color_string()}
    fn bg_lightgoldenrod2b(self) -> String {EightBitColors::LightGoldenrod2b.to_bg_color_string()}
    fn bg_lightgoldenrod2c(self) -> String {EightBitColors::LightGoldenrod2c.to_bg_color_string()}
    fn bg_navajowhite1(self) -> String {EightBitColors::NavajoWhite1.to_bg_color_string()}
    fn bg_mistyrose1(self) -> String {EightBitColors::MistyRose1.to_bg_color_string()}
    fn bg_thistle1(self) -> String {EightBitColors::Thistle1.to_bg_color_string()}
    fn bg_yellow1(self) -> String {EightBitColors::Yellow1.to_bg_color_string()}
    fn bg_lightgoldenrod1(self) -> String {EightBitColors::LightGoldenrod1.to_bg_color_string()}
    fn bg_khaki1(self) -> String {EightBitColors::Khaki1.to_bg_color_string()}
    fn bg_wheat1(self) -> String {EightBitColors::Wheat1.to_bg_color_string()}
    fn bg_cornsilk1(self) -> String {EightBitColors::CornSilk1.to_bg_color_string()}
    fn bg_grey100(self) -> String {EightBitColors::Grey100.to_bg_color_string()}
    fn bg_grey3(self) -> String {EightBitColors::Grey3.to_bg_color_string()}
    fn bg_grey7(self) -> String {EightBitColors::Grey7.to_bg_color_string()}
    fn bg_grey11(self) -> String {EightBitColors::Grey11.to_bg_color_string()}
    fn bg_grey15(self) -> String {EightBitColors::Grey15.to_bg_color_string()}
    fn bg_grey19(self) -> String {EightBitColors::Grey19.to_bg_color_string()}
    fn bg_grey23(self) -> String {EightBitColors::Grey23.to_bg_color_string()}
    fn bg_grey27(self) -> String {EightBitColors::Grey27.to_bg_color_string()}
    fn bg_grey30(self) -> String {EightBitColors::Grey30.to_bg_color_string()}
    fn bg_grey35(self) -> String {EightBitColors::Grey35.to_bg_color_string()}
    fn bg_grey39(self) -> String {EightBitColors::Grey39.to_bg_color_string()}
    fn bg_grey42(self) -> String {EightBitColors::Grey42.to_bg_color_string()}
    fn bg_grey46(self) -> String {EightBitColors::Grey46.to_bg_color_string()}
    fn bg_grey50(self) -> String {EightBitColors::Grey50.to_bg_color_string()}
    fn bg_grey54(self) -> String {EightBitColors::Grey54.to_bg_color_string()}
    fn bg_grey58(self) -> String {EightBitColors::Grey58.to_bg_color_string()}
    fn bg_grey62(self) -> String {EightBitColors::Grey62.to_bg_color_string()}
    fn bg_grey66(self) -> String {EightBitColors::Grey66.to_bg_color_string()}
    fn bg_grey70(self) -> String {EightBitColors::Grey70.to_bg_color_string()}
    fn bg_grey74(self) -> String {EightBitColors::Grey74.to_bg_color_string()}
    fn bg_grey78(self) -> String {EightBitColors::Grey78.to_bg_color_string()}
    fn bg_grey82(self) -> String {EightBitColors::Grey82.to_bg_color_string()}
    fn bg_grey85(self) -> String {EightBitColors::Grey85.to_bg_color_string()}
    fn bg_grey89(self) -> String {EightBitColors::Grey89.to_bg_color_string()}
    fn bg_grey93(self) -> String {EightBitColors::Grey93.to_bg_color_string()}
}

pub trait StringMarker {
    fn to_string(&self) -> String;
}
impl StringMarker for String { 
    fn to_string(&self) -> String { self.clone() }
}
impl<'a> StringMarker for &'a str {
    fn to_string(&self) -> String { String::from(*self) }
}
