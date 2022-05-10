use std::fmt::{Display, Formatter};

use colors::EightBitColors;

pub mod colors;
pub mod csi;
pub mod sys;

pub trait Color {
    fn fg_color<C: colors::Color>(self, color: C) -> String;
    fn bg_color<C: colors::Color>(self, color: C) -> String;
    fn black(self) -> String;
    fn red(self) -> String;
    fn green(self) -> String;
    fn yellow(self) -> String;
    fn blue(self) -> String;
    fn magenta(self) -> String;
    fn cyan(self) -> String;
    fn lightgray(self) -> String;
    fn darkgray(self) -> String;
    fn lightred(self) -> String;
    fn lightgreen(self) -> String;
    fn lightyellow(self) -> String;
    fn lightblue(self) -> String;
    fn lightmagenta(self) -> String;
    fn lightcyan(self) -> String;
    fn white(self) -> String;
    fn grey0(self) -> String;
    fn navyblue(self) -> String;
    fn darkblue(self) -> String;
    fn blue3a(self) -> String;
    fn blue3b(self) -> String;
    fn blue1(self) -> String;
    fn darkgreen(self) -> String;
    fn deepskyblue4a(self) -> String;
    fn deepskyblue4b(self) -> String;
    fn deepskyblue4c(self) -> String;
    fn dodgerblue3(self) -> String;
    fn dodgerblue2(self) -> String;
    fn green4(self) -> String;
    fn springgreen4(self) -> String;
    fn turquoise4(self) -> String;
    fn deepskyblue3a(self) -> String;
    fn deepskyblue3b(self) -> String;
    fn dodgerblue1(self) -> String;
    fn green3a(self) -> String;
    fn springgreen3a(self) -> String;
    fn darkcyan(self) -> String;
    fn lightseagreen(self) -> String;
    fn deepskyblue2(self) -> String;
    fn deepskyblue1(self) -> String;
    fn green3b(self) -> String;
    fn springgreen3b(self) -> String;
    fn springgreen2a(self) -> String;
    fn cyan3(self) -> String;
    fn darkturquoise(self) -> String;
    fn turquoise2(self) -> String;
    fn green1(self) -> String;
    fn springgreen2b(self) -> String;
    fn springgreen1(self) -> String;
    fn mediumspringgreen(self) -> String;
    fn cyan2(self) -> String;
    fn cyan1(self) -> String;
    fn darkred1(self) -> String;
    fn deeppink4a(self) -> String;
    fn purple4a(self) -> String;
    fn purple4b(self) -> String;
    fn purple3(self) -> String;
    fn blueviolet(self) -> String;
    fn orange4a(self) -> String;
    fn grey37(self) -> String;
    fn mediumpurple4(self) -> String;
    fn slateblue3a(self) -> String;
    fn slateblue3b(self) -> String;
    fn royalblue1(self) -> String;
    fn chartreuse4(self) -> String;
    fn darkseagreen4a(self) -> String;
    fn paleturquoise4(self) -> String;
    fn steelblue(self) -> String;
    fn steelblue3(self) -> String;
    fn cornflowerblue(self) -> String;
    fn chartreuse3a(self) -> String;
    fn darkseagreen4b(self) -> String;
    fn cadetblue2(self) -> String;
    fn cadetblue1(self) -> String;
    fn skyblue3(self) -> String;
    fn steelblue1a(self) -> String;
    fn chartreuse3b(self) -> String;
    fn palegreen3a(self) -> String;
    fn seagreen3(self) -> String;
    fn aquamarine3(self) -> String;
    fn mediumturquoise(self) -> String;
    fn steelblue1b(self) -> String;
    fn chartreuse2a(self) -> String;
    fn seagreen2(self) -> String;
    fn seagreen1a(self) -> String;
    fn seagreen1b(self) -> String;
    fn aquamarine1a(self) -> String;
    fn darkslategray2(self) -> String;
    fn darkred2(self) -> String;
    fn deeppink4b(self) -> String;
    fn darkmagenta1(self) -> String;
    fn darkmagenta2(self) -> String;
    fn darkviolet1a(self) -> String;
    fn purple1a(self) -> String;
    fn orange4b(self) -> String;
    fn lightpink4(self) -> String;
    fn plum4(self) -> String;
    fn mediumpurple3a(self) -> String;
    fn mediumpurple3b(self) -> String;
    fn slateblue1(self) -> String;
    fn yellow4a(self) -> String;
    fn wheat4(self) -> String;
    fn grey53(self) -> String;
    fn lightslategrey(self) -> String;
    fn mediumpurple(self) -> String;
    fn lightslateblue(self) -> String;
    fn yellow4b(self) -> String;
    fn darkolivegreen3a(self) -> String;
    fn darkgreensea(self) -> String;
    fn lightskyblue3a(self) -> String;
    fn lightskyblue3b(self) -> String;
    fn skyblue2(self) -> String;
    fn chartreuse2b(self) -> String;
    fn darkolivegreen3b(self) -> String;
    fn palegreen3b(self) -> String;
    fn darkseagreen3a(self) -> String;
    fn darkslategray3(self) -> String;
    fn skyblue1(self) -> String;
    fn chartreuse1(self) -> String;
    fn lightgreen2(self) -> String;
    fn lightgreen3(self) -> String;
    fn palegreen1a(self) -> String;
    fn aquamarine1b(self) -> String;
    fn darkslategray1(self) -> String;
    fn red3a(self) -> String;
    fn deeppink4c(self) -> String;
    fn mediumvioletred(self) -> String;
    fn magenta3a(self) -> String;
    fn darkviolet1b(self) -> String;
    fn purple1b(self) -> String;
    fn darkorange3a(self) -> String;
    fn indianred1a(self) -> String;
    fn hotpink3a(self) -> String;
    fn mediumorchid3(self) -> String;
    fn mediumorchid(self) -> String;
    fn mediumpurple2a(self) -> String;
    fn darkgoldenrod(self) -> String;
    fn lightsalmon3a(self) -> String;
    fn rosybrown(self) -> String;
    fn grey63(self) -> String;
    fn mediumpurple2b(self) -> String;
    fn mediumpurple1(self) -> String;
    fn gold3a(self) -> String;
    fn darkkhaki(self) -> String;
    fn navajowhite3(self) -> String;
    fn grey69(self) -> String;
    fn lightsteelblue3(self) -> String;
    fn lightsteelblue(self) -> String;
    fn yellow3a(self) -> String;
    fn darkolivegreen3(self) -> String;
    fn darkseagreen3b(self) -> String;
    fn darkseagreen2(self) -> String;
    fn lightcyan3(self) -> String;
    fn lightskyblue1(self) -> String;
    fn greenyellow(self) -> String;
    fn darkolivegreen2(self) -> String;
    fn palegreen1b(self) -> String;
    fn darkseagreen5b(self) -> String;
    fn darkseagreen5a(self) -> String;
    fn paleturquoise1(self) -> String;
    fn red3b(self) -> String;
    fn deeppink3a(self) -> String;
    fn deeppink3b(self) -> String;
    fn magenta3b(self) -> String;
    fn magenta3c(self) -> String;
    fn magenta2a(self) -> String;
    fn darkorange3b(self) -> String;
    fn indianred1b(self) -> String;
    fn hotpink3b(self) -> String;
    fn hotpink2(self) -> String;
    fn orchid(self) -> String;
    fn mediumorchid1a(self) -> String;
    fn orange3(self) -> String;
    fn lightsalmon3b(self) -> String;
    fn lightpink3(self) -> String;
    fn pink3(self) -> String;
    fn plum3(self) -> String;
    fn violet(self) -> String;
    fn gold3b(self) -> String;
    fn lightgoldenrod3(self) -> String;
    fn tan(self) -> String;
    fn mistyrose3(self) -> String;
    fn thistle3(self) -> String;
    fn plum2(self) -> String;
    fn yellow3b(self) -> String;
    fn khaki3(self) -> String;
    fn lightgoldenrod2a(self) -> String;
    fn lightyellow3(self) -> String;
    fn grey84(self) -> String;
    fn lightsteelblue1(self) -> String;
    fn yellow2(self) -> String;
    fn darkolivegreen1a(self) -> String;
    fn darkolivegreen1b(self) -> String;
    fn darkseagreen1(self) -> String;
    fn honeydew2(self) -> String;
    fn lightcyan1(self) -> String;
    fn red1(self) -> String;
    fn deeppink2(self) -> String;
    fn deeppink1a(self) -> String;
    fn deeppink1b(self) -> String;
    fn magenta2b(self) -> String;
    fn magenta1(self) -> String;
    fn orangered1(self) -> String;
    fn indianred1c(self) -> String;
    fn indianred1d(self) -> String;
    fn hotpink1a(self) -> String;
    fn hotpink1b(self) -> String;
    fn mediumorchid1b(self) -> String;
    fn darkorange(self) -> String;
    fn salmon1(self) -> String;
    fn lightcoral(self) -> String;
    fn palevioletred1(self) -> String;
    fn orchid2(self) -> String;
    fn orchid1(self) -> String;
    fn orange1(self) -> String;
    fn sandybrown(self) -> String;
    fn lightsalmon1(self) -> String;
    fn lightpink1(self) -> String;
    fn pink1(self) -> String;
    fn plum1(self) -> String;
    fn gold1(self) -> String;
    fn lightgoldenrod2b(self) -> String;
    fn lightgoldenrod2c(self) -> String;
    fn navajowhite1(self) -> String;
    fn mistyrose1(self) -> String;
    fn thistle1(self) -> String;
    fn yellow1(self) -> String;
    fn lightgoldenrod1(self) -> String;
    fn khaki1(self) -> String;
    fn wheat1(self) -> String;
    fn cornsilk1(self) -> String;
    fn grey100(self) -> String;
    fn grey3(self) -> String;
    fn grey7(self) -> String;
    fn grey11(self) -> String;
    fn grey15(self) -> String;
    fn grey19(self) -> String;
    fn grey23(self) -> String;
    fn grey27(self) -> String;
    fn grey30(self) -> String;
    fn grey35(self) -> String;
    fn grey39(self) -> String;
    fn grey42(self) -> String;
    fn grey46(self) -> String;
    fn grey50(self) -> String;
    fn grey54(self) -> String;
    fn grey58(self) -> String;
    fn grey62(self) -> String;
    fn grey66(self) -> String;
    fn grey70(self) -> String;
    fn grey74(self) -> String;
    fn grey78(self) -> String;
    fn grey82(self) -> String;
    fn grey85(self) -> String;
    fn grey89(self) -> String;
    fn grey93(self) -> String;
}

