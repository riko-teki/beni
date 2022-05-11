use colors::EightBitColors::*;
use colors::{ EightBitColors, Color };
use decorated_string::{ DString, StringMarker };


pub mod colors;
pub mod csi;
pub mod sys;
pub mod decorated_string;

pub trait ColorString {
    fn fg_color(self, c: EightBitColors) -> DString;
    fn bg_color(self, c: EightBitColors) -> DString;
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
    fn fg_color(self, c: EightBitColors) -> DString {
        decorated_string::DString::from_string_and_fg(self, c)
    }
    fn bg_color(self, c: EightBitColors) -> DString {
        decorated_string::DString::from_string_and_bg(self, c)
    }
    fn fg_black(self) -> String { Black.to_fg_color_string() }
    fn fg_red(self) -> String { Red.to_fg_color_string() }
    fn fg_green(self) -> String {Green.to_fg_color_string()}
    fn fg_yellow(self) -> String {Yellow.to_fg_color_string()}
    fn fg_blue(self) -> String {Blue.to_fg_color_string()}
    fn fg_magenta(self) -> String {Magenta.to_fg_color_string()}
    fn fg_cyan(self) -> String {Cyan.to_fg_color_string()}
    fn fg_lightgray(self) -> String {LightGray.to_fg_color_string()}
    fn fg_darkgray(self) -> String {DarkGray.to_fg_color_string()}
    fn fg_lightred(self) -> String {LightRed.to_fg_color_string()}
    fn fg_lightgreen(self) -> String {LightGreen.to_fg_color_string()}
    fn fg_lightyellow(self) -> String {LightYellow.to_fg_color_string()}
    fn fg_lightblue(self) -> String {LightBlue.to_fg_color_string()}
    fn fg_lightmagenta(self) -> String {LightMagenta.to_fg_color_string()}
    fn fg_lightcyan(self) -> String {LightCyan.to_fg_color_string()}
    fn fg_white(self) -> String {White.to_fg_color_string()}
    fn fg_grey0(self) -> String {Grey0.to_fg_color_string()}
    fn fg_navyblue(self) -> String {NavyBlue.to_fg_color_string()}
    fn fg_darkblue(self) -> String {DarkBlue.to_fg_color_string()}
    fn fg_blue3a(self) -> String {Blue3a.to_fg_color_string()}
    fn fg_blue3b(self) -> String {Blue3b.to_fg_color_string()}
    fn fg_blue1(self) -> String {Blue1.to_fg_color_string()}
    fn fg_darkgreen(self) -> String {DarkGreen.to_fg_color_string()}
    fn fg_deepskyblue4a(self) -> String {DeepSkyBlue4a.to_fg_color_string()}
    fn fg_deepskyblue4b(self) -> String {DeepSkyBlue4b.to_fg_color_string()}
    fn fg_deepskyblue4c(self) -> String {DeepSkyBlue4c.to_fg_color_string()}
    fn fg_dodgerblue3(self) -> String {DodgerBlue3.to_fg_color_string()}
    fn fg_dodgerblue2(self) -> String {DodgerBlue2.to_fg_color_string()}
    fn fg_green4(self) -> String {Green4.to_fg_color_string()}
    fn fg_springgreen4(self) -> String {SpringGreen4.to_fg_color_string()}
    fn fg_turquoise4(self) -> String {Turquoise4.to_fg_color_string()}
    fn fg_deepskyblue3a(self) -> String {DeepSkyBlue3a.to_fg_color_string()}
    fn fg_deepskyblue3b(self) -> String {DeepSkyBlue3b.to_fg_color_string()}
    fn fg_dodgerblue1(self) -> String {DodgerBlue1.to_fg_color_string()}
    fn fg_green3a(self) -> String {Green3a.to_fg_color_string()}
    fn fg_springgreen3a(self) -> String {SpringGreen3a.to_fg_color_string()}
    fn fg_darkcyan(self) -> String {DarkCyan.to_fg_color_string()}
    fn fg_lightseagreen(self) -> String {LightSeaGreen.to_fg_color_string()}
    fn fg_deepskyblue2(self) -> String {DeepSkyBlue2.to_fg_color_string()}
    fn fg_deepskyblue1(self) -> String {DeepSkyBlue1.to_fg_color_string()}
    fn fg_green3b(self) -> String {Green3b.to_fg_color_string()}
    fn fg_springgreen3b(self) -> String {SpringGreen3b.to_fg_color_string()}
    fn fg_springgreen2a(self) -> String {SpringGreen2a.to_fg_color_string()}
    fn fg_cyan3(self) -> String {Cyan3.to_fg_color_string()}
    fn fg_darkturquoise(self) -> String {DarkTurquoise.to_fg_color_string()}
    fn fg_turquoise2(self) -> String {Turquoise2.to_fg_color_string()}
    fn fg_green1(self) -> String {Green1.to_fg_color_string()}
    fn fg_springgreen2b(self) -> String {SpringGreen2b.to_fg_color_string()}
    fn fg_springgreen1(self) -> String {SpringGreen1.to_fg_color_string()}
    fn fg_mediumspringgreen(self) -> String {MediumSpringGreen.to_fg_color_string()}
    fn fg_cyan2(self) -> String {Cyan2.to_fg_color_string()}
    fn fg_cyan1(self) -> String {Cyan1.to_fg_color_string()}
    fn fg_darkred1(self) -> String {DarkRed1.to_fg_color_string()}
    fn fg_deeppink4a(self) -> String {DeepPink4a.to_fg_color_string()}
    fn fg_purple4a(self) -> String {Purple4a.to_fg_color_string()}
    fn fg_purple4b(self) -> String {Purple4b.to_fg_color_string()}
    fn fg_purple3(self) -> String {Purple3.to_fg_color_string()}
    fn fg_blueviolet(self) -> String {BlueViolet.to_fg_color_string()}
    fn fg_orange4a(self) -> String {Orange4a.to_fg_color_string()}
    fn fg_grey37(self) -> String {Grey37.to_fg_color_string()}
    fn fg_mediumpurple4(self) -> String {MediumPurple4.to_fg_color_string()}
    fn fg_slateblue3a(self) -> String {SlateBlue3a.to_fg_color_string()}
    fn fg_slateblue3b(self) -> String {SlateBlue3b.to_fg_color_string()}
    fn fg_royalblue1(self) -> String {RoyalBlue1.to_fg_color_string()}
    fn fg_chartreuse4(self) -> String {Chartreuse4.to_fg_color_string()}
    fn fg_darkseagreen4a(self) -> String {DarkSeaGreen4a.to_fg_color_string()}
    fn fg_paleturquoise4(self) -> String {PaleTurquoise4.to_fg_color_string()}
    fn fg_steelblue(self) -> String {SteelBlue.to_fg_color_string()}
    fn fg_steelblue3(self) -> String {SteelBlue3.to_fg_color_string()}
    fn fg_cornflowerblue(self) -> String {CornflowerBlue.to_fg_color_string()}
    fn fg_chartreuse3a(self) -> String {Chartreuse3a.to_fg_color_string()}
    fn fg_darkseagreen4b(self) -> String {DarkSeaGreen4b.to_fg_color_string()}
    fn fg_cadetblue2(self) -> String {CadetBlue2.to_fg_color_string()}
    fn fg_cadetblue1(self) -> String {CadetBlue1.to_fg_color_string()}
    fn fg_skyblue3(self) -> String {SkyBlue3.to_fg_color_string()}
    fn fg_steelblue1a(self) -> String {SteelBlue1a.to_fg_color_string()}
    fn fg_chartreuse3b(self) -> String {Chartreuse3b.to_fg_color_string()}
    fn fg_palegreen3a(self) -> String {PaleGreen3a.to_fg_color_string()}
    fn fg_seagreen3(self) -> String {SeaGreen3.to_fg_color_string()}
    fn fg_aquamarine3(self) -> String {Aquamarine3.to_fg_color_string()}
    fn fg_mediumturquoise(self) -> String {MediumTurquoise.to_fg_color_string()}
    fn fg_steelblue1b(self) -> String {SteelBlue1b.to_fg_color_string()}
    fn fg_chartreuse2a(self) -> String {Chartreuse2a.to_fg_color_string()}
    fn fg_seagreen2(self) -> String {SeaGreen2.to_fg_color_string()}
    fn fg_seagreen1a(self) -> String {SeaGreen1a.to_fg_color_string()}
    fn fg_seagreen1b(self) -> String {SeaGreen1b.to_fg_color_string()}
    fn fg_aquamarine1a(self) -> String {Aquamarine1a.to_fg_color_string()}
    fn fg_darkslategray2(self) -> String {DarkSlateGray2.to_fg_color_string()}
    fn fg_darkred2(self) -> String {DarkRed2.to_fg_color_string()}
    fn fg_deeppink4b(self) -> String {DeepPink4b.to_fg_color_string()}
    fn fg_darkmagenta1(self) -> String {DarkMagenta1.to_fg_color_string()}
    fn fg_darkmagenta2(self) -> String {DarkMagenta2.to_fg_color_string()}
    fn fg_darkviolet1a(self) -> String {DarkViolet1a.to_fg_color_string()}
    fn fg_purple1a(self) -> String {Purple1a.to_fg_color_string()}
    fn fg_orange4b(self) -> String {Orange4b.to_fg_color_string()}
    fn fg_lightpink4(self) -> String {LightPink4.to_fg_color_string()}
    fn fg_plum4(self) -> String {Plum4.to_fg_color_string()}
    fn fg_mediumpurple3a(self) -> String {MediumPurple3a.to_fg_color_string()}
    fn fg_mediumpurple3b(self) -> String {MediumPurple3b.to_fg_color_string()}
    fn fg_slateblue1(self) -> String {SlateBlue1.to_fg_color_string()}
    fn fg_yellow4a(self) -> String {Yellow4a.to_fg_color_string()}
    fn fg_wheat4(self) -> String {Wheat4.to_fg_color_string()}
    fn fg_grey53(self) -> String {Grey53.to_fg_color_string()}
    fn fg_lightslategrey(self) -> String {LightSlateGrey.to_fg_color_string()}
    fn fg_mediumpurple(self) -> String {MediumPurple.to_fg_color_string()}
    fn fg_lightslateblue(self) -> String {LightSlateBlue.to_fg_color_string()}
    fn fg_yellow4b(self) -> String {Yellow4b.to_fg_color_string()}
    fn fg_darkolivegreen3a(self) -> String {DarkOliveGreen3a.to_fg_color_string()}
    fn fg_darkgreensea(self) -> String {DarkGreenSea.to_fg_color_string()}
    fn fg_lightskyblue3a(self) -> String {LightSkyBlue3a.to_fg_color_string()}
    fn fg_lightskyblue3b(self) -> String {LightSkyBlue3b.to_fg_color_string()}
    fn fg_skyblue2(self) -> String {SkyBlue2.to_fg_color_string()}
    fn fg_chartreuse2b(self) -> String {Chartreuse2b.to_fg_color_string()}
    fn fg_darkolivegreen3b(self) -> String {DarkOliveGreen3b.to_fg_color_string()}
    fn fg_palegreen3b(self) -> String {PaleGreen3b.to_fg_color_string()}
    fn fg_darkseagreen3a(self) -> String {DarkSeaGreen3a.to_fg_color_string()}
    fn fg_darkslategray3(self) -> String {DarkSlateGray3.to_fg_color_string()}
    fn fg_skyblue1(self) -> String {SkyBlue1.to_fg_color_string()}
    fn fg_chartreuse1(self) -> String {Chartreuse1.to_fg_color_string()}
    fn fg_lightgreen2(self) -> String {LightGreen2.to_fg_color_string()}
    fn fg_lightgreen3(self) -> String {LightGreen3.to_fg_color_string()}
    fn fg_palegreen1a(self) -> String {PaleGreen1a.to_fg_color_string()}
    fn fg_aquamarine1b(self) -> String {Aquamarine1b.to_fg_color_string()}
    fn fg_darkslategray1(self) -> String {DarkSlateGray1.to_fg_color_string()}
    fn fg_red3a(self) -> String {Red3a.to_fg_color_string()}
    fn fg_deeppink4c(self) -> String {DeepPink4c.to_fg_color_string()}
    fn fg_mediumvioletred(self) -> String {MediumVioletRed.to_fg_color_string()}
    fn fg_magenta3a(self) -> String {Magenta3a.to_fg_color_string()}
    fn fg_darkviolet1b(self) -> String {DarkViolet1b.to_fg_color_string()}
    fn fg_purple1b(self) -> String {Purple1b.to_fg_color_string()}
    fn fg_darkorange3a(self) -> String {DarkOrange3a.to_fg_color_string()}
    fn fg_indianred1a(self) -> String {IndianRed1a.to_fg_color_string()}
    fn fg_hotpink3a(self) -> String {HotPink3a.to_fg_color_string()}
    fn fg_mediumorchid3(self) -> String {MediumOrchid3.to_fg_color_string()}
    fn fg_mediumorchid(self) -> String {MediumOrchid.to_fg_color_string()}
    fn fg_mediumpurple2a(self) -> String {MediumPurple2a.to_fg_color_string()}
    fn fg_darkgoldenrod(self) -> String {DarkGoldenrod.to_fg_color_string()}
    fn fg_lightsalmon3a(self) -> String {LightSalmon3a.to_fg_color_string()}
    fn fg_rosybrown(self) -> String {RosyBrown.to_fg_color_string()}
    fn fg_grey63(self) -> String {Grey63.to_fg_color_string()}
    fn fg_mediumpurple2b(self) -> String {MediumPurple2b.to_fg_color_string()}
    fn fg_mediumpurple1(self) -> String {MediumPurple1.to_fg_color_string()}
    fn fg_gold3a(self) -> String {Gold3a.to_fg_color_string()}
    fn fg_darkkhaki(self) -> String {DarkKhaki.to_fg_color_string()}
    fn fg_navajowhite3(self) -> String {NavajoWhite3.to_fg_color_string()}
    fn fg_grey69(self) -> String {Grey69.to_fg_color_string()}
    fn fg_lightsteelblue3(self) -> String {LightSteelBlue3.to_fg_color_string()}
    fn fg_lightsteelblue(self) -> String {LightSteelBlue.to_fg_color_string()}
    fn fg_yellow3a(self) -> String {Yellow3a.to_fg_color_string()}
    fn fg_darkolivegreen3(self) -> String {DarkOliveGreen3.to_fg_color_string()}
    fn fg_darkseagreen3b(self) -> String {DarkSeaGreen3b.to_fg_color_string()}
    fn fg_darkseagreen2(self) -> String {DarkSeaGreen2.to_fg_color_string()}
    fn fg_lightcyan3(self) -> String {LightCyan3.to_fg_color_string()}
    fn fg_lightskyblue1(self) -> String {LightSkyBlue1.to_fg_color_string()}
    fn fg_greenyellow(self) -> String {GreenYellow.to_fg_color_string()}
    fn fg_darkolivegreen2(self) -> String {DarkOliveGreen2.to_fg_color_string()}
    fn fg_palegreen1b(self) -> String {PaleGreen1b.to_fg_color_string()}
    fn fg_darkseagreen5b(self) -> String {DarkSeaGreen5b.to_fg_color_string()}
    fn fg_darkseagreen5a(self) -> String {DarkSeaGreen5a.to_fg_color_string()}
    fn fg_paleturquoise1(self) -> String {PaleTurquoise1.to_fg_color_string()}
    fn fg_red3b(self) -> String {Red3b.to_fg_color_string()}
    fn fg_deeppink3a(self) -> String {DeepPink3a.to_fg_color_string()}
    fn fg_deeppink3b(self) -> String {DeepPink3b.to_fg_color_string()}
    fn fg_magenta3b(self) -> String {Magenta3b.to_fg_color_string()}
    fn fg_magenta3c(self) -> String {Magenta3c.to_fg_color_string()}
    fn fg_magenta2a(self) -> String {Magenta2a.to_fg_color_string()}
    fn fg_darkorange3b(self) -> String {DarkOrange3b.to_fg_color_string()}
    fn fg_indianred1b(self) -> String {IndianRed1b.to_fg_color_string()}
    fn fg_hotpink3b(self) -> String {HotPink3b.to_fg_color_string()}
    fn fg_hotpink2(self) -> String {HotPink2.to_fg_color_string()}
    fn fg_orchid(self) -> String {Orchid.to_fg_color_string()}
    fn fg_mediumorchid1a(self) -> String {MediumOrchid1a.to_fg_color_string()}
    fn fg_orange3(self) -> String {Orange3.to_fg_color_string()}
    fn fg_lightsalmon3b(self) -> String {LightSalmon3b.to_fg_color_string()}
    fn fg_lightpink3(self) -> String {LightPink3.to_fg_color_string()}
    fn fg_pink3(self) -> String {Pink3.to_fg_color_string()}
    fn fg_plum3(self) -> String {Plum3.to_fg_color_string()}
    fn fg_violet(self) -> String {Violet.to_fg_color_string()}
    fn fg_gold3b(self) -> String {Gold3b.to_fg_color_string()}
    fn fg_lightgoldenrod3(self) -> String {LightGoldenrod3.to_fg_color_string()}
    fn fg_tan(self) -> String {Tan.to_fg_color_string()}
    fn fg_mistyrose3(self) -> String {MistyRose3.to_fg_color_string()}
    fn fg_thistle3(self) -> String {Thistle3.to_fg_color_string()}
    fn fg_plum2(self) -> String {Plum2.to_fg_color_string()}
    fn fg_yellow3b(self) -> String {Yellow3b.to_fg_color_string()}
    fn fg_khaki3(self) -> String {Khaki3.to_fg_color_string()}
    fn fg_lightgoldenrod2a(self) -> String {LightGoldenrod2a.to_fg_color_string()}
    fn fg_lightyellow3(self) -> String {LightYellow3.to_fg_color_string()}
    fn fg_grey84(self) -> String {Grey84.to_fg_color_string()}
    fn fg_lightsteelblue1(self) -> String {LightSteelBlue1.to_fg_color_string()}
    fn fg_yellow2(self) -> String {Yellow2.to_fg_color_string()}
    fn fg_darkolivegreen1a(self) -> String {DarkOliveGreen1a.to_fg_color_string()}
    fn fg_darkolivegreen1b(self) -> String {DarkOliveGreen1b.to_fg_color_string()}
    fn fg_darkseagreen1(self) -> String {DarkSeaGreen1.to_fg_color_string()}
    fn fg_honeydew2(self) -> String {Honeydew2.to_fg_color_string()}
    fn fg_lightcyan1(self) -> String {LightCyan1.to_fg_color_string()}
    fn fg_red1(self) -> String {Red1.to_fg_color_string()}
    fn fg_deeppink2(self) -> String {DeepPink2.to_fg_color_string()}
    fn fg_deeppink1a(self) -> String {DeepPink1a.to_fg_color_string()}
    fn fg_deeppink1b(self) -> String {DeepPink1b.to_fg_color_string()}
    fn fg_magenta2b(self) -> String {Magenta2b.to_fg_color_string()}
    fn fg_magenta1(self) -> String {Magenta1.to_fg_color_string()}
    fn fg_orangered1(self) -> String {OrangeRed1.to_fg_color_string()}
    fn fg_indianred1c(self) -> String {IndianRed1c.to_fg_color_string()}
    fn fg_indianred1d(self) -> String {IndianRed1d.to_fg_color_string()}
    fn fg_hotpink1a(self) -> String {HotPink1a.to_fg_color_string()}
    fn fg_hotpink1b(self) -> String {HotPink1b.to_fg_color_string()}
    fn fg_mediumorchid1b(self) -> String {MediumOrchid1b.to_fg_color_string()}
    fn fg_darkorange(self) -> String {DarkOrange.to_fg_color_string()}
    fn fg_salmon1(self) -> String {Salmon1.to_fg_color_string()}
    fn fg_lightcoral(self) -> String {LightCoral.to_fg_color_string()}
    fn fg_palevioletred1(self) -> String {PaleVioletRed1.to_fg_color_string()}
    fn fg_orchid2(self) -> String {Orchid2.to_fg_color_string()}
    fn fg_orchid1(self) -> String {Orchid1.to_fg_color_string()}
    fn fg_orange1(self) -> String {Orange1.to_fg_color_string()}
    fn fg_sandybrown(self) -> String {SandyBrown.to_fg_color_string()}
    fn fg_lightsalmon1(self) -> String {LightSalmon1.to_fg_color_string()}
    fn fg_lightpink1(self) -> String {LightPink1.to_fg_color_string()}
    fn fg_pink1(self) -> String {Pink1.to_fg_color_string()}
    fn fg_plum1(self) -> String {Plum1.to_fg_color_string()}
    fn fg_gold1(self) -> String {Gold1.to_fg_color_string()}
    fn fg_lightgoldenrod2b(self) -> String {LightGoldenrod2b.to_fg_color_string()}
    fn fg_lightgoldenrod2c(self) -> String {LightGoldenrod2c.to_fg_color_string()}
    fn fg_navajowhite1(self) -> String {NavajoWhite1.to_fg_color_string()}
    fn fg_mistyrose1(self) -> String {MistyRose1.to_fg_color_string()}
    fn fg_thistle1(self) -> String {Thistle1.to_fg_color_string()}
    fn fg_yellow1(self) -> String {Yellow1.to_fg_color_string()}
    fn fg_lightgoldenrod1(self) -> String {LightGoldenrod1.to_fg_color_string()}
    fn fg_khaki1(self) -> String {Khaki1.to_fg_color_string()}
    fn fg_wheat1(self) -> String {Wheat1.to_fg_color_string()}
    fn fg_cornsilk1(self) -> String {CornSilk1.to_fg_color_string()}
    fn fg_grey100(self) -> String {Grey100.to_fg_color_string()}
    fn fg_grey3(self) -> String {Grey3.to_fg_color_string()}
    fn fg_grey7(self) -> String {Grey7.to_fg_color_string()}
    fn fg_grey11(self) -> String {Grey11.to_fg_color_string()}
    fn fg_grey15(self) -> String {Grey15.to_fg_color_string()}
    fn fg_grey19(self) -> String {Grey19.to_fg_color_string()}
    fn fg_grey23(self) -> String {Grey23.to_fg_color_string()}
    fn fg_grey27(self) -> String {Grey27.to_fg_color_string()}
    fn fg_grey30(self) -> String {Grey30.to_fg_color_string()}
    fn fg_grey35(self) -> String {Grey35.to_fg_color_string()}
    fn fg_grey39(self) -> String {Grey39.to_fg_color_string()}
    fn fg_grey42(self) -> String {Grey42.to_fg_color_string()}
    fn fg_grey46(self) -> String {Grey46.to_fg_color_string()}
    fn fg_grey50(self) -> String {Grey50.to_fg_color_string()}
    fn fg_grey54(self) -> String {Grey54.to_fg_color_string()}
    fn fg_grey58(self) -> String {Grey58.to_fg_color_string()}
    fn fg_grey62(self) -> String {Grey62.to_fg_color_string()}
    fn fg_grey66(self) -> String {Grey66.to_fg_color_string()}
    fn fg_grey70(self) -> String {Grey70.to_fg_color_string()}
    fn fg_grey74(self) -> String {Grey74.to_fg_color_string()}
    fn fg_grey78(self) -> String {Grey78.to_fg_color_string()}
    fn fg_grey82(self) -> String {Grey82.to_fg_color_string()}
    fn fg_grey85(self) -> String {Grey85.to_fg_color_string()}
    fn fg_grey89(self) -> String {Grey89.to_fg_color_string()}
    fn fg_grey93(self) -> String {Grey93.to_fg_color_string()}

    fn bg_black(self) -> String { Black.to_bg_color_string() }
    fn bg_red(self) -> String { Red.to_bg_color_string() }
    fn bg_green(self) -> String {Green.to_bg_color_string()}
    fn bg_yellow(self) -> String {Yellow.to_bg_color_string()}
    fn bg_blue(self) -> String {Blue.to_bg_color_string()}
    fn bg_magenta(self) -> String {Magenta.to_bg_color_string()}
    fn bg_cyan(self) -> String {Cyan.to_bg_color_string()}
    fn bg_lightgray(self) -> String {LightGray.to_bg_color_string()}
    fn bg_darkgray(self) -> String {DarkGray.to_bg_color_string()}
    fn bg_lightred(self) -> String {LightRed.to_bg_color_string()}
    fn bg_lightgreen(self) -> String {LightGreen.to_bg_color_string()}
    fn bg_lightyellow(self) -> String {LightYellow.to_bg_color_string()}
    fn bg_lightblue(self) -> String {LightBlue.to_bg_color_string()}
    fn bg_lightmagenta(self) -> String {LightMagenta.to_bg_color_string()}
    fn bg_lightcyan(self) -> String {LightCyan.to_bg_color_string()}
    fn bg_white(self) -> String {White.to_bg_color_string()}
    fn bg_grey0(self) -> String {Grey0.to_bg_color_string()}
    fn bg_navyblue(self) -> String {NavyBlue.to_bg_color_string()}
    fn bg_darkblue(self) -> String {DarkBlue.to_bg_color_string()}
    fn bg_blue3a(self) -> String {Blue3a.to_bg_color_string()}
    fn bg_blue3b(self) -> String {Blue3b.to_bg_color_string()}
    fn bg_blue1(self) -> String {Blue1.to_bg_color_string()}
    fn bg_darkgreen(self) -> String {DarkGreen.to_bg_color_string()}
    fn bg_deepskyblue4a(self) -> String {DeepSkyBlue4a.to_bg_color_string()}
    fn bg_deepskyblue4b(self) -> String {DeepSkyBlue4b.to_bg_color_string()}
    fn bg_deepskyblue4c(self) -> String {DeepSkyBlue4c.to_bg_color_string()}
    fn bg_dodgerblue3(self) -> String {DodgerBlue3.to_bg_color_string()}
    fn bg_dodgerblue2(self) -> String {DodgerBlue2.to_bg_color_string()}
    fn bg_green4(self) -> String {Green4.to_bg_color_string()}
    fn bg_springgreen4(self) -> String {SpringGreen4.to_bg_color_string()}
    fn bg_turquoise4(self) -> String {Turquoise4.to_bg_color_string()}
    fn bg_deepskyblue3a(self) -> String {DeepSkyBlue3a.to_bg_color_string()}
    fn bg_deepskyblue3b(self) -> String {DeepSkyBlue3b.to_bg_color_string()}
    fn bg_dodgerblue1(self) -> String {DodgerBlue1.to_bg_color_string()}
    fn bg_green3a(self) -> String {Green3a.to_bg_color_string()}
    fn bg_springgreen3a(self) -> String {SpringGreen3a.to_bg_color_string()}
    fn bg_darkcyan(self) -> String {DarkCyan.to_bg_color_string()}
    fn bg_lightseagreen(self) -> String {LightSeaGreen.to_bg_color_string()}
    fn bg_deepskyblue2(self) -> String {DeepSkyBlue2.to_bg_color_string()}
    fn bg_deepskyblue1(self) -> String {DeepSkyBlue1.to_bg_color_string()}
    fn bg_green3b(self) -> String {Green3b.to_bg_color_string()}
    fn bg_springgreen3b(self) -> String {SpringGreen3b.to_bg_color_string()}
    fn bg_springgreen2a(self) -> String {SpringGreen2a.to_bg_color_string()}
    fn bg_cyan3(self) -> String {Cyan3.to_bg_color_string()}
    fn bg_darkturquoise(self) -> String {DarkTurquoise.to_bg_color_string()}
    fn bg_turquoise2(self) -> String {Turquoise2.to_bg_color_string()}
    fn bg_green1(self) -> String {Green1.to_bg_color_string()}
    fn bg_springgreen2b(self) -> String {SpringGreen2b.to_bg_color_string()}
    fn bg_springgreen1(self) -> String {SpringGreen1.to_bg_color_string()}
    fn bg_mediumspringgreen(self) -> String {MediumSpringGreen.to_bg_color_string()}
    fn bg_cyan2(self) -> String {Cyan2.to_bg_color_string()}
    fn bg_cyan1(self) -> String {Cyan1.to_bg_color_string()}
    fn bg_darkred1(self) -> String {DarkRed1.to_bg_color_string()}
    fn bg_deeppink4a(self) -> String {DeepPink4a.to_bg_color_string()}
    fn bg_purple4a(self) -> String {Purple4a.to_bg_color_string()}
    fn bg_purple4b(self) -> String {Purple4b.to_bg_color_string()}
    fn bg_purple3(self) -> String {Purple3.to_bg_color_string()}
    fn bg_blueviolet(self) -> String {BlueViolet.to_bg_color_string()}
    fn bg_orange4a(self) -> String {Orange4a.to_bg_color_string()}
    fn bg_grey37(self) -> String {Grey37.to_bg_color_string()}
    fn bg_mediumpurple4(self) -> String {MediumPurple4.to_bg_color_string()}
    fn bg_slateblue3a(self) -> String {SlateBlue3a.to_bg_color_string()}
    fn bg_slateblue3b(self) -> String {SlateBlue3b.to_bg_color_string()}
    fn bg_royalblue1(self) -> String {RoyalBlue1.to_bg_color_string()}
    fn bg_chartreuse4(self) -> String {Chartreuse4.to_bg_color_string()}
    fn bg_darkseagreen4a(self) -> String {DarkSeaGreen4a.to_bg_color_string()}
    fn bg_paleturquoise4(self) -> String {PaleTurquoise4.to_bg_color_string()}
    fn bg_steelblue(self) -> String {SteelBlue.to_bg_color_string()}
    fn bg_steelblue3(self) -> String {SteelBlue3.to_bg_color_string()}
    fn bg_cornflowerblue(self) -> String {CornflowerBlue.to_bg_color_string()}
    fn bg_chartreuse3a(self) -> String {Chartreuse3a.to_bg_color_string()}
    fn bg_darkseagreen4b(self) -> String {DarkSeaGreen4b.to_bg_color_string()}
    fn bg_cadetblue2(self) -> String {CadetBlue2.to_bg_color_string()}
    fn bg_cadetblue1(self) -> String {CadetBlue1.to_bg_color_string()}
    fn bg_skyblue3(self) -> String {SkyBlue3.to_bg_color_string()}
    fn bg_steelblue1a(self) -> String {SteelBlue1a.to_bg_color_string()}
    fn bg_chartreuse3b(self) -> String {Chartreuse3b.to_bg_color_string()}
    fn bg_palegreen3a(self) -> String {PaleGreen3a.to_bg_color_string()}
    fn bg_seagreen3(self) -> String {SeaGreen3.to_bg_color_string()}
    fn bg_aquamarine3(self) -> String {Aquamarine3.to_bg_color_string()}
    fn bg_mediumturquoise(self) -> String {MediumTurquoise.to_bg_color_string()}
    fn bg_steelblue1b(self) -> String {SteelBlue1b.to_bg_color_string()}
    fn bg_chartreuse2a(self) -> String {Chartreuse2a.to_bg_color_string()}
    fn bg_seagreen2(self) -> String {SeaGreen2.to_bg_color_string()}
    fn bg_seagreen1a(self) -> String {SeaGreen1a.to_bg_color_string()}
    fn bg_seagreen1b(self) -> String {SeaGreen1b.to_bg_color_string()}
    fn bg_aquamarine1a(self) -> String {Aquamarine1a.to_bg_color_string()}
    fn bg_darkslategray2(self) -> String {DarkSlateGray2.to_bg_color_string()}
    fn bg_darkred2(self) -> String {DarkRed2.to_bg_color_string()}
    fn bg_deeppink4b(self) -> String {DeepPink4b.to_bg_color_string()}
    fn bg_darkmagenta1(self) -> String {DarkMagenta1.to_bg_color_string()}
    fn bg_darkmagenta2(self) -> String {DarkMagenta2.to_bg_color_string()}
    fn bg_darkviolet1a(self) -> String {DarkViolet1a.to_bg_color_string()}
    fn bg_purple1a(self) -> String {Purple1a.to_bg_color_string()}
    fn bg_orange4b(self) -> String {Orange4b.to_bg_color_string()}
    fn bg_lightpink4(self) -> String {LightPink4.to_bg_color_string()}
    fn bg_plum4(self) -> String {Plum4.to_bg_color_string()}
    fn bg_mediumpurple3a(self) -> String {MediumPurple3a.to_bg_color_string()}
    fn bg_mediumpurple3b(self) -> String {MediumPurple3b.to_bg_color_string()}
    fn bg_slateblue1(self) -> String {SlateBlue1.to_bg_color_string()}
    fn bg_yellow4a(self) -> String {Yellow4a.to_bg_color_string()}
    fn bg_wheat4(self) -> String {Wheat4.to_bg_color_string()}
    fn bg_grey53(self) -> String {Grey53.to_bg_color_string()}
    fn bg_lightslategrey(self) -> String {LightSlateGrey.to_bg_color_string()}
    fn bg_mediumpurple(self) -> String {MediumPurple.to_bg_color_string()}
    fn bg_lightslateblue(self) -> String {LightSlateBlue.to_bg_color_string()}
    fn bg_yellow4b(self) -> String {Yellow4b.to_bg_color_string()}
    fn bg_darkolivegreen3a(self) -> String {DarkOliveGreen3a.to_bg_color_string()}
    fn bg_darkgreensea(self) -> String {DarkGreenSea.to_bg_color_string()}
    fn bg_lightskyblue3a(self) -> String {LightSkyBlue3a.to_bg_color_string()}
    fn bg_lightskyblue3b(self) -> String {LightSkyBlue3b.to_bg_color_string()}
    fn bg_skyblue2(self) -> String {SkyBlue2.to_bg_color_string()}
    fn bg_chartreuse2b(self) -> String {Chartreuse2b.to_bg_color_string()}
    fn bg_darkolivegreen3b(self) -> String {DarkOliveGreen3b.to_bg_color_string()}
    fn bg_palegreen3b(self) -> String {PaleGreen3b.to_bg_color_string()}
    fn bg_darkseagreen3a(self) -> String {DarkSeaGreen3a.to_bg_color_string()}
    fn bg_darkslategray3(self) -> String {DarkSlateGray3.to_bg_color_string()}
    fn bg_skyblue1(self) -> String {SkyBlue1.to_bg_color_string()}
    fn bg_chartreuse1(self) -> String {Chartreuse1.to_bg_color_string()}
    fn bg_lightgreen2(self) -> String {LightGreen2.to_bg_color_string()}
    fn bg_lightgreen3(self) -> String {LightGreen3.to_bg_color_string()}
    fn bg_palegreen1a(self) -> String {PaleGreen1a.to_bg_color_string()}
    fn bg_aquamarine1b(self) -> String {Aquamarine1b.to_bg_color_string()}
    fn bg_darkslategray1(self) -> String {DarkSlateGray1.to_bg_color_string()}
    fn bg_red3a(self) -> String {Red3a.to_bg_color_string()}
    fn bg_deeppink4c(self) -> String {DeepPink4c.to_bg_color_string()}
    fn bg_mediumvioletred(self) -> String {MediumVioletRed.to_bg_color_string()}
    fn bg_magenta3a(self) -> String {Magenta3a.to_bg_color_string()}
    fn bg_darkviolet1b(self) -> String {DarkViolet1b.to_bg_color_string()}
    fn bg_purple1b(self) -> String {Purple1b.to_bg_color_string()}
    fn bg_darkorange3a(self) -> String {DarkOrange3a.to_bg_color_string()}
    fn bg_indianred1a(self) -> String {IndianRed1a.to_bg_color_string()}
    fn bg_hotpink3a(self) -> String {HotPink3a.to_bg_color_string()}
    fn bg_mediumorchid3(self) -> String {MediumOrchid3.to_bg_color_string()}
    fn bg_mediumorchid(self) -> String {MediumOrchid.to_bg_color_string()}
    fn bg_mediumpurple2a(self) -> String {MediumPurple2a.to_bg_color_string()}
    fn bg_darkgoldenrod(self) -> String {DarkGoldenrod.to_bg_color_string()}
    fn bg_lightsalmon3a(self) -> String {LightSalmon3a.to_bg_color_string()}
    fn bg_rosybrown(self) -> String {RosyBrown.to_bg_color_string()}
    fn bg_grey63(self) -> String {Grey63.to_bg_color_string()}
    fn bg_mediumpurple2b(self) -> String {MediumPurple2b.to_bg_color_string()}
    fn bg_mediumpurple1(self) -> String {MediumPurple1.to_bg_color_string()}
    fn bg_gold3a(self) -> String {Gold3a.to_bg_color_string()}
    fn bg_darkkhaki(self) -> String {DarkKhaki.to_bg_color_string()}
    fn bg_navajowhite3(self) -> String {NavajoWhite3.to_bg_color_string()}
    fn bg_grey69(self) -> String {Grey69.to_bg_color_string()}
    fn bg_lightsteelblue3(self) -> String {LightSteelBlue3.to_bg_color_string()}
    fn bg_lightsteelblue(self) -> String {LightSteelBlue.to_bg_color_string()}
    fn bg_yellow3a(self) -> String {Yellow3a.to_bg_color_string()}
    fn bg_darkolivegreen3(self) -> String {DarkOliveGreen3.to_bg_color_string()}
    fn bg_darkseagreen3b(self) -> String {DarkSeaGreen3b.to_bg_color_string()}
    fn bg_darkseagreen2(self) -> String {DarkSeaGreen2.to_bg_color_string()}
    fn bg_lightcyan3(self) -> String {LightCyan3.to_bg_color_string()}
    fn bg_lightskyblue1(self) -> String {LightSkyBlue1.to_bg_color_string()}
    fn bg_greenyellow(self) -> String {GreenYellow.to_bg_color_string()}
    fn bg_darkolivegreen2(self) -> String {DarkOliveGreen2.to_bg_color_string()}
    fn bg_palegreen1b(self) -> String {PaleGreen1b.to_bg_color_string()}
    fn bg_darkseagreen5b(self) -> String {DarkSeaGreen5b.to_bg_color_string()}
    fn bg_darkseagreen5a(self) -> String {DarkSeaGreen5a.to_bg_color_string()}
    fn bg_paleturquoise1(self) -> String {PaleTurquoise1.to_bg_color_string()}
    fn bg_red3b(self) -> String {Red3b.to_bg_color_string()}
    fn bg_deeppink3a(self) -> String {DeepPink3a.to_bg_color_string()}
    fn bg_deeppink3b(self) -> String {DeepPink3b.to_bg_color_string()}
    fn bg_magenta3b(self) -> String {Magenta3b.to_bg_color_string()}
    fn bg_magenta3c(self) -> String {Magenta3c.to_bg_color_string()}
    fn bg_magenta2a(self) -> String {Magenta2a.to_bg_color_string()}
    fn bg_darkorange3b(self) -> String {DarkOrange3b.to_bg_color_string()}
    fn bg_indianred1b(self) -> String {IndianRed1b.to_bg_color_string()}
    fn bg_hotpink3b(self) -> String {HotPink3b.to_bg_color_string()}
    fn bg_hotpink2(self) -> String {HotPink2.to_bg_color_string()}
    fn bg_orchid(self) -> String {Orchid.to_bg_color_string()}
    fn bg_mediumorchid1a(self) -> String {MediumOrchid1a.to_bg_color_string()}
    fn bg_orange3(self) -> String {Orange3.to_bg_color_string()}
    fn bg_lightsalmon3b(self) -> String {LightSalmon3b.to_bg_color_string()}
    fn bg_lightpink3(self) -> String {LightPink3.to_bg_color_string()}
    fn bg_pink3(self) -> String {Pink3.to_bg_color_string()}
    fn bg_plum3(self) -> String {Plum3.to_bg_color_string()}
    fn bg_violet(self) -> String {Violet.to_bg_color_string()}
    fn bg_gold3b(self) -> String {Gold3b.to_bg_color_string()}
    fn bg_lightgoldenrod3(self) -> String {LightGoldenrod3.to_bg_color_string()}
    fn bg_tan(self) -> String {Tan.to_bg_color_string()}
    fn bg_mistyrose3(self) -> String {MistyRose3.to_bg_color_string()}
    fn bg_thistle3(self) -> String {Thistle3.to_bg_color_string()}
    fn bg_plum2(self) -> String {Plum2.to_bg_color_string()}
    fn bg_yellow3b(self) -> String {Yellow3b.to_bg_color_string()}
    fn bg_khaki3(self) -> String {Khaki3.to_bg_color_string()}
    fn bg_lightgoldenrod2a(self) -> String {LightGoldenrod2a.to_bg_color_string()}
    fn bg_lightyellow3(self) -> String {LightYellow3.to_bg_color_string()}
    fn bg_grey84(self) -> String {Grey84.to_bg_color_string()}
    fn bg_lightsteelblue1(self) -> String {LightSteelBlue1.to_bg_color_string()}
    fn bg_yellow2(self) -> String {Yellow2.to_bg_color_string()}
    fn bg_darkolivegreen1a(self) -> String {DarkOliveGreen1a.to_bg_color_string()}
    fn bg_darkolivegreen1b(self) -> String {DarkOliveGreen1b.to_bg_color_string()}
    fn bg_darkseagreen1(self) -> String {DarkSeaGreen1.to_bg_color_string()}
    fn bg_honeydew2(self) -> String {Honeydew2.to_bg_color_string()}
    fn bg_lightcyan1(self) -> String {LightCyan1.to_bg_color_string()}
    fn bg_red1(self) -> String {Red1.to_bg_color_string()}
    fn bg_deeppink2(self) -> String {DeepPink2.to_bg_color_string()}
    fn bg_deeppink1a(self) -> String {DeepPink1a.to_bg_color_string()}
    fn bg_deeppink1b(self) -> String {DeepPink1b.to_bg_color_string()}
    fn bg_magenta2b(self) -> String {Magenta2b.to_bg_color_string()}
    fn bg_magenta1(self) -> String {Magenta1.to_bg_color_string()}
    fn bg_orangered1(self) -> String {OrangeRed1.to_bg_color_string()}
    fn bg_indianred1c(self) -> String {IndianRed1c.to_bg_color_string()}
    fn bg_indianred1d(self) -> String {IndianRed1d.to_bg_color_string()}
    fn bg_hotpink1a(self) -> String {HotPink1a.to_bg_color_string()}
    fn bg_hotpink1b(self) -> String {HotPink1b.to_bg_color_string()}
    fn bg_mediumorchid1b(self) -> String {MediumOrchid1b.to_bg_color_string()}
    fn bg_darkorange(self) -> String {DarkOrange.to_bg_color_string()}
    fn bg_salmon1(self) -> String {Salmon1.to_bg_color_string()}
    fn bg_lightcoral(self) -> String {LightCoral.to_bg_color_string()}
    fn bg_palevioletred1(self) -> String {PaleVioletRed1.to_bg_color_string()}
    fn bg_orchid2(self) -> String {Orchid2.to_bg_color_string()}
    fn bg_orchid1(self) -> String {Orchid1.to_bg_color_string()}
    fn bg_orange1(self) -> String {Orange1.to_bg_color_string()}
    fn bg_sandybrown(self) -> String {SandyBrown.to_bg_color_string()}
    fn bg_lightsalmon1(self) -> String {LightSalmon1.to_bg_color_string()}
    fn bg_lightpink1(self) -> String {LightPink1.to_bg_color_string()}
    fn bg_pink1(self) -> String {Pink1.to_bg_color_string()}
    fn bg_plum1(self) -> String {Plum1.to_bg_color_string()}
    fn bg_gold1(self) -> String {Gold1.to_bg_color_string()}
    fn bg_lightgoldenrod2b(self) -> String {LightGoldenrod2b.to_bg_color_string()}
    fn bg_lightgoldenrod2c(self) -> String {LightGoldenrod2c.to_bg_color_string()}
    fn bg_navajowhite1(self) -> String {NavajoWhite1.to_bg_color_string()}
    fn bg_mistyrose1(self) -> String {MistyRose1.to_bg_color_string()}
    fn bg_thistle1(self) -> String {Thistle1.to_bg_color_string()}
    fn bg_yellow1(self) -> String {Yellow1.to_bg_color_string()}
    fn bg_lightgoldenrod1(self) -> String {LightGoldenrod1.to_bg_color_string()}
    fn bg_khaki1(self) -> String {Khaki1.to_bg_color_string()}
    fn bg_wheat1(self) -> String {Wheat1.to_bg_color_string()}
    fn bg_cornsilk1(self) -> String {CornSilk1.to_bg_color_string()}
    fn bg_grey100(self) -> String {Grey100.to_bg_color_string()}
    fn bg_grey3(self) -> String {Grey3.to_bg_color_string()}
    fn bg_grey7(self) -> String {Grey7.to_bg_color_string()}
    fn bg_grey11(self) -> String {Grey11.to_bg_color_string()}
    fn bg_grey15(self) -> String {Grey15.to_bg_color_string()}
    fn bg_grey19(self) -> String {Grey19.to_bg_color_string()}
    fn bg_grey23(self) -> String {Grey23.to_bg_color_string()}
    fn bg_grey27(self) -> String {Grey27.to_bg_color_string()}
    fn bg_grey30(self) -> String {Grey30.to_bg_color_string()}
    fn bg_grey35(self) -> String {Grey35.to_bg_color_string()}
    fn bg_grey39(self) -> String {Grey39.to_bg_color_string()}
    fn bg_grey42(self) -> String {Grey42.to_bg_color_string()}
    fn bg_grey46(self) -> String {Grey46.to_bg_color_string()}
    fn bg_grey50(self) -> String {Grey50.to_bg_color_string()}
    fn bg_grey54(self) -> String {Grey54.to_bg_color_string()}
    fn bg_grey58(self) -> String {Grey58.to_bg_color_string()}
    fn bg_grey62(self) -> String {Grey62.to_bg_color_string()}
    fn bg_grey66(self) -> String {Grey66.to_bg_color_string()}
    fn bg_grey70(self) -> String {Grey70.to_bg_color_string()}
    fn bg_grey74(self) -> String {Grey74.to_bg_color_string()}
    fn bg_grey78(self) -> String {Grey78.to_bg_color_string()}
    fn bg_grey82(self) -> String {Grey82.to_bg_color_string()}
    fn bg_grey85(self) -> String {Grey85.to_bg_color_string()}
    fn bg_grey89(self) -> String {Grey89.to_bg_color_string()}
    fn bg_grey93(self) -> String {Grey93.to_bg_color_string()}
}