impl<T> Color for T where T: StringMarker {
    fn fg_color<C: colors::Color>(self, color: C) -> String {
        format!("{}{}m{}{}",csi::SET_FG_COLOR,&color.to_color_string(),self.to_string(),csi::RESET)
    }
    fn bg_color<C: colors::Color>(self, color: C) -> String {
        format!("{}{}m{}{}",csi::SET_BG_COLOR,&color.to_color_string(),self.to_string(),csi::RESET)
    }
    fn black(self) -> String { format!("{}{}m{}{}",csi::SET_FG_COLOR,EightBitColors::Black,self.to_string(),csi::RESET) }
    fn red(self) -> String { format!("{}{}m{}{}",csi::SET_FG_COLOR,EightBitColors::Red,self.to_string(),csi::RESET) }
    fn green(self) -> String {"".to_string()}
    fn yellow(self) -> String {"".to_string()}
    fn blue(self) -> String {"".to_string()}
    fn magenta(self) -> String {"".to_string()}
    fn cyan(self) -> String {"".to_string()}
    fn lightgray(self) -> String {"".to_string()}
    fn darkgray(self) -> String {"".to_string()}
    fn lightred(self) -> String {"".to_string()}
    fn lightgreen(self) -> String {"".to_string()}
    fn lightyellow(self) -> String {"".to_string()}
    fn lightblue(self) -> String {"".to_string()}
    fn lightmagenta(self) -> String {"".to_string()}
    fn lightcyan(self) -> String {"".to_string()}
    fn white(self) -> String {"".to_string()}
    fn grey0(self) -> String {"".to_string()}
    fn navyblue(self) -> String {"".to_string()}
    fn darkblue(self) -> String {"".to_string()}
    fn blue3a(self) -> String {"".to_string()}
    fn blue3b(self) -> String {"".to_string()}
    fn blue1(self) -> String {"".to_string()}
    fn darkgreen(self) -> String {"".to_string()}
    fn deepskyblue4a(self) -> String {"".to_string()}
    fn deepskyblue4b(self) -> String {"".to_string()}
    fn deepskyblue4c(self) -> String {"".to_string()}
    fn dodgerblue3(self) -> String {"".to_string()}
    fn dodgerblue2(self) -> String {"".to_string()}
    fn green4(self) -> String {"".to_string()}
    fn springgreen4(self) -> String {"".to_string()}
    fn turquoise4(self) -> String {"".to_string()}
    fn deepskyblue3a(self) -> String {"".to_string()}
    fn deepskyblue3b(self) -> String {"".to_string()}
    fn dodgerblue1(self) -> String {"".to_string()}
    fn green3a(self) -> String {"".to_string()}
    fn springgreen3a(self) -> String {"".to_string()}
    fn darkcyan(self) -> String {"".to_string()}
    fn lightseagreen(self) -> String {"".to_string()}
    fn deepskyblue2(self) -> String {"".to_string()}
    fn deepskyblue1(self) -> String {"".to_string()}
    fn green3b(self) -> String {"".to_string()}
    fn springgreen3b(self) -> String {"".to_string()}
    fn springgreen2a(self) -> String {"".to_string()}
    fn cyan3(self) -> String {"".to_string()}
    fn darkturquoise(self) -> String {"".to_string()}
    fn turquoise2(self) -> String {"".to_string()}
    fn green1(self) -> String {"".to_string()}
    fn springgreen2b(self) -> String {"".to_string()}
    fn springgreen1(self) -> String {"".to_string()}
    fn mediumspringgreen(self) -> String {"".to_string()}
    fn cyan2(self) -> String {"".to_string()}
    fn cyan1(self) -> String {"".to_string()}
    fn darkred1(self) -> String {"".to_string()}
    fn deeppink4a(self) -> String {"".to_string()}
    fn purple4a(self) -> String {"".to_string()}
    fn purple4b(self) -> String {"".to_string()}
    fn purple3(self) -> String {"".to_string()}
    fn blueviolet(self) -> String {"".to_string()}
    fn orange4a(self) -> String {"".to_string()}
    fn grey37(self) -> String {"".to_string()}
    fn mediumpurple4(self) -> String {"".to_string()}
    fn slateblue3a(self) -> String {"".to_string()}
    fn slateblue3b(self) -> String {"".to_string()}
    fn royalblue1(self) -> String {"".to_string()}
    fn chartreuse4(self) -> String {"".to_string()}
    fn darkseagreen4a(self) -> String {"".to_string()}
    fn paleturquoise4(self) -> String {"".to_string()}
    fn steelblue(self) -> String {"".to_string()}
    fn steelblue3(self) -> String {"".to_string()}
    fn cornflowerblue(self) -> String {"".to_string()}
    fn chartreuse3a(self) -> String {"".to_string()}
    fn darkseagreen4b(self) -> String {"".to_string()}
    fn cadetblue2(self) -> String {"".to_string()}
    fn cadetblue1(self) -> String {"".to_string()}
    fn skyblue3(self) -> String {"".to_string()}
    fn steelblue1a(self) -> String {"".to_string()}
    fn chartreuse3b(self) -> String {"".to_string()}
    fn palegreen3a(self) -> String {"".to_string()}
    fn seagreen3(self) -> String {"".to_string()}
    fn aquamarine3(self) -> String {"".to_string()}
    fn mediumturquoise(self) -> String {"".to_string()}
    fn steelblue1b(self) -> String {"".to_string()}
    fn chartreuse2a(self) -> String {"".to_string()}
    fn seagreen2(self) -> String {"".to_string()}
    fn seagreen1a(self) -> String {"".to_string()}
    fn seagreen1b(self) -> String {"".to_string()}
    fn aquamarine1a(self) -> String {"".to_string()}
    fn darkslategray2(self) -> String {"".to_string()}
    fn darkred2(self) -> String {"".to_string()}
    fn deeppink4b(self) -> String {"".to_string()}
    fn darkmagenta1(self) -> String {"".to_string()}
    fn darkmagenta2(self) -> String {"".to_string()}
    fn darkviolet1a(self) -> String {"".to_string()}
    fn purple1a(self) -> String {"".to_string()}
    fn orange4b(self) -> String {"".to_string()}
    fn lightpink4(self) -> String {"".to_string()}
    fn plum4(self) -> String {"".to_string()}
    fn mediumpurple3a(self) -> String {"".to_string()}
    fn mediumpurple3b(self) -> String {"".to_string()}
    fn slateblue1(self) -> String {"".to_string()}
    fn yellow4a(self) -> String {"".to_string()}
    fn wheat4(self) -> String {"".to_string()}
    fn grey53(self) -> String {"".to_string()}
    fn lightslategrey(self) -> String {"".to_string()}
    fn mediumpurple(self) -> String {"".to_string()}
    fn lightslateblue(self) -> String {"".to_string()}
    fn yellow4b(self) -> String {"".to_string()}
    fn darkolivegreen3a(self) -> String {"".to_string()}
    fn darkgreensea(self) -> String {"".to_string()}
    fn lightskyblue3a(self) -> String {"".to_string()}
    fn lightskyblue3b(self) -> String {"".to_string()}
    fn skyblue2(self) -> String {"".to_string()}
    fn chartreuse2b(self) -> String {"".to_string()}
    fn darkolivegreen3b(self) -> String {"".to_string()}
    fn palegreen3b(self) -> String {"".to_string()}
    fn darkseagreen3a(self) -> String {"".to_string()}
    fn darkslategray3(self) -> String {"".to_string()}
    fn skyblue1(self) -> String {"".to_string()}
    fn chartreuse1(self) -> String {"".to_string()}
    fn lightgreen2(self) -> String {"".to_string()}
    fn lightgreen3(self) -> String {"".to_string()}
    fn palegreen1a(self) -> String {"".to_string()}
    fn aquamarine1b(self) -> String {"".to_string()}
    fn darkslategray1(self) -> String {"".to_string()}
    fn red3a(self) -> String {"".to_string()}
    fn deeppink4c(self) -> String {"".to_string()}
    fn mediumvioletred(self) -> String {"".to_string()}
    fn magenta3a(self) -> String {"".to_string()}
    fn darkviolet1b(self) -> String {"".to_string()}
    fn purple1b(self) -> String {"".to_string()}
    fn darkorange3a(self) -> String {"".to_string()}
    fn indianred1a(self) -> String {"".to_string()}
    fn hotpink3a(self) -> String {"".to_string()}
    fn mediumorchid3(self) -> String {"".to_string()}
    fn mediumorchid(self) -> String {"".to_string()}
    fn mediumpurple2a(self) -> String {"".to_string()}
    fn darkgoldenrod(self) -> String {"".to_string()}
    fn lightsalmon3a(self) -> String {"".to_string()}
    fn rosybrown(self) -> String {"".to_string()}
    fn grey63(self) -> String {"".to_string()}
    fn mediumpurple2b(self) -> String {"".to_string()}
    fn mediumpurple1(self) -> String {"".to_string()}
    fn gold3a(self) -> String {"".to_string()}
    fn darkkhaki(self) -> String {"".to_string()}
    fn navajowhite3(self) -> String {"".to_string()}
    fn grey69(self) -> String {"".to_string()}
    fn lightsteelblue3(self) -> String {"".to_string()}
    fn lightsteelblue(self) -> String {"".to_string()}
    fn yellow3a(self) -> String {"".to_string()}
    fn darkolivegreen3(self) -> String {"".to_string()}
    fn darkseagreen3b(self) -> String {"".to_string()}
    fn darkseagreen2(self) -> String {"".to_string()}
    fn lightcyan3(self) -> String {"".to_string()}
    fn lightskyblue1(self) -> String {"".to_string()}
    fn greenyellow(self) -> String {"".to_string()}
    fn darkolivegreen2(self) -> String {"".to_string()}
    fn palegreen1b(self) -> String {"".to_string()}
    fn darkseagreen5b(self) -> String {"".to_string()}
    fn darkseagreen5a(self) -> String {"".to_string()}
    fn paleturquoise1(self) -> String {"".to_string()}
    fn red3b(self) -> String {"".to_string()}
    fn deeppink3a(self) -> String {"".to_string()}
    fn deeppink3b(self) -> String {"".to_string()}
    fn magenta3b(self) -> String {"".to_string()}
    fn magenta3c(self) -> String {"".to_string()}
    fn magenta2a(self) -> String {"".to_string()}
    fn darkorange3b(self) -> String {"".to_string()}
    fn indianred1b(self) -> String {"".to_string()}
    fn hotpink3b(self) -> String {"".to_string()}
    fn hotpink2(self) -> String {"".to_string()}
    fn orchid(self) -> String {"".to_string()}
    fn mediumorchid1a(self) -> String {"".to_string()}
    fn orange3(self) -> String {"".to_string()}
    fn lightsalmon3b(self) -> String {"".to_string()}
    fn lightpink3(self) -> String {"".to_string()}
    fn pink3(self) -> String {"".to_string()}
    fn plum3(self) -> String {"".to_string()}
    fn violet(self) -> String {"".to_string()}
    fn gold3b(self) -> String {"".to_string()}
    fn lightgoldenrod3(self) -> String {"".to_string()}
    fn tan(self) -> String {"".to_string()}
    fn mistyrose3(self) -> String {"".to_string()}
    fn thistle3(self) -> String {"".to_string()}
    fn plum2(self) -> String {"".to_string()}
    fn yellow3b(self) -> String {"".to_string()}
    fn khaki3(self) -> String {"".to_string()}
    fn lightgoldenrod2a(self) -> String {"".to_string()}
    fn lightyellow3(self) -> String {"".to_string()}
    fn grey84(self) -> String {"".to_string()}
    fn lightsteelblue1(self) -> String {"".to_string()}
    fn yellow2(self) -> String {"".to_string()}
    fn darkolivegreen1a(self) -> String {"".to_string()}
    fn darkolivegreen1b(self) -> String {"".to_string()}
    fn darkseagreen1(self) -> String {"".to_string()}
    fn honeydew2(self) -> String {"".to_string()}
    fn lightcyan1(self) -> String {"".to_string()}
    fn red1(self) -> String {"".to_string()}
    fn deeppink2(self) -> String {"".to_string()}
    fn deeppink1a(self) -> String {"".to_string()}
    fn deeppink1b(self) -> String {"".to_string()}
    fn magenta2b(self) -> String {"".to_string()}
    fn magenta1(self) -> String {"".to_string()}
    fn orangered1(self) -> String {"".to_string()}
    fn indianred1c(self) -> String {"".to_string()}
    fn indianred1d(self) -> String {"".to_string()}
    fn hotpink1a(self) -> String {"".to_string()}
    fn hotpink1b(self) -> String {"".to_string()}
    fn mediumorchid1b(self) -> String {"".to_string()}
    fn darkorange(self) -> String {"".to_string()}
    fn salmon1(self) -> String {"".to_string()}
    fn lightcoral(self) -> String {"".to_string()}
    fn palevioletred1(self) -> String {"".to_string()}
    fn orchid2(self) -> String {"".to_string()}
    fn orchid1(self) -> String {"".to_string()}
    fn orange1(self) -> String {"".to_string()}
    fn sandybrown(self) -> String {"".to_string()}
    fn lightsalmon1(self) -> String {"".to_string()}
    fn lightpink1(self) -> String {"".to_string()}
    fn pink1(self) -> String {"".to_string()}
    fn plum1(self) -> String {"".to_string()}
    fn gold1(self) -> String {"".to_string()}
    fn lightgoldenrod2b(self) -> String {"".to_string()}
    fn lightgoldenrod2c(self) -> String {"".to_string()}
    fn navajowhite1(self) -> String {"".to_string()}
    fn mistyrose1(self) -> String {"".to_string()}
    fn thistle1(self) -> String {"".to_string()}
    fn yellow1(self) -> String {"".to_string()}
    fn lightgoldenrod1(self) -> String {"".to_string()}
    fn khaki1(self) -> String {"".to_string()}
    fn wheat1(self) -> String {"".to_string()}
    fn cornsilk1(self) -> String {"".to_string()}
    fn grey100(self) -> String {"".to_string()}
    fn grey3(self) -> String {"".to_string()}
    fn grey7(self) -> String {"".to_string()}
    fn grey11(self) -> String {"".to_string()}
    fn grey15(self) -> String {"".to_string()}
    fn grey19(self) -> String {"".to_string()}
    fn grey23(self) -> String {"".to_string()}
    fn grey27(self) -> String {"".to_string()}
    fn grey30(self) -> String {"".to_string()}
    fn grey35(self) -> String {"".to_string()}
    fn grey39(self) -> String {"".to_string()}
    fn grey42(self) -> String {"".to_string()}
    fn grey46(self) -> String {"".to_string()}
    fn grey50(self) -> String {"".to_string()}
    fn grey54(self) -> String {"".to_string()}
    fn grey58(self) -> String {"".to_string()}
    fn grey62(self) -> String {"".to_string()}
    fn grey66(self) -> String {"".to_string()}
    fn grey70(self) -> String {"".to_string()}
    fn grey74(self) -> String {"".to_string()}
    fn grey78(self) -> String {"".to_string()}
    fn grey82(self) -> String {"".to_string()}
    fn grey85(self) -> String {"".to_string()}
    fn grey89(self) -> String {"".to_string()}
    fn grey93(self) -> String {"".to_string()}
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
