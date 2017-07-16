#![feature(test)]
#![feature(plugin)]
#![plugin(phf_macros)]

extern crate phf;
#[macro_use]
extern crate static_map_macros;
#[macro_use]
extern crate static_map;
extern crate test;

use test::Bencher;
use test::black_box;

use static_map::Map;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Symbol {
    pub unicode: u32,
    pub atom_type: AtomType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AtomType {
    Punctuation,
    Ordinal,
    Open,
    Close,
    Binary,
    Relation,
    Accent,
    AccentWide,
    BotAccent,
    BotAccentWide,
    Alpha,
    Fence,
    Operator(bool), // bool := limits or nolimits?
    Over,
    Under,
    Inner,
    Transparent,
}


static SYMBOLS_PHF: phf::Map<&'static str, Symbol> = phf_map! {
    // unicode-math.dtx command table
    // "mathexclam" => Symbol { unicode: 0x21, atom_type: AtomType::Punctuation }, // Unicode: 0x21, exclamation mark
    // "mathoctothorpe" => Symbol { unicode: 0x23, atom_type: AtomType::Alpha }, // Unicode: 0x23, number sign
    // "mathdollar" => Symbol { unicode: 0x24, atom_type: AtomType::Alpha }, // Unicode: 0x24, dollar sign
    // "mathpercent" => Symbol { unicode: 0x25, atom_type: AtomType::Alpha }, // Unicode: 0x25, percent sign
    // "mathampersand" => Symbol { unicode: 0x26, atom_type: AtomType::Alpha }, // Unicode: 0x26, ampersand
    "lparen" => Symbol { unicode: 0x28, atom_type: AtomType::Open }, // Unicode: 0x28, left parenthesis
    "rparen" => Symbol { unicode: 0x29, atom_type: AtomType::Close }, // Unicode: 0x29, right parenthesis
    // "mathplus" => Symbol { unicode: 0x2B, atom_type: AtomType::Binary }, // Unicode: 0x2B, plus sign b:
    // "mathcomma" => Symbol { unicode: 0x2C, atom_type: AtomType::Punctuation }, // Unicode: 0x2C, comma
    // "mathperiod" => Symbol { unicode: 0x2E, atom_type: AtomType::Alpha }, // Unicode: 0x2E, full stop, period
    // "mathslash" => Symbol { unicode: 0x2F, atom_type: AtomType::Alpha }, // Unicode: 0x2F, solidus
    // "mathcolon" => Symbol { unicode: 0x3A, atom_type: AtomType::Punctuation }, // Unicode: 0x3A, colon
    // "mathsemicolon" => Symbol { unicode: 0x3B, atom_type: AtomType::Punctuation }, // Unicode: 0x3B, semicolon p:
    // "less" => Symbol { unicode: 0x3C, atom_type: AtomType::Relation }, // Unicode: 0x3C, less-than sign r:
    // "equal" => Symbol { unicode: 0x3D, atom_type: AtomType::Relation }, // Unicode: 0x3D, equals sign r:
    // "greater" => Symbol { unicode: 0x3E, atom_type: AtomType::Relation }, // Unicode: 0x3E, greater-than sign r:
    // "mathquestion" => Symbol { unicode: 0x3F, atom_type: AtomType::Alpha }, // Unicode: 0x3F, question mark
    // "mathatsign" => Symbol { unicode: 0x40, atom_type: AtomType::Alpha }, // Unicode: 0x40, commercial at
    "lbrack" => Symbol { unicode: 0x5B, atom_type: AtomType::Open }, // Unicode: 0x5B, left square bracket
    "backslash" => Symbol { unicode: 0x5C, atom_type: AtomType::Alpha }, // Unicode: 0x5C, reverse solidus
    "rbrack" => Symbol { unicode: 0x5D, atom_type: AtomType::Close }, // Unicode: 0x5D, right square bracket
    "lbrace" => Symbol { unicode: 0x7B, atom_type: AtomType::Open }, // Unicode: 0x7B, left curly bracket
    "vert" => Symbol { unicode: 0x7C, atom_type: AtomType::Fence }, // Unicode: 0x7C, vertical bar
    "rbrace" => Symbol { unicode: 0x7D, atom_type: AtomType::Close }, // Unicode: 0x7D, right curly bracket
    // "mathsterling" => Symbol { unicode: 0xA3, atom_type: AtomType::Alpha }, // Unicode: 0xA3, pound sign
    // "mathyen" => Symbol { unicode: 0xA5, atom_type: AtomType::Alpha }, // Unicode: 0xA5, yen sign
    "neg" => Symbol { unicode: 0xAC, atom_type: AtomType::Alpha }, // Unicode: 0xAC, /neg /lnot not sign
    "pm" => Symbol { unicode: 0xB1, atom_type: AtomType::Binary }, // Unicode: 0xB1, plus-or-minus sign
    "cdotp" => Symbol { unicode: 0xB7, atom_type: AtomType::Binary }, // Unicode: 0xB7, /centerdot b: middle dot
    "times" => Symbol { unicode: 0xD7, atom_type: AtomType::Binary }, // Unicode: 0xD7, multiply sign
    "matheth" => Symbol { unicode: 0xF0, atom_type: AtomType::Alpha }, // Unicode: 0xF0, eth
    "div" => Symbol { unicode: 0xF7, atom_type: AtomType::Binary }, // Unicode: 0xF7, divide sign
    "Zbar" => Symbol { unicode: 0x1B5, atom_type: AtomType::Alpha }, // Unicode: 0x1B5, impedance (latin capital letter z with stroke)
    "grave" => Symbol { unicode: 0x300, atom_type: AtomType::Accent }, // Unicode: 0x300, grave accent
    "acute" => Symbol { unicode: 0x301, atom_type: AtomType::Accent }, // Unicode: 0x301, acute accent
    "hat" => Symbol { unicode: 0x302, atom_type: AtomType::Accent }, // Unicode: 0x302, circumflex accent
    "widehat" => Symbol { unicode: 0x302, atom_type: AtomType::AccentWide }, // Unicode: 0x302, circumflex accent
    "tilde" => Symbol { unicode: 0x303, atom_type: AtomType::Accent }, // Unicode: 0x303, tilde
    "widetilde" => Symbol { unicode: 0x303, atom_type: AtomType::AccentWide }, // Unicode: 0x303, tilde
    "bar" => Symbol { unicode: 0x304, atom_type: AtomType::Accent }, // Unicode: 0x304, macron
    "overbar" => Symbol { unicode: 0x305, atom_type: AtomType::Accent }, // Unicode: 0x305, overbar embellishment
    "breve" => Symbol { unicode: 0x306, atom_type: AtomType::Accent }, // Unicode: 0x306, breve
    "dot" => Symbol { unicode: 0x307, atom_type: AtomType::Accent }, // Unicode: 0x307, dot above
    "ddot" => Symbol { unicode: 0x308, atom_type: AtomType::Accent }, // Unicode: 0x308, dieresis
    "ovhook" => Symbol { unicode: 0x309, atom_type: AtomType::Accent }, // Unicode: 0x309, combining hook above
    "ocirc" => Symbol { unicode: 0x30A, atom_type: AtomType::Accent }, // Unicode: 0x30A, ring
    "check" => Symbol { unicode: 0x30C, atom_type: AtomType::Accent }, // Unicode: 0x30C, caron
    "candra" => Symbol { unicode: 0x310, atom_type: AtomType::Accent }, // Unicode: 0x310, candrabindu (non-spacing)
    "oturnedcomma" => Symbol { unicode: 0x312, atom_type: AtomType::Accent }, // Unicode: 0x312, combining turned comma above
    "ocommatopright" => Symbol { unicode: 0x315, atom_type: AtomType::Accent }, // Unicode: 0x315, combining comma above right
    "droang" => Symbol { unicode: 0x31A, atom_type: AtomType::Accent }, // Unicode: 0x31A, left angle above (non-spacing)
    "wideutilde" => Symbol { unicode: 0x330, atom_type: AtomType::BotAccentWide }, // Unicode: 0x330, under tilde accent (multiple characters and non-spacing)
    "mathunderbar" => Symbol { unicode: 0x332, atom_type: AtomType::BotAccentWide }, // Unicode: 0x332, combining low line
    "not" => Symbol { unicode: 0x338, atom_type: AtomType::Accent }, // Unicode: 0x338, combining long solidus overlay
    "underleftrightarrow" => Symbol { unicode: 0x34D, atom_type: AtomType::BotAccentWide }, // Unicode: 0x34D, underleftrightarrow accent
    // "mupAlpha" => Symbol { unicode: 0x391, atom_type: AtomType::Alpha }, // Unicode: 0x391, capital alpha, greek
    // "mupBeta" => Symbol { unicode: 0x392, atom_type: AtomType::Alpha }, // Unicode: 0x392, capital beta, greek
    // "mupGamma" => Symbol { unicode: 0x393, atom_type: AtomType::Alpha }, // Unicode: 0x393, capital gamma, greek
    // "mupDelta" => Symbol { unicode: 0x394, atom_type: AtomType::Alpha }, // Unicode: 0x394, capital delta, greek
    // "mupEpsilon" => Symbol { unicode: 0x395, atom_type: AtomType::Alpha }, // Unicode: 0x395, capital epsilon, greek
    // "mupZeta" => Symbol { unicode: 0x396, atom_type: AtomType::Alpha }, // Unicode: 0x396, capital zeta, greek
    // "mupEta" => Symbol { unicode: 0x397, atom_type: AtomType::Alpha }, // Unicode: 0x397, capital eta, greek
    // "mupTheta" => Symbol { unicode: 0x398, atom_type: AtomType::Alpha }, // Unicode: 0x398, capital theta, greek
    // "mupIota" => Symbol { unicode: 0x399, atom_type: AtomType::Alpha }, // Unicode: 0x399, capital iota, greek
    // "mupKappa" => Symbol { unicode: 0x39A, atom_type: AtomType::Alpha }, // Unicode: 0x39A, capital kappa, greek
    // "mupLambda" => Symbol { unicode: 0x39B, atom_type: AtomType::Alpha }, // Unicode: 0x39B, capital lambda, greek
    // "mupMu" => Symbol { unicode: 0x39C, atom_type: AtomType::Alpha }, // Unicode: 0x39C, capital mu, greek
    // "mupNu" => Symbol { unicode: 0x39D, atom_type: AtomType::Alpha }, // Unicode: 0x39D, capital nu, greek
    // "mupXi" => Symbol { unicode: 0x39E, atom_type: AtomType::Alpha }, // Unicode: 0x39E, capital xi, greek
    // "mupOmicron" => Symbol { unicode: 0x39F, atom_type: AtomType::Alpha }, // Unicode: 0x39F, capital omicron, greek
    // "mupPi" => Symbol { unicode: 0x3A0, atom_type: AtomType::Alpha }, // Unicode: 0x3A0, capital pi, greek
    // "mupRho" => Symbol { unicode: 0x3A1, atom_type: AtomType::Alpha }, // Unicode: 0x3A1, capital rho, greek
    // "mupSigma" => Symbol { unicode: 0x3A3, atom_type: AtomType::Alpha }, // Unicode: 0x3A3, capital sigma, greek
    // "mupTau" => Symbol { unicode: 0x3A4, atom_type: AtomType::Alpha }, // Unicode: 0x3A4, capital tau, greek
    // "mupUpsilon" => Symbol { unicode: 0x3A5, atom_type: AtomType::Alpha }, // Unicode: 0x3A5, capital upsilon, greek
    // "mupPhi" => Symbol { unicode: 0x3A6, atom_type: AtomType::Alpha }, // Unicode: 0x3A6, capital phi, greek
    // "mupChi" => Symbol { unicode: 0x3A7, atom_type: AtomType::Alpha }, // Unicode: 0x3A7, capital chi, greek
    // "mupPsi" => Symbol { unicode: 0x3A8, atom_type: AtomType::Alpha }, // Unicode: 0x3A8, capital psi, greek
    // "mupOmega" => Symbol { unicode: 0x3A9, atom_type: AtomType::Alpha }, // Unicode: 0x3A9, capital omega, greek
    // "mupalpha" => Symbol { unicode: 0x3B1, atom_type: AtomType::Alpha }, // Unicode: 0x3B1, small alpha, greek
    // "mupbeta" => Symbol { unicode: 0x3B2, atom_type: AtomType::Alpha }, // Unicode: 0x3B2, small beta, greek
    // "mupgamma" => Symbol { unicode: 0x3B3, atom_type: AtomType::Alpha }, // Unicode: 0x3B3, small gamma, greek
    // "mupdelta" => Symbol { unicode: 0x3B4, atom_type: AtomType::Alpha }, // Unicode: 0x3B4, small delta, greek
    // "mupepsilon" => Symbol { unicode: 0x3B5, atom_type: AtomType::Alpha }, // Unicode: 0x3B5, rounded small epsilon, greek
    // "mupzeta" => Symbol { unicode: 0x3B6, atom_type: AtomType::Alpha }, // Unicode: 0x3B6, small zeta, greek
    // "mupeta" => Symbol { unicode: 0x3B7, atom_type: AtomType::Alpha }, // Unicode: 0x3B7, small eta, greek
    // "muptheta" => Symbol { unicode: 0x3B8, atom_type: AtomType::Alpha }, // Unicode: 0x3B8, straight theta, small theta, greek
    // "mupiota" => Symbol { unicode: 0x3B9, atom_type: AtomType::Alpha }, // Unicode: 0x3B9, small iota, greek
    // "mupkappa" => Symbol { unicode: 0x3BA, atom_type: AtomType::Alpha }, // Unicode: 0x3BA, small kappa, greek
    // "muplambda" => Symbol { unicode: 0x3BB, atom_type: AtomType::Alpha }, // Unicode: 0x3BB, small lambda, greek
    // "mupmu" => Symbol { unicode: 0x3BC, atom_type: AtomType::Alpha }, // Unicode: 0x3BC, small mu, greek
    // "mupnu" => Symbol { unicode: 0x3BD, atom_type: AtomType::Alpha }, // Unicode: 0x3BD, small nu, greek
    // "mupxi" => Symbol { unicode: 0x3BE, atom_type: AtomType::Alpha }, // Unicode: 0x3BE, small xi, greek
    // "mupomicron" => Symbol { unicode: 0x3BF, atom_type: AtomType::Alpha }, // Unicode: 0x3BF, small omicron, greek
    // "muppi" => Symbol { unicode: 0x3C0, atom_type: AtomType::Alpha }, // Unicode: 0x3C0, small pi, greek
    // "muprho" => Symbol { unicode: 0x3C1, atom_type: AtomType::Alpha }, // Unicode: 0x3C1, small rho, greek
    // "mupvarsigma" => Symbol { unicode: 0x3C2, atom_type: AtomType::Alpha }, // Unicode: 0x3C2, terminal sigma, greek
    // "mupsigma" => Symbol { unicode: 0x3C3, atom_type: AtomType::Alpha }, // Unicode: 0x3C3, small sigma, greek
    // "muptau" => Symbol { unicode: 0x3C4, atom_type: AtomType::Alpha }, // Unicode: 0x3C4, small tau, greek
    // "mupupsilon" => Symbol { unicode: 0x3C5, atom_type: AtomType::Alpha }, // Unicode: 0x3C5, small upsilon, greek
    // "mupvarphi" => Symbol { unicode: 0x3C6, atom_type: AtomType::Alpha }, // Unicode: 0x3C6, curly or open small phi, greek
    // "mupchi" => Symbol { unicode: 0x3C7, atom_type: AtomType::Alpha }, // Unicode: 0x3C7, small chi, greek
    // "muppsi" => Symbol { unicode: 0x3C8, atom_type: AtomType::Alpha }, // Unicode: 0x3C8, small psi, greek
    // "mupomega" => Symbol { unicode: 0x3C9, atom_type: AtomType::Alpha }, // Unicode: 0x3C9, small omega, greek
    // "mupvarbeta" => Symbol { unicode: 0x3D0, atom_type: AtomType::Alpha }, // Unicode: 0x3D0, rounded small beta, greek
    // "mupvartheta" => Symbol { unicode: 0x3D1, atom_type: AtomType::Alpha }, // Unicode: 0x3D1, /vartheta - curly or open theta
    // "mupphi" => Symbol { unicode: 0x3D5, atom_type: AtomType::Alpha }, // Unicode: 0x3D5, /straightphi - small phi, greek
    // "mupvarpi" => Symbol { unicode: 0x3D6, atom_type: AtomType::Alpha }, // Unicode: 0x3D6, rounded small pi (pomega), greek
    // "upoldKoppa" => Symbol { unicode: 0x3D8, atom_type: AtomType::Alpha }, // Unicode: 0x3D8, greek letter archaic koppa
    // "upoldkoppa" => Symbol { unicode: 0x3D9, atom_type: AtomType::Alpha }, // Unicode: 0x3D9, greek small letter archaic koppa
    // "upStigma" => Symbol { unicode: 0x3DA, atom_type: AtomType::Alpha }, // Unicode: 0x3DA, capital stigma
    // "upstigma" => Symbol { unicode: 0x3DB, atom_type: AtomType::Alpha }, // Unicode: 0x3DB, greek small letter stigma
    // "upDigamma" => Symbol { unicode: 0x3DC, atom_type: AtomType::Alpha }, // Unicode: 0x3DC, capital digamma
    // "updigamma" => Symbol { unicode: 0x3DD, atom_type: AtomType::Alpha }, // Unicode: 0x3DD, old greek small letter digamma
    // "upKoppa" => Symbol { unicode: 0x3DE, atom_type: AtomType::Alpha }, // Unicode: 0x3DE, capital koppa
    // "upkoppa" => Symbol { unicode: 0x3DF, atom_type: AtomType::Alpha }, // Unicode: 0x3DF, greek small letter koppa
    // "upSampi" => Symbol { unicode: 0x3E0, atom_type: AtomType::Alpha }, // Unicode: 0x3E0, capital sampi
    // "upsampi" => Symbol { unicode: 0x3E1, atom_type: AtomType::Alpha }, // Unicode: 0x3E1, greek small letter sampi
    // "mupvarkappa" => Symbol { unicode: 0x3F0, atom_type: AtomType::Alpha }, // Unicode: 0x3F0, rounded small kappa, greek
    // "mupvarrho" => Symbol { unicode: 0x3F1, atom_type: AtomType::Alpha }, // Unicode: 0x3F1, rounded small rho, greek
    // "mupvarTheta" => Symbol { unicode: 0x3F4, atom_type: AtomType::Alpha }, // Unicode: 0x3F4, greek capital theta symbol
    // "mupvarepsilon" => Symbol { unicode: 0x3F5, atom_type: AtomType::Alpha }, // Unicode: 0x3F5, greek lunate epsilon symbol
    // "upbackepsilon" => Symbol { unicode: 0x3F6, atom_type: AtomType::Alpha }, // Unicode: 0x3F6, greek reversed lunate epsilon symbol
    "horizbar" => Symbol { unicode: 0x2015, atom_type: AtomType::Alpha }, // Unicode: 0x2015, horizontal bar
    "Vert" => Symbol { unicode: 0x2016, atom_type: AtomType::Fence }, // Unicode: 0x2016, double vertical bar
    "twolowline" => Symbol { unicode: 0x2017, atom_type: AtomType::Alpha }, // Unicode: 0x2017, double low line (spacing)
    "dagger" => Symbol { unicode: 0x2020, atom_type: AtomType::Binary }, // Unicode: 0x2020, dagger relation
    "ddagger" => Symbol { unicode: 0x2021, atom_type: AtomType::Binary }, // Unicode: 0x2021, double dagger relation
    "smblkcircle" => Symbol { unicode: 0x2022, atom_type: AtomType::Binary }, // Unicode: 0x2022, /bullet b: round bullet, filled
    "enleadertwodots" => Symbol { unicode: 0x2025, atom_type: AtomType::Alpha }, // Unicode: 0x2025, double baseline dot (en leader)
    "unicodeellipsis" => Symbol { unicode: 0x2026, atom_type: AtomType::Alpha }, // Unicode: 0x2026, ellipsis (horizontal)
    "prime" => Symbol { unicode: 0x2032, atom_type: AtomType::Alpha }, // Unicode: 0x2032, prime or minute, not superscripted
    "dprime" => Symbol { unicode: 0x2033, atom_type: AtomType::Alpha }, // Unicode: 0x2033, double prime or second, not superscripted
    "trprime" => Symbol { unicode: 0x2034, atom_type: AtomType::Alpha }, // Unicode: 0x2034, triple prime (not superscripted)
    "backprime" => Symbol { unicode: 0x2035, atom_type: AtomType::Alpha }, // Unicode: 0x2035, reverse prime, not superscripted
    "backdprime" => Symbol { unicode: 0x2036, atom_type: AtomType::Alpha }, // Unicode: 0x2036, double reverse prime, not superscripted
    "backtrprime" => Symbol { unicode: 0x2037, atom_type: AtomType::Alpha }, // Unicode: 0x2037, triple reverse prime, not superscripted
    "caretinsert" => Symbol { unicode: 0x2038, atom_type: AtomType::Alpha }, // Unicode: 0x2038, caret (insertion mark)
    // "Exclam" => Symbol { unicode: 0x203C, atom_type: AtomType::Alpha }, // Unicode: 0x203C, double exclamation mark
    // "tieconcat" => Symbol { unicode: 0x2040, atom_type: AtomType::Binary }, // Unicode: 0x2040, character tie, z notation sequence concatenation
    // "hyphenbullet" => Symbol { unicode: 0x2043, atom_type: AtomType::Alpha }, // Unicode: 0x2043, rectangle, filled (hyphen bullet)
    // "fracslash" => Symbol { unicode: 0x2044, atom_type: AtomType::Binary }, // Unicode: 0x2044, fraction slash
    // "Question" => Symbol { unicode: 0x2047, atom_type: AtomType::Alpha }, // Unicode: 0x2047, double question mark
    // "closure" => Symbol { unicode: 0x2050, atom_type: AtomType::Relation }, // Unicode: 0x2050, close up
    // "qprime" => Symbol { unicode: 0x2057, atom_type: AtomType::Alpha }, // Unicode: 0x2057, quadruple prime, not superscripted
    // "euro" => Symbol { unicode: 0x20AC, atom_type: AtomType::Alpha }, // Unicode: 0x20AC, euro sign
    "leftharpoon" => Symbol { unicode: 0x20D0, atom_type: AtomType::Accent }, // Unicode: 0x20D0, combining left harpoon above
    "overleftharpoon" => Symbol { unicode: 0x20D0, atom_type: AtomType::AccentWide }, // Unicode: 0x20D0, combining left harpoon above
    "rightharpoon" => Symbol { unicode: 0x20D1, atom_type: AtomType::Accent }, // Unicode: 0x20D1, combining right harpoon above
    "overrightharpoon" => Symbol { unicode: 0x20D1, atom_type: AtomType::AccentWide }, // Unicode: 0x20D1, combining right harpoon above
    "vertoverlay" => Symbol { unicode: 0x20D2, atom_type: AtomType::Accent }, // Unicode: 0x20D2, combining long vertical line overlay
    "overleftarrow" => Symbol { unicode: 0x20D6, atom_type: AtomType::AccentWide }, // Unicode: 0x20D6, combining left arrow above
    "overrightarrow" => Symbol { unicode: 0x20D7, atom_type: AtomType::Accent }, // Unicode: 0x20D7, combining left arrow above
    "vec" => Symbol { unicode: 0x20D7, atom_type: AtomType::Accent }, // Unicode: 0x20D7, combining right arrow above
    "dddot" => Symbol { unicode: 0x20DB, atom_type: AtomType::Accent }, // Unicode: 0x20DB, combining three dots above
    "ddddot" => Symbol { unicode: 0x20DC, atom_type: AtomType::Accent }, // Unicode: 0x20DC, combining four dots above
    // "enclosecircle" => Symbol { unicode: 0x20DD, atom_type: AtomType::Alpha }, // Unicode: 0x20DD, combining enclosing circle
    // "enclosesquare" => Symbol { unicode: 0x20DE, atom_type: AtomType::Alpha }, // Unicode: 0x20DE, combining enclosing square
    // "enclosediamond" => Symbol { unicode: 0x20DF, atom_type: AtomType::Alpha }, // Unicode: 0x20DF, combining enclosing diamond
    // "overleftrightarrow" => Symbol { unicode: 0x20E1, atom_type: AtomType::AccentWide }, // Unicode: 0x20E1, combining left right arrow above
    // "enclosetriangle" => Symbol { unicode: 0x20E4, atom_type: AtomType::Alpha }, // Unicode: 0x20E4, combining enclosing upward pointing triangle
    // "annuity" => Symbol { unicode: 0x20E7, atom_type: AtomType::Accent }, // Unicode: 0x20E7, combining annuity symbol
    // "threeunderdot" => Symbol { unicode: 0x20E8, atom_type: AtomType::BotAccent }, // Unicode: 0x20E8, combining triple underdot
    "widebridgeabove" => Symbol { unicode: 0x20E9, atom_type: AtomType::Accent }, // Unicode: 0x20E9, combining wide bridge above
    "underrightharpoondown" => Symbol { unicode: 0x20EC, atom_type: AtomType::BotAccentWide }, // Unicode: 0x20EC, combining rightwards harpoon with barb downwards
    "underleftharpoondown" => Symbol { unicode: 0x20ED, atom_type: AtomType::BotAccentWide }, // Unicode: 0x20ED, combining leftwards harpoon with barb downwards
    "underleftarrow" => Symbol { unicode: 0x20EE, atom_type: AtomType::BotAccentWide }, // Unicode: 0x20EE, combining left arrow below
    "underrightarrow" => Symbol { unicode: 0x20EF, atom_type: AtomType::BotAccentWide }, // Unicode: 0x20EF, combining right arrow below
    "asteraccent" => Symbol { unicode: 0x20F0, atom_type: AtomType::Accent }, // Unicode: 0x20F0, combining asterisk above
    "BbbC" => Symbol { unicode: 0x2102, atom_type: AtomType::Alpha }, // Unicode: 0x2102, /bbb c, open face c
    "Eulerconst" => Symbol { unicode: 0x2107, atom_type: AtomType::Alpha }, // Unicode: 0x2107, euler constant
    "mscrg" => Symbol { unicode: 0x210A, atom_type: AtomType::Alpha }, // Unicode: 0x210A, /scr g, script letter g
    "mscrH" => Symbol { unicode: 0x210B, atom_type: AtomType::Alpha }, // Unicode: 0x210B, hamiltonian (script capital h)
    "mfrakH" => Symbol { unicode: 0x210C, atom_type: AtomType::Alpha }, // Unicode: 0x210C, /frak h, upper case h
    "BbbH" => Symbol { unicode: 0x210D, atom_type: AtomType::Alpha }, // Unicode: 0x210D, /bbb h, open face h
    "Planckconst" => Symbol { unicode: 0x210E, atom_type: AtomType::Alpha }, // Unicode: 0x210E, planck constant
    "hslash" => Symbol { unicode: 0x210F, atom_type: AtomType::Alpha }, // Unicode: 0x210F, /hslash - variant planck's over 2pi
    "mscrI" => Symbol { unicode: 0x2110, atom_type: AtomType::Alpha }, // Unicode: 0x2110, /scr i, script letter i
    "Im" => Symbol { unicode: 0x2111, atom_type: AtomType::Alpha }, // Unicode: 0x2111, imaginary part
    "mscrL" => Symbol { unicode: 0x2112, atom_type: AtomType::Alpha }, // Unicode: 0x2112, lagrangian (script capital l)
    "ell" => Symbol { unicode: 0x2113, atom_type: AtomType::Alpha }, // Unicode: 0x2113, cursive small l
    "BbbN" => Symbol { unicode: 0x2115, atom_type: AtomType::Alpha }, // Unicode: 0x2115, /bbb n, open face n
    "wp" => Symbol { unicode: 0x2118, atom_type: AtomType::Alpha }, // Unicode: 0x2118, weierstrass p
    "BbbP" => Symbol { unicode: 0x2119, atom_type: AtomType::Alpha }, // Unicode: 0x2119, /bbb p, open face p
    "BbbQ" => Symbol { unicode: 0x211A, atom_type: AtomType::Alpha }, // Unicode: 0x211A, /bbb q, open face q
    "mscrR" => Symbol { unicode: 0x211B, atom_type: AtomType::Alpha }, // Unicode: 0x211B, /scr r, script letter r
    "Re" => Symbol { unicode: 0x211C, atom_type: AtomType::Alpha }, // Unicode: 0x211C, real part
    "BbbR" => Symbol { unicode: 0x211D, atom_type: AtomType::Alpha }, // Unicode: 0x211D, /bbb r, open face r
    "BbbZ" => Symbol { unicode: 0x2124, atom_type: AtomType::Alpha }, // Unicode: 0x2124, /bbb z, open face z
    "mho" => Symbol { unicode: 0x2127, atom_type: AtomType::Alpha }, // Unicode: 0x2127, conductance
    "mfrakZ" => Symbol { unicode: 0x2128, atom_type: AtomType::Alpha }, // Unicode: 0x2128, /frak z, upper case z
    "turnediota" => Symbol { unicode: 0x2129, atom_type: AtomType::Alpha }, // Unicode: 0x2129, turned iota
    "Angstrom" => Symbol { unicode: 0x212B, atom_type: AtomType::Alpha }, // Unicode: 0x212B, angstrom capital a, ring
    "mscrB" => Symbol { unicode: 0x212C, atom_type: AtomType::Alpha }, // Unicode: 0x212C, bernoulli function (script capital b)
    "mfrakC" => Symbol { unicode: 0x212D, atom_type: AtomType::Alpha }, // Unicode: 0x212D, black-letter capital c
    "mscre" => Symbol { unicode: 0x212F, atom_type: AtomType::Alpha }, // Unicode: 0x212F, /scr e, script letter e
    "mscrE" => Symbol { unicode: 0x2130, atom_type: AtomType::Alpha }, // Unicode: 0x2130, /scr e, script letter e
    "mscrF" => Symbol { unicode: 0x2131, atom_type: AtomType::Alpha }, // Unicode: 0x2131, /scr f, script letter f
    "Finv" => Symbol { unicode: 0x2132, atom_type: AtomType::Alpha }, // Unicode: 0x2132, turned capital f
    "mscrM" => Symbol { unicode: 0x2133, atom_type: AtomType::Alpha }, // Unicode: 0x2133, physics m-matrix (script capital m)
    "mscro" => Symbol { unicode: 0x2134, atom_type: AtomType::Alpha }, // Unicode: 0x2134, order of (script small o)
    "aleph" => Symbol { unicode: 0x2135, atom_type: AtomType::Alpha }, // Unicode: 0x2135, aleph, hebrew
    "beth" => Symbol { unicode: 0x2136, atom_type: AtomType::Alpha }, // Unicode: 0x2136, beth, hebrew
    "gimel" => Symbol { unicode: 0x2137, atom_type: AtomType::Alpha }, // Unicode: 0x2137, gimel, hebrew
    "daleth" => Symbol { unicode: 0x2138, atom_type: AtomType::Alpha }, // Unicode: 0x2138, daleth, hebrew
    "Bbbpi" => Symbol { unicode: 0x213C, atom_type: AtomType::Alpha }, // Unicode: 0x213C, double-struck small pi
    "Bbbgamma" => Symbol { unicode: 0x213D, atom_type: AtomType::Alpha }, // Unicode: 0x213D, double-struck small gamma
    "BbbGamma" => Symbol { unicode: 0x213E, atom_type: AtomType::Alpha }, // Unicode: 0x213E, double-struck capital gamma
    "BbbPi" => Symbol { unicode: 0x213F, atom_type: AtomType::Alpha }, // Unicode: 0x213F, double-struck capital pi
    "Bbbsum" => Symbol { unicode: 0x2140, atom_type: AtomType::Operator(false) }, // Unicode: 0x2140, double-struck n-ary summation
    "Game" => Symbol { unicode: 0x2141, atom_type: AtomType::Alpha }, // Unicode: 0x2141, turned sans-serif capital g
    "sansLturned" => Symbol { unicode: 0x2142, atom_type: AtomType::Alpha }, // Unicode: 0x2142, turned sans-serif capital l
    "sansLmirrored" => Symbol { unicode: 0x2143, atom_type: AtomType::Alpha }, // Unicode: 0x2143, reversed sans-serif capital l
    "Yup" => Symbol { unicode: 0x2144, atom_type: AtomType::Alpha }, // Unicode: 0x2144, turned sans-serif capital y
    "mitBbbD" => Symbol { unicode: 0x2145, atom_type: AtomType::Alpha }, // Unicode: 0x2145, double-struck italic capital d
    "mitBbbd" => Symbol { unicode: 0x2146, atom_type: AtomType::Alpha }, // Unicode: 0x2146, double-struck italic small d
    "mitBbbe" => Symbol { unicode: 0x2147, atom_type: AtomType::Alpha }, // Unicode: 0x2147, double-struck italic small e
    "mitBbbi" => Symbol { unicode: 0x2148, atom_type: AtomType::Alpha }, // Unicode: 0x2148, double-struck italic small i
    "mitBbbj" => Symbol { unicode: 0x2149, atom_type: AtomType::Alpha }, // Unicode: 0x2149, double-struck italic small j
    "PropertyLine" => Symbol { unicode: 0x214A, atom_type: AtomType::Alpha }, // Unicode: 0x214A, property line
    "upand" => Symbol { unicode: 0x214B, atom_type: AtomType::Binary }, // Unicode: 0x214B, turned ampersand
    "leftarrow" => Symbol { unicode: 0x2190, atom_type: AtomType::Relation }, // Unicode: 0x2190, /leftarrow /gets a: leftward arrow
    "uparrow" => Symbol { unicode: 0x2191, atom_type: AtomType::Relation }, // Unicode: 0x2191, upward arrow
    "rightarrow" => Symbol { unicode: 0x2192, atom_type: AtomType::Relation }, // Unicode: 0x2192, /rightarrow /to a: rightward arrow
    "to" => Symbol { unicode: 0x2192, atom_type: AtomType::Relation }, // Unicode: 0x2192, /rightarrow /to a: rightward arrow
    "downarrow" => Symbol { unicode: 0x2193, atom_type: AtomType::Relation }, // Unicode: 0x2193, downward arrow
    "leftrightarrow" => Symbol { unicode: 0x2194, atom_type: AtomType::Relation }, // Unicode: 0x2194, left and right arrow
    "updownarrow" => Symbol { unicode: 0x2195, atom_type: AtomType::Relation }, // Unicode: 0x2195, up and down arrow
    "nwarrow" => Symbol { unicode: 0x2196, atom_type: AtomType::Relation }, // Unicode: 0x2196, nw pointing arrow
    "nearrow" => Symbol { unicode: 0x2197, atom_type: AtomType::Relation }, // Unicode: 0x2197, ne pointing arrow
    "searrow" => Symbol { unicode: 0x2198, atom_type: AtomType::Relation }, // Unicode: 0x2198, se pointing arrow
    "swarrow" => Symbol { unicode: 0x2199, atom_type: AtomType::Relation }, // Unicode: 0x2199, sw pointing arrow
    "nleftarrow" => Symbol { unicode: 0x219A, atom_type: AtomType::Relation }, // Unicode: 0x219A, not left arrow
    "nrightarrow" => Symbol { unicode: 0x219B, atom_type: AtomType::Relation }, // Unicode: 0x219B, not right arrow
    "leftwavearrow" => Symbol { unicode: 0x219C, atom_type: AtomType::Relation }, // Unicode: 0x219C, left arrow-wavy
    "rightwavearrow" => Symbol { unicode: 0x219D, atom_type: AtomType::Relation }, // Unicode: 0x219D, right arrow-wavy
    "twoheadleftarrow" => Symbol { unicode: 0x219E, atom_type: AtomType::Relation }, // Unicode: 0x219E, left two-headed arrow
    "twoheaduparrow" => Symbol { unicode: 0x219F, atom_type: AtomType::Relation }, // Unicode: 0x219F, up two-headed arrow
    "twoheadrightarrow" => Symbol { unicode: 0x21A0, atom_type: AtomType::Relation }, // Unicode: 0x21A0, right two-headed arrow
    "twoheaddownarrow" => Symbol { unicode: 0x21A1, atom_type: AtomType::Relation }, // Unicode: 0x21A1, down two-headed arrow
    "leftarrowtail" => Symbol { unicode: 0x21A2, atom_type: AtomType::Relation }, // Unicode: 0x21A2, left arrow-tailed
    "rightarrowtail" => Symbol { unicode: 0x21A3, atom_type: AtomType::Relation }, // Unicode: 0x21A3, right arrow-tailed
    "mapsfrom" => Symbol { unicode: 0x21A4, atom_type: AtomType::Relation }, // Unicode: 0x21A4, maps to, leftward
    "mapsup" => Symbol { unicode: 0x21A5, atom_type: AtomType::Relation }, // Unicode: 0x21A5, maps to, upward
    "mapsto" => Symbol { unicode: 0x21A6, atom_type: AtomType::Relation }, // Unicode: 0x21A6, maps to, rightward
    "mapsdown" => Symbol { unicode: 0x21A7, atom_type: AtomType::Relation }, // Unicode: 0x21A7, maps to, downward
    "updownarrowbar" => Symbol { unicode: 0x21A8, atom_type: AtomType::Alpha }, // Unicode: 0x21A8, up down arrow with base (perpendicular)
    "hookleftarrow" => Symbol { unicode: 0x21A9, atom_type: AtomType::Relation }, // Unicode: 0x21A9, left arrow-hooked
    "hookrightarrow" => Symbol { unicode: 0x21AA, atom_type: AtomType::Relation }, // Unicode: 0x21AA, right arrow-hooked
    "looparrowleft" => Symbol { unicode: 0x21AB, atom_type: AtomType::Relation }, // Unicode: 0x21AB, left arrow-looped
    "looparrowright" => Symbol { unicode: 0x21AC, atom_type: AtomType::Relation }, // Unicode: 0x21AC, right arrow-looped
    "leftrightsquigarrow" => Symbol { unicode: 0x21AD, atom_type: AtomType::Relation }, // Unicode: 0x21AD, left and right arr-wavy
    "nleftrightarrow" => Symbol { unicode: 0x21AE, atom_type: AtomType::Relation }, // Unicode: 0x21AE, not left and right arrow
    "downzigzagarrow" => Symbol { unicode: 0x21AF, atom_type: AtomType::Relation }, // Unicode: 0x21AF, downwards zigzag arrow
    "Lsh" => Symbol { unicode: 0x21B0, atom_type: AtomType::Relation }, // Unicode: 0x21B0, /lsh a:
    "Rsh" => Symbol { unicode: 0x21B1, atom_type: AtomType::Relation }, // Unicode: 0x21B1, /rsh a:
    "Ldsh" => Symbol { unicode: 0x21B2, atom_type: AtomType::Relation }, // Unicode: 0x21B2, left down angled arrow
    "Rdsh" => Symbol { unicode: 0x21B3, atom_type: AtomType::Relation }, // Unicode: 0x21B3, right down angled arrow
    "linefeed" => Symbol { unicode: 0x21B4, atom_type: AtomType::Alpha }, // Unicode: 0x21B4, rightwards arrow with corner downwards
    "carriagereturn" => Symbol { unicode: 0x21B5, atom_type: AtomType::Alpha }, // Unicode: 0x21B5, downwards arrow with corner leftward = carriage return
    "curvearrowleft" => Symbol { unicode: 0x21B6, atom_type: AtomType::Relation }, // Unicode: 0x21B6, left curved arrow
    "curvearrowright" => Symbol { unicode: 0x21B7, atom_type: AtomType::Relation }, // Unicode: 0x21B7, right curved arrow
    "barovernorthwestarrow" => Symbol { unicode: 0x21B8, atom_type: AtomType::Alpha }, // Unicode: 0x21B8, north west arrow to long bar
    "barleftarrowrightarrowbar" => Symbol { unicode: 0x21B9, atom_type: AtomType::Alpha }, // Unicode: 0x21B9, leftwards arrow to bar over rightwards arrow to bar
    "acwopencirclearrow" => Symbol { unicode: 0x21BA, atom_type: AtomType::Alpha }, // Unicode: 0x21BA, anticlockwise open circle arrow
    "cwopencirclearrow" => Symbol { unicode: 0x21BB, atom_type: AtomType::Alpha }, // Unicode: 0x21BB, clockwise open circle arrow
    "leftharpoonup" => Symbol { unicode: 0x21BC, atom_type: AtomType::Relation }, // Unicode: 0x21BC, left harpoon-up
    "leftharpoondown" => Symbol { unicode: 0x21BD, atom_type: AtomType::Relation }, // Unicode: 0x21BD, left harpoon-down
    "upharpoonright" => Symbol { unicode: 0x21BE, atom_type: AtomType::Relation }, // Unicode: 0x21BE, /upharpoonright /restriction a: up harpoon-right
    "upharpoonleft" => Symbol { unicode: 0x21BF, atom_type: AtomType::Relation }, // Unicode: 0x21BF, up harpoon-left
    "rightharpoonup" => Symbol { unicode: 0x21C0, atom_type: AtomType::Relation }, // Unicode: 0x21C0, right harpoon-up
    "rightharpoondown" => Symbol { unicode: 0x21C1, atom_type: AtomType::Relation }, // Unicode: 0x21C1, right harpoon-down
    "downharpoonright" => Symbol { unicode: 0x21C2, atom_type: AtomType::Relation }, // Unicode: 0x21C2, down harpoon-right
    "downharpoonleft" => Symbol { unicode: 0x21C3, atom_type: AtomType::Relation }, // Unicode: 0x21C3, down harpoon-left
    "rightleftarrows" => Symbol { unicode: 0x21C4, atom_type: AtomType::Relation }, // Unicode: 0x21C4, right arrow over left arrow
    "updownarrows" => Symbol { unicode: 0x21C5, atom_type: AtomType::Relation }, // Unicode: 0x21C5, up arrow, down arrow
    "leftrightarrows" => Symbol { unicode: 0x21C6, atom_type: AtomType::Relation }, // Unicode: 0x21C6, left arrow over right arrow
    "leftleftarrows" => Symbol { unicode: 0x21C7, atom_type: AtomType::Relation }, // Unicode: 0x21C7, two left arrows
    "upuparrows" => Symbol { unicode: 0x21C8, atom_type: AtomType::Relation }, // Unicode: 0x21C8, two up arrows
    "rightrightarrows" => Symbol { unicode: 0x21C9, atom_type: AtomType::Relation }, // Unicode: 0x21C9, two right arrows
    "downdownarrows" => Symbol { unicode: 0x21CA, atom_type: AtomType::Relation }, // Unicode: 0x21CA, two down arrows
    "leftrightharpoons" => Symbol { unicode: 0x21CB, atom_type: AtomType::Relation }, // Unicode: 0x21CB, left harpoon over right
    "rightleftharpoons" => Symbol { unicode: 0x21CC, atom_type: AtomType::Relation }, // Unicode: 0x21CC, right harpoon over left
    "nLeftarrow" => Symbol { unicode: 0x21CD, atom_type: AtomType::Relation }, // Unicode: 0x21CD, not implied by
    "nLeftrightarrow" => Symbol { unicode: 0x21CE, atom_type: AtomType::Relation }, // Unicode: 0x21CE, not left and right double arrows
    "nRightarrow" => Symbol { unicode: 0x21CF, atom_type: AtomType::Relation }, // Unicode: 0x21CF, not implies
    "Leftarrow" => Symbol { unicode: 0x21D0, atom_type: AtomType::Relation }, // Unicode: 0x21D0, is implied by
    "Uparrow" => Symbol { unicode: 0x21D1, atom_type: AtomType::Relation }, // Unicode: 0x21D1, up double arrow
    "Rightarrow" => Symbol { unicode: 0x21D2, atom_type: AtomType::Relation }, // Unicode: 0x21D2, implies
    "Downarrow" => Symbol { unicode: 0x21D3, atom_type: AtomType::Relation }, // Unicode: 0x21D3, down double arrow
    "Leftrightarrow" => Symbol { unicode: 0x21D4, atom_type: AtomType::Relation }, // Unicode: 0x21D4, left and right double arrow
    "Updownarrow" => Symbol { unicode: 0x21D5, atom_type: AtomType::Relation }, // Unicode: 0x21D5, up and down double arrow
    "Nwarrow" => Symbol { unicode: 0x21D6, atom_type: AtomType::Relation }, // Unicode: 0x21D6, nw pointing double arrow
    "Nearrow" => Symbol { unicode: 0x21D7, atom_type: AtomType::Relation }, // Unicode: 0x21D7, ne pointing double arrow
    "Searrow" => Symbol { unicode: 0x21D8, atom_type: AtomType::Relation }, // Unicode: 0x21D8, se pointing double arrow
    "Swarrow" => Symbol { unicode: 0x21D9, atom_type: AtomType::Relation }, // Unicode: 0x21D9, sw pointing double arrow
    "Lleftarrow" => Symbol { unicode: 0x21DA, atom_type: AtomType::Relation }, // Unicode: 0x21DA, left triple arrow
    "Rrightarrow" => Symbol { unicode: 0x21DB, atom_type: AtomType::Relation }, // Unicode: 0x21DB, right triple arrow
    "leftsquigarrow" => Symbol { unicode: 0x21DC, atom_type: AtomType::Relation }, // Unicode: 0x21DC, leftwards squiggle arrow
    "rightsquigarrow" => Symbol { unicode: 0x21DD, atom_type: AtomType::Relation }, // Unicode: 0x21DD, rightwards squiggle arrow
    "nHuparrow" => Symbol { unicode: 0x21DE, atom_type: AtomType::Alpha }, // Unicode: 0x21DE, upwards arrow with double stroke
    "nHdownarrow" => Symbol { unicode: 0x21DF, atom_type: AtomType::Alpha }, // Unicode: 0x21DF, downwards arrow with double stroke
    "leftdasharrow" => Symbol { unicode: 0x21E0, atom_type: AtomType::Alpha }, // Unicode: 0x21E0, leftwards dashed arrow
    "updasharrow" => Symbol { unicode: 0x21E1, atom_type: AtomType::Alpha }, // Unicode: 0x21E1, upwards dashed arrow
    "rightdasharrow" => Symbol { unicode: 0x21E2, atom_type: AtomType::Alpha }, // Unicode: 0x21E2, rightwards dashed arrow
    "downdasharrow" => Symbol { unicode: 0x21E3, atom_type: AtomType::Alpha }, // Unicode: 0x21E3, downwards dashed arrow
    "barleftarrow" => Symbol { unicode: 0x21E4, atom_type: AtomType::Relation }, // Unicode: 0x21E4, leftwards arrow to bar
    "rightarrowbar" => Symbol { unicode: 0x21E5, atom_type: AtomType::Relation }, // Unicode: 0x21E5, rightwards arrow to bar
    "leftwhitearrow" => Symbol { unicode: 0x21E6, atom_type: AtomType::Alpha }, // Unicode: 0x21E6, leftwards white arrow
    "upwhitearrow" => Symbol { unicode: 0x21E7, atom_type: AtomType::Alpha }, // Unicode: 0x21E7, upwards white arrow
    "rightwhitearrow" => Symbol { unicode: 0x21E8, atom_type: AtomType::Alpha }, // Unicode: 0x21E8, rightwards white arrow
    "downwhitearrow" => Symbol { unicode: 0x21E9, atom_type: AtomType::Alpha }, // Unicode: 0x21E9, downwards white arrow
    "whitearrowupfrombar" => Symbol { unicode: 0x21EA, atom_type: AtomType::Alpha }, // Unicode: 0x21EA, upwards white arrow from bar
    "circleonrightarrow" => Symbol { unicode: 0x21F4, atom_type: AtomType::Relation }, // Unicode: 0x21F4, right arrow with small circle
    "downuparrows" => Symbol { unicode: 0x21F5, atom_type: AtomType::Relation }, // Unicode: 0x21F5, downwards arrow leftwards of upwards arrow
    "rightthreearrows" => Symbol { unicode: 0x21F6, atom_type: AtomType::Relation }, // Unicode: 0x21F6, three rightwards arrows
    "nvleftarrow" => Symbol { unicode: 0x21F7, atom_type: AtomType::Relation }, // Unicode: 0x21F7, leftwards arrow with vertical stroke
    "nvrightarrow" => Symbol { unicode: 0x21F8, atom_type: AtomType::Relation }, // Unicode: 0x21F8, rightwards arrow with vertical stroke
    "nvleftrightarrow" => Symbol { unicode: 0x21F9, atom_type: AtomType::Relation }, // Unicode: 0x21F9, left right arrow with vertical stroke
    "nVleftarrow" => Symbol { unicode: 0x21FA, atom_type: AtomType::Relation }, // Unicode: 0x21FA, leftwards arrow with double vertical stroke
    "nVrightarrow" => Symbol { unicode: 0x21FB, atom_type: AtomType::Relation }, // Unicode: 0x21FB, rightwards arrow with double vertical stroke
    "nVleftrightarrow" => Symbol { unicode: 0x21FC, atom_type: AtomType::Relation }, // Unicode: 0x21FC, left right arrow with double vertical stroke
    "leftarrowtriangle" => Symbol { unicode: 0x21FD, atom_type: AtomType::Relation }, // Unicode: 0x21FD, leftwards open-headed arrow
    "rightarrowtriangle" => Symbol { unicode: 0x21FE, atom_type: AtomType::Relation }, // Unicode: 0x21FE, rightwards open-headed arrow
    "leftrightarrowtriangle" => Symbol { unicode: 0x21FF, atom_type: AtomType::Relation }, // Unicode: 0x21FF, left right open-headed arrow
    "forall" => Symbol { unicode: 0x2200, atom_type: AtomType::Alpha }, // Unicode: 0x2200, for all
    "complement" => Symbol { unicode: 0x2201, atom_type: AtomType::Alpha }, // Unicode: 0x2201, complement sign
    "partial" => Symbol { unicode: 0x2202, atom_type: AtomType::Alpha }, // Unicode: 0x2202, partial differential
    "exists" => Symbol { unicode: 0x2203, atom_type: AtomType::Alpha }, // Unicode: 0x2203, at least one exists
    "nexists" => Symbol { unicode: 0x2204, atom_type: AtomType::Alpha }, // Unicode: 0x2204, negated exists
    "varnothing" => Symbol { unicode: 0x2205, atom_type: AtomType::Alpha }, // Unicode: 0x2205, circle, slash
    "increment" => Symbol { unicode: 0x2206, atom_type: AtomType::Alpha }, // Unicode: 0x2206, laplacian (delta; nabla\string^2)
    "nabla" => Symbol { unicode: 0x2207, atom_type: AtomType::Alpha }, // Unicode: 0x2207, nabla, del, hamilton operator
    "in" => Symbol { unicode: 0x2208, atom_type: AtomType::Relation }, // Unicode: 0x2208, set membership, variant
    "notin" => Symbol { unicode: 0x2209, atom_type: AtomType::Relation }, // Unicode: 0x2209, negated set membership
    "smallin" => Symbol { unicode: 0x220A, atom_type: AtomType::Relation }, // Unicode: 0x220A, set membership (small set membership)
    "ni" => Symbol { unicode: 0x220B, atom_type: AtomType::Relation }, // Unicode: 0x220B, contains, variant
    "nni" => Symbol { unicode: 0x220C, atom_type: AtomType::Relation }, // Unicode: 0x220C, negated contains, variant
    "smallni" => Symbol { unicode: 0x220D, atom_type: AtomType::Relation }, // Unicode: 0x220D, /ni /owns r: contains (small contains as member)
    "QED" => Symbol { unicode: 0x220E, atom_type: AtomType::Alpha }, // Unicode: 0x220E, end of proof
    "prod" => Symbol { unicode: 0x220F, atom_type: AtomType::Operator(true) }, // Unicode: 0x220F, product operator
    "coprod" => Symbol { unicode: 0x2210, atom_type: AtomType::Operator(true) }, // Unicode: 0x2210, coproduct operator
    "sum" => Symbol { unicode: 0x2211, atom_type: AtomType::Operator(true) }, // Unicode: 0x2211, summation operator
    "minus" => Symbol { unicode: 0x2212, atom_type: AtomType::Binary }, // Unicode: 0x2212, minus sign
    "mp" => Symbol { unicode: 0x2213, atom_type: AtomType::Binary }, // Unicode: 0x2213, minus-or-plus sign
    "dotplus" => Symbol { unicode: 0x2214, atom_type: AtomType::Binary }, // Unicode: 0x2214, plus sign, dot above
    "divslash" => Symbol { unicode: 0x2215, atom_type: AtomType::Binary }, // Unicode: 0x2215, division slash
    "smallsetminus" => Symbol { unicode: 0x2216, atom_type: AtomType::Binary }, // Unicode: 0x2216, small set minus (cf. reverse solidus)
    "ast" => Symbol { unicode: 0x2217, atom_type: AtomType::Binary }, // Unicode: 0x2217, centered asterisk
    "vysmwhtcircle" => Symbol { unicode: 0x2218, atom_type: AtomType::Binary }, // Unicode: 0x2218, composite function (small circle)
    "vysmblkcircle" => Symbol { unicode: 0x2219, atom_type: AtomType::Binary }, // Unicode: 0x2219, bullet operator
    "sqrt" => Symbol { unicode: 0x221A, atom_type: AtomType::Open }, // Unicode: 0x221A, radical
    "cuberoot" => Symbol { unicode: 0x221B, atom_type: AtomType::Open }, // Unicode: 0x221B, cube root
    "fourthroot" => Symbol { unicode: 0x221C, atom_type: AtomType::Open }, // Unicode: 0x221C, fourth root
    "propto" => Symbol { unicode: 0x221D, atom_type: AtomType::Relation }, // Unicode: 0x221D, is proportional to
    "infty" => Symbol { unicode: 0x221E, atom_type: AtomType::Alpha }, // Unicode: 0x221E, infinity
    "rightangle" => Symbol { unicode: 0x221F, atom_type: AtomType::Alpha }, // Unicode: 0x221F, right (90 degree) angle
    "angle" => Symbol { unicode: 0x2220, atom_type: AtomType::Alpha }, // Unicode: 0x2220, angle
    "measuredangle" => Symbol { unicode: 0x2221, atom_type: AtomType::Alpha }, // Unicode: 0x2221, angle-measured
    "sphericalangle" => Symbol { unicode: 0x2222, atom_type: AtomType::Alpha }, // Unicode: 0x2222, angle-spherical
    "mid" => Symbol { unicode: 0x2223, atom_type: AtomType::Relation }, // Unicode: 0x2223, /mid r:
    "nmid" => Symbol { unicode: 0x2224, atom_type: AtomType::Relation }, // Unicode: 0x2224, negated mid
    "parallel" => Symbol { unicode: 0x2225, atom_type: AtomType::Relation }, // Unicode: 0x2225, parallel
    "nparallel" => Symbol { unicode: 0x2226, atom_type: AtomType::Relation }, // Unicode: 0x2226, not parallel
    "wedge" => Symbol { unicode: 0x2227, atom_type: AtomType::Binary }, // Unicode: 0x2227, /wedge /land b: logical and
    "vee" => Symbol { unicode: 0x2228, atom_type: AtomType::Binary }, // Unicode: 0x2228, /vee /lor b: logical or
    "cap" => Symbol { unicode: 0x2229, atom_type: AtomType::Binary }, // Unicode: 0x2229, intersection
    "cup" => Symbol { unicode: 0x222A, atom_type: AtomType::Binary }, // Unicode: 0x222A, union or logical sum
    "int" => Symbol { unicode: 0x222B, atom_type: AtomType::Operator(false) }, // Unicode: 0x222B, integral operator
    "iint" => Symbol { unicode: 0x222C, atom_type: AtomType::Operator(false) }, // Unicode: 0x222C, double integral operator
    "iiint" => Symbol { unicode: 0x222D, atom_type: AtomType::Operator(false) }, // Unicode: 0x222D, triple integral operator
    "oint" => Symbol { unicode: 0x222E, atom_type: AtomType::Operator(false) }, // Unicode: 0x222E, contour integral operator
    "oiint" => Symbol { unicode: 0x222F, atom_type: AtomType::Operator(false) }, // Unicode: 0x222F, double contour integral operator
    "oiiint" => Symbol { unicode: 0x2230, atom_type: AtomType::Operator(false) }, // Unicode: 0x2230, triple contour integral operator
    "intclockwise" => Symbol { unicode: 0x2231, atom_type: AtomType::Operator(false) }, // Unicode: 0x2231, clockwise integral
    "varointclockwise" => Symbol { unicode: 0x2232, atom_type: AtomType::Operator(false) }, // Unicode: 0x2232, contour integral, clockwise
    "ointctrclockwise" => Symbol { unicode: 0x2233, atom_type: AtomType::Operator(false) }, // Unicode: 0x2233, contour integral, anticlockwise
    "therefore" => Symbol { unicode: 0x2234, atom_type: AtomType::Alpha }, // Unicode: 0x2234, therefore
    "because" => Symbol { unicode: 0x2235, atom_type: AtomType::Alpha }, // Unicode: 0x2235, because
    "mathratio" => Symbol { unicode: 0x2236, atom_type: AtomType::Relation }, // Unicode: 0x2236, ratio
    "Colon" => Symbol { unicode: 0x2237, atom_type: AtomType::Relation }, // Unicode: 0x2237, two colons
    "dotminus" => Symbol { unicode: 0x2238, atom_type: AtomType::Binary }, // Unicode: 0x2238, minus sign, dot above
    "dashcolon" => Symbol { unicode: 0x2239, atom_type: AtomType::Relation }, // Unicode: 0x2239, excess (-:)
    "dotsminusdots" => Symbol { unicode: 0x223A, atom_type: AtomType::Relation }, // Unicode: 0x223A, minus with four dots, geometric properties
    "kernelcontraction" => Symbol { unicode: 0x223B, atom_type: AtomType::Relation }, // Unicode: 0x223B, homothetic
    "sim" => Symbol { unicode: 0x223C, atom_type: AtomType::Relation }, // Unicode: 0x223C, similar
    "backsim" => Symbol { unicode: 0x223D, atom_type: AtomType::Relation }, // Unicode: 0x223D, reverse similar
    "invlazys" => Symbol { unicode: 0x223E, atom_type: AtomType::Binary }, // Unicode: 0x223E, most positive [inverted lazy s]
    "sinewave" => Symbol { unicode: 0x223F, atom_type: AtomType::Alpha }, // Unicode: 0x223F, sine wave
    "wr" => Symbol { unicode: 0x2240, atom_type: AtomType::Binary }, // Unicode: 0x2240, wreath product
    "nsim" => Symbol { unicode: 0x2241, atom_type: AtomType::Relation }, // Unicode: 0x2241, not similar
    "eqsim" => Symbol { unicode: 0x2242, atom_type: AtomType::Relation }, // Unicode: 0x2242, equals, similar
    "simeq" => Symbol { unicode: 0x2243, atom_type: AtomType::Relation }, // Unicode: 0x2243, similar, equals
    "nsime" => Symbol { unicode: 0x2244, atom_type: AtomType::Relation }, // Unicode: 0x2244, not similar, equals
    "cong" => Symbol { unicode: 0x2245, atom_type: AtomType::Relation }, // Unicode: 0x2245, congruent with
    "simneqq" => Symbol { unicode: 0x2246, atom_type: AtomType::Relation }, // Unicode: 0x2246, similar, not equals [vert only for 9573 entity]
    "ncong" => Symbol { unicode: 0x2247, atom_type: AtomType::Relation }, // Unicode: 0x2247, not congruent with
    "approx" => Symbol { unicode: 0x2248, atom_type: AtomType::Relation }, // Unicode: 0x2248, approximate
    "napprox" => Symbol { unicode: 0x2249, atom_type: AtomType::Relation }, // Unicode: 0x2249, not approximate
    "approxeq" => Symbol { unicode: 0x224A, atom_type: AtomType::Relation }, // Unicode: 0x224A, approximate, equals
    "approxident" => Symbol { unicode: 0x224B, atom_type: AtomType::Relation }, // Unicode: 0x224B, approximately identical to
    "backcong" => Symbol { unicode: 0x224C, atom_type: AtomType::Relation }, // Unicode: 0x224C, all equal to
    "asymp" => Symbol { unicode: 0x224D, atom_type: AtomType::Relation }, // Unicode: 0x224D, asymptotically equal to
    "Bumpeq" => Symbol { unicode: 0x224E, atom_type: AtomType::Relation }, // Unicode: 0x224E, bumpy equals
    "bumpeq" => Symbol { unicode: 0x224F, atom_type: AtomType::Relation }, // Unicode: 0x224F, bumpy equals, equals
    "doteq" => Symbol { unicode: 0x2250, atom_type: AtomType::Relation }, // Unicode: 0x2250, equals, single dot above
    "Doteq" => Symbol { unicode: 0x2251, atom_type: AtomType::Relation }, // Unicode: 0x2251, /doteqdot /doteq r: equals, even dots
    "fallingdotseq" => Symbol { unicode: 0x2252, atom_type: AtomType::Relation }, // Unicode: 0x2252, equals, falling dots
    "risingdotseq" => Symbol { unicode: 0x2253, atom_type: AtomType::Relation }, // Unicode: 0x2253, equals, rising dots
    "coloneq" => Symbol { unicode: 0x2254, atom_type: AtomType::Relation }, // Unicode: 0x2254, colon, equals
    "eqcolon" => Symbol { unicode: 0x2255, atom_type: AtomType::Relation }, // Unicode: 0x2255, equals, colon
    "eqcirc" => Symbol { unicode: 0x2256, atom_type: AtomType::Relation }, // Unicode: 0x2256, circle on equals sign
    "circeq" => Symbol { unicode: 0x2257, atom_type: AtomType::Relation }, // Unicode: 0x2257, circle, equals
    "arceq" => Symbol { unicode: 0x2258, atom_type: AtomType::Relation }, // Unicode: 0x2258, arc, equals; corresponds to
    "wedgeq" => Symbol { unicode: 0x2259, atom_type: AtomType::Relation }, // Unicode: 0x2259, corresponds to (wedge, equals)
    "veeeq" => Symbol { unicode: 0x225A, atom_type: AtomType::Relation }, // Unicode: 0x225A, logical or, equals
    "stareq" => Symbol { unicode: 0x225B, atom_type: AtomType::Relation }, // Unicode: 0x225B, star equals
    "triangleq" => Symbol { unicode: 0x225C, atom_type: AtomType::Relation }, // Unicode: 0x225C, triangle, equals
    "eqdef" => Symbol { unicode: 0x225D, atom_type: AtomType::Relation }, // Unicode: 0x225D, equals by definition
    "measeq" => Symbol { unicode: 0x225E, atom_type: AtomType::Relation }, // Unicode: 0x225E, measured by (m over equals)
    "questeq" => Symbol { unicode: 0x225F, atom_type: AtomType::Relation }, // Unicode: 0x225F, equal with questionmark
    "ne" => Symbol { unicode: 0x2260, atom_type: AtomType::Relation }, // Unicode: 0x2260, /ne /neq r: not equal
    "equiv" => Symbol { unicode: 0x2261, atom_type: AtomType::Relation }, // Unicode: 0x2261, identical with
    "nequiv" => Symbol { unicode: 0x2262, atom_type: AtomType::Relation }, // Unicode: 0x2262, not identical with
    "Equiv" => Symbol { unicode: 0x2263, atom_type: AtomType::Relation }, // Unicode: 0x2263, strict equivalence (4 lines)
    "leq" => Symbol { unicode: 0x2264, atom_type: AtomType::Relation }, // Unicode: 0x2264, /leq /le r: less-than-or-equal
    "geq" => Symbol { unicode: 0x2265, atom_type: AtomType::Relation }, // Unicode: 0x2265, /geq /ge r: greater-than-or-equal
    "leqq" => Symbol { unicode: 0x2266, atom_type: AtomType::Relation }, // Unicode: 0x2266, less, double equals
    "geqq" => Symbol { unicode: 0x2267, atom_type: AtomType::Relation }, // Unicode: 0x2267, greater, double equals
    "lneqq" => Symbol { unicode: 0x2268, atom_type: AtomType::Relation }, // Unicode: 0x2268, less, not double equals
    "gneqq" => Symbol { unicode: 0x2269, atom_type: AtomType::Relation }, // Unicode: 0x2269, greater, not double equals
    "ll" => Symbol { unicode: 0x226A, atom_type: AtomType::Relation }, // Unicode: 0x226A, much less than, type 2
    "gg" => Symbol { unicode: 0x226B, atom_type: AtomType::Relation }, // Unicode: 0x226B, much greater than, type 2
    "between" => Symbol { unicode: 0x226C, atom_type: AtomType::Relation }, // Unicode: 0x226C, between
    "nasymp" => Symbol { unicode: 0x226D, atom_type: AtomType::Relation }, // Unicode: 0x226D, not asymptotically equal to
    "nless" => Symbol { unicode: 0x226E, atom_type: AtomType::Relation }, // Unicode: 0x226E, not less-than
    "ngtr" => Symbol { unicode: 0x226F, atom_type: AtomType::Relation }, // Unicode: 0x226F, not greater-than
    "nleq" => Symbol { unicode: 0x2270, atom_type: AtomType::Relation }, // Unicode: 0x2270, not less-than-or-equal
    "ngeq" => Symbol { unicode: 0x2271, atom_type: AtomType::Relation }, // Unicode: 0x2271, not greater-than-or-equal
    "lesssim" => Symbol { unicode: 0x2272, atom_type: AtomType::Relation }, // Unicode: 0x2272, less, similar
    "gtrsim" => Symbol { unicode: 0x2273, atom_type: AtomType::Relation }, // Unicode: 0x2273, greater, similar
    "nlesssim" => Symbol { unicode: 0x2274, atom_type: AtomType::Relation }, // Unicode: 0x2274, not less, similar
    "ngtrsim" => Symbol { unicode: 0x2275, atom_type: AtomType::Relation }, // Unicode: 0x2275, not greater, similar
    "lessgtr" => Symbol { unicode: 0x2276, atom_type: AtomType::Relation }, // Unicode: 0x2276, less, greater
    "gtrless" => Symbol { unicode: 0x2277, atom_type: AtomType::Relation }, // Unicode: 0x2277, greater, less
    "nlessgtr" => Symbol { unicode: 0x2278, atom_type: AtomType::Relation }, // Unicode: 0x2278, not less, greater
    "ngtrless" => Symbol { unicode: 0x2279, atom_type: AtomType::Relation }, // Unicode: 0x2279, not greater, less
    "prec" => Symbol { unicode: 0x227A, atom_type: AtomType::Relation }, // Unicode: 0x227A, precedes
    "succ" => Symbol { unicode: 0x227B, atom_type: AtomType::Relation }, // Unicode: 0x227B, succeeds
    "preccurlyeq" => Symbol { unicode: 0x227C, atom_type: AtomType::Relation }, // Unicode: 0x227C, precedes, curly equals
    "succcurlyeq" => Symbol { unicode: 0x227D, atom_type: AtomType::Relation }, // Unicode: 0x227D, succeeds, curly equals
    "precsim" => Symbol { unicode: 0x227E, atom_type: AtomType::Relation }, // Unicode: 0x227E, precedes, similar
    "succsim" => Symbol { unicode: 0x227F, atom_type: AtomType::Relation }, // Unicode: 0x227F, succeeds, similar
    "nprec" => Symbol { unicode: 0x2280, atom_type: AtomType::Relation }, // Unicode: 0x2280, not precedes
    "nsucc" => Symbol { unicode: 0x2281, atom_type: AtomType::Relation }, // Unicode: 0x2281, not succeeds
    "subset" => Symbol { unicode: 0x2282, atom_type: AtomType::Relation }, // Unicode: 0x2282, subset or is implied by
    "supset" => Symbol { unicode: 0x2283, atom_type: AtomType::Relation }, // Unicode: 0x2283, superset or implies
    "nsubset" => Symbol { unicode: 0x2284, atom_type: AtomType::Relation }, // Unicode: 0x2284, not subset, variant [slash negation]
    "nsupset" => Symbol { unicode: 0x2285, atom_type: AtomType::Relation }, // Unicode: 0x2285, not superset, variant [slash negation]
    "subseteq" => Symbol { unicode: 0x2286, atom_type: AtomType::Relation }, // Unicode: 0x2286, subset, equals
    "supseteq" => Symbol { unicode: 0x2287, atom_type: AtomType::Relation }, // Unicode: 0x2287, superset, equals
    "nsubseteq" => Symbol { unicode: 0x2288, atom_type: AtomType::Relation }, // Unicode: 0x2288, not subset, equals
    "nsupseteq" => Symbol { unicode: 0x2289, atom_type: AtomType::Relation }, // Unicode: 0x2289, not superset, equals
    "subsetneq" => Symbol { unicode: 0x228A, atom_type: AtomType::Relation }, // Unicode: 0x228A, subset, not equals
    "supsetneq" => Symbol { unicode: 0x228B, atom_type: AtomType::Relation }, // Unicode: 0x228B, superset, not equals
    "cupleftarrow" => Symbol { unicode: 0x228C, atom_type: AtomType::Binary }, // Unicode: 0x228C, multiset
    "cupdot" => Symbol { unicode: 0x228D, atom_type: AtomType::Binary }, // Unicode: 0x228D, union, with dot
    "uplus" => Symbol { unicode: 0x228E, atom_type: AtomType::Binary }, // Unicode: 0x228E, plus sign in union
    "sqsubset" => Symbol { unicode: 0x228F, atom_type: AtomType::Relation }, // Unicode: 0x228F, square subset
    "sqsupset" => Symbol { unicode: 0x2290, atom_type: AtomType::Relation }, // Unicode: 0x2290, square superset
    "sqsubseteq" => Symbol { unicode: 0x2291, atom_type: AtomType::Relation }, // Unicode: 0x2291, square subset, equals
    "sqsupseteq" => Symbol { unicode: 0x2292, atom_type: AtomType::Relation }, // Unicode: 0x2292, square superset, equals
    "sqcap" => Symbol { unicode: 0x2293, atom_type: AtomType::Binary }, // Unicode: 0x2293, square intersection
    "sqcup" => Symbol { unicode: 0x2294, atom_type: AtomType::Binary }, // Unicode: 0x2294, square union
    "oplus" => Symbol { unicode: 0x2295, atom_type: AtomType::Binary }, // Unicode: 0x2295, plus sign in circle
    "ominus" => Symbol { unicode: 0x2296, atom_type: AtomType::Binary }, // Unicode: 0x2296, minus sign in circle
    "otimes" => Symbol { unicode: 0x2297, atom_type: AtomType::Binary }, // Unicode: 0x2297, multiply sign in circle
    "oslash" => Symbol { unicode: 0x2298, atom_type: AtomType::Binary }, // Unicode: 0x2298, solidus in circle
    "odot" => Symbol { unicode: 0x2299, atom_type: AtomType::Binary }, // Unicode: 0x2299, middle dot in circle
    "circledcirc" => Symbol { unicode: 0x229A, atom_type: AtomType::Binary }, // Unicode: 0x229A, small circle in circle
    "circledast" => Symbol { unicode: 0x229B, atom_type: AtomType::Binary }, // Unicode: 0x229B, asterisk in circle
    "circledequal" => Symbol { unicode: 0x229C, atom_type: AtomType::Binary }, // Unicode: 0x229C, equal in circle
    "circleddash" => Symbol { unicode: 0x229D, atom_type: AtomType::Binary }, // Unicode: 0x229D, hyphen in circle
    "boxplus" => Symbol { unicode: 0x229E, atom_type: AtomType::Binary }, // Unicode: 0x229E, plus sign in box
    "boxminus" => Symbol { unicode: 0x229F, atom_type: AtomType::Binary }, // Unicode: 0x229F, minus sign in box
    "boxtimes" => Symbol { unicode: 0x22A0, atom_type: AtomType::Binary }, // Unicode: 0x22A0, multiply sign in box
    "boxdot" => Symbol { unicode: 0x22A1, atom_type: AtomType::Binary }, // Unicode: 0x22A1, /dotsquare /boxdot b: small dot in box
    "vdash" => Symbol { unicode: 0x22A2, atom_type: AtomType::Relation }, // Unicode: 0x22A2, vertical, dash
    "dashv" => Symbol { unicode: 0x22A3, atom_type: AtomType::Relation }, // Unicode: 0x22A3, dash, vertical
    "top" => Symbol { unicode: 0x22A4, atom_type: AtomType::Alpha }, // Unicode: 0x22A4, top
    "bot" => Symbol { unicode: 0x22A5, atom_type: AtomType::Alpha }, // Unicode: 0x22A5, bottom
    "assert" => Symbol { unicode: 0x22A6, atom_type: AtomType::Relation }, // Unicode: 0x22A6, assertion (vertical, short dash)
    "models" => Symbol { unicode: 0x22A7, atom_type: AtomType::Relation }, // Unicode: 0x22A7, models (vertical, short double dash)
    "vDash" => Symbol { unicode: 0x22A8, atom_type: AtomType::Relation }, // Unicode: 0x22A8, vertical, double dash
    "Vdash" => Symbol { unicode: 0x22A9, atom_type: AtomType::Relation }, // Unicode: 0x22A9, double vertical, dash
    "Vvdash" => Symbol { unicode: 0x22AA, atom_type: AtomType::Relation }, // Unicode: 0x22AA, triple vertical, dash
    "VDash" => Symbol { unicode: 0x22AB, atom_type: AtomType::Relation }, // Unicode: 0x22AB, double vert, double dash
    "nvdash" => Symbol { unicode: 0x22AC, atom_type: AtomType::Relation }, // Unicode: 0x22AC, not vertical, dash
    "nvDash" => Symbol { unicode: 0x22AD, atom_type: AtomType::Relation }, // Unicode: 0x22AD, not vertical, double dash
    "nVdash" => Symbol { unicode: 0x22AE, atom_type: AtomType::Relation }, // Unicode: 0x22AE, not double vertical, dash
    "nVDash" => Symbol { unicode: 0x22AF, atom_type: AtomType::Relation }, // Unicode: 0x22AF, not double vert, double dash
    "prurel" => Symbol { unicode: 0x22B0, atom_type: AtomType::Relation }, // Unicode: 0x22B0, element precedes under relation
    "scurel" => Symbol { unicode: 0x22B1, atom_type: AtomType::Relation }, // Unicode: 0x22B1, succeeds under relation
    "vartriangleleft" => Symbol { unicode: 0x22B2, atom_type: AtomType::Relation }, // Unicode: 0x22B2, left triangle, open, variant
    "vartriangleright" => Symbol { unicode: 0x22B3, atom_type: AtomType::Relation }, // Unicode: 0x22B3, right triangle, open, variant
    "trianglelefteq" => Symbol { unicode: 0x22B4, atom_type: AtomType::Relation }, // Unicode: 0x22B4, left triangle, equals
    "trianglerighteq" => Symbol { unicode: 0x22B5, atom_type: AtomType::Relation }, // Unicode: 0x22B5, right triangle, equals
    "origof" => Symbol { unicode: 0x22B6, atom_type: AtomType::Relation }, // Unicode: 0x22B6, original of
    "imageof" => Symbol { unicode: 0x22B7, atom_type: AtomType::Relation }, // Unicode: 0x22B7, image of
    "multimap" => Symbol { unicode: 0x22B8, atom_type: AtomType::Relation }, // Unicode: 0x22B8, /multimap a:
    "hermitmatrix" => Symbol { unicode: 0x22B9, atom_type: AtomType::Alpha }, // Unicode: 0x22B9, hermitian conjugate matrix
    "intercal" => Symbol { unicode: 0x22BA, atom_type: AtomType::Binary }, // Unicode: 0x22BA, intercal
    "veebar" => Symbol { unicode: 0x22BB, atom_type: AtomType::Binary }, // Unicode: 0x22BB, logical or, bar below (large vee); exclusive disjunction
    "barwedge" => Symbol { unicode: 0x22BC, atom_type: AtomType::Binary }, // Unicode: 0x22BC, bar, wedge (large wedge)
    "barvee" => Symbol { unicode: 0x22BD, atom_type: AtomType::Binary }, // Unicode: 0x22BD, bar, vee (large vee)
    "measuredrightangle" => Symbol { unicode: 0x22BE, atom_type: AtomType::Alpha }, // Unicode: 0x22BE, right angle-measured [with arc]
    "varlrtriangle" => Symbol { unicode: 0x22BF, atom_type: AtomType::Alpha }, // Unicode: 0x22BF, right triangle
    "bigwedge" => Symbol { unicode: 0x22C0, atom_type: AtomType::Operator(true) }, // Unicode: 0x22C0, logical or operator
    "bigvee" => Symbol { unicode: 0x22C1, atom_type: AtomType::Operator(true) }, // Unicode: 0x22C1, logical and operator
    "bigcap" => Symbol { unicode: 0x22C2, atom_type: AtomType::Operator(true) }, // Unicode: 0x22C2, intersection operator
    "bigcup" => Symbol { unicode: 0x22C3, atom_type: AtomType::Operator(true) }, // Unicode: 0x22C3, union operator
    "smwhtdiamond" => Symbol { unicode: 0x22C4, atom_type: AtomType::Binary }, // Unicode: 0x22C4, white diamond
    "cdot" => Symbol { unicode: 0x22C5, atom_type: AtomType::Binary }, // Unicode: 0x22C5, small middle dot
    "star" => Symbol { unicode: 0x22C6, atom_type: AtomType::Binary }, // Unicode: 0x22C6, small star, filled, low
    "divideontimes" => Symbol { unicode: 0x22C7, atom_type: AtomType::Binary }, // Unicode: 0x22C7, division on times
    "bowtie" => Symbol { unicode: 0x22C8, atom_type: AtomType::Relation }, // Unicode: 0x22C8, bowtie
    "ltimes" => Symbol { unicode: 0x22C9, atom_type: AtomType::Binary }, // Unicode: 0x22C9, times sign, left closed
    "rtimes" => Symbol { unicode: 0x22CA, atom_type: AtomType::Binary }, // Unicode: 0x22CA, times sign, right closed
    "leftthreetimes" => Symbol { unicode: 0x22CB, atom_type: AtomType::Binary }, // Unicode: 0x22CB, left semidirect product
    "rightthreetimes" => Symbol { unicode: 0x22CC, atom_type: AtomType::Binary }, // Unicode: 0x22CC, right semidirect product
    "backsimeq" => Symbol { unicode: 0x22CD, atom_type: AtomType::Relation }, // Unicode: 0x22CD, reverse similar, equals
    "curlyvee" => Symbol { unicode: 0x22CE, atom_type: AtomType::Binary }, // Unicode: 0x22CE, curly logical or
    "curlywedge" => Symbol { unicode: 0x22CF, atom_type: AtomType::Binary }, // Unicode: 0x22CF, curly logical and
    "Subset" => Symbol { unicode: 0x22D0, atom_type: AtomType::Relation }, // Unicode: 0x22D0, double subset
    "Supset" => Symbol { unicode: 0x22D1, atom_type: AtomType::Relation }, // Unicode: 0x22D1, double superset
    "Cap" => Symbol { unicode: 0x22D2, atom_type: AtomType::Binary }, // Unicode: 0x22D2, /cap /doublecap b: double intersection
    "Cup" => Symbol { unicode: 0x22D3, atom_type: AtomType::Binary }, // Unicode: 0x22D3, /cup /doublecup b: double union
    "pitchfork" => Symbol { unicode: 0x22D4, atom_type: AtomType::Relation }, // Unicode: 0x22D4, pitchfork
    "equalparallel" => Symbol { unicode: 0x22D5, atom_type: AtomType::Relation }, // Unicode: 0x22D5, parallel, equal; equal or parallel
    "lessdot" => Symbol { unicode: 0x22D6, atom_type: AtomType::Relation }, // Unicode: 0x22D6, less than, with dot
    "gtrdot" => Symbol { unicode: 0x22D7, atom_type: AtomType::Relation }, // Unicode: 0x22D7, greater than, with dot
    "lll" => Symbol { unicode: 0x22D8, atom_type: AtomType::Relation }, // Unicode: 0x22D8, /ll /lll /llless r: triple less-than
    "ggg" => Symbol { unicode: 0x22D9, atom_type: AtomType::Relation }, // Unicode: 0x22D9, /ggg /gg /gggtr r: triple greater-than
    "lesseqgtr" => Symbol { unicode: 0x22DA, atom_type: AtomType::Relation }, // Unicode: 0x22DA, less, equals, greater
    "gtreqless" => Symbol { unicode: 0x22DB, atom_type: AtomType::Relation }, // Unicode: 0x22DB, greater, equals, less
    "eqless" => Symbol { unicode: 0x22DC, atom_type: AtomType::Relation }, // Unicode: 0x22DC, equal-or-less
    "eqgtr" => Symbol { unicode: 0x22DD, atom_type: AtomType::Relation }, // Unicode: 0x22DD, equal-or-greater
    "curlyeqprec" => Symbol { unicode: 0x22DE, atom_type: AtomType::Relation }, // Unicode: 0x22DE, curly equals, precedes
    "curlyeqsucc" => Symbol { unicode: 0x22DF, atom_type: AtomType::Relation }, // Unicode: 0x22DF, curly equals, succeeds
    "npreccurlyeq" => Symbol { unicode: 0x22E0, atom_type: AtomType::Relation }, // Unicode: 0x22E0, not precedes, curly equals
    "nsucccurlyeq" => Symbol { unicode: 0x22E1, atom_type: AtomType::Relation }, // Unicode: 0x22E1, not succeeds, curly equals
    "nsqsubseteq" => Symbol { unicode: 0x22E2, atom_type: AtomType::Relation }, // Unicode: 0x22E2, not, square subset, equals
    "nsqsupseteq" => Symbol { unicode: 0x22E3, atom_type: AtomType::Relation }, // Unicode: 0x22E3, not, square superset, equals
    "sqsubsetneq" => Symbol { unicode: 0x22E4, atom_type: AtomType::Relation }, // Unicode: 0x22E4, square subset, not equals
    "sqsupsetneq" => Symbol { unicode: 0x22E5, atom_type: AtomType::Relation }, // Unicode: 0x22E5, square superset, not equals
    "lnsim" => Symbol { unicode: 0x22E6, atom_type: AtomType::Relation }, // Unicode: 0x22E6, less, not similar
    "gnsim" => Symbol { unicode: 0x22E7, atom_type: AtomType::Relation }, // Unicode: 0x22E7, greater, not similar
    "precnsim" => Symbol { unicode: 0x22E8, atom_type: AtomType::Relation }, // Unicode: 0x22E8, precedes, not similar
    "succnsim" => Symbol { unicode: 0x22E9, atom_type: AtomType::Relation }, // Unicode: 0x22E9, succeeds, not similar
    "nvartriangleleft" => Symbol { unicode: 0x22EA, atom_type: AtomType::Relation }, // Unicode: 0x22EA, not left triangle
    "nvartriangleright" => Symbol { unicode: 0x22EB, atom_type: AtomType::Relation }, // Unicode: 0x22EB, not right triangle
    "ntrianglelefteq" => Symbol { unicode: 0x22EC, atom_type: AtomType::Relation }, // Unicode: 0x22EC, not left triangle, equals
    "ntrianglerighteq" => Symbol { unicode: 0x22ED, atom_type: AtomType::Relation }, // Unicode: 0x22ED, not right triangle, equals
    "vdots" => Symbol { unicode: 0x22EE, atom_type: AtomType::Relation }, // Unicode: 0x22EE, vertical ellipsis
    "unicodecdots" => Symbol { unicode: 0x22EF, atom_type: AtomType::Alpha }, // Unicode: 0x22EF, three dots, centered
    "cdots" => Symbol { unicode: 0x22EF, atom_type: AtomType::Alpha }, // Unicode: 0x22EF, three dots, centered
    "adots" => Symbol { unicode: 0x22F0, atom_type: AtomType::Relation }, // Unicode: 0x22F0, three dots, ascending
    "ddots" => Symbol { unicode: 0x22F1, atom_type: AtomType::Relation }, // Unicode: 0x22F1, three dots, descending
    "disin" => Symbol { unicode: 0x22F2, atom_type: AtomType::Relation }, // Unicode: 0x22F2, element of with long horizontal stroke
    "varisins" => Symbol { unicode: 0x22F3, atom_type: AtomType::Relation }, // Unicode: 0x22F3, element of with vertical bar at end of horizontal stroke
    "isins" => Symbol { unicode: 0x22F4, atom_type: AtomType::Relation }, // Unicode: 0x22F4, small element of with vertical bar at end of horizontal stroke
    "isindot" => Symbol { unicode: 0x22F5, atom_type: AtomType::Relation }, // Unicode: 0x22F5, element of with dot above
    "varisinobar" => Symbol { unicode: 0x22F6, atom_type: AtomType::Relation }, // Unicode: 0x22F6, element of with overbar
    "isinobar" => Symbol { unicode: 0x22F7, atom_type: AtomType::Relation }, // Unicode: 0x22F7, small element of with overbar
    "isinvb" => Symbol { unicode: 0x22F8, atom_type: AtomType::Relation }, // Unicode: 0x22F8, element of with underbar
    "isinE" => Symbol { unicode: 0x22F9, atom_type: AtomType::Relation }, // Unicode: 0x22F9, element of with two horizontal strokes
    "nisd" => Symbol { unicode: 0x22FA, atom_type: AtomType::Relation }, // Unicode: 0x22FA, contains with long horizontal stroke
    "varnis" => Symbol { unicode: 0x22FB, atom_type: AtomType::Relation }, // Unicode: 0x22FB, contains with vertical bar at end of horizontal stroke
    "nis" => Symbol { unicode: 0x22FC, atom_type: AtomType::Relation }, // Unicode: 0x22FC, small contains with vertical bar at end of horizontal stroke
    "varniobar" => Symbol { unicode: 0x22FD, atom_type: AtomType::Relation }, // Unicode: 0x22FD, contains with overbar
    "niobar" => Symbol { unicode: 0x22FE, atom_type: AtomType::Relation }, // Unicode: 0x22FE, small contains with overbar
    "bagmember" => Symbol { unicode: 0x22FF, atom_type: AtomType::Relation }, // Unicode: 0x22FF, z notation bag membership
    "diameter" => Symbol { unicode: 0x2300, atom_type: AtomType::Alpha }, // Unicode: 0x2300, diameter sign
    "house" => Symbol { unicode: 0x2302, atom_type: AtomType::Alpha }, // Unicode: 0x2302, house
    "varbarwedge" => Symbol { unicode: 0x2305, atom_type: AtomType::Binary }, // Unicode: 0x2305, /barwedge b: logical and, bar above [projective (bar over small wedge)]
    "vardoublebarwedge" => Symbol { unicode: 0x2306, atom_type: AtomType::Binary }, // Unicode: 0x2306, /doublebarwedge b: logical and, double bar above [perspective (double bar over small wedge)]
    "lceil" => Symbol { unicode: 0x2308, atom_type: AtomType::Open }, // Unicode: 0x2308, left ceiling
    "rceil" => Symbol { unicode: 0x2309, atom_type: AtomType::Close }, // Unicode: 0x2309, right ceiling
    "lfloor" => Symbol { unicode: 0x230A, atom_type: AtomType::Open }, // Unicode: 0x230A, left floor
    "rfloor" => Symbol { unicode: 0x230B, atom_type: AtomType::Close }, // Unicode: 0x230B, right floor
    "invnot" => Symbol { unicode: 0x2310, atom_type: AtomType::Alpha }, // Unicode: 0x2310, reverse not
    "sqlozenge" => Symbol { unicode: 0x2311, atom_type: AtomType::Alpha }, // Unicode: 0x2311, square lozenge
    "profline" => Symbol { unicode: 0x2312, atom_type: AtomType::Alpha }, // Unicode: 0x2312, profile of a line
    "profsurf" => Symbol { unicode: 0x2313, atom_type: AtomType::Alpha }, // Unicode: 0x2313, profile of a surface
    "viewdata" => Symbol { unicode: 0x2317, atom_type: AtomType::Alpha }, // Unicode: 0x2317, viewdata square
    "turnednot" => Symbol { unicode: 0x2319, atom_type: AtomType::Alpha }, // Unicode: 0x2319, turned not sign
    "ulcorner" => Symbol { unicode: 0x231C, atom_type: AtomType::Open }, // Unicode: 0x231C, upper left corner
    "urcorner" => Symbol { unicode: 0x231D, atom_type: AtomType::Close }, // Unicode: 0x231D, upper right corner
    "llcorner" => Symbol { unicode: 0x231E, atom_type: AtomType::Open }, // Unicode: 0x231E, lower left corner
    "lrcorner" => Symbol { unicode: 0x231F, atom_type: AtomType::Close }, // Unicode: 0x231F, lower right corner
    "inttop" => Symbol { unicode: 0x2320, atom_type: AtomType::Alpha }, // Unicode: 0x2320, top half integral
    "intbottom" => Symbol { unicode: 0x2321, atom_type: AtomType::Alpha }, // Unicode: 0x2321, bottom half integral
    "frown" => Symbol { unicode: 0x2322, atom_type: AtomType::Relation }, // Unicode: 0x2322, down curve
    "smile" => Symbol { unicode: 0x2323, atom_type: AtomType::Relation }, // Unicode: 0x2323, up curve
    "varhexagonlrbonds" => Symbol { unicode: 0x232C, atom_type: AtomType::Alpha }, // Unicode: 0x232C, six carbon ring, corner down, double bonds lower right etc
    "conictaper" => Symbol { unicode: 0x2332, atom_type: AtomType::Alpha }, // Unicode: 0x2332, conical taper
    "topbot" => Symbol { unicode: 0x2336, atom_type: AtomType::Alpha }, // Unicode: 0x2336, top and bottom
    "obar" => Symbol { unicode: 0x233D, atom_type: AtomType::Binary }, // Unicode: 0x233D, circle with vertical bar
    "APLnotslash" => Symbol { unicode: 0x233F, atom_type: AtomType::Relation }, // Unicode: 0x233F, solidus, bar through (apl functional symbol slash bar)
    "APLnotbackslash" => Symbol { unicode: 0x2340, atom_type: AtomType::Alpha }, // Unicode: 0x2340, apl functional symbol backslash bar
    "APLboxupcaret" => Symbol { unicode: 0x2353, atom_type: AtomType::Alpha }, // Unicode: 0x2353, boxed up caret
    "APLboxquestion" => Symbol { unicode: 0x2370, atom_type: AtomType::Alpha }, // Unicode: 0x2370, boxed question mark
    "rangledownzigzagarrow" => Symbol { unicode: 0x237C, atom_type: AtomType::Alpha }, // Unicode: 0x237C, right angle with downwards zigzag arrow
    "hexagon" => Symbol { unicode: 0x2394, atom_type: AtomType::Alpha }, // Unicode: 0x2394, horizontal benzene ring [hexagon flat open]
    "lparenuend" => Symbol { unicode: 0x239B, atom_type: AtomType::Alpha }, // Unicode: 0x239B, left parenthesis upper hook
    "lparenextender" => Symbol { unicode: 0x239C, atom_type: AtomType::Alpha }, // Unicode: 0x239C, left parenthesis extension
    "lparenlend" => Symbol { unicode: 0x239D, atom_type: AtomType::Alpha }, // Unicode: 0x239D, left parenthesis lower hook
    "rparenuend" => Symbol { unicode: 0x239E, atom_type: AtomType::Alpha }, // Unicode: 0x239E, right parenthesis upper hook
    "rparenextender" => Symbol { unicode: 0x239F, atom_type: AtomType::Alpha }, // Unicode: 0x239F, right parenthesis extension
    "rparenlend" => Symbol { unicode: 0x23A0, atom_type: AtomType::Alpha }, // Unicode: 0x23A0, right parenthesis lower hook
    "lbrackuend" => Symbol { unicode: 0x23A1, atom_type: AtomType::Alpha }, // Unicode: 0x23A1, left square bracket upper corner
    "lbrackextender" => Symbol { unicode: 0x23A2, atom_type: AtomType::Alpha }, // Unicode: 0x23A2, left square bracket extension
    "lbracklend" => Symbol { unicode: 0x23A3, atom_type: AtomType::Alpha }, // Unicode: 0x23A3, left square bracket lower corner
    "rbrackuend" => Symbol { unicode: 0x23A4, atom_type: AtomType::Alpha }, // Unicode: 0x23A4, right square bracket upper corner
    "rbrackextender" => Symbol { unicode: 0x23A5, atom_type: AtomType::Alpha }, // Unicode: 0x23A5, right square bracket extension
    "rbracklend" => Symbol { unicode: 0x23A6, atom_type: AtomType::Alpha }, // Unicode: 0x23A6, right square bracket lower corner
    "lbraceuend" => Symbol { unicode: 0x23A7, atom_type: AtomType::Alpha }, // Unicode: 0x23A7, left curly bracket upper hook
    "lbracemid" => Symbol { unicode: 0x23A8, atom_type: AtomType::Alpha }, // Unicode: 0x23A8, left curly bracket middle piece
    "lbracelend" => Symbol { unicode: 0x23A9, atom_type: AtomType::Alpha }, // Unicode: 0x23A9, left curly bracket lower hook
    "vbraceextender" => Symbol { unicode: 0x23AA, atom_type: AtomType::Alpha }, // Unicode: 0x23AA, curly bracket extension
    "rbraceuend" => Symbol { unicode: 0x23AB, atom_type: AtomType::Alpha }, // Unicode: 0x23AB, right curly bracket upper hook
    "rbracemid" => Symbol { unicode: 0x23AC, atom_type: AtomType::Alpha }, // Unicode: 0x23AC, right curly bracket middle piece
    "rbracelend" => Symbol { unicode: 0x23AD, atom_type: AtomType::Alpha }, // Unicode: 0x23AD, right curly bracket lower hook
    "intextender" => Symbol { unicode: 0x23AE, atom_type: AtomType::Alpha }, // Unicode: 0x23AE, integral extension
    "harrowextender" => Symbol { unicode: 0x23AF, atom_type: AtomType::Alpha }, // Unicode: 0x23AF, horizontal line extension (used to extend arrows)
    "lmoustache" => Symbol { unicode: 0x23B0, atom_type: AtomType::Open }, // Unicode: 0x23B0, upper left or lower right curly bracket section
    "rmoustache" => Symbol { unicode: 0x23B1, atom_type: AtomType::Close }, // Unicode: 0x23B1, upper right or lower left curly bracket section
    "sumtop" => Symbol { unicode: 0x23B2, atom_type: AtomType::Alpha }, // Unicode: 0x23B2, summation top
    "sumbottom" => Symbol { unicode: 0x23B3, atom_type: AtomType::Alpha }, // Unicode: 0x23B3, summation bottom
    "overbracket" => Symbol { unicode: 0x23B4, atom_type: AtomType::Over }, // Unicode: 0x23B4, top square bracket
    "underbracket" => Symbol { unicode: 0x23B5, atom_type: AtomType::Under }, // Unicode: 0x23B5, bottom square bracket
    "bbrktbrk" => Symbol { unicode: 0x23B6, atom_type: AtomType::Alpha }, // Unicode: 0x23B6, bottom square bracket over top square bracket
    "sqrtbottom" => Symbol { unicode: 0x23B7, atom_type: AtomType::Alpha }, // Unicode: 0x23B7, radical symbol bottom
    "lvboxline" => Symbol { unicode: 0x23B8, atom_type: AtomType::Alpha }, // Unicode: 0x23B8, left vertical box line
    "rvboxline" => Symbol { unicode: 0x23B9, atom_type: AtomType::Alpha }, // Unicode: 0x23B9, right vertical box line
    "varcarriagereturn" => Symbol { unicode: 0x23CE, atom_type: AtomType::Alpha }, // Unicode: 0x23CE, return symbol
    "overparen" => Symbol { unicode: 0x23DC, atom_type: AtomType::Over }, // Unicode: 0x23DC, top parenthesis (mathematical use)
    "underparen" => Symbol { unicode: 0x23DD, atom_type: AtomType::Under }, // Unicode: 0x23DD, bottom parenthesis (mathematical use)
    "overbrace" => Symbol { unicode: 0x23DE, atom_type: AtomType::Accent }, // Unicode: 0x23DE, top curly bracket (mathematical use)
    "underbrace" => Symbol { unicode: 0x23DF, atom_type: AtomType::Under }, // Unicode: 0x23DF, bottom curly bracket (mathematical use)
    "obrbrak" => Symbol { unicode: 0x23E0, atom_type: AtomType::Alpha }, // Unicode: 0x23E0, top tortoise shell bracket (mathematical use)
    "ubrbrak" => Symbol { unicode: 0x23E1, atom_type: AtomType::Alpha }, // Unicode: 0x23E1, bottom tortoise shell bracket (mathematical use)
    "trapezium" => Symbol { unicode: 0x23E2, atom_type: AtomType::Alpha }, // Unicode: 0x23E2, white trapezium
    "benzenr" => Symbol { unicode: 0x23E3, atom_type: AtomType::Alpha }, // Unicode: 0x23E3, benzene ring with circle
    "strns" => Symbol { unicode: 0x23E4, atom_type: AtomType::Alpha }, // Unicode: 0x23E4, straightness
    "fltns" => Symbol { unicode: 0x23E5, atom_type: AtomType::Alpha }, // Unicode: 0x23E5, flatness
    "accurrent" => Symbol { unicode: 0x23E6, atom_type: AtomType::Alpha }, // Unicode: 0x23E6, ac current
    "elinters" => Symbol { unicode: 0x23E7, atom_type: AtomType::Alpha }, // Unicode: 0x23E7, electrical intersection
    "mathvisiblespace" => Symbol { unicode: 0x2423, atom_type: AtomType::Alpha }, // Unicode: 0x2423, open box
    "bdtriplevdash" => Symbol { unicode: 0x2506, atom_type: AtomType::Alpha }, // Unicode: 0x2506, doubly broken vert
    "blockuphalf" => Symbol { unicode: 0x2580, atom_type: AtomType::Alpha }, // Unicode: 0x2580, upper half block
    "blocklowhalf" => Symbol { unicode: 0x2584, atom_type: AtomType::Alpha }, // Unicode: 0x2584, lower half block
    "blockfull" => Symbol { unicode: 0x2588, atom_type: AtomType::Alpha }, // Unicode: 0x2588, full block
    "blocklefthalf" => Symbol { unicode: 0x258C, atom_type: AtomType::Alpha }, // Unicode: 0x258C, left half block
    "blockrighthalf" => Symbol { unicode: 0x2590, atom_type: AtomType::Alpha }, // Unicode: 0x2590, right half block
    "blockqtrshaded" => Symbol { unicode: 0x2591, atom_type: AtomType::Alpha }, // Unicode: 0x2591, 25\% shaded block
    "blockhalfshaded" => Symbol { unicode: 0x2592, atom_type: AtomType::Alpha }, // Unicode: 0x2592, 50\% shaded block
    "blockthreeqtrshaded" => Symbol { unicode: 0x2593, atom_type: AtomType::Alpha }, // Unicode: 0x2593, 75\% shaded block
    "mdlgblksquare" => Symbol { unicode: 0x25A0, atom_type: AtomType::Alpha }, // Unicode: 0x25A0, square, filled
    "mdlgwhtsquare" => Symbol { unicode: 0x25A1, atom_type: AtomType::Alpha }, // Unicode: 0x25A1, square, open
    "squoval" => Symbol { unicode: 0x25A2, atom_type: AtomType::Alpha }, // Unicode: 0x25A2, white square with rounded corners
    "blackinwhitesquare" => Symbol { unicode: 0x25A3, atom_type: AtomType::Alpha }, // Unicode: 0x25A3, white square containing black small square
    "squarehfill" => Symbol { unicode: 0x25A4, atom_type: AtomType::Alpha }, // Unicode: 0x25A4, square, horizontal rule filled
    "squarevfill" => Symbol { unicode: 0x25A5, atom_type: AtomType::Alpha }, // Unicode: 0x25A5, square, vertical rule filled
    "squarehvfill" => Symbol { unicode: 0x25A6, atom_type: AtomType::Alpha }, // Unicode: 0x25A6, square with orthogonal crosshatch fill
    "squarenwsefill" => Symbol { unicode: 0x25A7, atom_type: AtomType::Alpha }, // Unicode: 0x25A7, square, nw-to-se rule filled
    "squareneswfill" => Symbol { unicode: 0x25A8, atom_type: AtomType::Alpha }, // Unicode: 0x25A8, square, ne-to-sw rule filled
    "squarecrossfill" => Symbol { unicode: 0x25A9, atom_type: AtomType::Alpha }, // Unicode: 0x25A9, square with diagonal crosshatch fill
    "smblksquare" => Symbol { unicode: 0x25AA, atom_type: AtomType::Alpha }, // Unicode: 0x25AA, /blacksquare - sq bullet, filled
    "smwhtsquare" => Symbol { unicode: 0x25AB, atom_type: AtomType::Alpha }, // Unicode: 0x25AB, white small square
    "hrectangleblack" => Symbol { unicode: 0x25AC, atom_type: AtomType::Alpha }, // Unicode: 0x25AC, black rectangle
    "hrectangle" => Symbol { unicode: 0x25AD, atom_type: AtomType::Alpha }, // Unicode: 0x25AD, horizontal rectangle, open
    "vrectangleblack" => Symbol { unicode: 0x25AE, atom_type: AtomType::Alpha }, // Unicode: 0x25AE, black vertical rectangle
    "vrectangle" => Symbol { unicode: 0x25AF, atom_type: AtomType::Alpha }, // Unicode: 0x25AF, rectangle, white (vertical)
    "parallelogramblack" => Symbol { unicode: 0x25B0, atom_type: AtomType::Alpha }, // Unicode: 0x25B0, black parallelogram
    "parallelogram" => Symbol { unicode: 0x25B1, atom_type: AtomType::Alpha }, // Unicode: 0x25B1, parallelogram, open
    "bigblacktriangleup" => Symbol { unicode: 0x25B2, atom_type: AtomType::Alpha }, // Unicode: 0x25B2,    0x25b2 6 6d      black up-pointing triangle
    "bigtriangleup" => Symbol { unicode: 0x25B3, atom_type: AtomType::Binary }, // Unicode: 0x25B3, big up triangle, open
    "blacktriangle" => Symbol { unicode: 0x25B4, atom_type: AtomType::Alpha }, // Unicode: 0x25B4, up triangle, filled
    "vartriangle" => Symbol { unicode: 0x25B5, atom_type: AtomType::Relation }, // Unicode: 0x25B5, /triangle - up triangle, open
    "blacktriangleright" => Symbol { unicode: 0x25B6, atom_type: AtomType::Alpha }, // Unicode: 0x25B6, (large) right triangle, filled
    "triangleright" => Symbol { unicode: 0x25B7, atom_type: AtomType::Binary }, // Unicode: 0x25B7, (large) right triangle, open; z notation range restriction
    "smallblacktriangleright" => Symbol { unicode: 0x25B8, atom_type: AtomType::Alpha }, // Unicode: 0x25B8, right triangle, filled
    "smalltriangleright" => Symbol { unicode: 0x25B9, atom_type: AtomType::Alpha }, // Unicode: 0x25B9, right triangle, open
    "blackpointerright" => Symbol { unicode: 0x25BA, atom_type: AtomType::Alpha }, // Unicode: 0x25BA, black right-pointing pointer
    "whitepointerright" => Symbol { unicode: 0x25BB, atom_type: AtomType::Alpha }, // Unicode: 0x25BB, white right-pointing pointer
    "bigblacktriangledown" => Symbol { unicode: 0x25BC, atom_type: AtomType::Alpha }, // Unicode: 0x25BC, big down triangle, filled
    "bigtriangledown" => Symbol { unicode: 0x25BD, atom_type: AtomType::Alpha }, // Unicode: 0x25BD, big down triangle, open
    "blacktriangledown" => Symbol { unicode: 0x25BE, atom_type: AtomType::Alpha }, // Unicode: 0x25BE, down triangle, filled
    "triangledown" => Symbol { unicode: 0x25BF, atom_type: AtomType::Alpha }, // Unicode: 0x25BF, down triangle, open
    "blacktriangleleft" => Symbol { unicode: 0x25C0, atom_type: AtomType::Alpha }, // Unicode: 0x25C0, (large) left triangle, filled
    "triangleleft" => Symbol { unicode: 0x25C1, atom_type: AtomType::Binary }, // Unicode: 0x25C1, (large) left triangle, open; z notation domain restriction
    "smallblacktriangleleft" => Symbol { unicode: 0x25C2, atom_type: AtomType::Alpha }, // Unicode: 0x25C2, left triangle, filled
    "smalltriangleleft" => Symbol { unicode: 0x25C3, atom_type: AtomType::Alpha }, // Unicode: 0x25C3, left triangle, open
    "blackpointerleft" => Symbol { unicode: 0x25C4, atom_type: AtomType::Alpha }, // Unicode: 0x25C4, black left-pointing pointer
    "whitepointerleft" => Symbol { unicode: 0x25C5, atom_type: AtomType::Alpha }, // Unicode: 0x25C5, white left-pointing pointer
    "mdlgblkdiamond" => Symbol { unicode: 0x25C6, atom_type: AtomType::Alpha }, // Unicode: 0x25C6, black diamond
    "mdlgwhtdiamond" => Symbol { unicode: 0x25C7, atom_type: AtomType::Alpha }, // Unicode: 0x25C7, white diamond; diamond, open
    "blackinwhitediamond" => Symbol { unicode: 0x25C8, atom_type: AtomType::Alpha }, // Unicode: 0x25C8, white diamond containing black small diamond
    "fisheye" => Symbol { unicode: 0x25C9, atom_type: AtomType::Alpha }, // Unicode: 0x25C9, fisheye
    "mdlgwhtlozenge" => Symbol { unicode: 0x25CA, atom_type: AtomType::Alpha }, // Unicode: 0x25CA, lozenge or total mark
    "mdlgwhtcircle" => Symbol { unicode: 0x25CB, atom_type: AtomType::Binary }, // Unicode: 0x25CB, medium large circle
    "dottedcircle" => Symbol { unicode: 0x25CC, atom_type: AtomType::Alpha }, // Unicode: 0x25CC, dotted circle
    "circlevertfill" => Symbol { unicode: 0x25CD, atom_type: AtomType::Alpha }, // Unicode: 0x25CD, circle with vertical fill
    "bullseye" => Symbol { unicode: 0x25CE, atom_type: AtomType::Alpha }, // Unicode: 0x25CE, bullseye
    "mdlgblkcircle" => Symbol { unicode: 0x25CF, atom_type: AtomType::Alpha }, // Unicode: 0x25CF, circle, filled
    "circlelefthalfblack" => Symbol { unicode: 0x25D0, atom_type: AtomType::Alpha }, // Unicode: 0x25D0, circle, filled left half [harvey ball]
    "circlerighthalfblack" => Symbol { unicode: 0x25D1, atom_type: AtomType::Alpha }, // Unicode: 0x25D1, circle, filled right half
    "circlebottomhalfblack" => Symbol { unicode: 0x25D2, atom_type: AtomType::Alpha }, // Unicode: 0x25D2, circle, filled bottom half
    "circletophalfblack" => Symbol { unicode: 0x25D3, atom_type: AtomType::Alpha }, // Unicode: 0x25D3, circle, filled top half
    "circleurquadblack" => Symbol { unicode: 0x25D4, atom_type: AtomType::Alpha }, // Unicode: 0x25D4, circle with upper right quadrant black
    "blackcircleulquadwhite" => Symbol { unicode: 0x25D5, atom_type: AtomType::Alpha }, // Unicode: 0x25D5, circle with all but upper left quadrant black
    "blacklefthalfcircle" => Symbol { unicode: 0x25D6, atom_type: AtomType::Alpha }, // Unicode: 0x25D6, left half black circle
    "blackrighthalfcircle" => Symbol { unicode: 0x25D7, atom_type: AtomType::Alpha }, // Unicode: 0x25D7, right half black circle
    "inversebullet" => Symbol { unicode: 0x25D8, atom_type: AtomType::Alpha }, // Unicode: 0x25D8, inverse bullet
    "inversewhitecircle" => Symbol { unicode: 0x25D9, atom_type: AtomType::Alpha }, // Unicode: 0x25D9, inverse white circle
    "invwhiteupperhalfcircle" => Symbol { unicode: 0x25DA, atom_type: AtomType::Alpha }, // Unicode: 0x25DA, upper half inverse white circle
    "invwhitelowerhalfcircle" => Symbol { unicode: 0x25DB, atom_type: AtomType::Alpha }, // Unicode: 0x25DB, lower half inverse white circle
    "ularc" => Symbol { unicode: 0x25DC, atom_type: AtomType::Alpha }, // Unicode: 0x25DC, upper left quadrant circular arc
    "urarc" => Symbol { unicode: 0x25DD, atom_type: AtomType::Alpha }, // Unicode: 0x25DD, upper right quadrant circular arc
    "lrarc" => Symbol { unicode: 0x25DE, atom_type: AtomType::Alpha }, // Unicode: 0x25DE, lower right quadrant circular arc
    "llarc" => Symbol { unicode: 0x25DF, atom_type: AtomType::Alpha }, // Unicode: 0x25DF, lower left quadrant circular arc
    "topsemicircle" => Symbol { unicode: 0x25E0, atom_type: AtomType::Alpha }, // Unicode: 0x25E0, upper half circle
    "botsemicircle" => Symbol { unicode: 0x25E1, atom_type: AtomType::Alpha }, // Unicode: 0x25E1, lower half circle
    "lrblacktriangle" => Symbol { unicode: 0x25E2, atom_type: AtomType::Alpha }, // Unicode: 0x25E2, lower right triangle, filled
    "llblacktriangle" => Symbol { unicode: 0x25E3, atom_type: AtomType::Alpha }, // Unicode: 0x25E3, lower left triangle, filled
    "ulblacktriangle" => Symbol { unicode: 0x25E4, atom_type: AtomType::Alpha }, // Unicode: 0x25E4, upper left triangle, filled
    "urblacktriangle" => Symbol { unicode: 0x25E5, atom_type: AtomType::Alpha }, // Unicode: 0x25E5, upper right triangle, filled
    "smwhtcircle" => Symbol { unicode: 0x25E6, atom_type: AtomType::Alpha }, // Unicode: 0x25E6, white bullet
    "squareleftblack" => Symbol { unicode: 0x25E7, atom_type: AtomType::Alpha }, // Unicode: 0x25E7, square, filled left half
    "squarerightblack" => Symbol { unicode: 0x25E8, atom_type: AtomType::Alpha }, // Unicode: 0x25E8, square, filled right half
    "squareulblack" => Symbol { unicode: 0x25E9, atom_type: AtomType::Alpha }, // Unicode: 0x25E9, square, filled top left corner
    "squarelrblack" => Symbol { unicode: 0x25EA, atom_type: AtomType::Alpha }, // Unicode: 0x25EA, square, filled bottom right corner
    "boxbar" => Symbol { unicode: 0x25EB, atom_type: AtomType::Binary }, // Unicode: 0x25EB, vertical bar in box
    "trianglecdot" => Symbol { unicode: 0x25EC, atom_type: AtomType::Alpha }, // Unicode: 0x25EC, triangle with centered dot
    "triangleleftblack" => Symbol { unicode: 0x25ED, atom_type: AtomType::Alpha }, // Unicode: 0x25ED, up-pointing triangle with left half black
    "trianglerightblack" => Symbol { unicode: 0x25EE, atom_type: AtomType::Alpha }, // Unicode: 0x25EE, up-pointing triangle with right half black
    "lgwhtcircle" => Symbol { unicode: 0x25EF, atom_type: AtomType::Alpha }, // Unicode: 0x25EF, large circle
    "squareulquad" => Symbol { unicode: 0x25F0, atom_type: AtomType::Alpha }, // Unicode: 0x25F0, white square with upper left quadrant
    "squarellquad" => Symbol { unicode: 0x25F1, atom_type: AtomType::Alpha }, // Unicode: 0x25F1, white square with lower left quadrant
    "squarelrquad" => Symbol { unicode: 0x25F2, atom_type: AtomType::Alpha }, // Unicode: 0x25F2, white square with lower right quadrant
    "squareurquad" => Symbol { unicode: 0x25F3, atom_type: AtomType::Alpha }, // Unicode: 0x25F3, white square with upper right quadrant
    "circleulquad" => Symbol { unicode: 0x25F4, atom_type: AtomType::Alpha }, // Unicode: 0x25F4, white circle with upper left quadrant
    "circlellquad" => Symbol { unicode: 0x25F5, atom_type: AtomType::Alpha }, // Unicode: 0x25F5, white circle with lower left quadrant
    "circlelrquad" => Symbol { unicode: 0x25F6, atom_type: AtomType::Alpha }, // Unicode: 0x25F6, white circle with lower right quadrant
    "circleurquad" => Symbol { unicode: 0x25F7, atom_type: AtomType::Alpha }, // Unicode: 0x25F7, white circle with upper right quadrant
    "ultriangle" => Symbol { unicode: 0x25F8, atom_type: AtomType::Alpha }, // Unicode: 0x25F8, upper left triangle
    "urtriangle" => Symbol { unicode: 0x25F9, atom_type: AtomType::Alpha }, // Unicode: 0x25F9, upper right triangle
    "lltriangle" => Symbol { unicode: 0x25FA, atom_type: AtomType::Alpha }, // Unicode: 0x25FA, lower left triangle
    "mdwhtsquare" => Symbol { unicode: 0x25FB, atom_type: AtomType::Alpha }, // Unicode: 0x25FB, white medium square
    "mdblksquare" => Symbol { unicode: 0x25FC, atom_type: AtomType::Alpha }, // Unicode: 0x25FC, black medium square
    "mdsmwhtsquare" => Symbol { unicode: 0x25FD, atom_type: AtomType::Alpha }, // Unicode: 0x25FD, white medium small square
    "mdsmblksquare" => Symbol { unicode: 0x25FE, atom_type: AtomType::Alpha }, // Unicode: 0x25FE, black medium small square
    "lrtriangle" => Symbol { unicode: 0x25FF, atom_type: AtomType::Alpha }, // Unicode: 0x25FF, lower right triangle
    "bigstar" => Symbol { unicode: 0x2605, atom_type: AtomType::Alpha }, // Unicode: 0x2605, star, filled
    "bigwhitestar" => Symbol { unicode: 0x2606, atom_type: AtomType::Alpha }, // Unicode: 0x2606, star, open
    "astrosun" => Symbol { unicode: 0x2609, atom_type: AtomType::Alpha }, // Unicode: 0x2609, sun
    "danger" => Symbol { unicode: 0x2621, atom_type: AtomType::Alpha }, // Unicode: 0x2621, dangerous bend (caution sign)
    "blacksmiley" => Symbol { unicode: 0x263B, atom_type: AtomType::Alpha }, // Unicode: 0x263B, black smiling face
    "sun" => Symbol { unicode: 0x263C, atom_type: AtomType::Alpha }, // Unicode: 0x263C, white sun with rays
    "rightmoon" => Symbol { unicode: 0x263D, atom_type: AtomType::Alpha }, // Unicode: 0x263D, first quarter moon
    "leftmoon" => Symbol { unicode: 0x263E, atom_type: AtomType::Alpha }, // Unicode: 0x263E, last quarter moon
    "female" => Symbol { unicode: 0x2640, atom_type: AtomType::Alpha }, // Unicode: 0x2640, venus, female
    "male" => Symbol { unicode: 0x2642, atom_type: AtomType::Alpha }, // Unicode: 0x2642, mars, male
    "spadesuit" => Symbol { unicode: 0x2660, atom_type: AtomType::Alpha }, // Unicode: 0x2660, spades suit symbol
    "heartsuit" => Symbol { unicode: 0x2661, atom_type: AtomType::Alpha }, // Unicode: 0x2661, heart suit symbol
    "diamondsuit" => Symbol { unicode: 0x2662, atom_type: AtomType::Alpha }, // Unicode: 0x2662, diamond suit symbol
    "clubsuit" => Symbol { unicode: 0x2663, atom_type: AtomType::Alpha }, // Unicode: 0x2663, club suit symbol
    "varspadesuit" => Symbol { unicode: 0x2664, atom_type: AtomType::Alpha }, // Unicode: 0x2664, spade, white (card suit)
    "varheartsuit" => Symbol { unicode: 0x2665, atom_type: AtomType::Alpha }, // Unicode: 0x2665, filled heart (card suit)
    "vardiamondsuit" => Symbol { unicode: 0x2666, atom_type: AtomType::Alpha }, // Unicode: 0x2666, filled diamond (card suit)
    "varclubsuit" => Symbol { unicode: 0x2667, atom_type: AtomType::Alpha }, // Unicode: 0x2667, club, white (card suit)
    "quarternote" => Symbol { unicode: 0x2669, atom_type: AtomType::Alpha }, // Unicode: 0x2669, music note (sung text sign)
    "eighthnote" => Symbol { unicode: 0x266A, atom_type: AtomType::Alpha }, // Unicode: 0x266A, eighth note
    "twonotes" => Symbol { unicode: 0x266B, atom_type: AtomType::Alpha }, // Unicode: 0x266B, beamed eighth notes
    "flat" => Symbol { unicode: 0x266D, atom_type: AtomType::Alpha }, // Unicode: 0x266D, musical flat
    "natural" => Symbol { unicode: 0x266E, atom_type: AtomType::Alpha }, // Unicode: 0x266E, music natural
    "sharp" => Symbol { unicode: 0x266F, atom_type: AtomType::Alpha }, // Unicode: 0x266F, musical sharp
    "acidfree" => Symbol { unicode: 0x267E, atom_type: AtomType::Alpha }, // Unicode: 0x267E, permanent paper sign
    "dicei" => Symbol { unicode: 0x2680, atom_type: AtomType::Alpha }, // Unicode: 0x2680, die face-1
    "diceii" => Symbol { unicode: 0x2681, atom_type: AtomType::Alpha }, // Unicode: 0x2681, die face-2
    "diceiii" => Symbol { unicode: 0x2682, atom_type: AtomType::Alpha }, // Unicode: 0x2682, die face-3
    "diceiv" => Symbol { unicode: 0x2683, atom_type: AtomType::Alpha }, // Unicode: 0x2683, die face-4
    "dicev" => Symbol { unicode: 0x2684, atom_type: AtomType::Alpha }, // Unicode: 0x2684, die face-5
    "dicevi" => Symbol { unicode: 0x2685, atom_type: AtomType::Alpha }, // Unicode: 0x2685, die face-6
    "circledrightdot" => Symbol { unicode: 0x2686, atom_type: AtomType::Alpha }, // Unicode: 0x2686, white circle with dot right
    "circledtwodots" => Symbol { unicode: 0x2687, atom_type: AtomType::Alpha }, // Unicode: 0x2687, white circle with two dots
    "blackcircledrightdot" => Symbol { unicode: 0x2688, atom_type: AtomType::Alpha }, // Unicode: 0x2688, black circle with white dot right
    "blackcircledtwodots" => Symbol { unicode: 0x2689, atom_type: AtomType::Alpha }, // Unicode: 0x2689, black circle with two white dots
    "Hermaphrodite" => Symbol { unicode: 0x26A5, atom_type: AtomType::Alpha }, // Unicode: 0x26A5, male and female sign
    "mdwhtcircle" => Symbol { unicode: 0x26AA, atom_type: AtomType::Alpha }, // Unicode: 0x26AA, medium white circle
    "mdblkcircle" => Symbol { unicode: 0x26AB, atom_type: AtomType::Alpha }, // Unicode: 0x26AB, medium black circle
    "mdsmwhtcircle" => Symbol { unicode: 0x26AC, atom_type: AtomType::Alpha }, // Unicode: 0x26AC, medium small white circle
    "neuter" => Symbol { unicode: 0x26B2, atom_type: AtomType::Alpha }, // Unicode: 0x26B2, neuter
    "checkmark" => Symbol { unicode: 0x2713, atom_type: AtomType::Alpha }, // Unicode: 0x2713, tick, check mark
    "maltese" => Symbol { unicode: 0x2720, atom_type: AtomType::Alpha }, // Unicode: 0x2720, maltese cross
    "circledstar" => Symbol { unicode: 0x272A, atom_type: AtomType::Alpha }, // Unicode: 0x272A, circled white star
    "varstar" => Symbol { unicode: 0x2736, atom_type: AtomType::Alpha }, // Unicode: 0x2736, six pointed black star
    "dingasterisk" => Symbol { unicode: 0x273D, atom_type: AtomType::Alpha }, // Unicode: 0x273D, heavy teardrop-spoked asterisk
    "lbrbrak" => Symbol { unicode: 0x2772, atom_type: AtomType::Open }, // Unicode: 0x2772, light left tortoise shell bracket ornament
    "rbrbrak" => Symbol { unicode: 0x2773, atom_type: AtomType::Close }, // Unicode: 0x2773, light right tortoise shell bracket ornament
    "draftingarrow" => Symbol { unicode: 0x279B, atom_type: AtomType::Alpha }, // Unicode: 0x279B, right arrow with bold head (drafting)
    "threedangle" => Symbol { unicode: 0x27C0, atom_type: AtomType::Alpha }, // Unicode: 0x27C0, three dimensional angle
    "whiteinwhitetriangle" => Symbol { unicode: 0x27C1, atom_type: AtomType::Alpha }, // Unicode: 0x27C1, white triangle containing small white triangle
    "perp" => Symbol { unicode: 0x27C2, atom_type: AtomType::Relation }, // Unicode: 0x27C2, perpendicular
    "subsetcirc" => Symbol { unicode: 0x27C3, atom_type: AtomType::Alpha }, // Unicode: 0x27C3, open subset
    "supsetcirc" => Symbol { unicode: 0x27C4, atom_type: AtomType::Alpha }, // Unicode: 0x27C4, open superset
    "lbag" => Symbol { unicode: 0x27C5, atom_type: AtomType::Open }, // Unicode: 0x27C5, left s-shaped bag delimiter
    "rbag" => Symbol { unicode: 0x27C6, atom_type: AtomType::Close }, // Unicode: 0x27C6, right s-shaped bag delimiter
    "veedot" => Symbol { unicode: 0x27C7, atom_type: AtomType::Binary }, // Unicode: 0x27C7, or with dot inside
    "bsolhsub" => Symbol { unicode: 0x27C8, atom_type: AtomType::Relation }, // Unicode: 0x27C8, reverse solidus preceding subset
    "suphsol" => Symbol { unicode: 0x27C9, atom_type: AtomType::Relation }, // Unicode: 0x27C9, superset preceding solidus
    "longdivision" => Symbol { unicode: 0x27CC, atom_type: AtomType::Open }, // Unicode: 0x27CC, long division
    "diamondcdot" => Symbol { unicode: 0x27D0, atom_type: AtomType::Alpha }, // Unicode: 0x27D0, white diamond with centred dot
    "wedgedot" => Symbol { unicode: 0x27D1, atom_type: AtomType::Binary }, // Unicode: 0x27D1, and with dot
    "upin" => Symbol { unicode: 0x27D2, atom_type: AtomType::Relation }, // Unicode: 0x27D2, element of opening upwards
    "pullback" => Symbol { unicode: 0x27D3, atom_type: AtomType::Relation }, // Unicode: 0x27D3, lower right corner with dot
    "pushout" => Symbol { unicode: 0x27D4, atom_type: AtomType::Relation }, // Unicode: 0x27D4, upper left corner with dot
    "leftouterjoin" => Symbol { unicode: 0x27D5, atom_type: AtomType::Operator(false) }, // Unicode: 0x27D5, left outer join
    "rightouterjoin" => Symbol { unicode: 0x27D6, atom_type: AtomType::Operator(false) }, // Unicode: 0x27D6, right outer join
    "fullouterjoin" => Symbol { unicode: 0x27D7, atom_type: AtomType::Operator(false) }, // Unicode: 0x27D7, full outer join
    "bigbot" => Symbol { unicode: 0x27D8, atom_type: AtomType::Operator(false) }, // Unicode: 0x27D8, large up tack
    "bigtop" => Symbol { unicode: 0x27D9, atom_type: AtomType::Operator(false) }, // Unicode: 0x27D9, large down tack
    "DashVDash" => Symbol { unicode: 0x27DA, atom_type: AtomType::Relation }, // Unicode: 0x27DA, left and right double turnstile
    "dashVdash" => Symbol { unicode: 0x27DB, atom_type: AtomType::Relation }, // Unicode: 0x27DB, left and right tack
    "multimapinv" => Symbol { unicode: 0x27DC, atom_type: AtomType::Relation }, // Unicode: 0x27DC, left multimap
    "vlongdash" => Symbol { unicode: 0x27DD, atom_type: AtomType::Relation }, // Unicode: 0x27DD, long left tack
    "longdashv" => Symbol { unicode: 0x27DE, atom_type: AtomType::Relation }, // Unicode: 0x27DE, long right tack
    "cirbot" => Symbol { unicode: 0x27DF, atom_type: AtomType::Relation }, // Unicode: 0x27DF, up tack with circle above
    "lozengeminus" => Symbol { unicode: 0x27E0, atom_type: AtomType::Binary }, // Unicode: 0x27E0, lozenge divided by horizontal rule
    "concavediamond" => Symbol { unicode: 0x27E1, atom_type: AtomType::Binary }, // Unicode: 0x27E1, white concave-sided diamond
    "concavediamondtickleft" => Symbol { unicode: 0x27E2, atom_type: AtomType::Binary }, // Unicode: 0x27E2, white concave-sided diamond with leftwards tick
    "concavediamondtickright" => Symbol { unicode: 0x27E3, atom_type: AtomType::Binary }, // Unicode: 0x27E3, white concave-sided diamond with rightwards tick
    "whitesquaretickleft" => Symbol { unicode: 0x27E4, atom_type: AtomType::Binary }, // Unicode: 0x27E4, white square with leftwards tick
    "whitesquaretickright" => Symbol { unicode: 0x27E5, atom_type: AtomType::Binary }, // Unicode: 0x27E5, white square with rightwards tick
    "lBrack" => Symbol { unicode: 0x27E6, atom_type: AtomType::Open }, // Unicode: 0x27E6, mathematical left white square bracket
    "rBrack" => Symbol { unicode: 0x27E7, atom_type: AtomType::Close }, // Unicode: 0x27E7, mathematical right white square bracket
    "langle" => Symbol { unicode: 0x27E8, atom_type: AtomType::Open }, // Unicode: 0x27E8, mathematical left angle bracket
    "rangle" => Symbol { unicode: 0x27E9, atom_type: AtomType::Close }, // Unicode: 0x27E9, mathematical right angle bracket
    "lAngle" => Symbol { unicode: 0x27EA, atom_type: AtomType::Open }, // Unicode: 0x27EA, mathematical left double angle bracket
    "rAngle" => Symbol { unicode: 0x27EB, atom_type: AtomType::Close }, // Unicode: 0x27EB, mathematical right double angle bracket
    "Lbrbrak" => Symbol { unicode: 0x27EC, atom_type: AtomType::Open }, // Unicode: 0x27EC, mathematical left white tortoise shell bracket
    "Rbrbrak" => Symbol { unicode: 0x27ED, atom_type: AtomType::Close }, // Unicode: 0x27ED, mathematical right white tortoise shell bracket
    "lgroup" => Symbol { unicode: 0x27EE, atom_type: AtomType::Open }, // Unicode: 0x27EE, mathematical left flattened parenthesis
    "rgroup" => Symbol { unicode: 0x27EF, atom_type: AtomType::Close }, // Unicode: 0x27EF, mathematical right flattened parenthesis
    "UUparrow" => Symbol { unicode: 0x27F0, atom_type: AtomType::Relation }, // Unicode: 0x27F0, upwards quadruple arrow
    "DDownarrow" => Symbol { unicode: 0x27F1, atom_type: AtomType::Relation }, // Unicode: 0x27F1, downwards quadruple arrow
    "acwgapcirclearrow" => Symbol { unicode: 0x27F2, atom_type: AtomType::Relation }, // Unicode: 0x27F2, anticlockwise gapped circle arrow
    "cwgapcirclearrow" => Symbol { unicode: 0x27F3, atom_type: AtomType::Relation }, // Unicode: 0x27F3, clockwise gapped circle arrow
    "rightarrowonoplus" => Symbol { unicode: 0x27F4, atom_type: AtomType::Relation }, // Unicode: 0x27F4, right arrow with circled plus
    "longleftarrow" => Symbol { unicode: 0x27F5, atom_type: AtomType::Relation }, // Unicode: 0x27F5, long leftwards arrow
    "longrightarrow" => Symbol { unicode: 0x27F6, atom_type: AtomType::Relation }, // Unicode: 0x27F6, long rightwards arrow
    "longleftrightarrow" => Symbol { unicode: 0x27F7, atom_type: AtomType::Relation }, // Unicode: 0x27F7, long left right arrow
    "Longleftarrow" => Symbol { unicode: 0x27F8, atom_type: AtomType::Relation }, // Unicode: 0x27F8, long leftwards double arrow
    "Longrightarrow" => Symbol { unicode: 0x27F9, atom_type: AtomType::Relation }, // Unicode: 0x27F9, long rightwards double arrow
    "Longleftrightarrow" => Symbol { unicode: 0x27FA, atom_type: AtomType::Relation }, // Unicode: 0x27FA, long left right double arrow
    "longmapsfrom" => Symbol { unicode: 0x27FB, atom_type: AtomType::Relation }, // Unicode: 0x27FB, long leftwards arrow from bar
    "longmapsto" => Symbol { unicode: 0x27FC, atom_type: AtomType::Relation }, // Unicode: 0x27FC, long rightwards arrow from bar
    "Longmapsfrom" => Symbol { unicode: 0x27FD, atom_type: AtomType::Relation }, // Unicode: 0x27FD, long leftwards double arrow from bar
    "Longmapsto" => Symbol { unicode: 0x27FE, atom_type: AtomType::Relation }, // Unicode: 0x27FE, long rightwards double arrow from bar
    "longrightsquigarrow" => Symbol { unicode: 0x27FF, atom_type: AtomType::Relation }, // Unicode: 0x27FF, long rightwards squiggle arrow
    "nvtwoheadrightarrow" => Symbol { unicode: 0x2900, atom_type: AtomType::Relation }, // Unicode: 0x2900, rightwards two-headed arrow with vertical stroke
    "nVtwoheadrightarrow" => Symbol { unicode: 0x2901, atom_type: AtomType::Relation }, // Unicode: 0x2901, rightwards two-headed arrow with double vertical stroke
    "nvLeftarrow" => Symbol { unicode: 0x2902, atom_type: AtomType::Relation }, // Unicode: 0x2902, leftwards double arrow with vertical stroke
    "nvRightarrow" => Symbol { unicode: 0x2903, atom_type: AtomType::Relation }, // Unicode: 0x2903, rightwards double arrow with vertical stroke
    "nvLeftrightarrow" => Symbol { unicode: 0x2904, atom_type: AtomType::Relation }, // Unicode: 0x2904, left right double arrow with vertical stroke
    "twoheadmapsto" => Symbol { unicode: 0x2905, atom_type: AtomType::Relation }, // Unicode: 0x2905, rightwards two-headed arrow from bar
    "Mapsfrom" => Symbol { unicode: 0x2906, atom_type: AtomType::Relation }, // Unicode: 0x2906, leftwards double arrow from bar
    "Mapsto" => Symbol { unicode: 0x2907, atom_type: AtomType::Relation }, // Unicode: 0x2907, rightwards double arrow from bar
    "downarrowbarred" => Symbol { unicode: 0x2908, atom_type: AtomType::Relation }, // Unicode: 0x2908, downwards arrow with horizontal stroke
    "uparrowbarred" => Symbol { unicode: 0x2909, atom_type: AtomType::Relation }, // Unicode: 0x2909, upwards arrow with horizontal stroke
    "Uuparrow" => Symbol { unicode: 0x290A, atom_type: AtomType::Relation }, // Unicode: 0x290A, upwards triple arrow
    "Ddownarrow" => Symbol { unicode: 0x290B, atom_type: AtomType::Relation }, // Unicode: 0x290B, downwards triple arrow
    "leftbkarrow" => Symbol { unicode: 0x290C, atom_type: AtomType::Relation }, // Unicode: 0x290C, leftwards double dash arrow
    "rightbkarrow" => Symbol { unicode: 0x290D, atom_type: AtomType::Relation }, // Unicode: 0x290D, rightwards double dash arrow
    "leftdbkarrow" => Symbol { unicode: 0x290E, atom_type: AtomType::Relation }, // Unicode: 0x290E, leftwards triple dash arrow
    "dbkarrow" => Symbol { unicode: 0x290F, atom_type: AtomType::Relation }, // Unicode: 0x290F, rightwards triple dash arrow
    "drbkarrow" => Symbol { unicode: 0x2910, atom_type: AtomType::Relation }, // Unicode: 0x2910, rightwards two-headed triple dash arrow
    "rightdotarrow" => Symbol { unicode: 0x2911, atom_type: AtomType::Relation }, // Unicode: 0x2911, rightwards arrow with dotted stem
    "baruparrow" => Symbol { unicode: 0x2912, atom_type: AtomType::Relation }, // Unicode: 0x2912, upwards arrow to bar
    "downarrowbar" => Symbol { unicode: 0x2913, atom_type: AtomType::Relation }, // Unicode: 0x2913, downwards arrow to bar
    "nvrightarrowtail" => Symbol { unicode: 0x2914, atom_type: AtomType::Relation }, // Unicode: 0x2914, rightwards arrow with tail with vertical stroke
    "nVrightarrowtail" => Symbol { unicode: 0x2915, atom_type: AtomType::Relation }, // Unicode: 0x2915, rightwards arrow with tail with double vertical stroke
    "twoheadrightarrowtail" => Symbol { unicode: 0x2916, atom_type: AtomType::Relation }, // Unicode: 0x2916, rightwards two-headed arrow with tail
    "nvtwoheadrightarrowtail" => Symbol { unicode: 0x2917, atom_type: AtomType::Relation }, // Unicode: 0x2917, rightwards two-headed arrow with tail with vertical stroke
    "nVtwoheadrightarrowtail" => Symbol { unicode: 0x2918, atom_type: AtomType::Relation }, // Unicode: 0x2918, rightwards two-headed arrow with tail with double vertical stroke
    "lefttail" => Symbol { unicode: 0x2919, atom_type: AtomType::Relation }, // Unicode: 0x2919, leftwards arrow-tail
    "righttail" => Symbol { unicode: 0x291A, atom_type: AtomType::Relation }, // Unicode: 0x291A, rightwards arrow-tail
    "leftdbltail" => Symbol { unicode: 0x291B, atom_type: AtomType::Relation }, // Unicode: 0x291B, leftwards double arrow-tail
    "rightdbltail" => Symbol { unicode: 0x291C, atom_type: AtomType::Relation }, // Unicode: 0x291C, rightwards double arrow-tail
    "diamondleftarrow" => Symbol { unicode: 0x291D, atom_type: AtomType::Relation }, // Unicode: 0x291D, leftwards arrow to black diamond
    "rightarrowdiamond" => Symbol { unicode: 0x291E, atom_type: AtomType::Relation }, // Unicode: 0x291E, rightwards arrow to black diamond
    "diamondleftarrowbar" => Symbol { unicode: 0x291F, atom_type: AtomType::Relation }, // Unicode: 0x291F, leftwards arrow from bar to black diamond
    "barrightarrowdiamond" => Symbol { unicode: 0x2920, atom_type: AtomType::Relation }, // Unicode: 0x2920, rightwards arrow from bar to black diamond
    "nwsearrow" => Symbol { unicode: 0x2921, atom_type: AtomType::Relation }, // Unicode: 0x2921, north west and south east arrow
    "neswarrow" => Symbol { unicode: 0x2922, atom_type: AtomType::Relation }, // Unicode: 0x2922, north east and south west arrow
    "hknwarrow" => Symbol { unicode: 0x2923, atom_type: AtomType::Relation }, // Unicode: 0x2923, north west arrow with hook
    "hknearrow" => Symbol { unicode: 0x2924, atom_type: AtomType::Relation }, // Unicode: 0x2924, north east arrow with hook
    "hksearrow" => Symbol { unicode: 0x2925, atom_type: AtomType::Relation }, // Unicode: 0x2925, south east arrow with hook
    "hkswarrow" => Symbol { unicode: 0x2926, atom_type: AtomType::Relation }, // Unicode: 0x2926, south west arrow with hook
    "tona" => Symbol { unicode: 0x2927, atom_type: AtomType::Relation }, // Unicode: 0x2927, north west arrow and north east arrow
    "toea" => Symbol { unicode: 0x2928, atom_type: AtomType::Relation }, // Unicode: 0x2928, north east arrow and south east arrow
    "tosa" => Symbol { unicode: 0x2929, atom_type: AtomType::Relation }, // Unicode: 0x2929, south east arrow and south west arrow
    "towa" => Symbol { unicode: 0x292A, atom_type: AtomType::Relation }, // Unicode: 0x292A, south west arrow and north west arrow
    "rdiagovfdiag" => Symbol { unicode: 0x292B, atom_type: AtomType::Alpha }, // Unicode: 0x292B, rising diagonal crossing falling diagonal
    "fdiagovrdiag" => Symbol { unicode: 0x292C, atom_type: AtomType::Alpha }, // Unicode: 0x292C, falling diagonal crossing rising diagonal
    "seovnearrow" => Symbol { unicode: 0x292D, atom_type: AtomType::Alpha }, // Unicode: 0x292D, south east arrow crossing north east arrow
    "neovsearrow" => Symbol { unicode: 0x292E, atom_type: AtomType::Alpha }, // Unicode: 0x292E, north east arrow crossing south east arrow
    "fdiagovnearrow" => Symbol { unicode: 0x292F, atom_type: AtomType::Alpha }, // Unicode: 0x292F, falling diagonal crossing north east arrow
    "rdiagovsearrow" => Symbol { unicode: 0x2930, atom_type: AtomType::Alpha }, // Unicode: 0x2930, rising diagonal crossing south east arrow
    "neovnwarrow" => Symbol { unicode: 0x2931, atom_type: AtomType::Alpha }, // Unicode: 0x2931, north east arrow crossing north west arrow
    "nwovnearrow" => Symbol { unicode: 0x2932, atom_type: AtomType::Alpha }, // Unicode: 0x2932, north west arrow crossing north east arrow
    "rightcurvedarrow" => Symbol { unicode: 0x2933, atom_type: AtomType::Relation }, // Unicode: 0x2933, wave arrow pointing directly right
    "uprightcurvearrow" => Symbol { unicode: 0x2934, atom_type: AtomType::Alpha }, // Unicode: 0x2934, arrow pointing rightwards then curving upwards
    "downrightcurvedarrow" => Symbol { unicode: 0x2935, atom_type: AtomType::Alpha }, // Unicode: 0x2935, arrow pointing rightwards then curving downwards
    "leftdowncurvedarrow" => Symbol { unicode: 0x2936, atom_type: AtomType::Relation }, // Unicode: 0x2936, arrow pointing downwards then curving leftwards
    "rightdowncurvedarrow" => Symbol { unicode: 0x2937, atom_type: AtomType::Relation }, // Unicode: 0x2937, arrow pointing downwards then curving rightwards
    "cwrightarcarrow" => Symbol { unicode: 0x2938, atom_type: AtomType::Relation }, // Unicode: 0x2938, right-side arc clockwise arrow
    "acwleftarcarrow" => Symbol { unicode: 0x2939, atom_type: AtomType::Relation }, // Unicode: 0x2939, left-side arc anticlockwise arrow
    "acwoverarcarrow" => Symbol { unicode: 0x293A, atom_type: AtomType::Relation }, // Unicode: 0x293A, top arc anticlockwise arrow
    "acwunderarcarrow" => Symbol { unicode: 0x293B, atom_type: AtomType::Relation }, // Unicode: 0x293B, bottom arc anticlockwise arrow
    "curvearrowrightminus" => Symbol { unicode: 0x293C, atom_type: AtomType::Relation }, // Unicode: 0x293C, top arc clockwise arrow with minus
    "curvearrowleftplus" => Symbol { unicode: 0x293D, atom_type: AtomType::Relation }, // Unicode: 0x293D, top arc anticlockwise arrow with plus
    "cwundercurvearrow" => Symbol { unicode: 0x293E, atom_type: AtomType::Relation }, // Unicode: 0x293E, lower right semicircular clockwise arrow
    "ccwundercurvearrow" => Symbol { unicode: 0x293F, atom_type: AtomType::Relation }, // Unicode: 0x293F, lower left semicircular anticlockwise arrow
    "acwcirclearrow" => Symbol { unicode: 0x2940, atom_type: AtomType::Relation }, // Unicode: 0x2940, anticlockwise closed circle arrow
    "cwcirclearrow" => Symbol { unicode: 0x2941, atom_type: AtomType::Relation }, // Unicode: 0x2941, clockwise closed circle arrow
    "rightarrowshortleftarrow" => Symbol { unicode: 0x2942, atom_type: AtomType::Relation }, // Unicode: 0x2942, rightwards arrow above short leftwards arrow
    "leftarrowshortrightarrow" => Symbol { unicode: 0x2943, atom_type: AtomType::Relation }, // Unicode: 0x2943, leftwards arrow above short rightwards arrow
    "shortrightarrowleftarrow" => Symbol { unicode: 0x2944, atom_type: AtomType::Relation }, // Unicode: 0x2944, short rightwards arrow above leftwards arrow
    "rightarrowplus" => Symbol { unicode: 0x2945, atom_type: AtomType::Relation }, // Unicode: 0x2945, rightwards arrow with plus below
    "leftarrowplus" => Symbol { unicode: 0x2946, atom_type: AtomType::Relation }, // Unicode: 0x2946, leftwards arrow with plus below
    "rightarrowx" => Symbol { unicode: 0x2947, atom_type: AtomType::Relation }, // Unicode: 0x2947, rightwards arrow through x
    "leftrightarrowcircle" => Symbol { unicode: 0x2948, atom_type: AtomType::Relation }, // Unicode: 0x2948, left right arrow through small circle
    "twoheaduparrowcircle" => Symbol { unicode: 0x2949, atom_type: AtomType::Relation }, // Unicode: 0x2949, upwards two-headed arrow from small circle
    "leftrightharpoonupdown" => Symbol { unicode: 0x294A, atom_type: AtomType::Relation }, // Unicode: 0x294A, left barb up right barb down harpoon
    "leftrightharpoondownup" => Symbol { unicode: 0x294B, atom_type: AtomType::Relation }, // Unicode: 0x294B, left barb down right barb up harpoon
    "updownharpoonrightleft" => Symbol { unicode: 0x294C, atom_type: AtomType::Relation }, // Unicode: 0x294C, up barb right down barb left harpoon
    "updownharpoonleftright" => Symbol { unicode: 0x294D, atom_type: AtomType::Relation }, // Unicode: 0x294D, up barb left down barb right harpoon
    "leftrightharpoonupup" => Symbol { unicode: 0x294E, atom_type: AtomType::Relation }, // Unicode: 0x294E, left barb up right barb up harpoon
    "updownharpoonrightright" => Symbol { unicode: 0x294F, atom_type: AtomType::Relation }, // Unicode: 0x294F, up barb right down barb right harpoon
    "leftrightharpoondowndown" => Symbol { unicode: 0x2950, atom_type: AtomType::Relation }, // Unicode: 0x2950, left barb down right barb down harpoon
    "updownharpoonleftleft" => Symbol { unicode: 0x2951, atom_type: AtomType::Relation }, // Unicode: 0x2951, up barb left down barb left harpoon
    "barleftharpoonup" => Symbol { unicode: 0x2952, atom_type: AtomType::Relation }, // Unicode: 0x2952, leftwards harpoon with barb up to bar
    "rightharpoonupbar" => Symbol { unicode: 0x2953, atom_type: AtomType::Relation }, // Unicode: 0x2953, rightwards harpoon with barb up to bar
    "barupharpoonright" => Symbol { unicode: 0x2954, atom_type: AtomType::Relation }, // Unicode: 0x2954, upwards harpoon with barb right to bar
    "downharpoonrightbar" => Symbol { unicode: 0x2955, atom_type: AtomType::Relation }, // Unicode: 0x2955, downwards harpoon with barb right to bar
    "barleftharpoondown" => Symbol { unicode: 0x2956, atom_type: AtomType::Relation }, // Unicode: 0x2956, leftwards harpoon with barb down to bar
    "rightharpoondownbar" => Symbol { unicode: 0x2957, atom_type: AtomType::Relation }, // Unicode: 0x2957, rightwards harpoon with barb down to bar
    "barupharpoonleft" => Symbol { unicode: 0x2958, atom_type: AtomType::Relation }, // Unicode: 0x2958, upwards harpoon with barb left to bar
    "downharpoonleftbar" => Symbol { unicode: 0x2959, atom_type: AtomType::Relation }, // Unicode: 0x2959, downwards harpoon with barb left to bar
    "leftharpoonupbar" => Symbol { unicode: 0x295A, atom_type: AtomType::Relation }, // Unicode: 0x295A, leftwards harpoon with barb up from bar
    "barrightharpoonup" => Symbol { unicode: 0x295B, atom_type: AtomType::Relation }, // Unicode: 0x295B, rightwards harpoon with barb up from bar
    "upharpoonrightbar" => Symbol { unicode: 0x295C, atom_type: AtomType::Relation }, // Unicode: 0x295C, upwards harpoon with barb right from bar
    "bardownharpoonright" => Symbol { unicode: 0x295D, atom_type: AtomType::Relation }, // Unicode: 0x295D, downwards harpoon with barb right from bar
    "leftharpoondownbar" => Symbol { unicode: 0x295E, atom_type: AtomType::Relation }, // Unicode: 0x295E, leftwards harpoon with barb down from bar
    "barrightharpoondown" => Symbol { unicode: 0x295F, atom_type: AtomType::Relation }, // Unicode: 0x295F, rightwards harpoon with barb down from bar
    "upharpoonleftbar" => Symbol { unicode: 0x2960, atom_type: AtomType::Relation }, // Unicode: 0x2960, upwards harpoon with barb left from bar
    "bardownharpoonleft" => Symbol { unicode: 0x2961, atom_type: AtomType::Relation }, // Unicode: 0x2961, downwards harpoon with barb left from bar
    "leftharpoonsupdown" => Symbol { unicode: 0x2962, atom_type: AtomType::Relation }, // Unicode: 0x2962, leftwards harpoon with barb up above leftwards harpoon with barb down
    "upharpoonsleftright" => Symbol { unicode: 0x2963, atom_type: AtomType::Relation }, // Unicode: 0x2963, upwards harpoon with barb left beside upwards harpoon with barb right
    "rightharpoonsupdown" => Symbol { unicode: 0x2964, atom_type: AtomType::Relation }, // Unicode: 0x2964, rightwards harpoon with barb up above rightwards harpoon with barb down
    "downharpoonsleftright" => Symbol { unicode: 0x2965, atom_type: AtomType::Relation }, // Unicode: 0x2965, downwards harpoon with barb left beside downwards harpoon with barb right
    "leftrightharpoonsup" => Symbol { unicode: 0x2966, atom_type: AtomType::Relation }, // Unicode: 0x2966, leftwards harpoon with barb up above rightwards harpoon with barb up
    "leftrightharpoonsdown" => Symbol { unicode: 0x2967, atom_type: AtomType::Relation }, // Unicode: 0x2967, leftwards harpoon with barb down above rightwards harpoon with barb down
    "rightleftharpoonsup" => Symbol { unicode: 0x2968, atom_type: AtomType::Relation }, // Unicode: 0x2968, rightwards harpoon with barb up above leftwards harpoon with barb up
    "rightleftharpoonsdown" => Symbol { unicode: 0x2969, atom_type: AtomType::Relation }, // Unicode: 0x2969, rightwards harpoon with barb down above leftwards harpoon with barb down
    "leftharpoonupdash" => Symbol { unicode: 0x296A, atom_type: AtomType::Relation }, // Unicode: 0x296A, leftwards harpoon with barb up above long dash
    "dashleftharpoondown" => Symbol { unicode: 0x296B, atom_type: AtomType::Relation }, // Unicode: 0x296B, leftwards harpoon with barb down below long dash
    "rightharpoonupdash" => Symbol { unicode: 0x296C, atom_type: AtomType::Relation }, // Unicode: 0x296C, rightwards harpoon with barb up above long dash
    "dashrightharpoondown" => Symbol { unicode: 0x296D, atom_type: AtomType::Relation }, // Unicode: 0x296D, rightwards harpoon with barb down below long dash
    "updownharpoonsleftright" => Symbol { unicode: 0x296E, atom_type: AtomType::Relation }, // Unicode: 0x296E, upwards harpoon with barb left beside downwards harpoon with barb right
    "downupharpoonsleftright" => Symbol { unicode: 0x296F, atom_type: AtomType::Relation }, // Unicode: 0x296F, downwards harpoon with barb left beside upwards harpoon with barb right
    "rightimply" => Symbol { unicode: 0x2970, atom_type: AtomType::Relation }, // Unicode: 0x2970, right double arrow with rounded head
    "equalrightarrow" => Symbol { unicode: 0x2971, atom_type: AtomType::Relation }, // Unicode: 0x2971, equals sign above rightwards arrow
    "similarrightarrow" => Symbol { unicode: 0x2972, atom_type: AtomType::Relation }, // Unicode: 0x2972, tilde operator above rightwards arrow
    "leftarrowsimilar" => Symbol { unicode: 0x2973, atom_type: AtomType::Relation }, // Unicode: 0x2973, leftwards arrow above tilde operator
    "rightarrowsimilar" => Symbol { unicode: 0x2974, atom_type: AtomType::Relation }, // Unicode: 0x2974, rightwards arrow above tilde operator
    "rightarrowapprox" => Symbol { unicode: 0x2975, atom_type: AtomType::Relation }, // Unicode: 0x2975, rightwards arrow above almost equal to
    "ltlarr" => Symbol { unicode: 0x2976, atom_type: AtomType::Relation }, // Unicode: 0x2976, less-than above leftwards arrow
    "leftarrowless" => Symbol { unicode: 0x2977, atom_type: AtomType::Relation }, // Unicode: 0x2977, leftwards arrow through less-than
    "gtrarr" => Symbol { unicode: 0x2978, atom_type: AtomType::Relation }, // Unicode: 0x2978, greater-than above rightwards arrow
    "subrarr" => Symbol { unicode: 0x2979, atom_type: AtomType::Relation }, // Unicode: 0x2979, subset above rightwards arrow
    "leftarrowsubset" => Symbol { unicode: 0x297A, atom_type: AtomType::Relation }, // Unicode: 0x297A, leftwards arrow through subset
    "suplarr" => Symbol { unicode: 0x297B, atom_type: AtomType::Relation }, // Unicode: 0x297B, superset above leftwards arrow
    "leftfishtail" => Symbol { unicode: 0x297C, atom_type: AtomType::Relation }, // Unicode: 0x297C, left fish tail
    "rightfishtail" => Symbol { unicode: 0x297D, atom_type: AtomType::Relation }, // Unicode: 0x297D, right fish tail
    "upfishtail" => Symbol { unicode: 0x297E, atom_type: AtomType::Relation }, // Unicode: 0x297E, up fish tail
    "downfishtail" => Symbol { unicode: 0x297F, atom_type: AtomType::Relation }, // Unicode: 0x297F, down fish tail
    "Vvert" => Symbol { unicode: 0x2980, atom_type: AtomType::Fence }, // Unicode: 0x2980, triple vertical bar delimiter
    "mdsmblkcircle" => Symbol { unicode: 0x2981, atom_type: AtomType::Alpha }, // Unicode: 0x2981, z notation spot
    "typecolon" => Symbol { unicode: 0x2982, atom_type: AtomType::Binary }, // Unicode: 0x2982, z notation type colon
    "lBrace" => Symbol { unicode: 0x2983, atom_type: AtomType::Open }, // Unicode: 0x2983, left white curly bracket
    "rBrace" => Symbol { unicode: 0x2984, atom_type: AtomType::Close }, // Unicode: 0x2984, right white curly bracket
    "lParen" => Symbol { unicode: 0x2985, atom_type: AtomType::Open }, // Unicode: 0x2985, left white parenthesis
    "rParen" => Symbol { unicode: 0x2986, atom_type: AtomType::Close }, // Unicode: 0x2986, right white parenthesis
    "llparenthesis" => Symbol { unicode: 0x2987, atom_type: AtomType::Open }, // Unicode: 0x2987, z notation left image bracket
    "rrparenthesis" => Symbol { unicode: 0x2988, atom_type: AtomType::Close }, // Unicode: 0x2988, z notation right image bracket
    "llangle" => Symbol { unicode: 0x2989, atom_type: AtomType::Open }, // Unicode: 0x2989, z notation left binding bracket
    "rrangle" => Symbol { unicode: 0x298A, atom_type: AtomType::Close }, // Unicode: 0x298A, z notation right binding bracket
    "lbrackubar" => Symbol { unicode: 0x298B, atom_type: AtomType::Open }, // Unicode: 0x298B, left square bracket with underbar
    "rbrackubar" => Symbol { unicode: 0x298C, atom_type: AtomType::Close }, // Unicode: 0x298C, right square bracket with underbar
    "lbrackultick" => Symbol { unicode: 0x298D, atom_type: AtomType::Open }, // Unicode: 0x298D, left square bracket with tick in top corner
    "rbracklrtick" => Symbol { unicode: 0x298E, atom_type: AtomType::Close }, // Unicode: 0x298E, right square bracket with tick in bottom corner
    "lbracklltick" => Symbol { unicode: 0x298F, atom_type: AtomType::Open }, // Unicode: 0x298F, left square bracket with tick in bottom corner
    "rbrackurtick" => Symbol { unicode: 0x2990, atom_type: AtomType::Close }, // Unicode: 0x2990, right square bracket with tick in top corner
    "langledot" => Symbol { unicode: 0x2991, atom_type: AtomType::Open }, // Unicode: 0x2991, left angle bracket with dot
    "rangledot" => Symbol { unicode: 0x2992, atom_type: AtomType::Close }, // Unicode: 0x2992, right angle bracket with dot
    "lparenless" => Symbol { unicode: 0x2993, atom_type: AtomType::Open }, // Unicode: 0x2993, left arc less-than bracket
    "rparengtr" => Symbol { unicode: 0x2994, atom_type: AtomType::Close }, // Unicode: 0x2994, right arc greater-than bracket
    "Lparengtr" => Symbol { unicode: 0x2995, atom_type: AtomType::Open }, // Unicode: 0x2995, double left arc greater-than bracket
    "Rparenless" => Symbol { unicode: 0x2996, atom_type: AtomType::Close }, // Unicode: 0x2996, double right arc less-than bracket
    "lblkbrbrak" => Symbol { unicode: 0x2997, atom_type: AtomType::Open }, // Unicode: 0x2997, left black tortoise shell bracket
    "rblkbrbrak" => Symbol { unicode: 0x2998, atom_type: AtomType::Close }, // Unicode: 0x2998, right black tortoise shell bracket
    "fourvdots" => Symbol { unicode: 0x2999, atom_type: AtomType::Alpha }, // Unicode: 0x2999, dotted fence
    "vzigzag" => Symbol { unicode: 0x299A, atom_type: AtomType::Alpha }, // Unicode: 0x299A, vertical zigzag line
    "measuredangleleft" => Symbol { unicode: 0x299B, atom_type: AtomType::Alpha }, // Unicode: 0x299B, measured angle opening left
    "rightanglesqr" => Symbol { unicode: 0x299C, atom_type: AtomType::Alpha }, // Unicode: 0x299C, right angle variant with square
    "rightanglemdot" => Symbol { unicode: 0x299D, atom_type: AtomType::Alpha }, // Unicode: 0x299D, measured right angle with dot
    "angles" => Symbol { unicode: 0x299E, atom_type: AtomType::Alpha }, // Unicode: 0x299E, angle with s inside
    "angdnr" => Symbol { unicode: 0x299F, atom_type: AtomType::Alpha }, // Unicode: 0x299F, acute angle
    "gtlpar" => Symbol { unicode: 0x29A0, atom_type: AtomType::Alpha }, // Unicode: 0x29A0, spherical angle opening left
    "sphericalangleup" => Symbol { unicode: 0x29A1, atom_type: AtomType::Alpha }, // Unicode: 0x29A1, spherical angle opening up
    "turnangle" => Symbol { unicode: 0x29A2, atom_type: AtomType::Alpha }, // Unicode: 0x29A2, turned angle
    "revangle" => Symbol { unicode: 0x29A3, atom_type: AtomType::Alpha }, // Unicode: 0x29A3, reversed angle
    "angleubar" => Symbol { unicode: 0x29A4, atom_type: AtomType::Alpha }, // Unicode: 0x29A4, angle with underbar
    "revangleubar" => Symbol { unicode: 0x29A5, atom_type: AtomType::Alpha }, // Unicode: 0x29A5, reversed angle with underbar
    "wideangledown" => Symbol { unicode: 0x29A6, atom_type: AtomType::Alpha }, // Unicode: 0x29A6, oblique angle opening up
    "wideangleup" => Symbol { unicode: 0x29A7, atom_type: AtomType::Alpha }, // Unicode: 0x29A7, oblique angle opening down
    "measanglerutone" => Symbol { unicode: 0x29A8, atom_type: AtomType::Alpha }, // Unicode: 0x29A8, measured angle with open arm ending in arrow pointing up and right
    "measanglelutonw" => Symbol { unicode: 0x29A9, atom_type: AtomType::Alpha }, // Unicode: 0x29A9, measured angle with open arm ending in arrow pointing up and left
    "measanglerdtose" => Symbol { unicode: 0x29AA, atom_type: AtomType::Alpha }, // Unicode: 0x29AA, measured angle with open arm ending in arrow pointing down and right
    "measangleldtosw" => Symbol { unicode: 0x29AB, atom_type: AtomType::Alpha }, // Unicode: 0x29AB, measured angle with open arm ending in arrow pointing down and left
    "measangleurtone" => Symbol { unicode: 0x29AC, atom_type: AtomType::Alpha }, // Unicode: 0x29AC, measured angle with open arm ending in arrow pointing right and up
    "measangleultonw" => Symbol { unicode: 0x29AD, atom_type: AtomType::Alpha }, // Unicode: 0x29AD, measured angle with open arm ending in arrow pointing left and up
    "measangledrtose" => Symbol { unicode: 0x29AE, atom_type: AtomType::Alpha }, // Unicode: 0x29AE, measured angle with open arm ending in arrow pointing right and down
    "measangledltosw" => Symbol { unicode: 0x29AF, atom_type: AtomType::Alpha }, // Unicode: 0x29AF, measured angle with open arm ending in arrow pointing left and down
    "revemptyset" => Symbol { unicode: 0x29B0, atom_type: AtomType::Alpha }, // Unicode: 0x29B0, reversed empty set
    "emptysetobar" => Symbol { unicode: 0x29B1, atom_type: AtomType::Alpha }, // Unicode: 0x29B1, empty set with overbar
    "emptysetocirc" => Symbol { unicode: 0x29B2, atom_type: AtomType::Alpha }, // Unicode: 0x29B2, empty set with small circle above
    "emptysetoarr" => Symbol { unicode: 0x29B3, atom_type: AtomType::Alpha }, // Unicode: 0x29B3, empty set with right arrow above
    "emptysetoarrl" => Symbol { unicode: 0x29B4, atom_type: AtomType::Alpha }, // Unicode: 0x29B4, empty set with left arrow above
    "circlehbar" => Symbol { unicode: 0x29B5, atom_type: AtomType::Binary }, // Unicode: 0x29B5, circle with horizontal bar
    "circledvert" => Symbol { unicode: 0x29B6, atom_type: AtomType::Binary }, // Unicode: 0x29B6, circled vertical bar
    "circledparallel" => Symbol { unicode: 0x29B7, atom_type: AtomType::Binary }, // Unicode: 0x29B7, circled parallel
    "obslash" => Symbol { unicode: 0x29B8, atom_type: AtomType::Binary }, // Unicode: 0x29B8, circled reverse solidus
    "operp" => Symbol { unicode: 0x29B9, atom_type: AtomType::Binary }, // Unicode: 0x29B9, circled perpendicular
    "obot" => Symbol { unicode: 0x29BA, atom_type: AtomType::Alpha }, // Unicode: 0x29BA, circle divided by horizontal bar and top half divided by vertical bar
    "olcross" => Symbol { unicode: 0x29BB, atom_type: AtomType::Alpha }, // Unicode: 0x29BB, circle with superimposed x
    "odotslashdot" => Symbol { unicode: 0x29BC, atom_type: AtomType::Alpha }, // Unicode: 0x29BC, circled anticlockwise-rotated division sign
    "uparrowoncircle" => Symbol { unicode: 0x29BD, atom_type: AtomType::Alpha }, // Unicode: 0x29BD, up arrow through circle
    "circledwhitebullet" => Symbol { unicode: 0x29BE, atom_type: AtomType::Alpha }, // Unicode: 0x29BE, circled white bullet
    "circledbullet" => Symbol { unicode: 0x29BF, atom_type: AtomType::Alpha }, // Unicode: 0x29BF, circled bullet
    "olessthan" => Symbol { unicode: 0x29C0, atom_type: AtomType::Binary }, // Unicode: 0x29C0, circled less-than
    "ogreaterthan" => Symbol { unicode: 0x29C1, atom_type: AtomType::Binary }, // Unicode: 0x29C1, circled greater-than
    "cirscir" => Symbol { unicode: 0x29C2, atom_type: AtomType::Alpha }, // Unicode: 0x29C2, circle with small circle to the right
    "cirE" => Symbol { unicode: 0x29C3, atom_type: AtomType::Alpha }, // Unicode: 0x29C3, circle with two horizontal strokes to the right
    "boxdiag" => Symbol { unicode: 0x29C4, atom_type: AtomType::Binary }, // Unicode: 0x29C4, squared rising diagonal slash
    "boxbslash" => Symbol { unicode: 0x29C5, atom_type: AtomType::Binary }, // Unicode: 0x29C5, squared falling diagonal slash
    "boxast" => Symbol { unicode: 0x29C6, atom_type: AtomType::Binary }, // Unicode: 0x29C6, squared asterisk
    "boxcircle" => Symbol { unicode: 0x29C7, atom_type: AtomType::Binary }, // Unicode: 0x29C7, squared small circle
    "boxbox" => Symbol { unicode: 0x29C8, atom_type: AtomType::Binary }, // Unicode: 0x29C8, squared square
    "boxonbox" => Symbol { unicode: 0x29C9, atom_type: AtomType::Alpha }, // Unicode: 0x29C9, two joined squares
    "triangleodot" => Symbol { unicode: 0x29CA, atom_type: AtomType::Alpha }, // Unicode: 0x29CA, triangle with dot above
    "triangleubar" => Symbol { unicode: 0x29CB, atom_type: AtomType::Alpha }, // Unicode: 0x29CB, triangle with underbar
    "triangles" => Symbol { unicode: 0x29CC, atom_type: AtomType::Alpha }, // Unicode: 0x29CC, s in triangle
    "triangleserifs" => Symbol { unicode: 0x29CD, atom_type: AtomType::Binary }, // Unicode: 0x29CD, triangle with serifs at bottom
    "rtriltri" => Symbol { unicode: 0x29CE, atom_type: AtomType::Relation }, // Unicode: 0x29CE, right triangle above left triangle
    "ltrivb" => Symbol { unicode: 0x29CF, atom_type: AtomType::Relation }, // Unicode: 0x29CF, left triangle beside vertical bar
    "vbrtri" => Symbol { unicode: 0x29D0, atom_type: AtomType::Relation }, // Unicode: 0x29D0, vertical bar beside right triangle
    "lfbowtie" => Symbol { unicode: 0x29D1, atom_type: AtomType::Relation }, // Unicode: 0x29D1, left black bowtie
    "rfbowtie" => Symbol { unicode: 0x29D2, atom_type: AtomType::Relation }, // Unicode: 0x29D2, right black bowtie
    "fbowtie" => Symbol { unicode: 0x29D3, atom_type: AtomType::Relation }, // Unicode: 0x29D3, black bowtie
    "lftimes" => Symbol { unicode: 0x29D4, atom_type: AtomType::Relation }, // Unicode: 0x29D4, left black times
    "rftimes" => Symbol { unicode: 0x29D5, atom_type: AtomType::Relation }, // Unicode: 0x29D5, right black times
    "hourglass" => Symbol { unicode: 0x29D6, atom_type: AtomType::Binary }, // Unicode: 0x29D6, white hourglass
    "blackhourglass" => Symbol { unicode: 0x29D7, atom_type: AtomType::Binary }, // Unicode: 0x29D7, black hourglass
    "lvzigzag" => Symbol { unicode: 0x29D8, atom_type: AtomType::Open }, // Unicode: 0x29D8, left wiggly fence
    "rvzigzag" => Symbol { unicode: 0x29D9, atom_type: AtomType::Close }, // Unicode: 0x29D9, right wiggly fence
    "Lvzigzag" => Symbol { unicode: 0x29DA, atom_type: AtomType::Open }, // Unicode: 0x29DA, left double wiggly fence
    "Rvzigzag" => Symbol { unicode: 0x29DB, atom_type: AtomType::Close }, // Unicode: 0x29DB, right double wiggly fence
    "iinfin" => Symbol { unicode: 0x29DC, atom_type: AtomType::Alpha }, // Unicode: 0x29DC, incomplete infinity
    "tieinfty" => Symbol { unicode: 0x29DD, atom_type: AtomType::Alpha }, // Unicode: 0x29DD, tie over infinity
    "nvinfty" => Symbol { unicode: 0x29DE, atom_type: AtomType::Alpha }, // Unicode: 0x29DE, infinity negated with vertical bar
    "dualmap" => Symbol { unicode: 0x29DF, atom_type: AtomType::Relation }, // Unicode: 0x29DF, double-ended multimap
    "laplac" => Symbol { unicode: 0x29E0, atom_type: AtomType::Alpha }, // Unicode: 0x29E0, square with contoured outline
    "lrtriangleeq" => Symbol { unicode: 0x29E1, atom_type: AtomType::Relation }, // Unicode: 0x29E1, increases as
    "shuffle" => Symbol { unicode: 0x29E2, atom_type: AtomType::Binary }, // Unicode: 0x29E2, shuffle product
    "eparsl" => Symbol { unicode: 0x29E3, atom_type: AtomType::Relation }, // Unicode: 0x29E3, equals sign and slanted parallel
    "smeparsl" => Symbol { unicode: 0x29E4, atom_type: AtomType::Relation }, // Unicode: 0x29E4, equals sign and slanted parallel with tilde above
    "eqvparsl" => Symbol { unicode: 0x29E5, atom_type: AtomType::Relation }, // Unicode: 0x29E5, identical to and slanted parallel
    "gleichstark" => Symbol { unicode: 0x29E6, atom_type: AtomType::Relation }, // Unicode: 0x29E6, gleich stark
    "thermod" => Symbol { unicode: 0x29E7, atom_type: AtomType::Alpha }, // Unicode: 0x29E7, thermodynamic
    "downtriangleleftblack" => Symbol { unicode: 0x29E8, atom_type: AtomType::Alpha }, // Unicode: 0x29E8, down-pointing triangle with left half black
    "downtrianglerightblack" => Symbol { unicode: 0x29E9, atom_type: AtomType::Alpha }, // Unicode: 0x29E9, down-pointing triangle with right half black
    "blackdiamonddownarrow" => Symbol { unicode: 0x29EA, atom_type: AtomType::Alpha }, // Unicode: 0x29EA, black diamond with down arrow
    "mdlgblklozenge" => Symbol { unicode: 0x29EB, atom_type: AtomType::Binary }, // Unicode: 0x29EB, black lozenge
    "circledownarrow" => Symbol { unicode: 0x29EC, atom_type: AtomType::Alpha }, // Unicode: 0x29EC, white circle with down arrow
    "blackcircledownarrow" => Symbol { unicode: 0x29ED, atom_type: AtomType::Alpha }, // Unicode: 0x29ED, black circle with down arrow
    "errbarsquare" => Symbol { unicode: 0x29EE, atom_type: AtomType::Alpha }, // Unicode: 0x29EE, error-barred white square
    "errbarblacksquare" => Symbol { unicode: 0x29EF, atom_type: AtomType::Alpha }, // Unicode: 0x29EF, error-barred black square
    "errbardiamond" => Symbol { unicode: 0x29F0, atom_type: AtomType::Alpha }, // Unicode: 0x29F0, error-barred white diamond
    "errbarblackdiamond" => Symbol { unicode: 0x29F1, atom_type: AtomType::Alpha }, // Unicode: 0x29F1, error-barred black diamond
    "errbarcircle" => Symbol { unicode: 0x29F2, atom_type: AtomType::Alpha }, // Unicode: 0x29F2, error-barred white circle
    "errbarblackcircle" => Symbol { unicode: 0x29F3, atom_type: AtomType::Alpha }, // Unicode: 0x29F3, error-barred black circle
    "ruledelayed" => Symbol { unicode: 0x29F4, atom_type: AtomType::Relation }, // Unicode: 0x29F4, rule-delayed
    "setminus" => Symbol { unicode: 0x29F5, atom_type: AtomType::Binary }, // Unicode: 0x29F5, reverse solidus operator
    "dsol" => Symbol { unicode: 0x29F6, atom_type: AtomType::Binary }, // Unicode: 0x29F6, solidus with overbar
    "rsolbar" => Symbol { unicode: 0x29F7, atom_type: AtomType::Binary }, // Unicode: 0x29F7, reverse solidus with horizontal stroke
    "xsol" => Symbol { unicode: 0x29F8, atom_type: AtomType::Operator(false) }, // Unicode: 0x29F8, big solidus
    "xbsol" => Symbol { unicode: 0x29F9, atom_type: AtomType::Operator(false) }, // Unicode: 0x29F9, big reverse solidus
    "doubleplus" => Symbol { unicode: 0x29FA, atom_type: AtomType::Binary }, // Unicode: 0x29FA, double plus
    "tripleplus" => Symbol { unicode: 0x29FB, atom_type: AtomType::Binary }, // Unicode: 0x29FB, triple plus
    "lcurvyangle" => Symbol { unicode: 0x29FC, atom_type: AtomType::Open }, // Unicode: 0x29FC, left pointing curved angle bracket
    "rcurvyangle" => Symbol { unicode: 0x29FD, atom_type: AtomType::Close }, // Unicode: 0x29FD, right pointing curved angle bracket
    "tplus" => Symbol { unicode: 0x29FE, atom_type: AtomType::Binary }, // Unicode: 0x29FE, tiny
    "tminus" => Symbol { unicode: 0x29FF, atom_type: AtomType::Binary }, // Unicode: 0x29FF, miny
    "bigodot" => Symbol { unicode: 0x2A00, atom_type: AtomType::Operator(true) }, // Unicode: 0x2A00, n-ary circled dot operator
    "bigoplus" => Symbol { unicode: 0x2A01, atom_type: AtomType::Operator(true) }, // Unicode: 0x2A01, n-ary circled plus operator
    "bigotimes" => Symbol { unicode: 0x2A02, atom_type: AtomType::Operator(true) }, // Unicode: 0x2A02, n-ary circled times operator
    "bigcupdot" => Symbol { unicode: 0x2A03, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A03, n-ary union operator with dot
    "biguplus" => Symbol { unicode: 0x2A04, atom_type: AtomType::Operator(true) }, // Unicode: 0x2A04, n-ary union operator with plus
    "bigsqcap" => Symbol { unicode: 0x2A05, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A05, n-ary square intersection operator
    "bigsqcup" => Symbol { unicode: 0x2A06, atom_type: AtomType::Operator(true) }, // Unicode: 0x2A06, n-ary square union operator
    "conjquant" => Symbol { unicode: 0x2A07, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A07, two logical and operator
    "disjquant" => Symbol { unicode: 0x2A08, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A08, two logical or operator
    "bigtimes" => Symbol { unicode: 0x2A09, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A09, n-ary times operator
    "modtwosum" => Symbol { unicode: 0x2A0A, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A0A, modulo two sum
    "sumint" => Symbol { unicode: 0x2A0B, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A0B, summation with integral
    "iiiint" => Symbol { unicode: 0x2A0C, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A0C, quadruple integral operator
    "intbar" => Symbol { unicode: 0x2A0D, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A0D, finite part integral
    "intBar" => Symbol { unicode: 0x2A0E, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A0E, integral with double stroke
    "fint" => Symbol { unicode: 0x2A0F, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A0F, integral average with slash
    "cirfnint" => Symbol { unicode: 0x2A10, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A10, circulation function
    "awint" => Symbol { unicode: 0x2A11, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A11, anticlockwise integration
    "rppolint" => Symbol { unicode: 0x2A12, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A12, line integration with rectangular path around pole
    "scpolint" => Symbol { unicode: 0x2A13, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A13, line integration with semicircular path around pole
    "npolint" => Symbol { unicode: 0x2A14, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A14, line integration not including the pole
    "pointint" => Symbol { unicode: 0x2A15, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A15, integral around a point operator
    "sqint" => Symbol { unicode: 0x2A16, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A16, quaternion integral operator
    "intlarhk" => Symbol { unicode: 0x2A17, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A17, integral with leftwards arrow with hook
    "intx" => Symbol { unicode: 0x2A18, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A18, integral with times sign
    "intcap" => Symbol { unicode: 0x2A19, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A19, integral with intersection
    "intcup" => Symbol { unicode: 0x2A1A, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A1A, integral with union
    "upint" => Symbol { unicode: 0x2A1B, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A1B, integral with overbar
    "lowint" => Symbol { unicode: 0x2A1C, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A1C, integral with underbar
    "Join" => Symbol { unicode: 0x2A1D, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A1D, join
    "bigtriangleleft" => Symbol { unicode: 0x2A1E, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A1E, large left triangle operator
    "zcmp" => Symbol { unicode: 0x2A1F, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A1F, z notation schema composition
    "zpipe" => Symbol { unicode: 0x2A20, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A20, z notation schema piping
    "zproject" => Symbol { unicode: 0x2A21, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A21, z notation schema projection
    "ringplus" => Symbol { unicode: 0x2A22, atom_type: AtomType::Binary }, // Unicode: 0x2A22, plus sign with small circle above
    "plushat" => Symbol { unicode: 0x2A23, atom_type: AtomType::Binary }, // Unicode: 0x2A23, plus sign with circumflex accent above
    "simplus" => Symbol { unicode: 0x2A24, atom_type: AtomType::Binary }, // Unicode: 0x2A24, plus sign with tilde above
    "plusdot" => Symbol { unicode: 0x2A25, atom_type: AtomType::Binary }, // Unicode: 0x2A25, plus sign with dot below
    "plussim" => Symbol { unicode: 0x2A26, atom_type: AtomType::Binary }, // Unicode: 0x2A26, plus sign with tilde below
    "plussubtwo" => Symbol { unicode: 0x2A27, atom_type: AtomType::Binary }, // Unicode: 0x2A27, plus sign with subscript two
    "plustrif" => Symbol { unicode: 0x2A28, atom_type: AtomType::Binary }, // Unicode: 0x2A28, plus sign with black triangle
    "commaminus" => Symbol { unicode: 0x2A29, atom_type: AtomType::Binary }, // Unicode: 0x2A29, minus sign with comma above
    "minusdot" => Symbol { unicode: 0x2A2A, atom_type: AtomType::Binary }, // Unicode: 0x2A2A, minus sign with dot below
    "minusfdots" => Symbol { unicode: 0x2A2B, atom_type: AtomType::Binary }, // Unicode: 0x2A2B, minus sign with falling dots
    "minusrdots" => Symbol { unicode: 0x2A2C, atom_type: AtomType::Binary }, // Unicode: 0x2A2C, minus sign with rising dots
    "opluslhrim" => Symbol { unicode: 0x2A2D, atom_type: AtomType::Binary }, // Unicode: 0x2A2D, plus sign in left half circle
    "oplusrhrim" => Symbol { unicode: 0x2A2E, atom_type: AtomType::Binary }, // Unicode: 0x2A2E, plus sign in right half circle
    "vectimes" => Symbol { unicode: 0x2A2F, atom_type: AtomType::Binary }, // Unicode: 0x2A2F, vector or cross product
    "dottimes" => Symbol { unicode: 0x2A30, atom_type: AtomType::Binary }, // Unicode: 0x2A30, multiplication sign with dot above
    "timesbar" => Symbol { unicode: 0x2A31, atom_type: AtomType::Binary }, // Unicode: 0x2A31, multiplication sign with underbar
    "btimes" => Symbol { unicode: 0x2A32, atom_type: AtomType::Binary }, // Unicode: 0x2A32, semidirect product with bottom closed
    "smashtimes" => Symbol { unicode: 0x2A33, atom_type: AtomType::Binary }, // Unicode: 0x2A33, smash product
    "otimeslhrim" => Symbol { unicode: 0x2A34, atom_type: AtomType::Binary }, // Unicode: 0x2A34, multiplication sign in left half circle
    "otimesrhrim" => Symbol { unicode: 0x2A35, atom_type: AtomType::Binary }, // Unicode: 0x2A35, multiplication sign in right half circle
    "otimeshat" => Symbol { unicode: 0x2A36, atom_type: AtomType::Binary }, // Unicode: 0x2A36, circled multiplication sign with circumflex accent
    "Otimes" => Symbol { unicode: 0x2A37, atom_type: AtomType::Binary }, // Unicode: 0x2A37, multiplication sign in double circle
    "odiv" => Symbol { unicode: 0x2A38, atom_type: AtomType::Binary }, // Unicode: 0x2A38, circled division sign
    "triangleplus" => Symbol { unicode: 0x2A39, atom_type: AtomType::Binary }, // Unicode: 0x2A39, plus sign in triangle
    "triangleminus" => Symbol { unicode: 0x2A3A, atom_type: AtomType::Binary }, // Unicode: 0x2A3A, minus sign in triangle
    "triangletimes" => Symbol { unicode: 0x2A3B, atom_type: AtomType::Binary }, // Unicode: 0x2A3B, multiplication sign in triangle
    "intprod" => Symbol { unicode: 0x2A3C, atom_type: AtomType::Binary }, // Unicode: 0x2A3C, interior product
    "intprodr" => Symbol { unicode: 0x2A3D, atom_type: AtomType::Binary }, // Unicode: 0x2A3D, righthand interior product
    "fcmp" => Symbol { unicode: 0x2A3E, atom_type: AtomType::Binary }, // Unicode: 0x2A3E, z notation relational composition
    "amalg" => Symbol { unicode: 0x2A3F, atom_type: AtomType::Binary }, // Unicode: 0x2A3F, amalgamation or coproduct
    "capdot" => Symbol { unicode: 0x2A40, atom_type: AtomType::Binary }, // Unicode: 0x2A40, intersection with dot
    "uminus" => Symbol { unicode: 0x2A41, atom_type: AtomType::Binary }, // Unicode: 0x2A41, union with minus sign
    "barcup" => Symbol { unicode: 0x2A42, atom_type: AtomType::Binary }, // Unicode: 0x2A42, union with overbar
    "barcap" => Symbol { unicode: 0x2A43, atom_type: AtomType::Binary }, // Unicode: 0x2A43, intersection with overbar
    "capwedge" => Symbol { unicode: 0x2A44, atom_type: AtomType::Binary }, // Unicode: 0x2A44, intersection with logical and
    "cupvee" => Symbol { unicode: 0x2A45, atom_type: AtomType::Binary }, // Unicode: 0x2A45, union with logical or
    "cupovercap" => Symbol { unicode: 0x2A46, atom_type: AtomType::Binary }, // Unicode: 0x2A46, union above intersection
    "capovercup" => Symbol { unicode: 0x2A47, atom_type: AtomType::Binary }, // Unicode: 0x2A47, intersection above union
    "cupbarcap" => Symbol { unicode: 0x2A48, atom_type: AtomType::Binary }, // Unicode: 0x2A48, union above bar above intersection
    "capbarcup" => Symbol { unicode: 0x2A49, atom_type: AtomType::Binary }, // Unicode: 0x2A49, intersection above bar above union
    "twocups" => Symbol { unicode: 0x2A4A, atom_type: AtomType::Binary }, // Unicode: 0x2A4A, union beside and joined with union
    "twocaps" => Symbol { unicode: 0x2A4B, atom_type: AtomType::Binary }, // Unicode: 0x2A4B, intersection beside and joined with intersection
    "closedvarcup" => Symbol { unicode: 0x2A4C, atom_type: AtomType::Binary }, // Unicode: 0x2A4C, closed union with serifs
    "closedvarcap" => Symbol { unicode: 0x2A4D, atom_type: AtomType::Binary }, // Unicode: 0x2A4D, closed intersection with serifs
    "Sqcap" => Symbol { unicode: 0x2A4E, atom_type: AtomType::Binary }, // Unicode: 0x2A4E, double square intersection
    "Sqcup" => Symbol { unicode: 0x2A4F, atom_type: AtomType::Binary }, // Unicode: 0x2A4F, double square union
    "closedvarcupsmashprod" => Symbol { unicode: 0x2A50, atom_type: AtomType::Binary }, // Unicode: 0x2A50, closed union with serifs and smash product
    "wedgeodot" => Symbol { unicode: 0x2A51, atom_type: AtomType::Binary }, // Unicode: 0x2A51, logical and with dot above
    "veeodot" => Symbol { unicode: 0x2A52, atom_type: AtomType::Binary }, // Unicode: 0x2A52, logical or with dot above
    "Wedge" => Symbol { unicode: 0x2A53, atom_type: AtomType::Binary }, // Unicode: 0x2A53, double logical and
    "Vee" => Symbol { unicode: 0x2A54, atom_type: AtomType::Binary }, // Unicode: 0x2A54, double logical or
    "wedgeonwedge" => Symbol { unicode: 0x2A55, atom_type: AtomType::Binary }, // Unicode: 0x2A55, two intersecting logical and
    "veeonvee" => Symbol { unicode: 0x2A56, atom_type: AtomType::Binary }, // Unicode: 0x2A56, two intersecting logical or
    "bigslopedvee" => Symbol { unicode: 0x2A57, atom_type: AtomType::Binary }, // Unicode: 0x2A57, sloping large or
    "bigslopedwedge" => Symbol { unicode: 0x2A58, atom_type: AtomType::Binary }, // Unicode: 0x2A58, sloping large and
    "veeonwedge" => Symbol { unicode: 0x2A59, atom_type: AtomType::Relation }, // Unicode: 0x2A59, logical or overlapping logical and
    "wedgemidvert" => Symbol { unicode: 0x2A5A, atom_type: AtomType::Binary }, // Unicode: 0x2A5A, logical and with middle stem
    "veemidvert" => Symbol { unicode: 0x2A5B, atom_type: AtomType::Binary }, // Unicode: 0x2A5B, logical or with middle stem
    "midbarwedge" => Symbol { unicode: 0x2A5C, atom_type: AtomType::Binary }, // Unicode: 0x2A5C, ogical and with horizontal dash
    "midbarvee" => Symbol { unicode: 0x2A5D, atom_type: AtomType::Binary }, // Unicode: 0x2A5D, logical or with horizontal dash
    "doublebarwedge" => Symbol { unicode: 0x2A5E, atom_type: AtomType::Binary }, // Unicode: 0x2A5E, logical and with double overbar
    "wedgebar" => Symbol { unicode: 0x2A5F, atom_type: AtomType::Binary }, // Unicode: 0x2A5F, logical and with underbar
    "wedgedoublebar" => Symbol { unicode: 0x2A60, atom_type: AtomType::Binary }, // Unicode: 0x2A60, logical and with double underbar
    "varveebar" => Symbol { unicode: 0x2A61, atom_type: AtomType::Binary }, // Unicode: 0x2A61, small vee with underbar
    "doublebarvee" => Symbol { unicode: 0x2A62, atom_type: AtomType::Binary }, // Unicode: 0x2A62, logical or with double overbar
    "veedoublebar" => Symbol { unicode: 0x2A63, atom_type: AtomType::Binary }, // Unicode: 0x2A63, logical or with double underbar
    "dsub" => Symbol { unicode: 0x2A64, atom_type: AtomType::Binary }, // Unicode: 0x2A64, z notation domain antirestriction
    "rsub" => Symbol { unicode: 0x2A65, atom_type: AtomType::Binary }, // Unicode: 0x2A65, z notation range antirestriction
    "eqdot" => Symbol { unicode: 0x2A66, atom_type: AtomType::Relation }, // Unicode: 0x2A66, equals sign with dot below
    "dotequiv" => Symbol { unicode: 0x2A67, atom_type: AtomType::Relation }, // Unicode: 0x2A67, identical with dot above
    "equivVert" => Symbol { unicode: 0x2A68, atom_type: AtomType::Relation }, // Unicode: 0x2A68, triple horizontal bar with double vertical stroke
    "equivVvert" => Symbol { unicode: 0x2A69, atom_type: AtomType::Relation }, // Unicode: 0x2A69, triple horizontal bar with triple vertical stroke
    "dotsim" => Symbol { unicode: 0x2A6A, atom_type: AtomType::Relation }, // Unicode: 0x2A6A, tilde operator with dot above
    "simrdots" => Symbol { unicode: 0x2A6B, atom_type: AtomType::Relation }, // Unicode: 0x2A6B, tilde operator with rising dots
    "simminussim" => Symbol { unicode: 0x2A6C, atom_type: AtomType::Relation }, // Unicode: 0x2A6C, similar minus similar
    "congdot" => Symbol { unicode: 0x2A6D, atom_type: AtomType::Relation }, // Unicode: 0x2A6D, congruent with dot above
    "asteq" => Symbol { unicode: 0x2A6E, atom_type: AtomType::Relation }, // Unicode: 0x2A6E, equals with asterisk
    "hatapprox" => Symbol { unicode: 0x2A6F, atom_type: AtomType::Relation }, // Unicode: 0x2A6F, almost equal to with circumflex accent
    "approxeqq" => Symbol { unicode: 0x2A70, atom_type: AtomType::Relation }, // Unicode: 0x2A70, approximately equal or equal to
    "eqqplus" => Symbol { unicode: 0x2A71, atom_type: AtomType::Binary }, // Unicode: 0x2A71, equals sign above plus sign
    "pluseqq" => Symbol { unicode: 0x2A72, atom_type: AtomType::Binary }, // Unicode: 0x2A72, plus sign above equals sign
    "eqqsim" => Symbol { unicode: 0x2A73, atom_type: AtomType::Relation }, // Unicode: 0x2A73, equals sign above tilde operator
    "Coloneq" => Symbol { unicode: 0x2A74, atom_type: AtomType::Relation }, // Unicode: 0x2A74, double colon equal
    "eqeq" => Symbol { unicode: 0x2A75, atom_type: AtomType::Relation }, // Unicode: 0x2A75, two consecutive equals signs
    "eqeqeq" => Symbol { unicode: 0x2A76, atom_type: AtomType::Relation }, // Unicode: 0x2A76, three consecutive equals signs
    "ddotseq" => Symbol { unicode: 0x2A77, atom_type: AtomType::Relation }, // Unicode: 0x2A77, equals sign with two dots above and two dots below
    "equivDD" => Symbol { unicode: 0x2A78, atom_type: AtomType::Relation }, // Unicode: 0x2A78, equivalent with four dots above
    "ltcir" => Symbol { unicode: 0x2A79, atom_type: AtomType::Relation }, // Unicode: 0x2A79, less-than with circle inside
    "gtcir" => Symbol { unicode: 0x2A7A, atom_type: AtomType::Relation }, // Unicode: 0x2A7A, greater-than with circle inside
    "ltquest" => Symbol { unicode: 0x2A7B, atom_type: AtomType::Relation }, // Unicode: 0x2A7B, less-than with question mark above
    "gtquest" => Symbol { unicode: 0x2A7C, atom_type: AtomType::Relation }, // Unicode: 0x2A7C, greater-than with question mark above
    "leqslant" => Symbol { unicode: 0x2A7D, atom_type: AtomType::Relation }, // Unicode: 0x2A7D, less-than or slanted equal to
    "geqslant" => Symbol { unicode: 0x2A7E, atom_type: AtomType::Relation }, // Unicode: 0x2A7E, greater-than or slanted equal to
    "lesdot" => Symbol { unicode: 0x2A7F, atom_type: AtomType::Relation }, // Unicode: 0x2A7F, less-than or slanted equal to with dot inside
    "gesdot" => Symbol { unicode: 0x2A80, atom_type: AtomType::Relation }, // Unicode: 0x2A80, greater-than or slanted equal to with dot inside
    "lesdoto" => Symbol { unicode: 0x2A81, atom_type: AtomType::Relation }, // Unicode: 0x2A81, less-than or slanted equal to with dot above
    "gesdoto" => Symbol { unicode: 0x2A82, atom_type: AtomType::Relation }, // Unicode: 0x2A82, greater-than or slanted equal to with dot above
    "lesdotor" => Symbol { unicode: 0x2A83, atom_type: AtomType::Relation }, // Unicode: 0x2A83, less-than or slanted equal to with dot above right
    "gesdotol" => Symbol { unicode: 0x2A84, atom_type: AtomType::Relation }, // Unicode: 0x2A84, greater-than or slanted equal to with dot above left
    "lessapprox" => Symbol { unicode: 0x2A85, atom_type: AtomType::Relation }, // Unicode: 0x2A85, less-than or approximate
    "gtrapprox" => Symbol { unicode: 0x2A86, atom_type: AtomType::Relation }, // Unicode: 0x2A86, greater-than or approximate
    "lneq" => Symbol { unicode: 0x2A87, atom_type: AtomType::Relation }, // Unicode: 0x2A87, less-than and single-line not equal to
    "gneq" => Symbol { unicode: 0x2A88, atom_type: AtomType::Relation }, // Unicode: 0x2A88, greater-than and single-line not equal to
    "lnapprox" => Symbol { unicode: 0x2A89, atom_type: AtomType::Relation }, // Unicode: 0x2A89, less-than and not approximate
    "gnapprox" => Symbol { unicode: 0x2A8A, atom_type: AtomType::Relation }, // Unicode: 0x2A8A, greater-than and not approximate
    "lesseqqgtr" => Symbol { unicode: 0x2A8B, atom_type: AtomType::Relation }, // Unicode: 0x2A8B, less-than above double-line equal above greater-than
    "gtreqqless" => Symbol { unicode: 0x2A8C, atom_type: AtomType::Relation }, // Unicode: 0x2A8C, greater-than above double-line equal above less-than
    "lsime" => Symbol { unicode: 0x2A8D, atom_type: AtomType::Relation }, // Unicode: 0x2A8D, less-than above similar or equal
    "gsime" => Symbol { unicode: 0x2A8E, atom_type: AtomType::Relation }, // Unicode: 0x2A8E, greater-than above similar or equal
    "lsimg" => Symbol { unicode: 0x2A8F, atom_type: AtomType::Relation }, // Unicode: 0x2A8F, less-than above similar above greater-than
    "gsiml" => Symbol { unicode: 0x2A90, atom_type: AtomType::Relation }, // Unicode: 0x2A90, greater-than above similar above less-than
    "lgE" => Symbol { unicode: 0x2A91, atom_type: AtomType::Relation }, // Unicode: 0x2A91, less-than above greater-than above double-line equal
    "glE" => Symbol { unicode: 0x2A92, atom_type: AtomType::Relation }, // Unicode: 0x2A92, greater-than above less-than above double-line equal
    "lesges" => Symbol { unicode: 0x2A93, atom_type: AtomType::Relation }, // Unicode: 0x2A93, less-than above slanted equal above greater-than above slanted equal
    "gesles" => Symbol { unicode: 0x2A94, atom_type: AtomType::Relation }, // Unicode: 0x2A94, greater-than above slanted equal above less-than above slanted equal
    "eqslantless" => Symbol { unicode: 0x2A95, atom_type: AtomType::Relation }, // Unicode: 0x2A95, slanted equal to or less-than
    "eqslantgtr" => Symbol { unicode: 0x2A96, atom_type: AtomType::Relation }, // Unicode: 0x2A96, slanted equal to or greater-than
    "elsdot" => Symbol { unicode: 0x2A97, atom_type: AtomType::Relation }, // Unicode: 0x2A97, slanted equal to or less-than with dot inside
    "egsdot" => Symbol { unicode: 0x2A98, atom_type: AtomType::Relation }, // Unicode: 0x2A98, slanted equal to or greater-than with dot inside
    "eqqless" => Symbol { unicode: 0x2A99, atom_type: AtomType::Relation }, // Unicode: 0x2A99, double-line equal to or less-than
    "eqqgtr" => Symbol { unicode: 0x2A9A, atom_type: AtomType::Relation }, // Unicode: 0x2A9A, double-line equal to or greater-than
    "eqqslantless" => Symbol { unicode: 0x2A9B, atom_type: AtomType::Relation }, // Unicode: 0x2A9B, double-line slanted equal to or less-than
    "eqqslantgtr" => Symbol { unicode: 0x2A9C, atom_type: AtomType::Relation }, // Unicode: 0x2A9C, double-line slanted equal to or greater-than
    "simless" => Symbol { unicode: 0x2A9D, atom_type: AtomType::Relation }, // Unicode: 0x2A9D, similar or less-than
    "simgtr" => Symbol { unicode: 0x2A9E, atom_type: AtomType::Relation }, // Unicode: 0x2A9E, similar or greater-than
    "simlE" => Symbol { unicode: 0x2A9F, atom_type: AtomType::Relation }, // Unicode: 0x2A9F, similar above less-than above equals sign
    "simgE" => Symbol { unicode: 0x2AA0, atom_type: AtomType::Relation }, // Unicode: 0x2AA0, similar above greater-than above equals sign
    "Lt" => Symbol { unicode: 0x2AA1, atom_type: AtomType::Relation }, // Unicode: 0x2AA1, double nested less-than
    "Gt" => Symbol { unicode: 0x2AA2, atom_type: AtomType::Relation }, // Unicode: 0x2AA2, double nested greater-than
    "partialmeetcontraction" => Symbol { unicode: 0x2AA3, atom_type: AtomType::Relation }, // Unicode: 0x2AA3, double less-than with underbar
    "glj" => Symbol { unicode: 0x2AA4, atom_type: AtomType::Relation }, // Unicode: 0x2AA4, greater-than overlapping less-than
    "gla" => Symbol { unicode: 0x2AA5, atom_type: AtomType::Relation }, // Unicode: 0x2AA5, greater-than beside less-than
    "ltcc" => Symbol { unicode: 0x2AA6, atom_type: AtomType::Relation }, // Unicode: 0x2AA6, less-than closed by curve
    "gtcc" => Symbol { unicode: 0x2AA7, atom_type: AtomType::Relation }, // Unicode: 0x2AA7, greater-than closed by curve
    "lescc" => Symbol { unicode: 0x2AA8, atom_type: AtomType::Relation }, // Unicode: 0x2AA8, less-than closed by curve above slanted equal
    "gescc" => Symbol { unicode: 0x2AA9, atom_type: AtomType::Relation }, // Unicode: 0x2AA9, greater-than closed by curve above slanted equal
    "smt" => Symbol { unicode: 0x2AAA, atom_type: AtomType::Relation }, // Unicode: 0x2AAA, smaller than
    "lat" => Symbol { unicode: 0x2AAB, atom_type: AtomType::Relation }, // Unicode: 0x2AAB, larger than
    "smte" => Symbol { unicode: 0x2AAC, atom_type: AtomType::Relation }, // Unicode: 0x2AAC, smaller than or equal to
    "late" => Symbol { unicode: 0x2AAD, atom_type: AtomType::Relation }, // Unicode: 0x2AAD, larger than or equal to
    "bumpeqq" => Symbol { unicode: 0x2AAE, atom_type: AtomType::Relation }, // Unicode: 0x2AAE, equals sign with bumpy above
    "preceq" => Symbol { unicode: 0x2AAF, atom_type: AtomType::Relation }, // Unicode: 0x2AAF, precedes above single-line equals sign
    "succeq" => Symbol { unicode: 0x2AB0, atom_type: AtomType::Relation }, // Unicode: 0x2AB0, succeeds above single-line equals sign
    "precneq" => Symbol { unicode: 0x2AB1, atom_type: AtomType::Relation }, // Unicode: 0x2AB1, precedes above single-line not equal to
    "succneq" => Symbol { unicode: 0x2AB2, atom_type: AtomType::Relation }, // Unicode: 0x2AB2, succeeds above single-line not equal to
    "preceqq" => Symbol { unicode: 0x2AB3, atom_type: AtomType::Relation }, // Unicode: 0x2AB3, precedes above equals sign
    "succeqq" => Symbol { unicode: 0x2AB4, atom_type: AtomType::Relation }, // Unicode: 0x2AB4, succeeds above equals sign
    "precneqq" => Symbol { unicode: 0x2AB5, atom_type: AtomType::Relation }, // Unicode: 0x2AB5, precedes above not equal to
    "succneqq" => Symbol { unicode: 0x2AB6, atom_type: AtomType::Relation }, // Unicode: 0x2AB6, succeeds above not equal to
    "precapprox" => Symbol { unicode: 0x2AB7, atom_type: AtomType::Relation }, // Unicode: 0x2AB7, precedes above almost equal to
    "succapprox" => Symbol { unicode: 0x2AB8, atom_type: AtomType::Relation }, // Unicode: 0x2AB8, succeeds above almost equal to
    "precnapprox" => Symbol { unicode: 0x2AB9, atom_type: AtomType::Relation }, // Unicode: 0x2AB9, precedes above not almost equal to
    "succnapprox" => Symbol { unicode: 0x2ABA, atom_type: AtomType::Relation }, // Unicode: 0x2ABA, succeeds above not almost equal to
    "Prec" => Symbol { unicode: 0x2ABB, atom_type: AtomType::Relation }, // Unicode: 0x2ABB, double precedes
    "Succ" => Symbol { unicode: 0x2ABC, atom_type: AtomType::Relation }, // Unicode: 0x2ABC, double succeeds
    "subsetdot" => Symbol { unicode: 0x2ABD, atom_type: AtomType::Relation }, // Unicode: 0x2ABD, subset with dot
    "supsetdot" => Symbol { unicode: 0x2ABE, atom_type: AtomType::Relation }, // Unicode: 0x2ABE, superset with dot
    "subsetplus" => Symbol { unicode: 0x2ABF, atom_type: AtomType::Relation }, // Unicode: 0x2ABF, subset with plus sign below
    "supsetplus" => Symbol { unicode: 0x2AC0, atom_type: AtomType::Relation }, // Unicode: 0x2AC0, superset with plus sign below
    "submult" => Symbol { unicode: 0x2AC1, atom_type: AtomType::Relation }, // Unicode: 0x2AC1, subset with multiplication sign below
    "supmult" => Symbol { unicode: 0x2AC2, atom_type: AtomType::Relation }, // Unicode: 0x2AC2, superset with multiplication sign below
    "subedot" => Symbol { unicode: 0x2AC3, atom_type: AtomType::Relation }, // Unicode: 0x2AC3, subset of or equal to with dot above
    "supedot" => Symbol { unicode: 0x2AC4, atom_type: AtomType::Relation }, // Unicode: 0x2AC4, superset of or equal to with dot above
    "subseteqq" => Symbol { unicode: 0x2AC5, atom_type: AtomType::Relation }, // Unicode: 0x2AC5, subset of above equals sign
    "supseteqq" => Symbol { unicode: 0x2AC6, atom_type: AtomType::Relation }, // Unicode: 0x2AC6, superset of above equals sign
    "subsim" => Symbol { unicode: 0x2AC7, atom_type: AtomType::Relation }, // Unicode: 0x2AC7, subset of above tilde operator
    "supsim" => Symbol { unicode: 0x2AC8, atom_type: AtomType::Relation }, // Unicode: 0x2AC8, superset of above tilde operator
    "subsetapprox" => Symbol { unicode: 0x2AC9, atom_type: AtomType::Relation }, // Unicode: 0x2AC9, subset of above almost equal to
    "supsetapprox" => Symbol { unicode: 0x2ACA, atom_type: AtomType::Relation }, // Unicode: 0x2ACA, superset of above almost equal to
    "subsetneqq" => Symbol { unicode: 0x2ACB, atom_type: AtomType::Relation }, // Unicode: 0x2ACB, subset of above not equal to
    "supsetneqq" => Symbol { unicode: 0x2ACC, atom_type: AtomType::Relation }, // Unicode: 0x2ACC, superset of above not equal to
    "lsqhook" => Symbol { unicode: 0x2ACD, atom_type: AtomType::Relation }, // Unicode: 0x2ACD, square left open box operator
    "rsqhook" => Symbol { unicode: 0x2ACE, atom_type: AtomType::Relation }, // Unicode: 0x2ACE, square right open box operator
    "csub" => Symbol { unicode: 0x2ACF, atom_type: AtomType::Relation }, // Unicode: 0x2ACF, closed subset
    "csup" => Symbol { unicode: 0x2AD0, atom_type: AtomType::Relation }, // Unicode: 0x2AD0, closed superset
    "csube" => Symbol { unicode: 0x2AD1, atom_type: AtomType::Relation }, // Unicode: 0x2AD1, closed subset or equal to
    "csupe" => Symbol { unicode: 0x2AD2, atom_type: AtomType::Relation }, // Unicode: 0x2AD2, closed superset or equal to
    "subsup" => Symbol { unicode: 0x2AD3, atom_type: AtomType::Relation }, // Unicode: 0x2AD3, subset above superset
    "supsub" => Symbol { unicode: 0x2AD4, atom_type: AtomType::Relation }, // Unicode: 0x2AD4, superset above subset
    "subsub" => Symbol { unicode: 0x2AD5, atom_type: AtomType::Relation }, // Unicode: 0x2AD5, subset above subset
    "supsup" => Symbol { unicode: 0x2AD6, atom_type: AtomType::Relation }, // Unicode: 0x2AD6, superset above superset
    "suphsub" => Symbol { unicode: 0x2AD7, atom_type: AtomType::Relation }, // Unicode: 0x2AD7, superset beside subset
    "supdsub" => Symbol { unicode: 0x2AD8, atom_type: AtomType::Relation }, // Unicode: 0x2AD8, superset beside and joined by dash with subset
    "forkv" => Symbol { unicode: 0x2AD9, atom_type: AtomType::Relation }, // Unicode: 0x2AD9, element of opening downwards
    "topfork" => Symbol { unicode: 0x2ADA, atom_type: AtomType::Relation }, // Unicode: 0x2ADA, pitchfork with tee top
    "mlcp" => Symbol { unicode: 0x2ADB, atom_type: AtomType::Relation }, // Unicode: 0x2ADB, transversal intersection
    "forks" => Symbol { unicode: 0x2ADC, atom_type: AtomType::Relation }, // Unicode: 0x2ADC, forking
    "forksnot" => Symbol { unicode: 0x2ADD, atom_type: AtomType::Relation }, // Unicode: 0x2ADD, nonforking
    "shortlefttack" => Symbol { unicode: 0x2ADE, atom_type: AtomType::Relation }, // Unicode: 0x2ADE, short left tack
    "shortdowntack" => Symbol { unicode: 0x2ADF, atom_type: AtomType::Relation }, // Unicode: 0x2ADF, short down tack
    "shortuptack" => Symbol { unicode: 0x2AE0, atom_type: AtomType::Relation }, // Unicode: 0x2AE0, short up tack
    "perps" => Symbol { unicode: 0x2AE1, atom_type: AtomType::Alpha }, // Unicode: 0x2AE1, perpendicular with s
    "vDdash" => Symbol { unicode: 0x2AE2, atom_type: AtomType::Relation }, // Unicode: 0x2AE2, vertical bar triple right turnstile
    "dashV" => Symbol { unicode: 0x2AE3, atom_type: AtomType::Relation }, // Unicode: 0x2AE3, double vertical bar left turnstile
    "Dashv" => Symbol { unicode: 0x2AE4, atom_type: AtomType::Relation }, // Unicode: 0x2AE4, vertical bar double left turnstile
    "DashV" => Symbol { unicode: 0x2AE5, atom_type: AtomType::Relation }, // Unicode: 0x2AE5, double vertical bar double left turnstile
    "varVdash" => Symbol { unicode: 0x2AE6, atom_type: AtomType::Relation }, // Unicode: 0x2AE6, long dash from left member of double vertical
    "Barv" => Symbol { unicode: 0x2AE7, atom_type: AtomType::Relation }, // Unicode: 0x2AE7, short down tack with overbar
    "vBar" => Symbol { unicode: 0x2AE8, atom_type: AtomType::Relation }, // Unicode: 0x2AE8, short up tack with underbar
    "vBarv" => Symbol { unicode: 0x2AE9, atom_type: AtomType::Relation }, // Unicode: 0x2AE9, short up tack above short down tack
    "barV" => Symbol { unicode: 0x2AEA, atom_type: AtomType::Relation }, // Unicode: 0x2AEA, double down tack
    "Vbar" => Symbol { unicode: 0x2AEB, atom_type: AtomType::Relation }, // Unicode: 0x2AEB, double up tack
    "Not" => Symbol { unicode: 0x2AEC, atom_type: AtomType::Relation }, // Unicode: 0x2AEC, double stroke not sign
    "bNot" => Symbol { unicode: 0x2AED, atom_type: AtomType::Relation }, // Unicode: 0x2AED, reversed double stroke not sign
    "revnmid" => Symbol { unicode: 0x2AEE, atom_type: AtomType::Relation }, // Unicode: 0x2AEE, does not divide with reversed negation slash
    "cirmid" => Symbol { unicode: 0x2AEF, atom_type: AtomType::Relation }, // Unicode: 0x2AEF, vertical line with circle above
    "midcir" => Symbol { unicode: 0x2AF0, atom_type: AtomType::Relation }, // Unicode: 0x2AF0, vertical line with circle below
    "topcir" => Symbol { unicode: 0x2AF1, atom_type: AtomType::Alpha }, // Unicode: 0x2AF1, down tack with circle below
    "nhpar" => Symbol { unicode: 0x2AF2, atom_type: AtomType::Relation }, // Unicode: 0x2AF2, parallel with horizontal stroke
    "parsim" => Symbol { unicode: 0x2AF3, atom_type: AtomType::Relation }, // Unicode: 0x2AF3, parallel with tilde operator
    "interleave" => Symbol { unicode: 0x2AF4, atom_type: AtomType::Binary }, // Unicode: 0x2AF4, triple vertical bar binary relation
    "nhVvert" => Symbol { unicode: 0x2AF5, atom_type: AtomType::Binary }, // Unicode: 0x2AF5, triple vertical bar with horizontal stroke
    "threedotcolon" => Symbol { unicode: 0x2AF6, atom_type: AtomType::Binary }, // Unicode: 0x2AF6, triple colon operator
    "lllnest" => Symbol { unicode: 0x2AF7, atom_type: AtomType::Relation }, // Unicode: 0x2AF7, stacked very much less-than
    "gggnest" => Symbol { unicode: 0x2AF8, atom_type: AtomType::Relation }, // Unicode: 0x2AF8, stacked very much greater-than
    "leqqslant" => Symbol { unicode: 0x2AF9, atom_type: AtomType::Relation }, // Unicode: 0x2AF9, double-line slanted less-than or equal to
    "geqqslant" => Symbol { unicode: 0x2AFA, atom_type: AtomType::Relation }, // Unicode: 0x2AFA, double-line slanted greater-than or equal to
    "trslash" => Symbol { unicode: 0x2AFB, atom_type: AtomType::Binary }, // Unicode: 0x2AFB, triple solidus binary relation
    "biginterleave" => Symbol { unicode: 0x2AFC, atom_type: AtomType::Operator(false) }, // Unicode: 0x2AFC, large triple vertical bar operator
    "sslash" => Symbol { unicode: 0x2AFD, atom_type: AtomType::Binary }, // Unicode: 0x2AFD, double solidus operator
    "talloblong" => Symbol { unicode: 0x2AFE, atom_type: AtomType::Binary }, // Unicode: 0x2AFE, white vertical bar
    "bigtalloblong" => Symbol { unicode: 0x2AFF, atom_type: AtomType::Operator(false) }, // Unicode: 0x2AFF, n-ary white vertical bar
    "squaretopblack" => Symbol { unicode: 0x2B12, atom_type: AtomType::Alpha }, // Unicode: 0x2B12, square with top half black
    "squarebotblack" => Symbol { unicode: 0x2B13, atom_type: AtomType::Alpha }, // Unicode: 0x2B13, square with bottom half black
    "squareurblack" => Symbol { unicode: 0x2B14, atom_type: AtomType::Alpha }, // Unicode: 0x2B14, square with upper right diagonal half black
    "squarellblack" => Symbol { unicode: 0x2B15, atom_type: AtomType::Alpha }, // Unicode: 0x2B15, square with lower left diagonal half black
    "diamondleftblack" => Symbol { unicode: 0x2B16, atom_type: AtomType::Alpha }, // Unicode: 0x2B16, diamond with left half black
    "diamondrightblack" => Symbol { unicode: 0x2B17, atom_type: AtomType::Alpha }, // Unicode: 0x2B17, diamond with right half black
    "diamondtopblack" => Symbol { unicode: 0x2B18, atom_type: AtomType::Alpha }, // Unicode: 0x2B18, diamond with top half black
    "diamondbotblack" => Symbol { unicode: 0x2B19, atom_type: AtomType::Alpha }, // Unicode: 0x2B19, diamond with bottom half black
    "dottedsquare" => Symbol { unicode: 0x2B1A, atom_type: AtomType::Alpha }, // Unicode: 0x2B1A, dotted square
    "lgblksquare" => Symbol { unicode: 0x2B1B, atom_type: AtomType::Alpha }, // Unicode: 0x2B1B, black large square
    "lgwhtsquare" => Symbol { unicode: 0x2B1C, atom_type: AtomType::Alpha }, // Unicode: 0x2B1C, white large square
    "vysmblksquare" => Symbol { unicode: 0x2B1D, atom_type: AtomType::Alpha }, // Unicode: 0x2B1D, black very small square
    "vysmwhtsquare" => Symbol { unicode: 0x2B1E, atom_type: AtomType::Alpha }, // Unicode: 0x2B1E, white very small square
    "pentagonblack" => Symbol { unicode: 0x2B1F, atom_type: AtomType::Alpha }, // Unicode: 0x2B1F, black pentagon
    "pentagon" => Symbol { unicode: 0x2B20, atom_type: AtomType::Alpha }, // Unicode: 0x2B20, white pentagon
    "varhexagon" => Symbol { unicode: 0x2B21, atom_type: AtomType::Alpha }, // Unicode: 0x2B21, white hexagon
    "varhexagonblack" => Symbol { unicode: 0x2B22, atom_type: AtomType::Alpha }, // Unicode: 0x2B22, black hexagon
    "hexagonblack" => Symbol { unicode: 0x2B23, atom_type: AtomType::Alpha }, // Unicode: 0x2B23, horizontal black hexagon
    "lgblkcircle" => Symbol { unicode: 0x2B24, atom_type: AtomType::Alpha }, // Unicode: 0x2B24, black large circle
    "mdblkdiamond" => Symbol { unicode: 0x2B25, atom_type: AtomType::Alpha }, // Unicode: 0x2B25, black medium diamond
    "mdwhtdiamond" => Symbol { unicode: 0x2B26, atom_type: AtomType::Alpha }, // Unicode: 0x2B26, white medium diamond
    "mdblklozenge" => Symbol { unicode: 0x2B27, atom_type: AtomType::Alpha }, // Unicode: 0x2B27, black medium lozenge
    "mdwhtlozenge" => Symbol { unicode: 0x2B28, atom_type: AtomType::Alpha }, // Unicode: 0x2B28, white medium lozenge
    "smblkdiamond" => Symbol { unicode: 0x2B29, atom_type: AtomType::Alpha }, // Unicode: 0x2B29, black small diamond
    "smblklozenge" => Symbol { unicode: 0x2B2A, atom_type: AtomType::Alpha }, // Unicode: 0x2B2A, black small lozenge
    "smwhtlozenge" => Symbol { unicode: 0x2B2B, atom_type: AtomType::Alpha }, // Unicode: 0x2B2B, white small lozenge
    "blkhorzoval" => Symbol { unicode: 0x2B2C, atom_type: AtomType::Alpha }, // Unicode: 0x2B2C, black horizontal ellipse
    "whthorzoval" => Symbol { unicode: 0x2B2D, atom_type: AtomType::Alpha }, // Unicode: 0x2B2D, white horizontal ellipse
    "blkvertoval" => Symbol { unicode: 0x2B2E, atom_type: AtomType::Alpha }, // Unicode: 0x2B2E, black vertical ellipse
    "whtvertoval" => Symbol { unicode: 0x2B2F, atom_type: AtomType::Alpha }, // Unicode: 0x2B2F, white vertical ellipse
    "circleonleftarrow" => Symbol { unicode: 0x2B30, atom_type: AtomType::Relation }, // Unicode: 0x2B30, left arrow with small circle
    "leftthreearrows" => Symbol { unicode: 0x2B31, atom_type: AtomType::Relation }, // Unicode: 0x2B31, three leftwards arrows
    "leftarrowonoplus" => Symbol { unicode: 0x2B32, atom_type: AtomType::Relation }, // Unicode: 0x2B32, left arrow with circled plus
    "longleftsquigarrow" => Symbol { unicode: 0x2B33, atom_type: AtomType::Relation }, // Unicode: 0x2B33, long leftwards squiggle arrow
    "nvtwoheadleftarrow" => Symbol { unicode: 0x2B34, atom_type: AtomType::Relation }, // Unicode: 0x2B34, leftwards two-headed arrow with vertical stroke
    "nVtwoheadleftarrow" => Symbol { unicode: 0x2B35, atom_type: AtomType::Relation }, // Unicode: 0x2B35, leftwards two-headed arrow with double vertical stroke
    "twoheadmapsfrom" => Symbol { unicode: 0x2B36, atom_type: AtomType::Relation }, // Unicode: 0x2B36, leftwards two-headed arrow from bar
    "twoheadleftdbkarrow" => Symbol { unicode: 0x2B37, atom_type: AtomType::Relation }, // Unicode: 0x2B37, leftwards two-headed triple-dash arrow
    "leftdotarrow" => Symbol { unicode: 0x2B38, atom_type: AtomType::Relation }, // Unicode: 0x2B38, leftwards arrow with dotted stem
    "nvleftarrowtail" => Symbol { unicode: 0x2B39, atom_type: AtomType::Relation }, // Unicode: 0x2B39, leftwards arrow with tail with vertical stroke
    "nVleftarrowtail" => Symbol { unicode: 0x2B3A, atom_type: AtomType::Relation }, // Unicode: 0x2B3A, leftwards arrow with tail with double vertical stroke
    "twoheadleftarrowtail" => Symbol { unicode: 0x2B3B, atom_type: AtomType::Relation }, // Unicode: 0x2B3B, leftwards two-headed arrow with tail
    "nvtwoheadleftarrowtail" => Symbol { unicode: 0x2B3C, atom_type: AtomType::Relation }, // Unicode: 0x2B3C, leftwards two-headed arrow with tail with vertical stroke
    "nVtwoheadleftarrowtail" => Symbol { unicode: 0x2B3D, atom_type: AtomType::Relation }, // Unicode: 0x2B3D, leftwards two-headed arrow with tail with double vertical stroke
    "leftarrowx" => Symbol { unicode: 0x2B3E, atom_type: AtomType::Relation }, // Unicode: 0x2B3E, leftwards arrow through x
    "leftcurvedarrow" => Symbol { unicode: 0x2B3F, atom_type: AtomType::Relation }, // Unicode: 0x2B3F, wave arrow pointing directly left
    "equalleftarrow" => Symbol { unicode: 0x2B40, atom_type: AtomType::Relation }, // Unicode: 0x2B40, equals sign above leftwards arrow
    "bsimilarleftarrow" => Symbol { unicode: 0x2B41, atom_type: AtomType::Relation }, // Unicode: 0x2B41, reverse tilde operator above leftwards arrow
    "leftarrowbackapprox" => Symbol { unicode: 0x2B42, atom_type: AtomType::Relation }, // Unicode: 0x2B42, leftwards arrow above reverse almost equal to
    "rightarrowgtr" => Symbol { unicode: 0x2B43, atom_type: AtomType::Relation }, // Unicode: 0x2B43, rightwards arrow through greater-than
    "rightarrowsupset" => Symbol { unicode: 0x2B44, atom_type: AtomType::Relation }, // Unicode: 0x2B44, rightwards arrow through subset
    "LLeftarrow" => Symbol { unicode: 0x2B45, atom_type: AtomType::Relation }, // Unicode: 0x2B45, leftwards quadruple arrow
    "RRightarrow" => Symbol { unicode: 0x2B46, atom_type: AtomType::Relation }, // Unicode: 0x2B46, rightwards quadruple arrow
    "bsimilarrightarrow" => Symbol { unicode: 0x2B47, atom_type: AtomType::Relation }, // Unicode: 0x2B47, reverse tilde operator above rightwards arrow
    "rightarrowbackapprox" => Symbol { unicode: 0x2B48, atom_type: AtomType::Relation }, // Unicode: 0x2B48, rightwards arrow above reverse almost equal to
    "similarleftarrow" => Symbol { unicode: 0x2B49, atom_type: AtomType::Relation }, // Unicode: 0x2B49, tilde operator above leftwards arrow
    "leftarrowapprox" => Symbol { unicode: 0x2B4A, atom_type: AtomType::Relation }, // Unicode: 0x2B4A, leftwards arrow above almost equal to
    "leftarrowbsimilar" => Symbol { unicode: 0x2B4B, atom_type: AtomType::Relation }, // Unicode: 0x2B4B, leftwards arrow above reverse tilde operator
    "rightarrowbsimilar" => Symbol { unicode: 0x2B4C, atom_type: AtomType::Relation }, // Unicode: 0x2B4C, righttwards arrow above reverse tilde operator
    "medwhitestar" => Symbol { unicode: 0x2B50, atom_type: AtomType::Alpha }, // Unicode: 0x2B50, white medium star
    "medblackstar" => Symbol { unicode: 0x2B51, atom_type: AtomType::Alpha }, // Unicode: 0x2B51, black medium star
    "smwhitestar" => Symbol { unicode: 0x2B52, atom_type: AtomType::Alpha }, // Unicode: 0x2B52, white small star
    "rightpentagonblack" => Symbol { unicode: 0x2B53, atom_type: AtomType::Alpha }, // Unicode: 0x2B53, black right-pointing pentagon
    "rightpentagon" => Symbol { unicode: 0x2B54, atom_type: AtomType::Alpha }, // Unicode: 0x2B54, white right-pointing pentagon
    "postalmark" => Symbol { unicode: 0x3012, atom_type: AtomType::Alpha }, // Unicode: 0x3012, postal mark
    "hzigzag" => Symbol { unicode: 0x3030, atom_type: AtomType::Alpha }, // Unicode: 0x3030, zigzag

    // Additional commands from TeX
    "Alpha" => Symbol { unicode: 0x391, atom_type: AtomType::Alpha }, // Unicode: 0x391,
    "Beta" => Symbol { unicode: 0x392, atom_type: AtomType::Alpha }, // Unicode: 0x392,
    "Gamma" => Symbol { unicode: 0x393, atom_type: AtomType::Alpha }, // Unicode: 0x393,
    "Delta" => Symbol { unicode: 0x394, atom_type: AtomType::Alpha }, // Unicode: 0x394,
    "Epsilon" => Symbol { unicode: 0x395, atom_type: AtomType::Alpha }, // Unicode: 0x395,
    "Zeta" => Symbol { unicode: 0x396, atom_type: AtomType::Alpha }, // Unicode: 0x396,
    "Eta" => Symbol { unicode: 0x397, atom_type: AtomType::Alpha }, // Unicode: 0x397,
    "Theta" => Symbol { unicode: 0x398, atom_type: AtomType::Alpha }, // Unicode: 0x398,
    "Iota" => Symbol { unicode: 0x399, atom_type: AtomType::Alpha }, // Unicode: 0x399,
    "Kappa" => Symbol { unicode: 0x39A, atom_type: AtomType::Alpha }, // Unicode: 0x39A,
    "Lambda" => Symbol { unicode: 0x39B, atom_type: AtomType::Alpha }, // Unicode: 0x39B,
    "Mu" => Symbol { unicode: 0x39C, atom_type: AtomType::Alpha }, // Unicode: 0x39C,
    "Nu" => Symbol { unicode: 0x39D, atom_type: AtomType::Alpha }, // Unicode: 0x39D,
    "Xi" => Symbol { unicode: 0x39E, atom_type: AtomType::Alpha }, // Unicode: 0x39E,
    "Omicron" => Symbol { unicode: 0x39F, atom_type: AtomType::Alpha }, // Unicode: 0x39F,
    "Pi" => Symbol { unicode: 0x3A0, atom_type: AtomType::Alpha }, // Unicode: 0x3A0,
    "Rho" => Symbol { unicode: 0x3A1, atom_type: AtomType::Alpha }, // Unicode: 0x3A1,
    "Sigma" => Symbol { unicode: 0x3A3, atom_type: AtomType::Alpha }, // Unicode: 0x3A3,
    "Tau" => Symbol { unicode: 0x3A4, atom_type: AtomType::Alpha }, // Unicode: 0x3A4,
    "Upsilon" => Symbol { unicode: 0x3A5, atom_type: AtomType::Alpha }, // Unicode: 0x3A5,
    "Phi" => Symbol { unicode: 0x3A6, atom_type: AtomType::Alpha }, // Unicode: 0x3A6,
    "Chi" => Symbol { unicode: 0x3A7, atom_type: AtomType::Alpha }, // Unicode: 0x3A7,
    "Psi" => Symbol { unicode: 0x3A8, atom_type: AtomType::Alpha }, // Unicode: 0x3A8,
    "Omega" => Symbol { unicode: 0x3A9, atom_type: AtomType::Alpha }, // Unicode: 0x3A9,
    "alpha" => Symbol { unicode: 0x3B1, atom_type: AtomType::Alpha }, // Unicode: 0x3B1,
    "beta" => Symbol { unicode: 0x3B2, atom_type: AtomType::Alpha }, // Unicode: 0x3B2,
    "gamma" => Symbol { unicode: 0x3B3, atom_type: AtomType::Alpha }, // Unicode: 0x3B3,
    "delta" => Symbol { unicode: 0x3B4, atom_type: AtomType::Alpha }, // Unicode: 0x3B4,
    "epsilon" => Symbol { unicode: 0x3B5, atom_type: AtomType::Alpha }, // Unicode: 0x3B5,
    "zeta" => Symbol { unicode: 0x3B6, atom_type: AtomType::Alpha }, // Unicode: 0x3B6,
    "eta" => Symbol { unicode: 0x3B7, atom_type: AtomType::Alpha }, // Unicode: 0x3B7,
    "theta" => Symbol { unicode: 0x3B8, atom_type: AtomType::Alpha }, // Unicode: 0x3B8,
    "iota" => Symbol { unicode: 0x3B9, atom_type: AtomType::Alpha }, // Unicode: 0x3B9,
    "kappa" => Symbol { unicode: 0x3BA, atom_type: AtomType::Alpha }, // Unicode: 0x3BA,
    "lambda" => Symbol { unicode: 0x3BB, atom_type: AtomType::Alpha }, // Unicode: 0x3BB,
    "mu" => Symbol { unicode: 0x3BC, atom_type: AtomType::Alpha }, // Unicode: 0x3BC,
    "nu" => Symbol { unicode: 0x3BD, atom_type: AtomType::Alpha }, // Unicode: 0x3BD,
    "xi" => Symbol { unicode: 0x3BE, atom_type: AtomType::Alpha }, // Unicode: 0x3BE,
    "omicron" => Symbol { unicode: 0x3BF, atom_type: AtomType::Alpha }, // Unicode: 0x3BF,
    "pi" => Symbol { unicode: 0x3C0, atom_type: AtomType::Alpha }, // Unicode: 0x3C0,
    "rho" => Symbol { unicode: 0x3C1, atom_type: AtomType::Alpha }, // Unicode: 0x3C1,
    "sigma" => Symbol { unicode: 0x3C3, atom_type: AtomType::Alpha }, // Unicode: 0x3C3,
    "tau" => Symbol { unicode: 0x3C4, atom_type: AtomType::Alpha }, // Unicode: 0x3C4,
    "upsilon" => Symbol { unicode: 0x3C5, atom_type: AtomType::Alpha }, // Unicode: 0x3C5,
    "phi" => Symbol { unicode: 0x3C6, atom_type: AtomType::Alpha }, // Unicode: 0x3C6,
    "chi" => Symbol { unicode: 0x3C7, atom_type: AtomType::Alpha }, // Unicode: 0x3C7,
    "psi" => Symbol { unicode: 0x3C8, atom_type: AtomType::Alpha }, // Unicode: 0x3C8,
    "omega" => Symbol { unicode: 0x3C9, atom_type: AtomType::Alpha }, // Unicode: 0x3C9,
    "varphi" => Symbol { unicode: 0x3C6, atom_type: AtomType::Alpha }, // Unicode: 0x3C6, curly or open small phi, greek
    "varsigma" => Symbol { unicode: 0x3C2, atom_type: AtomType::Alpha }, // Unicode: 0x3C2, terminal sigma, greek
    "varbeta" => Symbol { unicode: 0x3D0, atom_type: AtomType::Alpha }, // Unicode: 0x3D0, rounded small beta, greek
    "vartheta" => Symbol { unicode: 0x3D1, atom_type: AtomType::Alpha }, // Unicode: 0x3D1, /vartheta - curly or open theta
    "varpi" => Symbol { unicode: 0x3D6, atom_type: AtomType::Alpha }, // Unicode: 0x3D6, rounded small pi (pomega), greek
    "varkappa" => Symbol { unicode: 0x3F0, atom_type: AtomType::Alpha }, // Unicode: 0x3F0, rounded small kappa, greek
    "varrho" => Symbol { unicode: 0x3F1, atom_type: AtomType::Alpha }, // Unicode: 0x3F1, rounded small rho, greek
    "varTheta" => Symbol { unicode: 0x3F4, atom_type: AtomType::Alpha }, // Unicode: 0x3F4, greek capital theta symbol
    "varepsilon" => Symbol { unicode: 0x3F5, atom_type: AtomType::Alpha }, // Unicode: 0x3F5, greek lunate epsilon symbol
};


static SYMBOLS_STATIC_MAP: Map<&'static str, Symbol> = static_map! {
    Default: Symbol { unicode: 0x00, atom_type: AtomType::Accent },
    // unicode-math.dtx command table
    // "mathexclam" => Symbol { unicode: 0x21, atom_type: AtomType::Punctuation }, // Unicode: 0x21, exclamation mark
    // "mathoctothorpe" => Symbol { unicode: 0x23, atom_type: AtomType::Alpha }, // Unicode: 0x23, number sign
    // "mathdollar" => Symbol { unicode: 0x24, atom_type: AtomType::Alpha }, // Unicode: 0x24, dollar sign
    // "mathpercent" => Symbol { unicode: 0x25, atom_type: AtomType::Alpha }, // Unicode: 0x25, percent sign
    // "mathampersand" => Symbol { unicode: 0x26, atom_type: AtomType::Alpha }, // Unicode: 0x26, ampersand
    "lparen" => Symbol { unicode: 0x28, atom_type: AtomType::Open }, // Unicode: 0x28, left parenthesis
    "rparen" => Symbol { unicode: 0x29, atom_type: AtomType::Close }, // Unicode: 0x29, right parenthesis
    // "mathplus" => Symbol { unicode: 0x2B, atom_type: AtomType::Binary }, // Unicode: 0x2B, plus sign b:
    // "mathcomma" => Symbol { unicode: 0x2C, atom_type: AtomType::Punctuation }, // Unicode: 0x2C, comma
    // "mathperiod" => Symbol { unicode: 0x2E, atom_type: AtomType::Alpha }, // Unicode: 0x2E, full stop, period
    // "mathslash" => Symbol { unicode: 0x2F, atom_type: AtomType::Alpha }, // Unicode: 0x2F, solidus
    // "mathcolon" => Symbol { unicode: 0x3A, atom_type: AtomType::Punctuation }, // Unicode: 0x3A, colon
    // "mathsemicolon" => Symbol { unicode: 0x3B, atom_type: AtomType::Punctuation }, // Unicode: 0x3B, semicolon p:
    // "less" => Symbol { unicode: 0x3C, atom_type: AtomType::Relation }, // Unicode: 0x3C, less-than sign r:
    // "equal" => Symbol { unicode: 0x3D, atom_type: AtomType::Relation }, // Unicode: 0x3D, equals sign r:
    // "greater" => Symbol { unicode: 0x3E, atom_type: AtomType::Relation }, // Unicode: 0x3E, greater-than sign r:
    // "mathquestion" => Symbol { unicode: 0x3F, atom_type: AtomType::Alpha }, // Unicode: 0x3F, question mark
    // "mathatsign" => Symbol { unicode: 0x40, atom_type: AtomType::Alpha }, // Unicode: 0x40, commercial at
    "lbrack" => Symbol { unicode: 0x5B, atom_type: AtomType::Open }, // Unicode: 0x5B, left square bracket
    "backslash" => Symbol { unicode: 0x5C, atom_type: AtomType::Alpha }, // Unicode: 0x5C, reverse solidus
    "rbrack" => Symbol { unicode: 0x5D, atom_type: AtomType::Close }, // Unicode: 0x5D, right square bracket
    "lbrace" => Symbol { unicode: 0x7B, atom_type: AtomType::Open }, // Unicode: 0x7B, left curly bracket
    "vert" => Symbol { unicode: 0x7C, atom_type: AtomType::Fence }, // Unicode: 0x7C, vertical bar
    "rbrace" => Symbol { unicode: 0x7D, atom_type: AtomType::Close }, // Unicode: 0x7D, right curly bracket
    // "mathsterling" => Symbol { unicode: 0xA3, atom_type: AtomType::Alpha }, // Unicode: 0xA3, pound sign
    // "mathyen" => Symbol { unicode: 0xA5, atom_type: AtomType::Alpha }, // Unicode: 0xA5, yen sign
    "neg" => Symbol { unicode: 0xAC, atom_type: AtomType::Alpha }, // Unicode: 0xAC, /neg /lnot not sign
    "pm" => Symbol { unicode: 0xB1, atom_type: AtomType::Binary }, // Unicode: 0xB1, plus-or-minus sign
    "cdotp" => Symbol { unicode: 0xB7, atom_type: AtomType::Binary }, // Unicode: 0xB7, /centerdot b: middle dot
    "times" => Symbol { unicode: 0xD7, atom_type: AtomType::Binary }, // Unicode: 0xD7, multiply sign
    "matheth" => Symbol { unicode: 0xF0, atom_type: AtomType::Alpha }, // Unicode: 0xF0, eth
    "div" => Symbol { unicode: 0xF7, atom_type: AtomType::Binary }, // Unicode: 0xF7, divide sign
    "Zbar" => Symbol { unicode: 0x1B5, atom_type: AtomType::Alpha }, // Unicode: 0x1B5, impedance (latin capital letter z with stroke)
    "grave" => Symbol { unicode: 0x300, atom_type: AtomType::Accent }, // Unicode: 0x300, grave accent
    "acute" => Symbol { unicode: 0x301, atom_type: AtomType::Accent }, // Unicode: 0x301, acute accent
    "hat" => Symbol { unicode: 0x302, atom_type: AtomType::Accent }, // Unicode: 0x302, circumflex accent
    "widehat" => Symbol { unicode: 0x302, atom_type: AtomType::AccentWide }, // Unicode: 0x302, circumflex accent
    "tilde" => Symbol { unicode: 0x303, atom_type: AtomType::Accent }, // Unicode: 0x303, tilde
    "widetilde" => Symbol { unicode: 0x303, atom_type: AtomType::AccentWide }, // Unicode: 0x303, tilde
    "bar" => Symbol { unicode: 0x304, atom_type: AtomType::Accent }, // Unicode: 0x304, macron
    "overbar" => Symbol { unicode: 0x305, atom_type: AtomType::Accent }, // Unicode: 0x305, overbar embellishment
    "breve" => Symbol { unicode: 0x306, atom_type: AtomType::Accent }, // Unicode: 0x306, breve
    "dot" => Symbol { unicode: 0x307, atom_type: AtomType::Accent }, // Unicode: 0x307, dot above
    "ddot" => Symbol { unicode: 0x308, atom_type: AtomType::Accent }, // Unicode: 0x308, dieresis
    "ovhook" => Symbol { unicode: 0x309, atom_type: AtomType::Accent }, // Unicode: 0x309, combining hook above
    "ocirc" => Symbol { unicode: 0x30A, atom_type: AtomType::Accent }, // Unicode: 0x30A, ring
    "check" => Symbol { unicode: 0x30C, atom_type: AtomType::Accent }, // Unicode: 0x30C, caron
    "candra" => Symbol { unicode: 0x310, atom_type: AtomType::Accent }, // Unicode: 0x310, candrabindu (non-spacing)
    "oturnedcomma" => Symbol { unicode: 0x312, atom_type: AtomType::Accent }, // Unicode: 0x312, combining turned comma above
    "ocommatopright" => Symbol { unicode: 0x315, atom_type: AtomType::Accent }, // Unicode: 0x315, combining comma above right
    "droang" => Symbol { unicode: 0x31A, atom_type: AtomType::Accent }, // Unicode: 0x31A, left angle above (non-spacing)
    "wideutilde" => Symbol { unicode: 0x330, atom_type: AtomType::BotAccentWide }, // Unicode: 0x330, under tilde accent (multiple characters and non-spacing)
    "mathunderbar" => Symbol { unicode: 0x332, atom_type: AtomType::BotAccentWide }, // Unicode: 0x332, combining low line
    "not" => Symbol { unicode: 0x338, atom_type: AtomType::Accent }, // Unicode: 0x338, combining long solidus overlay
    "underleftrightarrow" => Symbol { unicode: 0x34D, atom_type: AtomType::BotAccentWide }, // Unicode: 0x34D, underleftrightarrow accent
    // "mupAlpha" => Symbol { unicode: 0x391, atom_type: AtomType::Alpha }, // Unicode: 0x391, capital alpha, greek
    // "mupBeta" => Symbol { unicode: 0x392, atom_type: AtomType::Alpha }, // Unicode: 0x392, capital beta, greek
    // "mupGamma" => Symbol { unicode: 0x393, atom_type: AtomType::Alpha }, // Unicode: 0x393, capital gamma, greek
    // "mupDelta" => Symbol { unicode: 0x394, atom_type: AtomType::Alpha }, // Unicode: 0x394, capital delta, greek
    // "mupEpsilon" => Symbol { unicode: 0x395, atom_type: AtomType::Alpha }, // Unicode: 0x395, capital epsilon, greek
    // "mupZeta" => Symbol { unicode: 0x396, atom_type: AtomType::Alpha }, // Unicode: 0x396, capital zeta, greek
    // "mupEta" => Symbol { unicode: 0x397, atom_type: AtomType::Alpha }, // Unicode: 0x397, capital eta, greek
    // "mupTheta" => Symbol { unicode: 0x398, atom_type: AtomType::Alpha }, // Unicode: 0x398, capital theta, greek
    // "mupIota" => Symbol { unicode: 0x399, atom_type: AtomType::Alpha }, // Unicode: 0x399, capital iota, greek
    // "mupKappa" => Symbol { unicode: 0x39A, atom_type: AtomType::Alpha }, // Unicode: 0x39A, capital kappa, greek
    // "mupLambda" => Symbol { unicode: 0x39B, atom_type: AtomType::Alpha }, // Unicode: 0x39B, capital lambda, greek
    // "mupMu" => Symbol { unicode: 0x39C, atom_type: AtomType::Alpha }, // Unicode: 0x39C, capital mu, greek
    // "mupNu" => Symbol { unicode: 0x39D, atom_type: AtomType::Alpha }, // Unicode: 0x39D, capital nu, greek
    // "mupXi" => Symbol { unicode: 0x39E, atom_type: AtomType::Alpha }, // Unicode: 0x39E, capital xi, greek
    // "mupOmicron" => Symbol { unicode: 0x39F, atom_type: AtomType::Alpha }, // Unicode: 0x39F, capital omicron, greek
    // "mupPi" => Symbol { unicode: 0x3A0, atom_type: AtomType::Alpha }, // Unicode: 0x3A0, capital pi, greek
    // "mupRho" => Symbol { unicode: 0x3A1, atom_type: AtomType::Alpha }, // Unicode: 0x3A1, capital rho, greek
    // "mupSigma" => Symbol { unicode: 0x3A3, atom_type: AtomType::Alpha }, // Unicode: 0x3A3, capital sigma, greek
    // "mupTau" => Symbol { unicode: 0x3A4, atom_type: AtomType::Alpha }, // Unicode: 0x3A4, capital tau, greek
    // "mupUpsilon" => Symbol { unicode: 0x3A5, atom_type: AtomType::Alpha }, // Unicode: 0x3A5, capital upsilon, greek
    // "mupPhi" => Symbol { unicode: 0x3A6, atom_type: AtomType::Alpha }, // Unicode: 0x3A6, capital phi, greek
    // "mupChi" => Symbol { unicode: 0x3A7, atom_type: AtomType::Alpha }, // Unicode: 0x3A7, capital chi, greek
    // "mupPsi" => Symbol { unicode: 0x3A8, atom_type: AtomType::Alpha }, // Unicode: 0x3A8, capital psi, greek
    // "mupOmega" => Symbol { unicode: 0x3A9, atom_type: AtomType::Alpha }, // Unicode: 0x3A9, capital omega, greek
    // "mupalpha" => Symbol { unicode: 0x3B1, atom_type: AtomType::Alpha }, // Unicode: 0x3B1, small alpha, greek
    // "mupbeta" => Symbol { unicode: 0x3B2, atom_type: AtomType::Alpha }, // Unicode: 0x3B2, small beta, greek
    // "mupgamma" => Symbol { unicode: 0x3B3, atom_type: AtomType::Alpha }, // Unicode: 0x3B3, small gamma, greek
    // "mupdelta" => Symbol { unicode: 0x3B4, atom_type: AtomType::Alpha }, // Unicode: 0x3B4, small delta, greek
    // "mupepsilon" => Symbol { unicode: 0x3B5, atom_type: AtomType::Alpha }, // Unicode: 0x3B5, rounded small epsilon, greek
    // "mupzeta" => Symbol { unicode: 0x3B6, atom_type: AtomType::Alpha }, // Unicode: 0x3B6, small zeta, greek
    // "mupeta" => Symbol { unicode: 0x3B7, atom_type: AtomType::Alpha }, // Unicode: 0x3B7, small eta, greek
    // "muptheta" => Symbol { unicode: 0x3B8, atom_type: AtomType::Alpha }, // Unicode: 0x3B8, straight theta, small theta, greek
    // "mupiota" => Symbol { unicode: 0x3B9, atom_type: AtomType::Alpha }, // Unicode: 0x3B9, small iota, greek
    // "mupkappa" => Symbol { unicode: 0x3BA, atom_type: AtomType::Alpha }, // Unicode: 0x3BA, small kappa, greek
    // "muplambda" => Symbol { unicode: 0x3BB, atom_type: AtomType::Alpha }, // Unicode: 0x3BB, small lambda, greek
    // "mupmu" => Symbol { unicode: 0x3BC, atom_type: AtomType::Alpha }, // Unicode: 0x3BC, small mu, greek
    // "mupnu" => Symbol { unicode: 0x3BD, atom_type: AtomType::Alpha }, // Unicode: 0x3BD, small nu, greek
    // "mupxi" => Symbol { unicode: 0x3BE, atom_type: AtomType::Alpha }, // Unicode: 0x3BE, small xi, greek
    // "mupomicron" => Symbol { unicode: 0x3BF, atom_type: AtomType::Alpha }, // Unicode: 0x3BF, small omicron, greek
    // "muppi" => Symbol { unicode: 0x3C0, atom_type: AtomType::Alpha }, // Unicode: 0x3C0, small pi, greek
    // "muprho" => Symbol { unicode: 0x3C1, atom_type: AtomType::Alpha }, // Unicode: 0x3C1, small rho, greek
    // "mupvarsigma" => Symbol { unicode: 0x3C2, atom_type: AtomType::Alpha }, // Unicode: 0x3C2, terminal sigma, greek
    // "mupsigma" => Symbol { unicode: 0x3C3, atom_type: AtomType::Alpha }, // Unicode: 0x3C3, small sigma, greek
    // "muptau" => Symbol { unicode: 0x3C4, atom_type: AtomType::Alpha }, // Unicode: 0x3C4, small tau, greek
    // "mupupsilon" => Symbol { unicode: 0x3C5, atom_type: AtomType::Alpha }, // Unicode: 0x3C5, small upsilon, greek
    // "mupvarphi" => Symbol { unicode: 0x3C6, atom_type: AtomType::Alpha }, // Unicode: 0x3C6, curly or open small phi, greek
    // "mupchi" => Symbol { unicode: 0x3C7, atom_type: AtomType::Alpha }, // Unicode: 0x3C7, small chi, greek
    // "muppsi" => Symbol { unicode: 0x3C8, atom_type: AtomType::Alpha }, // Unicode: 0x3C8, small psi, greek
    // "mupomega" => Symbol { unicode: 0x3C9, atom_type: AtomType::Alpha }, // Unicode: 0x3C9, small omega, greek
    // "mupvarbeta" => Symbol { unicode: 0x3D0, atom_type: AtomType::Alpha }, // Unicode: 0x3D0, rounded small beta, greek
    // "mupvartheta" => Symbol { unicode: 0x3D1, atom_type: AtomType::Alpha }, // Unicode: 0x3D1, /vartheta - curly or open theta
    // "mupphi" => Symbol { unicode: 0x3D5, atom_type: AtomType::Alpha }, // Unicode: 0x3D5, /straightphi - small phi, greek
    // "mupvarpi" => Symbol { unicode: 0x3D6, atom_type: AtomType::Alpha }, // Unicode: 0x3D6, rounded small pi (pomega), greek
    // "upoldKoppa" => Symbol { unicode: 0x3D8, atom_type: AtomType::Alpha }, // Unicode: 0x3D8, greek letter archaic koppa
    // "upoldkoppa" => Symbol { unicode: 0x3D9, atom_type: AtomType::Alpha }, // Unicode: 0x3D9, greek small letter archaic koppa
    // "upStigma" => Symbol { unicode: 0x3DA, atom_type: AtomType::Alpha }, // Unicode: 0x3DA, capital stigma
    // "upstigma" => Symbol { unicode: 0x3DB, atom_type: AtomType::Alpha }, // Unicode: 0x3DB, greek small letter stigma
    // "upDigamma" => Symbol { unicode: 0x3DC, atom_type: AtomType::Alpha }, // Unicode: 0x3DC, capital digamma
    // "updigamma" => Symbol { unicode: 0x3DD, atom_type: AtomType::Alpha }, // Unicode: 0x3DD, old greek small letter digamma
    // "upKoppa" => Symbol { unicode: 0x3DE, atom_type: AtomType::Alpha }, // Unicode: 0x3DE, capital koppa
    // "upkoppa" => Symbol { unicode: 0x3DF, atom_type: AtomType::Alpha }, // Unicode: 0x3DF, greek small letter koppa
    // "upSampi" => Symbol { unicode: 0x3E0, atom_type: AtomType::Alpha }, // Unicode: 0x3E0, capital sampi
    // "upsampi" => Symbol { unicode: 0x3E1, atom_type: AtomType::Alpha }, // Unicode: 0x3E1, greek small letter sampi
    // "mupvarkappa" => Symbol { unicode: 0x3F0, atom_type: AtomType::Alpha }, // Unicode: 0x3F0, rounded small kappa, greek
    // "mupvarrho" => Symbol { unicode: 0x3F1, atom_type: AtomType::Alpha }, // Unicode: 0x3F1, rounded small rho, greek
    // "mupvarTheta" => Symbol { unicode: 0x3F4, atom_type: AtomType::Alpha }, // Unicode: 0x3F4, greek capital theta symbol
    // "mupvarepsilon" => Symbol { unicode: 0x3F5, atom_type: AtomType::Alpha }, // Unicode: 0x3F5, greek lunate epsilon symbol
    // "upbackepsilon" => Symbol { unicode: 0x3F6, atom_type: AtomType::Alpha }, // Unicode: 0x3F6, greek reversed lunate epsilon symbol
    "horizbar" => Symbol { unicode: 0x2015, atom_type: AtomType::Alpha }, // Unicode: 0x2015, horizontal bar
    "Vert" => Symbol { unicode: 0x2016, atom_type: AtomType::Fence }, // Unicode: 0x2016, double vertical bar
    "twolowline" => Symbol { unicode: 0x2017, atom_type: AtomType::Alpha }, // Unicode: 0x2017, double low line (spacing)
    "dagger" => Symbol { unicode: 0x2020, atom_type: AtomType::Binary }, // Unicode: 0x2020, dagger relation
    "ddagger" => Symbol { unicode: 0x2021, atom_type: AtomType::Binary }, // Unicode: 0x2021, double dagger relation
    "smblkcircle" => Symbol { unicode: 0x2022, atom_type: AtomType::Binary }, // Unicode: 0x2022, /bullet b: round bullet, filled
    "enleadertwodots" => Symbol { unicode: 0x2025, atom_type: AtomType::Alpha }, // Unicode: 0x2025, double baseline dot (en leader)
    "unicodeellipsis" => Symbol { unicode: 0x2026, atom_type: AtomType::Alpha }, // Unicode: 0x2026, ellipsis (horizontal)
    "prime" => Symbol { unicode: 0x2032, atom_type: AtomType::Alpha }, // Unicode: 0x2032, prime or minute, not superscripted
    "dprime" => Symbol { unicode: 0x2033, atom_type: AtomType::Alpha }, // Unicode: 0x2033, double prime or second, not superscripted
    "trprime" => Symbol { unicode: 0x2034, atom_type: AtomType::Alpha }, // Unicode: 0x2034, triple prime (not superscripted)
    "backprime" => Symbol { unicode: 0x2035, atom_type: AtomType::Alpha }, // Unicode: 0x2035, reverse prime, not superscripted
    "backdprime" => Symbol { unicode: 0x2036, atom_type: AtomType::Alpha }, // Unicode: 0x2036, double reverse prime, not superscripted
    "backtrprime" => Symbol { unicode: 0x2037, atom_type: AtomType::Alpha }, // Unicode: 0x2037, triple reverse prime, not superscripted
    "caretinsert" => Symbol { unicode: 0x2038, atom_type: AtomType::Alpha }, // Unicode: 0x2038, caret (insertion mark)
    // "Exclam" => Symbol { unicode: 0x203C, atom_type: AtomType::Alpha }, // Unicode: 0x203C, double exclamation mark
    // "tieconcat" => Symbol { unicode: 0x2040, atom_type: AtomType::Binary }, // Unicode: 0x2040, character tie, z notation sequence concatenation
    // "hyphenbullet" => Symbol { unicode: 0x2043, atom_type: AtomType::Alpha }, // Unicode: 0x2043, rectangle, filled (hyphen bullet)
    // "fracslash" => Symbol { unicode: 0x2044, atom_type: AtomType::Binary }, // Unicode: 0x2044, fraction slash
    // "Question" => Symbol { unicode: 0x2047, atom_type: AtomType::Alpha }, // Unicode: 0x2047, double question mark
    // "closure" => Symbol { unicode: 0x2050, atom_type: AtomType::Relation }, // Unicode: 0x2050, close up
    // "qprime" => Symbol { unicode: 0x2057, atom_type: AtomType::Alpha }, // Unicode: 0x2057, quadruple prime, not superscripted
    // "euro" => Symbol { unicode: 0x20AC, atom_type: AtomType::Alpha }, // Unicode: 0x20AC, euro sign
    "leftharpoon" => Symbol { unicode: 0x20D0, atom_type: AtomType::Accent }, // Unicode: 0x20D0, combining left harpoon above
    "overleftharpoon" => Symbol { unicode: 0x20D0, atom_type: AtomType::AccentWide }, // Unicode: 0x20D0, combining left harpoon above
    "rightharpoon" => Symbol { unicode: 0x20D1, atom_type: AtomType::Accent }, // Unicode: 0x20D1, combining right harpoon above
    "overrightharpoon" => Symbol { unicode: 0x20D1, atom_type: AtomType::AccentWide }, // Unicode: 0x20D1, combining right harpoon above
    "vertoverlay" => Symbol { unicode: 0x20D2, atom_type: AtomType::Accent }, // Unicode: 0x20D2, combining long vertical line overlay
    "overleftarrow" => Symbol { unicode: 0x20D6, atom_type: AtomType::AccentWide }, // Unicode: 0x20D6, combining left arrow above
    "overrightarrow" => Symbol { unicode: 0x20D7, atom_type: AtomType::Accent }, // Unicode: 0x20D7, combining left arrow above
    "vec" => Symbol { unicode: 0x20D7, atom_type: AtomType::Accent }, // Unicode: 0x20D7, combining right arrow above
    "dddot" => Symbol { unicode: 0x20DB, atom_type: AtomType::Accent }, // Unicode: 0x20DB, combining three dots above
    "ddddot" => Symbol { unicode: 0x20DC, atom_type: AtomType::Accent }, // Unicode: 0x20DC, combining four dots above
    // "enclosecircle" => Symbol { unicode: 0x20DD, atom_type: AtomType::Alpha }, // Unicode: 0x20DD, combining enclosing circle
    // "enclosesquare" => Symbol { unicode: 0x20DE, atom_type: AtomType::Alpha }, // Unicode: 0x20DE, combining enclosing square
    // "enclosediamond" => Symbol { unicode: 0x20DF, atom_type: AtomType::Alpha }, // Unicode: 0x20DF, combining enclosing diamond
    // "overleftrightarrow" => Symbol { unicode: 0x20E1, atom_type: AtomType::AccentWide }, // Unicode: 0x20E1, combining left right arrow above
    // "enclosetriangle" => Symbol { unicode: 0x20E4, atom_type: AtomType::Alpha }, // Unicode: 0x20E4, combining enclosing upward pointing triangle
    // "annuity" => Symbol { unicode: 0x20E7, atom_type: AtomType::Accent }, // Unicode: 0x20E7, combining annuity symbol
    // "threeunderdot" => Symbol { unicode: 0x20E8, atom_type: AtomType::BotAccent }, // Unicode: 0x20E8, combining triple underdot
    "widebridgeabove" => Symbol { unicode: 0x20E9, atom_type: AtomType::Accent }, // Unicode: 0x20E9, combining wide bridge above
    "underrightharpoondown" => Symbol { unicode: 0x20EC, atom_type: AtomType::BotAccentWide }, // Unicode: 0x20EC, combining rightwards harpoon with barb downwards
    "underleftharpoondown" => Symbol { unicode: 0x20ED, atom_type: AtomType::BotAccentWide }, // Unicode: 0x20ED, combining leftwards harpoon with barb downwards
    "underleftarrow" => Symbol { unicode: 0x20EE, atom_type: AtomType::BotAccentWide }, // Unicode: 0x20EE, combining left arrow below
    "underrightarrow" => Symbol { unicode: 0x20EF, atom_type: AtomType::BotAccentWide }, // Unicode: 0x20EF, combining right arrow below
    "asteraccent" => Symbol { unicode: 0x20F0, atom_type: AtomType::Accent }, // Unicode: 0x20F0, combining asterisk above
    "BbbC" => Symbol { unicode: 0x2102, atom_type: AtomType::Alpha }, // Unicode: 0x2102, /bbb c, open face c
    "Eulerconst" => Symbol { unicode: 0x2107, atom_type: AtomType::Alpha }, // Unicode: 0x2107, euler constant
    "mscrg" => Symbol { unicode: 0x210A, atom_type: AtomType::Alpha }, // Unicode: 0x210A, /scr g, script letter g
    "mscrH" => Symbol { unicode: 0x210B, atom_type: AtomType::Alpha }, // Unicode: 0x210B, hamiltonian (script capital h)
    "mfrakH" => Symbol { unicode: 0x210C, atom_type: AtomType::Alpha }, // Unicode: 0x210C, /frak h, upper case h
    "BbbH" => Symbol { unicode: 0x210D, atom_type: AtomType::Alpha }, // Unicode: 0x210D, /bbb h, open face h
    "Planckconst" => Symbol { unicode: 0x210E, atom_type: AtomType::Alpha }, // Unicode: 0x210E, planck constant
    "hslash" => Symbol { unicode: 0x210F, atom_type: AtomType::Alpha }, // Unicode: 0x210F, /hslash - variant planck's over 2pi
    "mscrI" => Symbol { unicode: 0x2110, atom_type: AtomType::Alpha }, // Unicode: 0x2110, /scr i, script letter i
    "Im" => Symbol { unicode: 0x2111, atom_type: AtomType::Alpha }, // Unicode: 0x2111, imaginary part
    "mscrL" => Symbol { unicode: 0x2112, atom_type: AtomType::Alpha }, // Unicode: 0x2112, lagrangian (script capital l)
    "ell" => Symbol { unicode: 0x2113, atom_type: AtomType::Alpha }, // Unicode: 0x2113, cursive small l
    "BbbN" => Symbol { unicode: 0x2115, atom_type: AtomType::Alpha }, // Unicode: 0x2115, /bbb n, open face n
    "wp" => Symbol { unicode: 0x2118, atom_type: AtomType::Alpha }, // Unicode: 0x2118, weierstrass p
    "BbbP" => Symbol { unicode: 0x2119, atom_type: AtomType::Alpha }, // Unicode: 0x2119, /bbb p, open face p
    "BbbQ" => Symbol { unicode: 0x211A, atom_type: AtomType::Alpha }, // Unicode: 0x211A, /bbb q, open face q
    "mscrR" => Symbol { unicode: 0x211B, atom_type: AtomType::Alpha }, // Unicode: 0x211B, /scr r, script letter r
    "Re" => Symbol { unicode: 0x211C, atom_type: AtomType::Alpha }, // Unicode: 0x211C, real part
    "BbbR" => Symbol { unicode: 0x211D, atom_type: AtomType::Alpha }, // Unicode: 0x211D, /bbb r, open face r
    "BbbZ" => Symbol { unicode: 0x2124, atom_type: AtomType::Alpha }, // Unicode: 0x2124, /bbb z, open face z
    "mho" => Symbol { unicode: 0x2127, atom_type: AtomType::Alpha }, // Unicode: 0x2127, conductance
    "mfrakZ" => Symbol { unicode: 0x2128, atom_type: AtomType::Alpha }, // Unicode: 0x2128, /frak z, upper case z
    "turnediota" => Symbol { unicode: 0x2129, atom_type: AtomType::Alpha }, // Unicode: 0x2129, turned iota
    "Angstrom" => Symbol { unicode: 0x212B, atom_type: AtomType::Alpha }, // Unicode: 0x212B, angstrom capital a, ring
    "mscrB" => Symbol { unicode: 0x212C, atom_type: AtomType::Alpha }, // Unicode: 0x212C, bernoulli function (script capital b)
    "mfrakC" => Symbol { unicode: 0x212D, atom_type: AtomType::Alpha }, // Unicode: 0x212D, black-letter capital c
    "mscre" => Symbol { unicode: 0x212F, atom_type: AtomType::Alpha }, // Unicode: 0x212F, /scr e, script letter e
    "mscrE" => Symbol { unicode: 0x2130, atom_type: AtomType::Alpha }, // Unicode: 0x2130, /scr e, script letter e
    "mscrF" => Symbol { unicode: 0x2131, atom_type: AtomType::Alpha }, // Unicode: 0x2131, /scr f, script letter f
    "Finv" => Symbol { unicode: 0x2132, atom_type: AtomType::Alpha }, // Unicode: 0x2132, turned capital f
    "mscrM" => Symbol { unicode: 0x2133, atom_type: AtomType::Alpha }, // Unicode: 0x2133, physics m-matrix (script capital m)
    "mscro" => Symbol { unicode: 0x2134, atom_type: AtomType::Alpha }, // Unicode: 0x2134, order of (script small o)
    "aleph" => Symbol { unicode: 0x2135, atom_type: AtomType::Alpha }, // Unicode: 0x2135, aleph, hebrew
    "beth" => Symbol { unicode: 0x2136, atom_type: AtomType::Alpha }, // Unicode: 0x2136, beth, hebrew
    "gimel" => Symbol { unicode: 0x2137, atom_type: AtomType::Alpha }, // Unicode: 0x2137, gimel, hebrew
    "daleth" => Symbol { unicode: 0x2138, atom_type: AtomType::Alpha }, // Unicode: 0x2138, daleth, hebrew
    "Bbbpi" => Symbol { unicode: 0x213C, atom_type: AtomType::Alpha }, // Unicode: 0x213C, double-struck small pi
    "Bbbgamma" => Symbol { unicode: 0x213D, atom_type: AtomType::Alpha }, // Unicode: 0x213D, double-struck small gamma
    "BbbGamma" => Symbol { unicode: 0x213E, atom_type: AtomType::Alpha }, // Unicode: 0x213E, double-struck capital gamma
    "BbbPi" => Symbol { unicode: 0x213F, atom_type: AtomType::Alpha }, // Unicode: 0x213F, double-struck capital pi
    "Bbbsum" => Symbol { unicode: 0x2140, atom_type: AtomType::Operator(false) }, // Unicode: 0x2140, double-struck n-ary summation
    "Game" => Symbol { unicode: 0x2141, atom_type: AtomType::Alpha }, // Unicode: 0x2141, turned sans-serif capital g
    "sansLturned" => Symbol { unicode: 0x2142, atom_type: AtomType::Alpha }, // Unicode: 0x2142, turned sans-serif capital l
    "sansLmirrored" => Symbol { unicode: 0x2143, atom_type: AtomType::Alpha }, // Unicode: 0x2143, reversed sans-serif capital l
    "Yup" => Symbol { unicode: 0x2144, atom_type: AtomType::Alpha }, // Unicode: 0x2144, turned sans-serif capital y
    "mitBbbD" => Symbol { unicode: 0x2145, atom_type: AtomType::Alpha }, // Unicode: 0x2145, double-struck italic capital d
    "mitBbbd" => Symbol { unicode: 0x2146, atom_type: AtomType::Alpha }, // Unicode: 0x2146, double-struck italic small d
    "mitBbbe" => Symbol { unicode: 0x2147, atom_type: AtomType::Alpha }, // Unicode: 0x2147, double-struck italic small e
    "mitBbbi" => Symbol { unicode: 0x2148, atom_type: AtomType::Alpha }, // Unicode: 0x2148, double-struck italic small i
    "mitBbbj" => Symbol { unicode: 0x2149, atom_type: AtomType::Alpha }, // Unicode: 0x2149, double-struck italic small j
    "PropertyLine" => Symbol { unicode: 0x214A, atom_type: AtomType::Alpha }, // Unicode: 0x214A, property line
    "upand" => Symbol { unicode: 0x214B, atom_type: AtomType::Binary }, // Unicode: 0x214B, turned ampersand
    "leftarrow" => Symbol { unicode: 0x2190, atom_type: AtomType::Relation }, // Unicode: 0x2190, /leftarrow /gets a: leftward arrow
    "uparrow" => Symbol { unicode: 0x2191, atom_type: AtomType::Relation }, // Unicode: 0x2191, upward arrow
    "rightarrow" => Symbol { unicode: 0x2192, atom_type: AtomType::Relation }, // Unicode: 0x2192, /rightarrow /to a: rightward arrow
    "to" => Symbol { unicode: 0x2192, atom_type: AtomType::Relation }, // Unicode: 0x2192, /rightarrow /to a: rightward arrow
    "downarrow" => Symbol { unicode: 0x2193, atom_type: AtomType::Relation }, // Unicode: 0x2193, downward arrow
    "leftrightarrow" => Symbol { unicode: 0x2194, atom_type: AtomType::Relation }, // Unicode: 0x2194, left and right arrow
    "updownarrow" => Symbol { unicode: 0x2195, atom_type: AtomType::Relation }, // Unicode: 0x2195, up and down arrow
    "nwarrow" => Symbol { unicode: 0x2196, atom_type: AtomType::Relation }, // Unicode: 0x2196, nw pointing arrow
    "nearrow" => Symbol { unicode: 0x2197, atom_type: AtomType::Relation }, // Unicode: 0x2197, ne pointing arrow
    "searrow" => Symbol { unicode: 0x2198, atom_type: AtomType::Relation }, // Unicode: 0x2198, se pointing arrow
    "swarrow" => Symbol { unicode: 0x2199, atom_type: AtomType::Relation }, // Unicode: 0x2199, sw pointing arrow
    "nleftarrow" => Symbol { unicode: 0x219A, atom_type: AtomType::Relation }, // Unicode: 0x219A, not left arrow
    "nrightarrow" => Symbol { unicode: 0x219B, atom_type: AtomType::Relation }, // Unicode: 0x219B, not right arrow
    "leftwavearrow" => Symbol { unicode: 0x219C, atom_type: AtomType::Relation }, // Unicode: 0x219C, left arrow-wavy
    "rightwavearrow" => Symbol { unicode: 0x219D, atom_type: AtomType::Relation }, // Unicode: 0x219D, right arrow-wavy
    "twoheadleftarrow" => Symbol { unicode: 0x219E, atom_type: AtomType::Relation }, // Unicode: 0x219E, left two-headed arrow
    "twoheaduparrow" => Symbol { unicode: 0x219F, atom_type: AtomType::Relation }, // Unicode: 0x219F, up two-headed arrow
    "twoheadrightarrow" => Symbol { unicode: 0x21A0, atom_type: AtomType::Relation }, // Unicode: 0x21A0, right two-headed arrow
    "twoheaddownarrow" => Symbol { unicode: 0x21A1, atom_type: AtomType::Relation }, // Unicode: 0x21A1, down two-headed arrow
    "leftarrowtail" => Symbol { unicode: 0x21A2, atom_type: AtomType::Relation }, // Unicode: 0x21A2, left arrow-tailed
    "rightarrowtail" => Symbol { unicode: 0x21A3, atom_type: AtomType::Relation }, // Unicode: 0x21A3, right arrow-tailed
    "mapsfrom" => Symbol { unicode: 0x21A4, atom_type: AtomType::Relation }, // Unicode: 0x21A4, maps to, leftward
    "mapsup" => Symbol { unicode: 0x21A5, atom_type: AtomType::Relation }, // Unicode: 0x21A5, maps to, upward
    "mapsto" => Symbol { unicode: 0x21A6, atom_type: AtomType::Relation }, // Unicode: 0x21A6, maps to, rightward
    "mapsdown" => Symbol { unicode: 0x21A7, atom_type: AtomType::Relation }, // Unicode: 0x21A7, maps to, downward
    "updownarrowbar" => Symbol { unicode: 0x21A8, atom_type: AtomType::Alpha }, // Unicode: 0x21A8, up down arrow with base (perpendicular)
    "hookleftarrow" => Symbol { unicode: 0x21A9, atom_type: AtomType::Relation }, // Unicode: 0x21A9, left arrow-hooked
    "hookrightarrow" => Symbol { unicode: 0x21AA, atom_type: AtomType::Relation }, // Unicode: 0x21AA, right arrow-hooked
    "looparrowleft" => Symbol { unicode: 0x21AB, atom_type: AtomType::Relation }, // Unicode: 0x21AB, left arrow-looped
    "looparrowright" => Symbol { unicode: 0x21AC, atom_type: AtomType::Relation }, // Unicode: 0x21AC, right arrow-looped
    "leftrightsquigarrow" => Symbol { unicode: 0x21AD, atom_type: AtomType::Relation }, // Unicode: 0x21AD, left and right arr-wavy
    "nleftrightarrow" => Symbol { unicode: 0x21AE, atom_type: AtomType::Relation }, // Unicode: 0x21AE, not left and right arrow
    "downzigzagarrow" => Symbol { unicode: 0x21AF, atom_type: AtomType::Relation }, // Unicode: 0x21AF, downwards zigzag arrow
    "Lsh" => Symbol { unicode: 0x21B0, atom_type: AtomType::Relation }, // Unicode: 0x21B0, /lsh a:
    "Rsh" => Symbol { unicode: 0x21B1, atom_type: AtomType::Relation }, // Unicode: 0x21B1, /rsh a:
    "Ldsh" => Symbol { unicode: 0x21B2, atom_type: AtomType::Relation }, // Unicode: 0x21B2, left down angled arrow
    "Rdsh" => Symbol { unicode: 0x21B3, atom_type: AtomType::Relation }, // Unicode: 0x21B3, right down angled arrow
    "linefeed" => Symbol { unicode: 0x21B4, atom_type: AtomType::Alpha }, // Unicode: 0x21B4, rightwards arrow with corner downwards
    "carriagereturn" => Symbol { unicode: 0x21B5, atom_type: AtomType::Alpha }, // Unicode: 0x21B5, downwards arrow with corner leftward = carriage return
    "curvearrowleft" => Symbol { unicode: 0x21B6, atom_type: AtomType::Relation }, // Unicode: 0x21B6, left curved arrow
    "curvearrowright" => Symbol { unicode: 0x21B7, atom_type: AtomType::Relation }, // Unicode: 0x21B7, right curved arrow
    "barovernorthwestarrow" => Symbol { unicode: 0x21B8, atom_type: AtomType::Alpha }, // Unicode: 0x21B8, north west arrow to long bar
    "barleftarrowrightarrowbar" => Symbol { unicode: 0x21B9, atom_type: AtomType::Alpha }, // Unicode: 0x21B9, leftwards arrow to bar over rightwards arrow to bar
    "acwopencirclearrow" => Symbol { unicode: 0x21BA, atom_type: AtomType::Alpha }, // Unicode: 0x21BA, anticlockwise open circle arrow
    "cwopencirclearrow" => Symbol { unicode: 0x21BB, atom_type: AtomType::Alpha }, // Unicode: 0x21BB, clockwise open circle arrow
    "leftharpoonup" => Symbol { unicode: 0x21BC, atom_type: AtomType::Relation }, // Unicode: 0x21BC, left harpoon-up
    "leftharpoondown" => Symbol { unicode: 0x21BD, atom_type: AtomType::Relation }, // Unicode: 0x21BD, left harpoon-down
    "upharpoonright" => Symbol { unicode: 0x21BE, atom_type: AtomType::Relation }, // Unicode: 0x21BE, /upharpoonright /restriction a: up harpoon-right
    "upharpoonleft" => Symbol { unicode: 0x21BF, atom_type: AtomType::Relation }, // Unicode: 0x21BF, up harpoon-left
    "rightharpoonup" => Symbol { unicode: 0x21C0, atom_type: AtomType::Relation }, // Unicode: 0x21C0, right harpoon-up
    "rightharpoondown" => Symbol { unicode: 0x21C1, atom_type: AtomType::Relation }, // Unicode: 0x21C1, right harpoon-down
    "downharpoonright" => Symbol { unicode: 0x21C2, atom_type: AtomType::Relation }, // Unicode: 0x21C2, down harpoon-right
    "downharpoonleft" => Symbol { unicode: 0x21C3, atom_type: AtomType::Relation }, // Unicode: 0x21C3, down harpoon-left
    "rightleftarrows" => Symbol { unicode: 0x21C4, atom_type: AtomType::Relation }, // Unicode: 0x21C4, right arrow over left arrow
    "updownarrows" => Symbol { unicode: 0x21C5, atom_type: AtomType::Relation }, // Unicode: 0x21C5, up arrow, down arrow
    "leftrightarrows" => Symbol { unicode: 0x21C6, atom_type: AtomType::Relation }, // Unicode: 0x21C6, left arrow over right arrow
    "leftleftarrows" => Symbol { unicode: 0x21C7, atom_type: AtomType::Relation }, // Unicode: 0x21C7, two left arrows
    "upuparrows" => Symbol { unicode: 0x21C8, atom_type: AtomType::Relation }, // Unicode: 0x21C8, two up arrows
    "rightrightarrows" => Symbol { unicode: 0x21C9, atom_type: AtomType::Relation }, // Unicode: 0x21C9, two right arrows
    "downdownarrows" => Symbol { unicode: 0x21CA, atom_type: AtomType::Relation }, // Unicode: 0x21CA, two down arrows
    "leftrightharpoons" => Symbol { unicode: 0x21CB, atom_type: AtomType::Relation }, // Unicode: 0x21CB, left harpoon over right
    "rightleftharpoons" => Symbol { unicode: 0x21CC, atom_type: AtomType::Relation }, // Unicode: 0x21CC, right harpoon over left
    "nLeftarrow" => Symbol { unicode: 0x21CD, atom_type: AtomType::Relation }, // Unicode: 0x21CD, not implied by
    "nLeftrightarrow" => Symbol { unicode: 0x21CE, atom_type: AtomType::Relation }, // Unicode: 0x21CE, not left and right double arrows
    "nRightarrow" => Symbol { unicode: 0x21CF, atom_type: AtomType::Relation }, // Unicode: 0x21CF, not implies
    "Leftarrow" => Symbol { unicode: 0x21D0, atom_type: AtomType::Relation }, // Unicode: 0x21D0, is implied by
    "Uparrow" => Symbol { unicode: 0x21D1, atom_type: AtomType::Relation }, // Unicode: 0x21D1, up double arrow
    "Rightarrow" => Symbol { unicode: 0x21D2, atom_type: AtomType::Relation }, // Unicode: 0x21D2, implies
    "Downarrow" => Symbol { unicode: 0x21D3, atom_type: AtomType::Relation }, // Unicode: 0x21D3, down double arrow
    "Leftrightarrow" => Symbol { unicode: 0x21D4, atom_type: AtomType::Relation }, // Unicode: 0x21D4, left and right double arrow
    "Updownarrow" => Symbol { unicode: 0x21D5, atom_type: AtomType::Relation }, // Unicode: 0x21D5, up and down double arrow
    "Nwarrow" => Symbol { unicode: 0x21D6, atom_type: AtomType::Relation }, // Unicode: 0x21D6, nw pointing double arrow
    "Nearrow" => Symbol { unicode: 0x21D7, atom_type: AtomType::Relation }, // Unicode: 0x21D7, ne pointing double arrow
    "Searrow" => Symbol { unicode: 0x21D8, atom_type: AtomType::Relation }, // Unicode: 0x21D8, se pointing double arrow
    "Swarrow" => Symbol { unicode: 0x21D9, atom_type: AtomType::Relation }, // Unicode: 0x21D9, sw pointing double arrow
    "Lleftarrow" => Symbol { unicode: 0x21DA, atom_type: AtomType::Relation }, // Unicode: 0x21DA, left triple arrow
    "Rrightarrow" => Symbol { unicode: 0x21DB, atom_type: AtomType::Relation }, // Unicode: 0x21DB, right triple arrow
    "leftsquigarrow" => Symbol { unicode: 0x21DC, atom_type: AtomType::Relation }, // Unicode: 0x21DC, leftwards squiggle arrow
    "rightsquigarrow" => Symbol { unicode: 0x21DD, atom_type: AtomType::Relation }, // Unicode: 0x21DD, rightwards squiggle arrow
    "nHuparrow" => Symbol { unicode: 0x21DE, atom_type: AtomType::Alpha }, // Unicode: 0x21DE, upwards arrow with double stroke
    "nHdownarrow" => Symbol { unicode: 0x21DF, atom_type: AtomType::Alpha }, // Unicode: 0x21DF, downwards arrow with double stroke
    "leftdasharrow" => Symbol { unicode: 0x21E0, atom_type: AtomType::Alpha }, // Unicode: 0x21E0, leftwards dashed arrow
    "updasharrow" => Symbol { unicode: 0x21E1, atom_type: AtomType::Alpha }, // Unicode: 0x21E1, upwards dashed arrow
    "rightdasharrow" => Symbol { unicode: 0x21E2, atom_type: AtomType::Alpha }, // Unicode: 0x21E2, rightwards dashed arrow
    "downdasharrow" => Symbol { unicode: 0x21E3, atom_type: AtomType::Alpha }, // Unicode: 0x21E3, downwards dashed arrow
    "barleftarrow" => Symbol { unicode: 0x21E4, atom_type: AtomType::Relation }, // Unicode: 0x21E4, leftwards arrow to bar
    "rightarrowbar" => Symbol { unicode: 0x21E5, atom_type: AtomType::Relation }, // Unicode: 0x21E5, rightwards arrow to bar
    "leftwhitearrow" => Symbol { unicode: 0x21E6, atom_type: AtomType::Alpha }, // Unicode: 0x21E6, leftwards white arrow
    "upwhitearrow" => Symbol { unicode: 0x21E7, atom_type: AtomType::Alpha }, // Unicode: 0x21E7, upwards white arrow
    "rightwhitearrow" => Symbol { unicode: 0x21E8, atom_type: AtomType::Alpha }, // Unicode: 0x21E8, rightwards white arrow
    "downwhitearrow" => Symbol { unicode: 0x21E9, atom_type: AtomType::Alpha }, // Unicode: 0x21E9, downwards white arrow
    "whitearrowupfrombar" => Symbol { unicode: 0x21EA, atom_type: AtomType::Alpha }, // Unicode: 0x21EA, upwards white arrow from bar
    "circleonrightarrow" => Symbol { unicode: 0x21F4, atom_type: AtomType::Relation }, // Unicode: 0x21F4, right arrow with small circle
    "downuparrows" => Symbol { unicode: 0x21F5, atom_type: AtomType::Relation }, // Unicode: 0x21F5, downwards arrow leftwards of upwards arrow
    "rightthreearrows" => Symbol { unicode: 0x21F6, atom_type: AtomType::Relation }, // Unicode: 0x21F6, three rightwards arrows
    "nvleftarrow" => Symbol { unicode: 0x21F7, atom_type: AtomType::Relation }, // Unicode: 0x21F7, leftwards arrow with vertical stroke
    "nvrightarrow" => Symbol { unicode: 0x21F8, atom_type: AtomType::Relation }, // Unicode: 0x21F8, rightwards arrow with vertical stroke
    "nvleftrightarrow" => Symbol { unicode: 0x21F9, atom_type: AtomType::Relation }, // Unicode: 0x21F9, left right arrow with vertical stroke
    "nVleftarrow" => Symbol { unicode: 0x21FA, atom_type: AtomType::Relation }, // Unicode: 0x21FA, leftwards arrow with double vertical stroke
    "nVrightarrow" => Symbol { unicode: 0x21FB, atom_type: AtomType::Relation }, // Unicode: 0x21FB, rightwards arrow with double vertical stroke
    "nVleftrightarrow" => Symbol { unicode: 0x21FC, atom_type: AtomType::Relation }, // Unicode: 0x21FC, left right arrow with double vertical stroke
    "leftarrowtriangle" => Symbol { unicode: 0x21FD, atom_type: AtomType::Relation }, // Unicode: 0x21FD, leftwards open-headed arrow
    "rightarrowtriangle" => Symbol { unicode: 0x21FE, atom_type: AtomType::Relation }, // Unicode: 0x21FE, rightwards open-headed arrow
    "leftrightarrowtriangle" => Symbol { unicode: 0x21FF, atom_type: AtomType::Relation }, // Unicode: 0x21FF, left right open-headed arrow
    "forall" => Symbol { unicode: 0x2200, atom_type: AtomType::Alpha }, // Unicode: 0x2200, for all
    "complement" => Symbol { unicode: 0x2201, atom_type: AtomType::Alpha }, // Unicode: 0x2201, complement sign
    "partial" => Symbol { unicode: 0x2202, atom_type: AtomType::Alpha }, // Unicode: 0x2202, partial differential
    "exists" => Symbol { unicode: 0x2203, atom_type: AtomType::Alpha }, // Unicode: 0x2203, at least one exists
    "nexists" => Symbol { unicode: 0x2204, atom_type: AtomType::Alpha }, // Unicode: 0x2204, negated exists
    "varnothing" => Symbol { unicode: 0x2205, atom_type: AtomType::Alpha }, // Unicode: 0x2205, circle, slash
    "increment" => Symbol { unicode: 0x2206, atom_type: AtomType::Alpha }, // Unicode: 0x2206, laplacian (delta; nabla\string^2)
    "nabla" => Symbol { unicode: 0x2207, atom_type: AtomType::Alpha }, // Unicode: 0x2207, nabla, del, hamilton operator
    "in" => Symbol { unicode: 0x2208, atom_type: AtomType::Relation }, // Unicode: 0x2208, set membership, variant
    "notin" => Symbol { unicode: 0x2209, atom_type: AtomType::Relation }, // Unicode: 0x2209, negated set membership
    "smallin" => Symbol { unicode: 0x220A, atom_type: AtomType::Relation }, // Unicode: 0x220A, set membership (small set membership)
    "ni" => Symbol { unicode: 0x220B, atom_type: AtomType::Relation }, // Unicode: 0x220B, contains, variant
    "nni" => Symbol { unicode: 0x220C, atom_type: AtomType::Relation }, // Unicode: 0x220C, negated contains, variant
    "smallni" => Symbol { unicode: 0x220D, atom_type: AtomType::Relation }, // Unicode: 0x220D, /ni /owns r: contains (small contains as member)
    "QED" => Symbol { unicode: 0x220E, atom_type: AtomType::Alpha }, // Unicode: 0x220E, end of proof
    "prod" => Symbol { unicode: 0x220F, atom_type: AtomType::Operator(true) }, // Unicode: 0x220F, product operator
    "coprod" => Symbol { unicode: 0x2210, atom_type: AtomType::Operator(true) }, // Unicode: 0x2210, coproduct operator
    "sum" => Symbol { unicode: 0x2211, atom_type: AtomType::Operator(true) }, // Unicode: 0x2211, summation operator
    "minus" => Symbol { unicode: 0x2212, atom_type: AtomType::Binary }, // Unicode: 0x2212, minus sign
    "mp" => Symbol { unicode: 0x2213, atom_type: AtomType::Binary }, // Unicode: 0x2213, minus-or-plus sign
    "dotplus" => Symbol { unicode: 0x2214, atom_type: AtomType::Binary }, // Unicode: 0x2214, plus sign, dot above
    "divslash" => Symbol { unicode: 0x2215, atom_type: AtomType::Binary }, // Unicode: 0x2215, division slash
    "smallsetminus" => Symbol { unicode: 0x2216, atom_type: AtomType::Binary }, // Unicode: 0x2216, small set minus (cf. reverse solidus)
    "ast" => Symbol { unicode: 0x2217, atom_type: AtomType::Binary }, // Unicode: 0x2217, centered asterisk
    "vysmwhtcircle" => Symbol { unicode: 0x2218, atom_type: AtomType::Binary }, // Unicode: 0x2218, composite function (small circle)
    "vysmblkcircle" => Symbol { unicode: 0x2219, atom_type: AtomType::Binary }, // Unicode: 0x2219, bullet operator
    "sqrt" => Symbol { unicode: 0x221A, atom_type: AtomType::Open }, // Unicode: 0x221A, radical
    "cuberoot" => Symbol { unicode: 0x221B, atom_type: AtomType::Open }, // Unicode: 0x221B, cube root
    "fourthroot" => Symbol { unicode: 0x221C, atom_type: AtomType::Open }, // Unicode: 0x221C, fourth root
    "propto" => Symbol { unicode: 0x221D, atom_type: AtomType::Relation }, // Unicode: 0x221D, is proportional to
    "infty" => Symbol { unicode: 0x221E, atom_type: AtomType::Alpha }, // Unicode: 0x221E, infinity
    "rightangle" => Symbol { unicode: 0x221F, atom_type: AtomType::Alpha }, // Unicode: 0x221F, right (90 degree) angle
    "angle" => Symbol { unicode: 0x2220, atom_type: AtomType::Alpha }, // Unicode: 0x2220, angle
    "measuredangle" => Symbol { unicode: 0x2221, atom_type: AtomType::Alpha }, // Unicode: 0x2221, angle-measured
    "sphericalangle" => Symbol { unicode: 0x2222, atom_type: AtomType::Alpha }, // Unicode: 0x2222, angle-spherical
    "mid" => Symbol { unicode: 0x2223, atom_type: AtomType::Relation }, // Unicode: 0x2223, /mid r:
    "nmid" => Symbol { unicode: 0x2224, atom_type: AtomType::Relation }, // Unicode: 0x2224, negated mid
    "parallel" => Symbol { unicode: 0x2225, atom_type: AtomType::Relation }, // Unicode: 0x2225, parallel
    "nparallel" => Symbol { unicode: 0x2226, atom_type: AtomType::Relation }, // Unicode: 0x2226, not parallel
    "wedge" => Symbol { unicode: 0x2227, atom_type: AtomType::Binary }, // Unicode: 0x2227, /wedge /land b: logical and
    "vee" => Symbol { unicode: 0x2228, atom_type: AtomType::Binary }, // Unicode: 0x2228, /vee /lor b: logical or
    "cap" => Symbol { unicode: 0x2229, atom_type: AtomType::Binary }, // Unicode: 0x2229, intersection
    "cup" => Symbol { unicode: 0x222A, atom_type: AtomType::Binary }, // Unicode: 0x222A, union or logical sum
    "int" => Symbol { unicode: 0x222B, atom_type: AtomType::Operator(false) }, // Unicode: 0x222B, integral operator
    "iint" => Symbol { unicode: 0x222C, atom_type: AtomType::Operator(false) }, // Unicode: 0x222C, double integral operator
    "iiint" => Symbol { unicode: 0x222D, atom_type: AtomType::Operator(false) }, // Unicode: 0x222D, triple integral operator
    "oint" => Symbol { unicode: 0x222E, atom_type: AtomType::Operator(false) }, // Unicode: 0x222E, contour integral operator
    "oiint" => Symbol { unicode: 0x222F, atom_type: AtomType::Operator(false) }, // Unicode: 0x222F, double contour integral operator
    "oiiint" => Symbol { unicode: 0x2230, atom_type: AtomType::Operator(false) }, // Unicode: 0x2230, triple contour integral operator
    "intclockwise" => Symbol { unicode: 0x2231, atom_type: AtomType::Operator(false) }, // Unicode: 0x2231, clockwise integral
    "varointclockwise" => Symbol { unicode: 0x2232, atom_type: AtomType::Operator(false) }, // Unicode: 0x2232, contour integral, clockwise
    "ointctrclockwise" => Symbol { unicode: 0x2233, atom_type: AtomType::Operator(false) }, // Unicode: 0x2233, contour integral, anticlockwise
    "therefore" => Symbol { unicode: 0x2234, atom_type: AtomType::Alpha }, // Unicode: 0x2234, therefore
    "because" => Symbol { unicode: 0x2235, atom_type: AtomType::Alpha }, // Unicode: 0x2235, because
    "mathratio" => Symbol { unicode: 0x2236, atom_type: AtomType::Relation }, // Unicode: 0x2236, ratio
    "Colon" => Symbol { unicode: 0x2237, atom_type: AtomType::Relation }, // Unicode: 0x2237, two colons
    "dotminus" => Symbol { unicode: 0x2238, atom_type: AtomType::Binary }, // Unicode: 0x2238, minus sign, dot above
    "dashcolon" => Symbol { unicode: 0x2239, atom_type: AtomType::Relation }, // Unicode: 0x2239, excess (-:)
    "dotsminusdots" => Symbol { unicode: 0x223A, atom_type: AtomType::Relation }, // Unicode: 0x223A, minus with four dots, geometric properties
    "kernelcontraction" => Symbol { unicode: 0x223B, atom_type: AtomType::Relation }, // Unicode: 0x223B, homothetic
    "sim" => Symbol { unicode: 0x223C, atom_type: AtomType::Relation }, // Unicode: 0x223C, similar
    "backsim" => Symbol { unicode: 0x223D, atom_type: AtomType::Relation }, // Unicode: 0x223D, reverse similar
    "invlazys" => Symbol { unicode: 0x223E, atom_type: AtomType::Binary }, // Unicode: 0x223E, most positive [inverted lazy s]
    "sinewave" => Symbol { unicode: 0x223F, atom_type: AtomType::Alpha }, // Unicode: 0x223F, sine wave
    "wr" => Symbol { unicode: 0x2240, atom_type: AtomType::Binary }, // Unicode: 0x2240, wreath product
    "nsim" => Symbol { unicode: 0x2241, atom_type: AtomType::Relation }, // Unicode: 0x2241, not similar
    "eqsim" => Symbol { unicode: 0x2242, atom_type: AtomType::Relation }, // Unicode: 0x2242, equals, similar
    "simeq" => Symbol { unicode: 0x2243, atom_type: AtomType::Relation }, // Unicode: 0x2243, similar, equals
    "nsime" => Symbol { unicode: 0x2244, atom_type: AtomType::Relation }, // Unicode: 0x2244, not similar, equals
    "cong" => Symbol { unicode: 0x2245, atom_type: AtomType::Relation }, // Unicode: 0x2245, congruent with
    "simneqq" => Symbol { unicode: 0x2246, atom_type: AtomType::Relation }, // Unicode: 0x2246, similar, not equals [vert only for 9573 entity]
    "ncong" => Symbol { unicode: 0x2247, atom_type: AtomType::Relation }, // Unicode: 0x2247, not congruent with
    "approx" => Symbol { unicode: 0x2248, atom_type: AtomType::Relation }, // Unicode: 0x2248, approximate
    "napprox" => Symbol { unicode: 0x2249, atom_type: AtomType::Relation }, // Unicode: 0x2249, not approximate
    "approxeq" => Symbol { unicode: 0x224A, atom_type: AtomType::Relation }, // Unicode: 0x224A, approximate, equals
    "approxident" => Symbol { unicode: 0x224B, atom_type: AtomType::Relation }, // Unicode: 0x224B, approximately identical to
    "backcong" => Symbol { unicode: 0x224C, atom_type: AtomType::Relation }, // Unicode: 0x224C, all equal to
    "asymp" => Symbol { unicode: 0x224D, atom_type: AtomType::Relation }, // Unicode: 0x224D, asymptotically equal to
    "Bumpeq" => Symbol { unicode: 0x224E, atom_type: AtomType::Relation }, // Unicode: 0x224E, bumpy equals
    "bumpeq" => Symbol { unicode: 0x224F, atom_type: AtomType::Relation }, // Unicode: 0x224F, bumpy equals, equals
    "doteq" => Symbol { unicode: 0x2250, atom_type: AtomType::Relation }, // Unicode: 0x2250, equals, single dot above
    "Doteq" => Symbol { unicode: 0x2251, atom_type: AtomType::Relation }, // Unicode: 0x2251, /doteqdot /doteq r: equals, even dots
    "fallingdotseq" => Symbol { unicode: 0x2252, atom_type: AtomType::Relation }, // Unicode: 0x2252, equals, falling dots
    "risingdotseq" => Symbol { unicode: 0x2253, atom_type: AtomType::Relation }, // Unicode: 0x2253, equals, rising dots
    "coloneq" => Symbol { unicode: 0x2254, atom_type: AtomType::Relation }, // Unicode: 0x2254, colon, equals
    "eqcolon" => Symbol { unicode: 0x2255, atom_type: AtomType::Relation }, // Unicode: 0x2255, equals, colon
    "eqcirc" => Symbol { unicode: 0x2256, atom_type: AtomType::Relation }, // Unicode: 0x2256, circle on equals sign
    "circeq" => Symbol { unicode: 0x2257, atom_type: AtomType::Relation }, // Unicode: 0x2257, circle, equals
    "arceq" => Symbol { unicode: 0x2258, atom_type: AtomType::Relation }, // Unicode: 0x2258, arc, equals; corresponds to
    "wedgeq" => Symbol { unicode: 0x2259, atom_type: AtomType::Relation }, // Unicode: 0x2259, corresponds to (wedge, equals)
    "veeeq" => Symbol { unicode: 0x225A, atom_type: AtomType::Relation }, // Unicode: 0x225A, logical or, equals
    "stareq" => Symbol { unicode: 0x225B, atom_type: AtomType::Relation }, // Unicode: 0x225B, star equals
    "triangleq" => Symbol { unicode: 0x225C, atom_type: AtomType::Relation }, // Unicode: 0x225C, triangle, equals
    "eqdef" => Symbol { unicode: 0x225D, atom_type: AtomType::Relation }, // Unicode: 0x225D, equals by definition
    "measeq" => Symbol { unicode: 0x225E, atom_type: AtomType::Relation }, // Unicode: 0x225E, measured by (m over equals)
    "questeq" => Symbol { unicode: 0x225F, atom_type: AtomType::Relation }, // Unicode: 0x225F, equal with questionmark
    "ne" => Symbol { unicode: 0x2260, atom_type: AtomType::Relation }, // Unicode: 0x2260, /ne /neq r: not equal
    "equiv" => Symbol { unicode: 0x2261, atom_type: AtomType::Relation }, // Unicode: 0x2261, identical with
    "nequiv" => Symbol { unicode: 0x2262, atom_type: AtomType::Relation }, // Unicode: 0x2262, not identical with
    "Equiv" => Symbol { unicode: 0x2263, atom_type: AtomType::Relation }, // Unicode: 0x2263, strict equivalence (4 lines)
    "leq" => Symbol { unicode: 0x2264, atom_type: AtomType::Relation }, // Unicode: 0x2264, /leq /le r: less-than-or-equal
    "geq" => Symbol { unicode: 0x2265, atom_type: AtomType::Relation }, // Unicode: 0x2265, /geq /ge r: greater-than-or-equal
    "leqq" => Symbol { unicode: 0x2266, atom_type: AtomType::Relation }, // Unicode: 0x2266, less, double equals
    "geqq" => Symbol { unicode: 0x2267, atom_type: AtomType::Relation }, // Unicode: 0x2267, greater, double equals
    "lneqq" => Symbol { unicode: 0x2268, atom_type: AtomType::Relation }, // Unicode: 0x2268, less, not double equals
    "gneqq" => Symbol { unicode: 0x2269, atom_type: AtomType::Relation }, // Unicode: 0x2269, greater, not double equals
    "ll" => Symbol { unicode: 0x226A, atom_type: AtomType::Relation }, // Unicode: 0x226A, much less than, type 2
    "gg" => Symbol { unicode: 0x226B, atom_type: AtomType::Relation }, // Unicode: 0x226B, much greater than, type 2
    "between" => Symbol { unicode: 0x226C, atom_type: AtomType::Relation }, // Unicode: 0x226C, between
    "nasymp" => Symbol { unicode: 0x226D, atom_type: AtomType::Relation }, // Unicode: 0x226D, not asymptotically equal to
    "nless" => Symbol { unicode: 0x226E, atom_type: AtomType::Relation }, // Unicode: 0x226E, not less-than
    "ngtr" => Symbol { unicode: 0x226F, atom_type: AtomType::Relation }, // Unicode: 0x226F, not greater-than
    "nleq" => Symbol { unicode: 0x2270, atom_type: AtomType::Relation }, // Unicode: 0x2270, not less-than-or-equal
    "ngeq" => Symbol { unicode: 0x2271, atom_type: AtomType::Relation }, // Unicode: 0x2271, not greater-than-or-equal
    "lesssim" => Symbol { unicode: 0x2272, atom_type: AtomType::Relation }, // Unicode: 0x2272, less, similar
    "gtrsim" => Symbol { unicode: 0x2273, atom_type: AtomType::Relation }, // Unicode: 0x2273, greater, similar
    "nlesssim" => Symbol { unicode: 0x2274, atom_type: AtomType::Relation }, // Unicode: 0x2274, not less, similar
    "ngtrsim" => Symbol { unicode: 0x2275, atom_type: AtomType::Relation }, // Unicode: 0x2275, not greater, similar
    "lessgtr" => Symbol { unicode: 0x2276, atom_type: AtomType::Relation }, // Unicode: 0x2276, less, greater
    "gtrless" => Symbol { unicode: 0x2277, atom_type: AtomType::Relation }, // Unicode: 0x2277, greater, less
    "nlessgtr" => Symbol { unicode: 0x2278, atom_type: AtomType::Relation }, // Unicode: 0x2278, not less, greater
    "ngtrless" => Symbol { unicode: 0x2279, atom_type: AtomType::Relation }, // Unicode: 0x2279, not greater, less
    "prec" => Symbol { unicode: 0x227A, atom_type: AtomType::Relation }, // Unicode: 0x227A, precedes
    "succ" => Symbol { unicode: 0x227B, atom_type: AtomType::Relation }, // Unicode: 0x227B, succeeds
    "preccurlyeq" => Symbol { unicode: 0x227C, atom_type: AtomType::Relation }, // Unicode: 0x227C, precedes, curly equals
    "succcurlyeq" => Symbol { unicode: 0x227D, atom_type: AtomType::Relation }, // Unicode: 0x227D, succeeds, curly equals
    "precsim" => Symbol { unicode: 0x227E, atom_type: AtomType::Relation }, // Unicode: 0x227E, precedes, similar
    "succsim" => Symbol { unicode: 0x227F, atom_type: AtomType::Relation }, // Unicode: 0x227F, succeeds, similar
    "nprec" => Symbol { unicode: 0x2280, atom_type: AtomType::Relation }, // Unicode: 0x2280, not precedes
    "nsucc" => Symbol { unicode: 0x2281, atom_type: AtomType::Relation }, // Unicode: 0x2281, not succeeds
    "subset" => Symbol { unicode: 0x2282, atom_type: AtomType::Relation }, // Unicode: 0x2282, subset or is implied by
    "supset" => Symbol { unicode: 0x2283, atom_type: AtomType::Relation }, // Unicode: 0x2283, superset or implies
    "nsubset" => Symbol { unicode: 0x2284, atom_type: AtomType::Relation }, // Unicode: 0x2284, not subset, variant [slash negation]
    "nsupset" => Symbol { unicode: 0x2285, atom_type: AtomType::Relation }, // Unicode: 0x2285, not superset, variant [slash negation]
    "subseteq" => Symbol { unicode: 0x2286, atom_type: AtomType::Relation }, // Unicode: 0x2286, subset, equals
    "supseteq" => Symbol { unicode: 0x2287, atom_type: AtomType::Relation }, // Unicode: 0x2287, superset, equals
    "nsubseteq" => Symbol { unicode: 0x2288, atom_type: AtomType::Relation }, // Unicode: 0x2288, not subset, equals
    "nsupseteq" => Symbol { unicode: 0x2289, atom_type: AtomType::Relation }, // Unicode: 0x2289, not superset, equals
    "subsetneq" => Symbol { unicode: 0x228A, atom_type: AtomType::Relation }, // Unicode: 0x228A, subset, not equals
    "supsetneq" => Symbol { unicode: 0x228B, atom_type: AtomType::Relation }, // Unicode: 0x228B, superset, not equals
    "cupleftarrow" => Symbol { unicode: 0x228C, atom_type: AtomType::Binary }, // Unicode: 0x228C, multiset
    "cupdot" => Symbol { unicode: 0x228D, atom_type: AtomType::Binary }, // Unicode: 0x228D, union, with dot
    "uplus" => Symbol { unicode: 0x228E, atom_type: AtomType::Binary }, // Unicode: 0x228E, plus sign in union
    "sqsubset" => Symbol { unicode: 0x228F, atom_type: AtomType::Relation }, // Unicode: 0x228F, square subset
    "sqsupset" => Symbol { unicode: 0x2290, atom_type: AtomType::Relation }, // Unicode: 0x2290, square superset
    "sqsubseteq" => Symbol { unicode: 0x2291, atom_type: AtomType::Relation }, // Unicode: 0x2291, square subset, equals
    "sqsupseteq" => Symbol { unicode: 0x2292, atom_type: AtomType::Relation }, // Unicode: 0x2292, square superset, equals
    "sqcap" => Symbol { unicode: 0x2293, atom_type: AtomType::Binary }, // Unicode: 0x2293, square intersection
    "sqcup" => Symbol { unicode: 0x2294, atom_type: AtomType::Binary }, // Unicode: 0x2294, square union
    "oplus" => Symbol { unicode: 0x2295, atom_type: AtomType::Binary }, // Unicode: 0x2295, plus sign in circle
    "ominus" => Symbol { unicode: 0x2296, atom_type: AtomType::Binary }, // Unicode: 0x2296, minus sign in circle
    "otimes" => Symbol { unicode: 0x2297, atom_type: AtomType::Binary }, // Unicode: 0x2297, multiply sign in circle
    "oslash" => Symbol { unicode: 0x2298, atom_type: AtomType::Binary }, // Unicode: 0x2298, solidus in circle
    "odot" => Symbol { unicode: 0x2299, atom_type: AtomType::Binary }, // Unicode: 0x2299, middle dot in circle
    "circledcirc" => Symbol { unicode: 0x229A, atom_type: AtomType::Binary }, // Unicode: 0x229A, small circle in circle
    "circledast" => Symbol { unicode: 0x229B, atom_type: AtomType::Binary }, // Unicode: 0x229B, asterisk in circle
    "circledequal" => Symbol { unicode: 0x229C, atom_type: AtomType::Binary }, // Unicode: 0x229C, equal in circle
    "circleddash" => Symbol { unicode: 0x229D, atom_type: AtomType::Binary }, // Unicode: 0x229D, hyphen in circle
    "boxplus" => Symbol { unicode: 0x229E, atom_type: AtomType::Binary }, // Unicode: 0x229E, plus sign in box
    "boxminus" => Symbol { unicode: 0x229F, atom_type: AtomType::Binary }, // Unicode: 0x229F, minus sign in box
    "boxtimes" => Symbol { unicode: 0x22A0, atom_type: AtomType::Binary }, // Unicode: 0x22A0, multiply sign in box
    "boxdot" => Symbol { unicode: 0x22A1, atom_type: AtomType::Binary }, // Unicode: 0x22A1, /dotsquare /boxdot b: small dot in box
    "vdash" => Symbol { unicode: 0x22A2, atom_type: AtomType::Relation }, // Unicode: 0x22A2, vertical, dash
    "dashv" => Symbol { unicode: 0x22A3, atom_type: AtomType::Relation }, // Unicode: 0x22A3, dash, vertical
    "top" => Symbol { unicode: 0x22A4, atom_type: AtomType::Alpha }, // Unicode: 0x22A4, top
    "bot" => Symbol { unicode: 0x22A5, atom_type: AtomType::Alpha }, // Unicode: 0x22A5, bottom
    "assert" => Symbol { unicode: 0x22A6, atom_type: AtomType::Relation }, // Unicode: 0x22A6, assertion (vertical, short dash)
    "models" => Symbol { unicode: 0x22A7, atom_type: AtomType::Relation }, // Unicode: 0x22A7, models (vertical, short double dash)
    "vDash" => Symbol { unicode: 0x22A8, atom_type: AtomType::Relation }, // Unicode: 0x22A8, vertical, double dash
    "Vdash" => Symbol { unicode: 0x22A9, atom_type: AtomType::Relation }, // Unicode: 0x22A9, double vertical, dash
    "Vvdash" => Symbol { unicode: 0x22AA, atom_type: AtomType::Relation }, // Unicode: 0x22AA, triple vertical, dash
    "VDash" => Symbol { unicode: 0x22AB, atom_type: AtomType::Relation }, // Unicode: 0x22AB, double vert, double dash
    "nvdash" => Symbol { unicode: 0x22AC, atom_type: AtomType::Relation }, // Unicode: 0x22AC, not vertical, dash
    "nvDash" => Symbol { unicode: 0x22AD, atom_type: AtomType::Relation }, // Unicode: 0x22AD, not vertical, double dash
    "nVdash" => Symbol { unicode: 0x22AE, atom_type: AtomType::Relation }, // Unicode: 0x22AE, not double vertical, dash
    "nVDash" => Symbol { unicode: 0x22AF, atom_type: AtomType::Relation }, // Unicode: 0x22AF, not double vert, double dash
    "prurel" => Symbol { unicode: 0x22B0, atom_type: AtomType::Relation }, // Unicode: 0x22B0, element precedes under relation
    "scurel" => Symbol { unicode: 0x22B1, atom_type: AtomType::Relation }, // Unicode: 0x22B1, succeeds under relation
    "vartriangleleft" => Symbol { unicode: 0x22B2, atom_type: AtomType::Relation }, // Unicode: 0x22B2, left triangle, open, variant
    "vartriangleright" => Symbol { unicode: 0x22B3, atom_type: AtomType::Relation }, // Unicode: 0x22B3, right triangle, open, variant
    "trianglelefteq" => Symbol { unicode: 0x22B4, atom_type: AtomType::Relation }, // Unicode: 0x22B4, left triangle, equals
    "trianglerighteq" => Symbol { unicode: 0x22B5, atom_type: AtomType::Relation }, // Unicode: 0x22B5, right triangle, equals
    "origof" => Symbol { unicode: 0x22B6, atom_type: AtomType::Relation }, // Unicode: 0x22B6, original of
    "imageof" => Symbol { unicode: 0x22B7, atom_type: AtomType::Relation }, // Unicode: 0x22B7, image of
    "multimap" => Symbol { unicode: 0x22B8, atom_type: AtomType::Relation }, // Unicode: 0x22B8, /multimap a:
    "hermitmatrix" => Symbol { unicode: 0x22B9, atom_type: AtomType::Alpha }, // Unicode: 0x22B9, hermitian conjugate matrix
    "intercal" => Symbol { unicode: 0x22BA, atom_type: AtomType::Binary }, // Unicode: 0x22BA, intercal
    "veebar" => Symbol { unicode: 0x22BB, atom_type: AtomType::Binary }, // Unicode: 0x22BB, logical or, bar below (large vee); exclusive disjunction
    "barwedge" => Symbol { unicode: 0x22BC, atom_type: AtomType::Binary }, // Unicode: 0x22BC, bar, wedge (large wedge)
    "barvee" => Symbol { unicode: 0x22BD, atom_type: AtomType::Binary }, // Unicode: 0x22BD, bar, vee (large vee)
    "measuredrightangle" => Symbol { unicode: 0x22BE, atom_type: AtomType::Alpha }, // Unicode: 0x22BE, right angle-measured [with arc]
    "varlrtriangle" => Symbol { unicode: 0x22BF, atom_type: AtomType::Alpha }, // Unicode: 0x22BF, right triangle
    "bigwedge" => Symbol { unicode: 0x22C0, atom_type: AtomType::Operator(true) }, // Unicode: 0x22C0, logical or operator
    "bigvee" => Symbol { unicode: 0x22C1, atom_type: AtomType::Operator(true) }, // Unicode: 0x22C1, logical and operator
    "bigcap" => Symbol { unicode: 0x22C2, atom_type: AtomType::Operator(true) }, // Unicode: 0x22C2, intersection operator
    "bigcup" => Symbol { unicode: 0x22C3, atom_type: AtomType::Operator(true) }, // Unicode: 0x22C3, union operator
    "smwhtdiamond" => Symbol { unicode: 0x22C4, atom_type: AtomType::Binary }, // Unicode: 0x22C4, white diamond
    "cdot" => Symbol { unicode: 0x22C5, atom_type: AtomType::Binary }, // Unicode: 0x22C5, small middle dot
    "star" => Symbol { unicode: 0x22C6, atom_type: AtomType::Binary }, // Unicode: 0x22C6, small star, filled, low
    "divideontimes" => Symbol { unicode: 0x22C7, atom_type: AtomType::Binary }, // Unicode: 0x22C7, division on times
    "bowtie" => Symbol { unicode: 0x22C8, atom_type: AtomType::Relation }, // Unicode: 0x22C8, bowtie
    "ltimes" => Symbol { unicode: 0x22C9, atom_type: AtomType::Binary }, // Unicode: 0x22C9, times sign, left closed
    "rtimes" => Symbol { unicode: 0x22CA, atom_type: AtomType::Binary }, // Unicode: 0x22CA, times sign, right closed
    "leftthreetimes" => Symbol { unicode: 0x22CB, atom_type: AtomType::Binary }, // Unicode: 0x22CB, left semidirect product
    "rightthreetimes" => Symbol { unicode: 0x22CC, atom_type: AtomType::Binary }, // Unicode: 0x22CC, right semidirect product
    "backsimeq" => Symbol { unicode: 0x22CD, atom_type: AtomType::Relation }, // Unicode: 0x22CD, reverse similar, equals
    "curlyvee" => Symbol { unicode: 0x22CE, atom_type: AtomType::Binary }, // Unicode: 0x22CE, curly logical or
    "curlywedge" => Symbol { unicode: 0x22CF, atom_type: AtomType::Binary }, // Unicode: 0x22CF, curly logical and
    "Subset" => Symbol { unicode: 0x22D0, atom_type: AtomType::Relation }, // Unicode: 0x22D0, double subset
    "Supset" => Symbol { unicode: 0x22D1, atom_type: AtomType::Relation }, // Unicode: 0x22D1, double superset
    "Cap" => Symbol { unicode: 0x22D2, atom_type: AtomType::Binary }, // Unicode: 0x22D2, /cap /doublecap b: double intersection
    "Cup" => Symbol { unicode: 0x22D3, atom_type: AtomType::Binary }, // Unicode: 0x22D3, /cup /doublecup b: double union
    "pitchfork" => Symbol { unicode: 0x22D4, atom_type: AtomType::Relation }, // Unicode: 0x22D4, pitchfork
    "equalparallel" => Symbol { unicode: 0x22D5, atom_type: AtomType::Relation }, // Unicode: 0x22D5, parallel, equal; equal or parallel
    "lessdot" => Symbol { unicode: 0x22D6, atom_type: AtomType::Relation }, // Unicode: 0x22D6, less than, with dot
    "gtrdot" => Symbol { unicode: 0x22D7, atom_type: AtomType::Relation }, // Unicode: 0x22D7, greater than, with dot
    "lll" => Symbol { unicode: 0x22D8, atom_type: AtomType::Relation }, // Unicode: 0x22D8, /ll /lll /llless r: triple less-than
    "ggg" => Symbol { unicode: 0x22D9, atom_type: AtomType::Relation }, // Unicode: 0x22D9, /ggg /gg /gggtr r: triple greater-than
    "lesseqgtr" => Symbol { unicode: 0x22DA, atom_type: AtomType::Relation }, // Unicode: 0x22DA, less, equals, greater
    "gtreqless" => Symbol { unicode: 0x22DB, atom_type: AtomType::Relation }, // Unicode: 0x22DB, greater, equals, less
    "eqless" => Symbol { unicode: 0x22DC, atom_type: AtomType::Relation }, // Unicode: 0x22DC, equal-or-less
    "eqgtr" => Symbol { unicode: 0x22DD, atom_type: AtomType::Relation }, // Unicode: 0x22DD, equal-or-greater
    "curlyeqprec" => Symbol { unicode: 0x22DE, atom_type: AtomType::Relation }, // Unicode: 0x22DE, curly equals, precedes
    "curlyeqsucc" => Symbol { unicode: 0x22DF, atom_type: AtomType::Relation }, // Unicode: 0x22DF, curly equals, succeeds
    "npreccurlyeq" => Symbol { unicode: 0x22E0, atom_type: AtomType::Relation }, // Unicode: 0x22E0, not precedes, curly equals
    "nsucccurlyeq" => Symbol { unicode: 0x22E1, atom_type: AtomType::Relation }, // Unicode: 0x22E1, not succeeds, curly equals
    "nsqsubseteq" => Symbol { unicode: 0x22E2, atom_type: AtomType::Relation }, // Unicode: 0x22E2, not, square subset, equals
    "nsqsupseteq" => Symbol { unicode: 0x22E3, atom_type: AtomType::Relation }, // Unicode: 0x22E3, not, square superset, equals
    "sqsubsetneq" => Symbol { unicode: 0x22E4, atom_type: AtomType::Relation }, // Unicode: 0x22E4, square subset, not equals
    "sqsupsetneq" => Symbol { unicode: 0x22E5, atom_type: AtomType::Relation }, // Unicode: 0x22E5, square superset, not equals
    "lnsim" => Symbol { unicode: 0x22E6, atom_type: AtomType::Relation }, // Unicode: 0x22E6, less, not similar
    "gnsim" => Symbol { unicode: 0x22E7, atom_type: AtomType::Relation }, // Unicode: 0x22E7, greater, not similar
    "precnsim" => Symbol { unicode: 0x22E8, atom_type: AtomType::Relation }, // Unicode: 0x22E8, precedes, not similar
    "succnsim" => Symbol { unicode: 0x22E9, atom_type: AtomType::Relation }, // Unicode: 0x22E9, succeeds, not similar
    "nvartriangleleft" => Symbol { unicode: 0x22EA, atom_type: AtomType::Relation }, // Unicode: 0x22EA, not left triangle
    "nvartriangleright" => Symbol { unicode: 0x22EB, atom_type: AtomType::Relation }, // Unicode: 0x22EB, not right triangle
    "ntrianglelefteq" => Symbol { unicode: 0x22EC, atom_type: AtomType::Relation }, // Unicode: 0x22EC, not left triangle, equals
    "ntrianglerighteq" => Symbol { unicode: 0x22ED, atom_type: AtomType::Relation }, // Unicode: 0x22ED, not right triangle, equals
    "vdots" => Symbol { unicode: 0x22EE, atom_type: AtomType::Relation }, // Unicode: 0x22EE, vertical ellipsis
    "unicodecdots" => Symbol { unicode: 0x22EF, atom_type: AtomType::Alpha }, // Unicode: 0x22EF, three dots, centered
    "cdots" => Symbol { unicode: 0x22EF, atom_type: AtomType::Alpha }, // Unicode: 0x22EF, three dots, centered
    "adots" => Symbol { unicode: 0x22F0, atom_type: AtomType::Relation }, // Unicode: 0x22F0, three dots, ascending
    "ddots" => Symbol { unicode: 0x22F1, atom_type: AtomType::Relation }, // Unicode: 0x22F1, three dots, descending
    "disin" => Symbol { unicode: 0x22F2, atom_type: AtomType::Relation }, // Unicode: 0x22F2, element of with long horizontal stroke
    "varisins" => Symbol { unicode: 0x22F3, atom_type: AtomType::Relation }, // Unicode: 0x22F3, element of with vertical bar at end of horizontal stroke
    "isins" => Symbol { unicode: 0x22F4, atom_type: AtomType::Relation }, // Unicode: 0x22F4, small element of with vertical bar at end of horizontal stroke
    "isindot" => Symbol { unicode: 0x22F5, atom_type: AtomType::Relation }, // Unicode: 0x22F5, element of with dot above
    "varisinobar" => Symbol { unicode: 0x22F6, atom_type: AtomType::Relation }, // Unicode: 0x22F6, element of with overbar
    "isinobar" => Symbol { unicode: 0x22F7, atom_type: AtomType::Relation }, // Unicode: 0x22F7, small element of with overbar
    "isinvb" => Symbol { unicode: 0x22F8, atom_type: AtomType::Relation }, // Unicode: 0x22F8, element of with underbar
    "isinE" => Symbol { unicode: 0x22F9, atom_type: AtomType::Relation }, // Unicode: 0x22F9, element of with two horizontal strokes
    "nisd" => Symbol { unicode: 0x22FA, atom_type: AtomType::Relation }, // Unicode: 0x22FA, contains with long horizontal stroke
    "varnis" => Symbol { unicode: 0x22FB, atom_type: AtomType::Relation }, // Unicode: 0x22FB, contains with vertical bar at end of horizontal stroke
    "nis" => Symbol { unicode: 0x22FC, atom_type: AtomType::Relation }, // Unicode: 0x22FC, small contains with vertical bar at end of horizontal stroke
    "varniobar" => Symbol { unicode: 0x22FD, atom_type: AtomType::Relation }, // Unicode: 0x22FD, contains with overbar
    "niobar" => Symbol { unicode: 0x22FE, atom_type: AtomType::Relation }, // Unicode: 0x22FE, small contains with overbar
    "bagmember" => Symbol { unicode: 0x22FF, atom_type: AtomType::Relation }, // Unicode: 0x22FF, z notation bag membership
    "diameter" => Symbol { unicode: 0x2300, atom_type: AtomType::Alpha }, // Unicode: 0x2300, diameter sign
    "house" => Symbol { unicode: 0x2302, atom_type: AtomType::Alpha }, // Unicode: 0x2302, house
    "varbarwedge" => Symbol { unicode: 0x2305, atom_type: AtomType::Binary }, // Unicode: 0x2305, /barwedge b: logical and, bar above [projective (bar over small wedge)]
    "vardoublebarwedge" => Symbol { unicode: 0x2306, atom_type: AtomType::Binary }, // Unicode: 0x2306, /doublebarwedge b: logical and, double bar above [perspective (double bar over small wedge)]
    "lceil" => Symbol { unicode: 0x2308, atom_type: AtomType::Open }, // Unicode: 0x2308, left ceiling
    "rceil" => Symbol { unicode: 0x2309, atom_type: AtomType::Close }, // Unicode: 0x2309, right ceiling
    "lfloor" => Symbol { unicode: 0x230A, atom_type: AtomType::Open }, // Unicode: 0x230A, left floor
    "rfloor" => Symbol { unicode: 0x230B, atom_type: AtomType::Close }, // Unicode: 0x230B, right floor
    "invnot" => Symbol { unicode: 0x2310, atom_type: AtomType::Alpha }, // Unicode: 0x2310, reverse not
    "sqlozenge" => Symbol { unicode: 0x2311, atom_type: AtomType::Alpha }, // Unicode: 0x2311, square lozenge
    "profline" => Symbol { unicode: 0x2312, atom_type: AtomType::Alpha }, // Unicode: 0x2312, profile of a line
    "profsurf" => Symbol { unicode: 0x2313, atom_type: AtomType::Alpha }, // Unicode: 0x2313, profile of a surface
    "viewdata" => Symbol { unicode: 0x2317, atom_type: AtomType::Alpha }, // Unicode: 0x2317, viewdata square
    "turnednot" => Symbol { unicode: 0x2319, atom_type: AtomType::Alpha }, // Unicode: 0x2319, turned not sign
    "ulcorner" => Symbol { unicode: 0x231C, atom_type: AtomType::Open }, // Unicode: 0x231C, upper left corner
    "urcorner" => Symbol { unicode: 0x231D, atom_type: AtomType::Close }, // Unicode: 0x231D, upper right corner
    "llcorner" => Symbol { unicode: 0x231E, atom_type: AtomType::Open }, // Unicode: 0x231E, lower left corner
    "lrcorner" => Symbol { unicode: 0x231F, atom_type: AtomType::Close }, // Unicode: 0x231F, lower right corner
    "inttop" => Symbol { unicode: 0x2320, atom_type: AtomType::Alpha }, // Unicode: 0x2320, top half integral
    "intbottom" => Symbol { unicode: 0x2321, atom_type: AtomType::Alpha }, // Unicode: 0x2321, bottom half integral
    "frown" => Symbol { unicode: 0x2322, atom_type: AtomType::Relation }, // Unicode: 0x2322, down curve
    "smile" => Symbol { unicode: 0x2323, atom_type: AtomType::Relation }, // Unicode: 0x2323, up curve
    "varhexagonlrbonds" => Symbol { unicode: 0x232C, atom_type: AtomType::Alpha }, // Unicode: 0x232C, six carbon ring, corner down, double bonds lower right etc
    "conictaper" => Symbol { unicode: 0x2332, atom_type: AtomType::Alpha }, // Unicode: 0x2332, conical taper
    "topbot" => Symbol { unicode: 0x2336, atom_type: AtomType::Alpha }, // Unicode: 0x2336, top and bottom
    "obar" => Symbol { unicode: 0x233D, atom_type: AtomType::Binary }, // Unicode: 0x233D, circle with vertical bar
    "APLnotslash" => Symbol { unicode: 0x233F, atom_type: AtomType::Relation }, // Unicode: 0x233F, solidus, bar through (apl functional symbol slash bar)
    "APLnotbackslash" => Symbol { unicode: 0x2340, atom_type: AtomType::Alpha }, // Unicode: 0x2340, apl functional symbol backslash bar
    "APLboxupcaret" => Symbol { unicode: 0x2353, atom_type: AtomType::Alpha }, // Unicode: 0x2353, boxed up caret
    "APLboxquestion" => Symbol { unicode: 0x2370, atom_type: AtomType::Alpha }, // Unicode: 0x2370, boxed question mark
    "rangledownzigzagarrow" => Symbol { unicode: 0x237C, atom_type: AtomType::Alpha }, // Unicode: 0x237C, right angle with downwards zigzag arrow
    "hexagon" => Symbol { unicode: 0x2394, atom_type: AtomType::Alpha }, // Unicode: 0x2394, horizontal benzene ring [hexagon flat open]
    "lparenuend" => Symbol { unicode: 0x239B, atom_type: AtomType::Alpha }, // Unicode: 0x239B, left parenthesis upper hook
    "lparenextender" => Symbol { unicode: 0x239C, atom_type: AtomType::Alpha }, // Unicode: 0x239C, left parenthesis extension
    "lparenlend" => Symbol { unicode: 0x239D, atom_type: AtomType::Alpha }, // Unicode: 0x239D, left parenthesis lower hook
    "rparenuend" => Symbol { unicode: 0x239E, atom_type: AtomType::Alpha }, // Unicode: 0x239E, right parenthesis upper hook
    "rparenextender" => Symbol { unicode: 0x239F, atom_type: AtomType::Alpha }, // Unicode: 0x239F, right parenthesis extension
    "rparenlend" => Symbol { unicode: 0x23A0, atom_type: AtomType::Alpha }, // Unicode: 0x23A0, right parenthesis lower hook
    "lbrackuend" => Symbol { unicode: 0x23A1, atom_type: AtomType::Alpha }, // Unicode: 0x23A1, left square bracket upper corner
    "lbrackextender" => Symbol { unicode: 0x23A2, atom_type: AtomType::Alpha }, // Unicode: 0x23A2, left square bracket extension
    "lbracklend" => Symbol { unicode: 0x23A3, atom_type: AtomType::Alpha }, // Unicode: 0x23A3, left square bracket lower corner
    "rbrackuend" => Symbol { unicode: 0x23A4, atom_type: AtomType::Alpha }, // Unicode: 0x23A4, right square bracket upper corner
    "rbrackextender" => Symbol { unicode: 0x23A5, atom_type: AtomType::Alpha }, // Unicode: 0x23A5, right square bracket extension
    "rbracklend" => Symbol { unicode: 0x23A6, atom_type: AtomType::Alpha }, // Unicode: 0x23A6, right square bracket lower corner
    "lbraceuend" => Symbol { unicode: 0x23A7, atom_type: AtomType::Alpha }, // Unicode: 0x23A7, left curly bracket upper hook
    "lbracemid" => Symbol { unicode: 0x23A8, atom_type: AtomType::Alpha }, // Unicode: 0x23A8, left curly bracket middle piece
    "lbracelend" => Symbol { unicode: 0x23A9, atom_type: AtomType::Alpha }, // Unicode: 0x23A9, left curly bracket lower hook
    "vbraceextender" => Symbol { unicode: 0x23AA, atom_type: AtomType::Alpha }, // Unicode: 0x23AA, curly bracket extension
    "rbraceuend" => Symbol { unicode: 0x23AB, atom_type: AtomType::Alpha }, // Unicode: 0x23AB, right curly bracket upper hook
    "rbracemid" => Symbol { unicode: 0x23AC, atom_type: AtomType::Alpha }, // Unicode: 0x23AC, right curly bracket middle piece
    "rbracelend" => Symbol { unicode: 0x23AD, atom_type: AtomType::Alpha }, // Unicode: 0x23AD, right curly bracket lower hook
    "intextender" => Symbol { unicode: 0x23AE, atom_type: AtomType::Alpha }, // Unicode: 0x23AE, integral extension
    "harrowextender" => Symbol { unicode: 0x23AF, atom_type: AtomType::Alpha }, // Unicode: 0x23AF, horizontal line extension (used to extend arrows)
    "lmoustache" => Symbol { unicode: 0x23B0, atom_type: AtomType::Open }, // Unicode: 0x23B0, upper left or lower right curly bracket section
    "rmoustache" => Symbol { unicode: 0x23B1, atom_type: AtomType::Close }, // Unicode: 0x23B1, upper right or lower left curly bracket section
    "sumtop" => Symbol { unicode: 0x23B2, atom_type: AtomType::Alpha }, // Unicode: 0x23B2, summation top
    "sumbottom" => Symbol { unicode: 0x23B3, atom_type: AtomType::Alpha }, // Unicode: 0x23B3, summation bottom
    "overbracket" => Symbol { unicode: 0x23B4, atom_type: AtomType::Over }, // Unicode: 0x23B4, top square bracket
    "underbracket" => Symbol { unicode: 0x23B5, atom_type: AtomType::Under }, // Unicode: 0x23B5, bottom square bracket
    "bbrktbrk" => Symbol { unicode: 0x23B6, atom_type: AtomType::Alpha }, // Unicode: 0x23B6, bottom square bracket over top square bracket
    "sqrtbottom" => Symbol { unicode: 0x23B7, atom_type: AtomType::Alpha }, // Unicode: 0x23B7, radical symbol bottom
    "lvboxline" => Symbol { unicode: 0x23B8, atom_type: AtomType::Alpha }, // Unicode: 0x23B8, left vertical box line
    "rvboxline" => Symbol { unicode: 0x23B9, atom_type: AtomType::Alpha }, // Unicode: 0x23B9, right vertical box line
    "varcarriagereturn" => Symbol { unicode: 0x23CE, atom_type: AtomType::Alpha }, // Unicode: 0x23CE, return symbol
    "overparen" => Symbol { unicode: 0x23DC, atom_type: AtomType::Over }, // Unicode: 0x23DC, top parenthesis (mathematical use)
    "underparen" => Symbol { unicode: 0x23DD, atom_type: AtomType::Under }, // Unicode: 0x23DD, bottom parenthesis (mathematical use)
    "overbrace" => Symbol { unicode: 0x23DE, atom_type: AtomType::Accent }, // Unicode: 0x23DE, top curly bracket (mathematical use)
    "underbrace" => Symbol { unicode: 0x23DF, atom_type: AtomType::Under }, // Unicode: 0x23DF, bottom curly bracket (mathematical use)
    "obrbrak" => Symbol { unicode: 0x23E0, atom_type: AtomType::Alpha }, // Unicode: 0x23E0, top tortoise shell bracket (mathematical use)
    "ubrbrak" => Symbol { unicode: 0x23E1, atom_type: AtomType::Alpha }, // Unicode: 0x23E1, bottom tortoise shell bracket (mathematical use)
    "trapezium" => Symbol { unicode: 0x23E2, atom_type: AtomType::Alpha }, // Unicode: 0x23E2, white trapezium
    "benzenr" => Symbol { unicode: 0x23E3, atom_type: AtomType::Alpha }, // Unicode: 0x23E3, benzene ring with circle
    "strns" => Symbol { unicode: 0x23E4, atom_type: AtomType::Alpha }, // Unicode: 0x23E4, straightness
    "fltns" => Symbol { unicode: 0x23E5, atom_type: AtomType::Alpha }, // Unicode: 0x23E5, flatness
    "accurrent" => Symbol { unicode: 0x23E6, atom_type: AtomType::Alpha }, // Unicode: 0x23E6, ac current
    "elinters" => Symbol { unicode: 0x23E7, atom_type: AtomType::Alpha }, // Unicode: 0x23E7, electrical intersection
    "mathvisiblespace" => Symbol { unicode: 0x2423, atom_type: AtomType::Alpha }, // Unicode: 0x2423, open box
    "bdtriplevdash" => Symbol { unicode: 0x2506, atom_type: AtomType::Alpha }, // Unicode: 0x2506, doubly broken vert
    "blockuphalf" => Symbol { unicode: 0x2580, atom_type: AtomType::Alpha }, // Unicode: 0x2580, upper half block
    "blocklowhalf" => Symbol { unicode: 0x2584, atom_type: AtomType::Alpha }, // Unicode: 0x2584, lower half block
    "blockfull" => Symbol { unicode: 0x2588, atom_type: AtomType::Alpha }, // Unicode: 0x2588, full block
    "blocklefthalf" => Symbol { unicode: 0x258C, atom_type: AtomType::Alpha }, // Unicode: 0x258C, left half block
    "blockrighthalf" => Symbol { unicode: 0x2590, atom_type: AtomType::Alpha }, // Unicode: 0x2590, right half block
    "blockqtrshaded" => Symbol { unicode: 0x2591, atom_type: AtomType::Alpha }, // Unicode: 0x2591, 25\% shaded block
    "blockhalfshaded" => Symbol { unicode: 0x2592, atom_type: AtomType::Alpha }, // Unicode: 0x2592, 50\% shaded block
    "blockthreeqtrshaded" => Symbol { unicode: 0x2593, atom_type: AtomType::Alpha }, // Unicode: 0x2593, 75\% shaded block
    "mdlgblksquare" => Symbol { unicode: 0x25A0, atom_type: AtomType::Alpha }, // Unicode: 0x25A0, square, filled
    "mdlgwhtsquare" => Symbol { unicode: 0x25A1, atom_type: AtomType::Alpha }, // Unicode: 0x25A1, square, open
    "squoval" => Symbol { unicode: 0x25A2, atom_type: AtomType::Alpha }, // Unicode: 0x25A2, white square with rounded corners
    "blackinwhitesquare" => Symbol { unicode: 0x25A3, atom_type: AtomType::Alpha }, // Unicode: 0x25A3, white square containing black small square
    "squarehfill" => Symbol { unicode: 0x25A4, atom_type: AtomType::Alpha }, // Unicode: 0x25A4, square, horizontal rule filled
    "squarevfill" => Symbol { unicode: 0x25A5, atom_type: AtomType::Alpha }, // Unicode: 0x25A5, square, vertical rule filled
    "squarehvfill" => Symbol { unicode: 0x25A6, atom_type: AtomType::Alpha }, // Unicode: 0x25A6, square with orthogonal crosshatch fill
    "squarenwsefill" => Symbol { unicode: 0x25A7, atom_type: AtomType::Alpha }, // Unicode: 0x25A7, square, nw-to-se rule filled
    "squareneswfill" => Symbol { unicode: 0x25A8, atom_type: AtomType::Alpha }, // Unicode: 0x25A8, square, ne-to-sw rule filled
    "squarecrossfill" => Symbol { unicode: 0x25A9, atom_type: AtomType::Alpha }, // Unicode: 0x25A9, square with diagonal crosshatch fill
    "smblksquare" => Symbol { unicode: 0x25AA, atom_type: AtomType::Alpha }, // Unicode: 0x25AA, /blacksquare - sq bullet, filled
    "smwhtsquare" => Symbol { unicode: 0x25AB, atom_type: AtomType::Alpha }, // Unicode: 0x25AB, white small square
    "hrectangleblack" => Symbol { unicode: 0x25AC, atom_type: AtomType::Alpha }, // Unicode: 0x25AC, black rectangle
    "hrectangle" => Symbol { unicode: 0x25AD, atom_type: AtomType::Alpha }, // Unicode: 0x25AD, horizontal rectangle, open
    "vrectangleblack" => Symbol { unicode: 0x25AE, atom_type: AtomType::Alpha }, // Unicode: 0x25AE, black vertical rectangle
    "vrectangle" => Symbol { unicode: 0x25AF, atom_type: AtomType::Alpha }, // Unicode: 0x25AF, rectangle, white (vertical)
    "parallelogramblack" => Symbol { unicode: 0x25B0, atom_type: AtomType::Alpha }, // Unicode: 0x25B0, black parallelogram
    "parallelogram" => Symbol { unicode: 0x25B1, atom_type: AtomType::Alpha }, // Unicode: 0x25B1, parallelogram, open
    "bigblacktriangleup" => Symbol { unicode: 0x25B2, atom_type: AtomType::Alpha }, // Unicode: 0x25B2,    0x25b2 6 6d      black up-pointing triangle
    "bigtriangleup" => Symbol { unicode: 0x25B3, atom_type: AtomType::Binary }, // Unicode: 0x25B3, big up triangle, open
    "blacktriangle" => Symbol { unicode: 0x25B4, atom_type: AtomType::Alpha }, // Unicode: 0x25B4, up triangle, filled
    "vartriangle" => Symbol { unicode: 0x25B5, atom_type: AtomType::Relation }, // Unicode: 0x25B5, /triangle - up triangle, open
    "blacktriangleright" => Symbol { unicode: 0x25B6, atom_type: AtomType::Alpha }, // Unicode: 0x25B6, (large) right triangle, filled
    "triangleright" => Symbol { unicode: 0x25B7, atom_type: AtomType::Binary }, // Unicode: 0x25B7, (large) right triangle, open; z notation range restriction
    "smallblacktriangleright" => Symbol { unicode: 0x25B8, atom_type: AtomType::Alpha }, // Unicode: 0x25B8, right triangle, filled
    "smalltriangleright" => Symbol { unicode: 0x25B9, atom_type: AtomType::Alpha }, // Unicode: 0x25B9, right triangle, open
    "blackpointerright" => Symbol { unicode: 0x25BA, atom_type: AtomType::Alpha }, // Unicode: 0x25BA, black right-pointing pointer
    "whitepointerright" => Symbol { unicode: 0x25BB, atom_type: AtomType::Alpha }, // Unicode: 0x25BB, white right-pointing pointer
    "bigblacktriangledown" => Symbol { unicode: 0x25BC, atom_type: AtomType::Alpha }, // Unicode: 0x25BC, big down triangle, filled
    "bigtriangledown" => Symbol { unicode: 0x25BD, atom_type: AtomType::Alpha }, // Unicode: 0x25BD, big down triangle, open
    "blacktriangledown" => Symbol { unicode: 0x25BE, atom_type: AtomType::Alpha }, // Unicode: 0x25BE, down triangle, filled
    "triangledown" => Symbol { unicode: 0x25BF, atom_type: AtomType::Alpha }, // Unicode: 0x25BF, down triangle, open
    "blacktriangleleft" => Symbol { unicode: 0x25C0, atom_type: AtomType::Alpha }, // Unicode: 0x25C0, (large) left triangle, filled
    "triangleleft" => Symbol { unicode: 0x25C1, atom_type: AtomType::Binary }, // Unicode: 0x25C1, (large) left triangle, open; z notation domain restriction
    "smallblacktriangleleft" => Symbol { unicode: 0x25C2, atom_type: AtomType::Alpha }, // Unicode: 0x25C2, left triangle, filled
    "smalltriangleleft" => Symbol { unicode: 0x25C3, atom_type: AtomType::Alpha }, // Unicode: 0x25C3, left triangle, open
    "blackpointerleft" => Symbol { unicode: 0x25C4, atom_type: AtomType::Alpha }, // Unicode: 0x25C4, black left-pointing pointer
    "whitepointerleft" => Symbol { unicode: 0x25C5, atom_type: AtomType::Alpha }, // Unicode: 0x25C5, white left-pointing pointer
    "mdlgblkdiamond" => Symbol { unicode: 0x25C6, atom_type: AtomType::Alpha }, // Unicode: 0x25C6, black diamond
    "mdlgwhtdiamond" => Symbol { unicode: 0x25C7, atom_type: AtomType::Alpha }, // Unicode: 0x25C7, white diamond; diamond, open
    "blackinwhitediamond" => Symbol { unicode: 0x25C8, atom_type: AtomType::Alpha }, // Unicode: 0x25C8, white diamond containing black small diamond
    "fisheye" => Symbol { unicode: 0x25C9, atom_type: AtomType::Alpha }, // Unicode: 0x25C9, fisheye
    "mdlgwhtlozenge" => Symbol { unicode: 0x25CA, atom_type: AtomType::Alpha }, // Unicode: 0x25CA, lozenge or total mark
    "mdlgwhtcircle" => Symbol { unicode: 0x25CB, atom_type: AtomType::Binary }, // Unicode: 0x25CB, medium large circle
    "dottedcircle" => Symbol { unicode: 0x25CC, atom_type: AtomType::Alpha }, // Unicode: 0x25CC, dotted circle
    "circlevertfill" => Symbol { unicode: 0x25CD, atom_type: AtomType::Alpha }, // Unicode: 0x25CD, circle with vertical fill
    "bullseye" => Symbol { unicode: 0x25CE, atom_type: AtomType::Alpha }, // Unicode: 0x25CE, bullseye
    "mdlgblkcircle" => Symbol { unicode: 0x25CF, atom_type: AtomType::Alpha }, // Unicode: 0x25CF, circle, filled
    "circlelefthalfblack" => Symbol { unicode: 0x25D0, atom_type: AtomType::Alpha }, // Unicode: 0x25D0, circle, filled left half [harvey ball]
    "circlerighthalfblack" => Symbol { unicode: 0x25D1, atom_type: AtomType::Alpha }, // Unicode: 0x25D1, circle, filled right half
    "circlebottomhalfblack" => Symbol { unicode: 0x25D2, atom_type: AtomType::Alpha }, // Unicode: 0x25D2, circle, filled bottom half
    "circletophalfblack" => Symbol { unicode: 0x25D3, atom_type: AtomType::Alpha }, // Unicode: 0x25D3, circle, filled top half
    "circleurquadblack" => Symbol { unicode: 0x25D4, atom_type: AtomType::Alpha }, // Unicode: 0x25D4, circle with upper right quadrant black
    "blackcircleulquadwhite" => Symbol { unicode: 0x25D5, atom_type: AtomType::Alpha }, // Unicode: 0x25D5, circle with all but upper left quadrant black
    "blacklefthalfcircle" => Symbol { unicode: 0x25D6, atom_type: AtomType::Alpha }, // Unicode: 0x25D6, left half black circle
    "blackrighthalfcircle" => Symbol { unicode: 0x25D7, atom_type: AtomType::Alpha }, // Unicode: 0x25D7, right half black circle
    "inversebullet" => Symbol { unicode: 0x25D8, atom_type: AtomType::Alpha }, // Unicode: 0x25D8, inverse bullet
    "inversewhitecircle" => Symbol { unicode: 0x25D9, atom_type: AtomType::Alpha }, // Unicode: 0x25D9, inverse white circle
    "invwhiteupperhalfcircle" => Symbol { unicode: 0x25DA, atom_type: AtomType::Alpha }, // Unicode: 0x25DA, upper half inverse white circle
    "invwhitelowerhalfcircle" => Symbol { unicode: 0x25DB, atom_type: AtomType::Alpha }, // Unicode: 0x25DB, lower half inverse white circle
    "ularc" => Symbol { unicode: 0x25DC, atom_type: AtomType::Alpha }, // Unicode: 0x25DC, upper left quadrant circular arc
    "urarc" => Symbol { unicode: 0x25DD, atom_type: AtomType::Alpha }, // Unicode: 0x25DD, upper right quadrant circular arc
    "lrarc" => Symbol { unicode: 0x25DE, atom_type: AtomType::Alpha }, // Unicode: 0x25DE, lower right quadrant circular arc
    "llarc" => Symbol { unicode: 0x25DF, atom_type: AtomType::Alpha }, // Unicode: 0x25DF, lower left quadrant circular arc
    "topsemicircle" => Symbol { unicode: 0x25E0, atom_type: AtomType::Alpha }, // Unicode: 0x25E0, upper half circle
    "botsemicircle" => Symbol { unicode: 0x25E1, atom_type: AtomType::Alpha }, // Unicode: 0x25E1, lower half circle
    "lrblacktriangle" => Symbol { unicode: 0x25E2, atom_type: AtomType::Alpha }, // Unicode: 0x25E2, lower right triangle, filled
    "llblacktriangle" => Symbol { unicode: 0x25E3, atom_type: AtomType::Alpha }, // Unicode: 0x25E3, lower left triangle, filled
    "ulblacktriangle" => Symbol { unicode: 0x25E4, atom_type: AtomType::Alpha }, // Unicode: 0x25E4, upper left triangle, filled
    "urblacktriangle" => Symbol { unicode: 0x25E5, atom_type: AtomType::Alpha }, // Unicode: 0x25E5, upper right triangle, filled
    "smwhtcircle" => Symbol { unicode: 0x25E6, atom_type: AtomType::Alpha }, // Unicode: 0x25E6, white bullet
    "squareleftblack" => Symbol { unicode: 0x25E7, atom_type: AtomType::Alpha }, // Unicode: 0x25E7, square, filled left half
    "squarerightblack" => Symbol { unicode: 0x25E8, atom_type: AtomType::Alpha }, // Unicode: 0x25E8, square, filled right half
    "squareulblack" => Symbol { unicode: 0x25E9, atom_type: AtomType::Alpha }, // Unicode: 0x25E9, square, filled top left corner
    "squarelrblack" => Symbol { unicode: 0x25EA, atom_type: AtomType::Alpha }, // Unicode: 0x25EA, square, filled bottom right corner
    "boxbar" => Symbol { unicode: 0x25EB, atom_type: AtomType::Binary }, // Unicode: 0x25EB, vertical bar in box
    "trianglecdot" => Symbol { unicode: 0x25EC, atom_type: AtomType::Alpha }, // Unicode: 0x25EC, triangle with centered dot
    "triangleleftblack" => Symbol { unicode: 0x25ED, atom_type: AtomType::Alpha }, // Unicode: 0x25ED, up-pointing triangle with left half black
    "trianglerightblack" => Symbol { unicode: 0x25EE, atom_type: AtomType::Alpha }, // Unicode: 0x25EE, up-pointing triangle with right half black
    "lgwhtcircle" => Symbol { unicode: 0x25EF, atom_type: AtomType::Alpha }, // Unicode: 0x25EF, large circle
    "squareulquad" => Symbol { unicode: 0x25F0, atom_type: AtomType::Alpha }, // Unicode: 0x25F0, white square with upper left quadrant
    "squarellquad" => Symbol { unicode: 0x25F1, atom_type: AtomType::Alpha }, // Unicode: 0x25F1, white square with lower left quadrant
    "squarelrquad" => Symbol { unicode: 0x25F2, atom_type: AtomType::Alpha }, // Unicode: 0x25F2, white square with lower right quadrant
    "squareurquad" => Symbol { unicode: 0x25F3, atom_type: AtomType::Alpha }, // Unicode: 0x25F3, white square with upper right quadrant
    "circleulquad" => Symbol { unicode: 0x25F4, atom_type: AtomType::Alpha }, // Unicode: 0x25F4, white circle with upper left quadrant
    "circlellquad" => Symbol { unicode: 0x25F5, atom_type: AtomType::Alpha }, // Unicode: 0x25F5, white circle with lower left quadrant
    "circlelrquad" => Symbol { unicode: 0x25F6, atom_type: AtomType::Alpha }, // Unicode: 0x25F6, white circle with lower right quadrant
    "circleurquad" => Symbol { unicode: 0x25F7, atom_type: AtomType::Alpha }, // Unicode: 0x25F7, white circle with upper right quadrant
    "ultriangle" => Symbol { unicode: 0x25F8, atom_type: AtomType::Alpha }, // Unicode: 0x25F8, upper left triangle
    "urtriangle" => Symbol { unicode: 0x25F9, atom_type: AtomType::Alpha }, // Unicode: 0x25F9, upper right triangle
    "lltriangle" => Symbol { unicode: 0x25FA, atom_type: AtomType::Alpha }, // Unicode: 0x25FA, lower left triangle
    "mdwhtsquare" => Symbol { unicode: 0x25FB, atom_type: AtomType::Alpha }, // Unicode: 0x25FB, white medium square
    "mdblksquare" => Symbol { unicode: 0x25FC, atom_type: AtomType::Alpha }, // Unicode: 0x25FC, black medium square
    "mdsmwhtsquare" => Symbol { unicode: 0x25FD, atom_type: AtomType::Alpha }, // Unicode: 0x25FD, white medium small square
    "mdsmblksquare" => Symbol { unicode: 0x25FE, atom_type: AtomType::Alpha }, // Unicode: 0x25FE, black medium small square
    "lrtriangle" => Symbol { unicode: 0x25FF, atom_type: AtomType::Alpha }, // Unicode: 0x25FF, lower right triangle
    "bigstar" => Symbol { unicode: 0x2605, atom_type: AtomType::Alpha }, // Unicode: 0x2605, star, filled
    "bigwhitestar" => Symbol { unicode: 0x2606, atom_type: AtomType::Alpha }, // Unicode: 0x2606, star, open
    "astrosun" => Symbol { unicode: 0x2609, atom_type: AtomType::Alpha }, // Unicode: 0x2609, sun
    "danger" => Symbol { unicode: 0x2621, atom_type: AtomType::Alpha }, // Unicode: 0x2621, dangerous bend (caution sign)
    "blacksmiley" => Symbol { unicode: 0x263B, atom_type: AtomType::Alpha }, // Unicode: 0x263B, black smiling face
    "sun" => Symbol { unicode: 0x263C, atom_type: AtomType::Alpha }, // Unicode: 0x263C, white sun with rays
    "rightmoon" => Symbol { unicode: 0x263D, atom_type: AtomType::Alpha }, // Unicode: 0x263D, first quarter moon
    "leftmoon" => Symbol { unicode: 0x263E, atom_type: AtomType::Alpha }, // Unicode: 0x263E, last quarter moon
    "female" => Symbol { unicode: 0x2640, atom_type: AtomType::Alpha }, // Unicode: 0x2640, venus, female
    "male" => Symbol { unicode: 0x2642, atom_type: AtomType::Alpha }, // Unicode: 0x2642, mars, male
    "spadesuit" => Symbol { unicode: 0x2660, atom_type: AtomType::Alpha }, // Unicode: 0x2660, spades suit symbol
    "heartsuit" => Symbol { unicode: 0x2661, atom_type: AtomType::Alpha }, // Unicode: 0x2661, heart suit symbol
    "diamondsuit" => Symbol { unicode: 0x2662, atom_type: AtomType::Alpha }, // Unicode: 0x2662, diamond suit symbol
    "clubsuit" => Symbol { unicode: 0x2663, atom_type: AtomType::Alpha }, // Unicode: 0x2663, club suit symbol
    "varspadesuit" => Symbol { unicode: 0x2664, atom_type: AtomType::Alpha }, // Unicode: 0x2664, spade, white (card suit)
    "varheartsuit" => Symbol { unicode: 0x2665, atom_type: AtomType::Alpha }, // Unicode: 0x2665, filled heart (card suit)
    "vardiamondsuit" => Symbol { unicode: 0x2666, atom_type: AtomType::Alpha }, // Unicode: 0x2666, filled diamond (card suit)
    "varclubsuit" => Symbol { unicode: 0x2667, atom_type: AtomType::Alpha }, // Unicode: 0x2667, club, white (card suit)
    "quarternote" => Symbol { unicode: 0x2669, atom_type: AtomType::Alpha }, // Unicode: 0x2669, music note (sung text sign)
    "eighthnote" => Symbol { unicode: 0x266A, atom_type: AtomType::Alpha }, // Unicode: 0x266A, eighth note
    "twonotes" => Symbol { unicode: 0x266B, atom_type: AtomType::Alpha }, // Unicode: 0x266B, beamed eighth notes
    "flat" => Symbol { unicode: 0x266D, atom_type: AtomType::Alpha }, // Unicode: 0x266D, musical flat
    "natural" => Symbol { unicode: 0x266E, atom_type: AtomType::Alpha }, // Unicode: 0x266E, music natural
    "sharp" => Symbol { unicode: 0x266F, atom_type: AtomType::Alpha }, // Unicode: 0x266F, musical sharp
    "acidfree" => Symbol { unicode: 0x267E, atom_type: AtomType::Alpha }, // Unicode: 0x267E, permanent paper sign
    "dicei" => Symbol { unicode: 0x2680, atom_type: AtomType::Alpha }, // Unicode: 0x2680, die face-1
    "diceii" => Symbol { unicode: 0x2681, atom_type: AtomType::Alpha }, // Unicode: 0x2681, die face-2
    "diceiii" => Symbol { unicode: 0x2682, atom_type: AtomType::Alpha }, // Unicode: 0x2682, die face-3
    "diceiv" => Symbol { unicode: 0x2683, atom_type: AtomType::Alpha }, // Unicode: 0x2683, die face-4
    "dicev" => Symbol { unicode: 0x2684, atom_type: AtomType::Alpha }, // Unicode: 0x2684, die face-5
    "dicevi" => Symbol { unicode: 0x2685, atom_type: AtomType::Alpha }, // Unicode: 0x2685, die face-6
    "circledrightdot" => Symbol { unicode: 0x2686, atom_type: AtomType::Alpha }, // Unicode: 0x2686, white circle with dot right
    "circledtwodots" => Symbol { unicode: 0x2687, atom_type: AtomType::Alpha }, // Unicode: 0x2687, white circle with two dots
    "blackcircledrightdot" => Symbol { unicode: 0x2688, atom_type: AtomType::Alpha }, // Unicode: 0x2688, black circle with white dot right
    "blackcircledtwodots" => Symbol { unicode: 0x2689, atom_type: AtomType::Alpha }, // Unicode: 0x2689, black circle with two white dots
    "Hermaphrodite" => Symbol { unicode: 0x26A5, atom_type: AtomType::Alpha }, // Unicode: 0x26A5, male and female sign
    "mdwhtcircle" => Symbol { unicode: 0x26AA, atom_type: AtomType::Alpha }, // Unicode: 0x26AA, medium white circle
    "mdblkcircle" => Symbol { unicode: 0x26AB, atom_type: AtomType::Alpha }, // Unicode: 0x26AB, medium black circle
    "mdsmwhtcircle" => Symbol { unicode: 0x26AC, atom_type: AtomType::Alpha }, // Unicode: 0x26AC, medium small white circle
    "neuter" => Symbol { unicode: 0x26B2, atom_type: AtomType::Alpha }, // Unicode: 0x26B2, neuter
    "checkmark" => Symbol { unicode: 0x2713, atom_type: AtomType::Alpha }, // Unicode: 0x2713, tick, check mark
    "maltese" => Symbol { unicode: 0x2720, atom_type: AtomType::Alpha }, // Unicode: 0x2720, maltese cross
    "circledstar" => Symbol { unicode: 0x272A, atom_type: AtomType::Alpha }, // Unicode: 0x272A, circled white star
    "varstar" => Symbol { unicode: 0x2736, atom_type: AtomType::Alpha }, // Unicode: 0x2736, six pointed black star
    "dingasterisk" => Symbol { unicode: 0x273D, atom_type: AtomType::Alpha }, // Unicode: 0x273D, heavy teardrop-spoked asterisk
    "lbrbrak" => Symbol { unicode: 0x2772, atom_type: AtomType::Open }, // Unicode: 0x2772, light left tortoise shell bracket ornament
    "rbrbrak" => Symbol { unicode: 0x2773, atom_type: AtomType::Close }, // Unicode: 0x2773, light right tortoise shell bracket ornament
    "draftingarrow" => Symbol { unicode: 0x279B, atom_type: AtomType::Alpha }, // Unicode: 0x279B, right arrow with bold head (drafting)
    "threedangle" => Symbol { unicode: 0x27C0, atom_type: AtomType::Alpha }, // Unicode: 0x27C0, three dimensional angle
    "whiteinwhitetriangle" => Symbol { unicode: 0x27C1, atom_type: AtomType::Alpha }, // Unicode: 0x27C1, white triangle containing small white triangle
    "perp" => Symbol { unicode: 0x27C2, atom_type: AtomType::Relation }, // Unicode: 0x27C2, perpendicular
    "subsetcirc" => Symbol { unicode: 0x27C3, atom_type: AtomType::Alpha }, // Unicode: 0x27C3, open subset
    "supsetcirc" => Symbol { unicode: 0x27C4, atom_type: AtomType::Alpha }, // Unicode: 0x27C4, open superset
    "lbag" => Symbol { unicode: 0x27C5, atom_type: AtomType::Open }, // Unicode: 0x27C5, left s-shaped bag delimiter
    "rbag" => Symbol { unicode: 0x27C6, atom_type: AtomType::Close }, // Unicode: 0x27C6, right s-shaped bag delimiter
    "veedot" => Symbol { unicode: 0x27C7, atom_type: AtomType::Binary }, // Unicode: 0x27C7, or with dot inside
    "bsolhsub" => Symbol { unicode: 0x27C8, atom_type: AtomType::Relation }, // Unicode: 0x27C8, reverse solidus preceding subset
    "suphsol" => Symbol { unicode: 0x27C9, atom_type: AtomType::Relation }, // Unicode: 0x27C9, superset preceding solidus
    "longdivision" => Symbol { unicode: 0x27CC, atom_type: AtomType::Open }, // Unicode: 0x27CC, long division
    "diamondcdot" => Symbol { unicode: 0x27D0, atom_type: AtomType::Alpha }, // Unicode: 0x27D0, white diamond with centred dot
    "wedgedot" => Symbol { unicode: 0x27D1, atom_type: AtomType::Binary }, // Unicode: 0x27D1, and with dot
    "upin" => Symbol { unicode: 0x27D2, atom_type: AtomType::Relation }, // Unicode: 0x27D2, element of opening upwards
    "pullback" => Symbol { unicode: 0x27D3, atom_type: AtomType::Relation }, // Unicode: 0x27D3, lower right corner with dot
    "pushout" => Symbol { unicode: 0x27D4, atom_type: AtomType::Relation }, // Unicode: 0x27D4, upper left corner with dot
    "leftouterjoin" => Symbol { unicode: 0x27D5, atom_type: AtomType::Operator(false) }, // Unicode: 0x27D5, left outer join
    "rightouterjoin" => Symbol { unicode: 0x27D6, atom_type: AtomType::Operator(false) }, // Unicode: 0x27D6, right outer join
    "fullouterjoin" => Symbol { unicode: 0x27D7, atom_type: AtomType::Operator(false) }, // Unicode: 0x27D7, full outer join
    "bigbot" => Symbol { unicode: 0x27D8, atom_type: AtomType::Operator(false) }, // Unicode: 0x27D8, large up tack
    "bigtop" => Symbol { unicode: 0x27D9, atom_type: AtomType::Operator(false) }, // Unicode: 0x27D9, large down tack
    "DashVDash" => Symbol { unicode: 0x27DA, atom_type: AtomType::Relation }, // Unicode: 0x27DA, left and right double turnstile
    "dashVdash" => Symbol { unicode: 0x27DB, atom_type: AtomType::Relation }, // Unicode: 0x27DB, left and right tack
    "multimapinv" => Symbol { unicode: 0x27DC, atom_type: AtomType::Relation }, // Unicode: 0x27DC, left multimap
    "vlongdash" => Symbol { unicode: 0x27DD, atom_type: AtomType::Relation }, // Unicode: 0x27DD, long left tack
    "longdashv" => Symbol { unicode: 0x27DE, atom_type: AtomType::Relation }, // Unicode: 0x27DE, long right tack
    "cirbot" => Symbol { unicode: 0x27DF, atom_type: AtomType::Relation }, // Unicode: 0x27DF, up tack with circle above
    "lozengeminus" => Symbol { unicode: 0x27E0, atom_type: AtomType::Binary }, // Unicode: 0x27E0, lozenge divided by horizontal rule
    "concavediamond" => Symbol { unicode: 0x27E1, atom_type: AtomType::Binary }, // Unicode: 0x27E1, white concave-sided diamond
    "concavediamondtickleft" => Symbol { unicode: 0x27E2, atom_type: AtomType::Binary }, // Unicode: 0x27E2, white concave-sided diamond with leftwards tick
    "concavediamondtickright" => Symbol { unicode: 0x27E3, atom_type: AtomType::Binary }, // Unicode: 0x27E3, white concave-sided diamond with rightwards tick
    "whitesquaretickleft" => Symbol { unicode: 0x27E4, atom_type: AtomType::Binary }, // Unicode: 0x27E4, white square with leftwards tick
    "whitesquaretickright" => Symbol { unicode: 0x27E5, atom_type: AtomType::Binary }, // Unicode: 0x27E5, white square with rightwards tick
    "lBrack" => Symbol { unicode: 0x27E6, atom_type: AtomType::Open }, // Unicode: 0x27E6, mathematical left white square bracket
    "rBrack" => Symbol { unicode: 0x27E7, atom_type: AtomType::Close }, // Unicode: 0x27E7, mathematical right white square bracket
    "langle" => Symbol { unicode: 0x27E8, atom_type: AtomType::Open }, // Unicode: 0x27E8, mathematical left angle bracket
    "rangle" => Symbol { unicode: 0x27E9, atom_type: AtomType::Close }, // Unicode: 0x27E9, mathematical right angle bracket
    "lAngle" => Symbol { unicode: 0x27EA, atom_type: AtomType::Open }, // Unicode: 0x27EA, mathematical left double angle bracket
    "rAngle" => Symbol { unicode: 0x27EB, atom_type: AtomType::Close }, // Unicode: 0x27EB, mathematical right double angle bracket
    "Lbrbrak" => Symbol { unicode: 0x27EC, atom_type: AtomType::Open }, // Unicode: 0x27EC, mathematical left white tortoise shell bracket
    "Rbrbrak" => Symbol { unicode: 0x27ED, atom_type: AtomType::Close }, // Unicode: 0x27ED, mathematical right white tortoise shell bracket
    "lgroup" => Symbol { unicode: 0x27EE, atom_type: AtomType::Open }, // Unicode: 0x27EE, mathematical left flattened parenthesis
    "rgroup" => Symbol { unicode: 0x27EF, atom_type: AtomType::Close }, // Unicode: 0x27EF, mathematical right flattened parenthesis
    "UUparrow" => Symbol { unicode: 0x27F0, atom_type: AtomType::Relation }, // Unicode: 0x27F0, upwards quadruple arrow
    "DDownarrow" => Symbol { unicode: 0x27F1, atom_type: AtomType::Relation }, // Unicode: 0x27F1, downwards quadruple arrow
    "acwgapcirclearrow" => Symbol { unicode: 0x27F2, atom_type: AtomType::Relation }, // Unicode: 0x27F2, anticlockwise gapped circle arrow
    "cwgapcirclearrow" => Symbol { unicode: 0x27F3, atom_type: AtomType::Relation }, // Unicode: 0x27F3, clockwise gapped circle arrow
    "rightarrowonoplus" => Symbol { unicode: 0x27F4, atom_type: AtomType::Relation }, // Unicode: 0x27F4, right arrow with circled plus
    "longleftarrow" => Symbol { unicode: 0x27F5, atom_type: AtomType::Relation }, // Unicode: 0x27F5, long leftwards arrow
    "longrightarrow" => Symbol { unicode: 0x27F6, atom_type: AtomType::Relation }, // Unicode: 0x27F6, long rightwards arrow
    "longleftrightarrow" => Symbol { unicode: 0x27F7, atom_type: AtomType::Relation }, // Unicode: 0x27F7, long left right arrow
    "Longleftarrow" => Symbol { unicode: 0x27F8, atom_type: AtomType::Relation }, // Unicode: 0x27F8, long leftwards double arrow
    "Longrightarrow" => Symbol { unicode: 0x27F9, atom_type: AtomType::Relation }, // Unicode: 0x27F9, long rightwards double arrow
    "Longleftrightarrow" => Symbol { unicode: 0x27FA, atom_type: AtomType::Relation }, // Unicode: 0x27FA, long left right double arrow
    "longmapsfrom" => Symbol { unicode: 0x27FB, atom_type: AtomType::Relation }, // Unicode: 0x27FB, long leftwards arrow from bar
    "longmapsto" => Symbol { unicode: 0x27FC, atom_type: AtomType::Relation }, // Unicode: 0x27FC, long rightwards arrow from bar
    "Longmapsfrom" => Symbol { unicode: 0x27FD, atom_type: AtomType::Relation }, // Unicode: 0x27FD, long leftwards double arrow from bar
    "Longmapsto" => Symbol { unicode: 0x27FE, atom_type: AtomType::Relation }, // Unicode: 0x27FE, long rightwards double arrow from bar
    "longrightsquigarrow" => Symbol { unicode: 0x27FF, atom_type: AtomType::Relation }, // Unicode: 0x27FF, long rightwards squiggle arrow
    "nvtwoheadrightarrow" => Symbol { unicode: 0x2900, atom_type: AtomType::Relation }, // Unicode: 0x2900, rightwards two-headed arrow with vertical stroke
    "nVtwoheadrightarrow" => Symbol { unicode: 0x2901, atom_type: AtomType::Relation }, // Unicode: 0x2901, rightwards two-headed arrow with double vertical stroke
    "nvLeftarrow" => Symbol { unicode: 0x2902, atom_type: AtomType::Relation }, // Unicode: 0x2902, leftwards double arrow with vertical stroke
    "nvRightarrow" => Symbol { unicode: 0x2903, atom_type: AtomType::Relation }, // Unicode: 0x2903, rightwards double arrow with vertical stroke
    "nvLeftrightarrow" => Symbol { unicode: 0x2904, atom_type: AtomType::Relation }, // Unicode: 0x2904, left right double arrow with vertical stroke
    "twoheadmapsto" => Symbol { unicode: 0x2905, atom_type: AtomType::Relation }, // Unicode: 0x2905, rightwards two-headed arrow from bar
    "Mapsfrom" => Symbol { unicode: 0x2906, atom_type: AtomType::Relation }, // Unicode: 0x2906, leftwards double arrow from bar
    "Mapsto" => Symbol { unicode: 0x2907, atom_type: AtomType::Relation }, // Unicode: 0x2907, rightwards double arrow from bar
    "downarrowbarred" => Symbol { unicode: 0x2908, atom_type: AtomType::Relation }, // Unicode: 0x2908, downwards arrow with horizontal stroke
    "uparrowbarred" => Symbol { unicode: 0x2909, atom_type: AtomType::Relation }, // Unicode: 0x2909, upwards arrow with horizontal stroke
    "Uuparrow" => Symbol { unicode: 0x290A, atom_type: AtomType::Relation }, // Unicode: 0x290A, upwards triple arrow
    "Ddownarrow" => Symbol { unicode: 0x290B, atom_type: AtomType::Relation }, // Unicode: 0x290B, downwards triple arrow
    "leftbkarrow" => Symbol { unicode: 0x290C, atom_type: AtomType::Relation }, // Unicode: 0x290C, leftwards double dash arrow
    "rightbkarrow" => Symbol { unicode: 0x290D, atom_type: AtomType::Relation }, // Unicode: 0x290D, rightwards double dash arrow
    "leftdbkarrow" => Symbol { unicode: 0x290E, atom_type: AtomType::Relation }, // Unicode: 0x290E, leftwards triple dash arrow
    "dbkarrow" => Symbol { unicode: 0x290F, atom_type: AtomType::Relation }, // Unicode: 0x290F, rightwards triple dash arrow
    "drbkarrow" => Symbol { unicode: 0x2910, atom_type: AtomType::Relation }, // Unicode: 0x2910, rightwards two-headed triple dash arrow
    "rightdotarrow" => Symbol { unicode: 0x2911, atom_type: AtomType::Relation }, // Unicode: 0x2911, rightwards arrow with dotted stem
    "baruparrow" => Symbol { unicode: 0x2912, atom_type: AtomType::Relation }, // Unicode: 0x2912, upwards arrow to bar
    "downarrowbar" => Symbol { unicode: 0x2913, atom_type: AtomType::Relation }, // Unicode: 0x2913, downwards arrow to bar
    "nvrightarrowtail" => Symbol { unicode: 0x2914, atom_type: AtomType::Relation }, // Unicode: 0x2914, rightwards arrow with tail with vertical stroke
    "nVrightarrowtail" => Symbol { unicode: 0x2915, atom_type: AtomType::Relation }, // Unicode: 0x2915, rightwards arrow with tail with double vertical stroke
    "twoheadrightarrowtail" => Symbol { unicode: 0x2916, atom_type: AtomType::Relation }, // Unicode: 0x2916, rightwards two-headed arrow with tail
    "nvtwoheadrightarrowtail" => Symbol { unicode: 0x2917, atom_type: AtomType::Relation }, // Unicode: 0x2917, rightwards two-headed arrow with tail with vertical stroke
    "nVtwoheadrightarrowtail" => Symbol { unicode: 0x2918, atom_type: AtomType::Relation }, // Unicode: 0x2918, rightwards two-headed arrow with tail with double vertical stroke
    "lefttail" => Symbol { unicode: 0x2919, atom_type: AtomType::Relation }, // Unicode: 0x2919, leftwards arrow-tail
    "righttail" => Symbol { unicode: 0x291A, atom_type: AtomType::Relation }, // Unicode: 0x291A, rightwards arrow-tail
    "leftdbltail" => Symbol { unicode: 0x291B, atom_type: AtomType::Relation }, // Unicode: 0x291B, leftwards double arrow-tail
    "rightdbltail" => Symbol { unicode: 0x291C, atom_type: AtomType::Relation }, // Unicode: 0x291C, rightwards double arrow-tail
    "diamondleftarrow" => Symbol { unicode: 0x291D, atom_type: AtomType::Relation }, // Unicode: 0x291D, leftwards arrow to black diamond
    "rightarrowdiamond" => Symbol { unicode: 0x291E, atom_type: AtomType::Relation }, // Unicode: 0x291E, rightwards arrow to black diamond
    "diamondleftarrowbar" => Symbol { unicode: 0x291F, atom_type: AtomType::Relation }, // Unicode: 0x291F, leftwards arrow from bar to black diamond
    "barrightarrowdiamond" => Symbol { unicode: 0x2920, atom_type: AtomType::Relation }, // Unicode: 0x2920, rightwards arrow from bar to black diamond
    "nwsearrow" => Symbol { unicode: 0x2921, atom_type: AtomType::Relation }, // Unicode: 0x2921, north west and south east arrow
    "neswarrow" => Symbol { unicode: 0x2922, atom_type: AtomType::Relation }, // Unicode: 0x2922, north east and south west arrow
    "hknwarrow" => Symbol { unicode: 0x2923, atom_type: AtomType::Relation }, // Unicode: 0x2923, north west arrow with hook
    "hknearrow" => Symbol { unicode: 0x2924, atom_type: AtomType::Relation }, // Unicode: 0x2924, north east arrow with hook
    "hksearrow" => Symbol { unicode: 0x2925, atom_type: AtomType::Relation }, // Unicode: 0x2925, south east arrow with hook
    "hkswarrow" => Symbol { unicode: 0x2926, atom_type: AtomType::Relation }, // Unicode: 0x2926, south west arrow with hook
    "tona" => Symbol { unicode: 0x2927, atom_type: AtomType::Relation }, // Unicode: 0x2927, north west arrow and north east arrow
    "toea" => Symbol { unicode: 0x2928, atom_type: AtomType::Relation }, // Unicode: 0x2928, north east arrow and south east arrow
    "tosa" => Symbol { unicode: 0x2929, atom_type: AtomType::Relation }, // Unicode: 0x2929, south east arrow and south west arrow
    "towa" => Symbol { unicode: 0x292A, atom_type: AtomType::Relation }, // Unicode: 0x292A, south west arrow and north west arrow
    "rdiagovfdiag" => Symbol { unicode: 0x292B, atom_type: AtomType::Alpha }, // Unicode: 0x292B, rising diagonal crossing falling diagonal
    "fdiagovrdiag" => Symbol { unicode: 0x292C, atom_type: AtomType::Alpha }, // Unicode: 0x292C, falling diagonal crossing rising diagonal
    "seovnearrow" => Symbol { unicode: 0x292D, atom_type: AtomType::Alpha }, // Unicode: 0x292D, south east arrow crossing north east arrow
    "neovsearrow" => Symbol { unicode: 0x292E, atom_type: AtomType::Alpha }, // Unicode: 0x292E, north east arrow crossing south east arrow
    "fdiagovnearrow" => Symbol { unicode: 0x292F, atom_type: AtomType::Alpha }, // Unicode: 0x292F, falling diagonal crossing north east arrow
    "rdiagovsearrow" => Symbol { unicode: 0x2930, atom_type: AtomType::Alpha }, // Unicode: 0x2930, rising diagonal crossing south east arrow
    "neovnwarrow" => Symbol { unicode: 0x2931, atom_type: AtomType::Alpha }, // Unicode: 0x2931, north east arrow crossing north west arrow
    "nwovnearrow" => Symbol { unicode: 0x2932, atom_type: AtomType::Alpha }, // Unicode: 0x2932, north west arrow crossing north east arrow
    "rightcurvedarrow" => Symbol { unicode: 0x2933, atom_type: AtomType::Relation }, // Unicode: 0x2933, wave arrow pointing directly right
    "uprightcurvearrow" => Symbol { unicode: 0x2934, atom_type: AtomType::Alpha }, // Unicode: 0x2934, arrow pointing rightwards then curving upwards
    "downrightcurvedarrow" => Symbol { unicode: 0x2935, atom_type: AtomType::Alpha }, // Unicode: 0x2935, arrow pointing rightwards then curving downwards
    "leftdowncurvedarrow" => Symbol { unicode: 0x2936, atom_type: AtomType::Relation }, // Unicode: 0x2936, arrow pointing downwards then curving leftwards
    "rightdowncurvedarrow" => Symbol { unicode: 0x2937, atom_type: AtomType::Relation }, // Unicode: 0x2937, arrow pointing downwards then curving rightwards
    "cwrightarcarrow" => Symbol { unicode: 0x2938, atom_type: AtomType::Relation }, // Unicode: 0x2938, right-side arc clockwise arrow
    "acwleftarcarrow" => Symbol { unicode: 0x2939, atom_type: AtomType::Relation }, // Unicode: 0x2939, left-side arc anticlockwise arrow
    "acwoverarcarrow" => Symbol { unicode: 0x293A, atom_type: AtomType::Relation }, // Unicode: 0x293A, top arc anticlockwise arrow
    "acwunderarcarrow" => Symbol { unicode: 0x293B, atom_type: AtomType::Relation }, // Unicode: 0x293B, bottom arc anticlockwise arrow
    "curvearrowrightminus" => Symbol { unicode: 0x293C, atom_type: AtomType::Relation }, // Unicode: 0x293C, top arc clockwise arrow with minus
    "curvearrowleftplus" => Symbol { unicode: 0x293D, atom_type: AtomType::Relation }, // Unicode: 0x293D, top arc anticlockwise arrow with plus
    "cwundercurvearrow" => Symbol { unicode: 0x293E, atom_type: AtomType::Relation }, // Unicode: 0x293E, lower right semicircular clockwise arrow
    "ccwundercurvearrow" => Symbol { unicode: 0x293F, atom_type: AtomType::Relation }, // Unicode: 0x293F, lower left semicircular anticlockwise arrow
    "acwcirclearrow" => Symbol { unicode: 0x2940, atom_type: AtomType::Relation }, // Unicode: 0x2940, anticlockwise closed circle arrow
    "cwcirclearrow" => Symbol { unicode: 0x2941, atom_type: AtomType::Relation }, // Unicode: 0x2941, clockwise closed circle arrow
    "rightarrowshortleftarrow" => Symbol { unicode: 0x2942, atom_type: AtomType::Relation }, // Unicode: 0x2942, rightwards arrow above short leftwards arrow
    "leftarrowshortrightarrow" => Symbol { unicode: 0x2943, atom_type: AtomType::Relation }, // Unicode: 0x2943, leftwards arrow above short rightwards arrow
    "shortrightarrowleftarrow" => Symbol { unicode: 0x2944, atom_type: AtomType::Relation }, // Unicode: 0x2944, short rightwards arrow above leftwards arrow
    "rightarrowplus" => Symbol { unicode: 0x2945, atom_type: AtomType::Relation }, // Unicode: 0x2945, rightwards arrow with plus below
    "leftarrowplus" => Symbol { unicode: 0x2946, atom_type: AtomType::Relation }, // Unicode: 0x2946, leftwards arrow with plus below
    "rightarrowx" => Symbol { unicode: 0x2947, atom_type: AtomType::Relation }, // Unicode: 0x2947, rightwards arrow through x
    "leftrightarrowcircle" => Symbol { unicode: 0x2948, atom_type: AtomType::Relation }, // Unicode: 0x2948, left right arrow through small circle
    "twoheaduparrowcircle" => Symbol { unicode: 0x2949, atom_type: AtomType::Relation }, // Unicode: 0x2949, upwards two-headed arrow from small circle
    "leftrightharpoonupdown" => Symbol { unicode: 0x294A, atom_type: AtomType::Relation }, // Unicode: 0x294A, left barb up right barb down harpoon
    "leftrightharpoondownup" => Symbol { unicode: 0x294B, atom_type: AtomType::Relation }, // Unicode: 0x294B, left barb down right barb up harpoon
    "updownharpoonrightleft" => Symbol { unicode: 0x294C, atom_type: AtomType::Relation }, // Unicode: 0x294C, up barb right down barb left harpoon
    "updownharpoonleftright" => Symbol { unicode: 0x294D, atom_type: AtomType::Relation }, // Unicode: 0x294D, up barb left down barb right harpoon
    "leftrightharpoonupup" => Symbol { unicode: 0x294E, atom_type: AtomType::Relation }, // Unicode: 0x294E, left barb up right barb up harpoon
    "updownharpoonrightright" => Symbol { unicode: 0x294F, atom_type: AtomType::Relation }, // Unicode: 0x294F, up barb right down barb right harpoon
    "leftrightharpoondowndown" => Symbol { unicode: 0x2950, atom_type: AtomType::Relation }, // Unicode: 0x2950, left barb down right barb down harpoon
    "updownharpoonleftleft" => Symbol { unicode: 0x2951, atom_type: AtomType::Relation }, // Unicode: 0x2951, up barb left down barb left harpoon
    "barleftharpoonup" => Symbol { unicode: 0x2952, atom_type: AtomType::Relation }, // Unicode: 0x2952, leftwards harpoon with barb up to bar
    "rightharpoonupbar" => Symbol { unicode: 0x2953, atom_type: AtomType::Relation }, // Unicode: 0x2953, rightwards harpoon with barb up to bar
    "barupharpoonright" => Symbol { unicode: 0x2954, atom_type: AtomType::Relation }, // Unicode: 0x2954, upwards harpoon with barb right to bar
    "downharpoonrightbar" => Symbol { unicode: 0x2955, atom_type: AtomType::Relation }, // Unicode: 0x2955, downwards harpoon with barb right to bar
    "barleftharpoondown" => Symbol { unicode: 0x2956, atom_type: AtomType::Relation }, // Unicode: 0x2956, leftwards harpoon with barb down to bar
    "rightharpoondownbar" => Symbol { unicode: 0x2957, atom_type: AtomType::Relation }, // Unicode: 0x2957, rightwards harpoon with barb down to bar
    "barupharpoonleft" => Symbol { unicode: 0x2958, atom_type: AtomType::Relation }, // Unicode: 0x2958, upwards harpoon with barb left to bar
    "downharpoonleftbar" => Symbol { unicode: 0x2959, atom_type: AtomType::Relation }, // Unicode: 0x2959, downwards harpoon with barb left to bar
    "leftharpoonupbar" => Symbol { unicode: 0x295A, atom_type: AtomType::Relation }, // Unicode: 0x295A, leftwards harpoon with barb up from bar
    "barrightharpoonup" => Symbol { unicode: 0x295B, atom_type: AtomType::Relation }, // Unicode: 0x295B, rightwards harpoon with barb up from bar
    "upharpoonrightbar" => Symbol { unicode: 0x295C, atom_type: AtomType::Relation }, // Unicode: 0x295C, upwards harpoon with barb right from bar
    "bardownharpoonright" => Symbol { unicode: 0x295D, atom_type: AtomType::Relation }, // Unicode: 0x295D, downwards harpoon with barb right from bar
    "leftharpoondownbar" => Symbol { unicode: 0x295E, atom_type: AtomType::Relation }, // Unicode: 0x295E, leftwards harpoon with barb down from bar
    "barrightharpoondown" => Symbol { unicode: 0x295F, atom_type: AtomType::Relation }, // Unicode: 0x295F, rightwards harpoon with barb down from bar
    "upharpoonleftbar" => Symbol { unicode: 0x2960, atom_type: AtomType::Relation }, // Unicode: 0x2960, upwards harpoon with barb left from bar
    "bardownharpoonleft" => Symbol { unicode: 0x2961, atom_type: AtomType::Relation }, // Unicode: 0x2961, downwards harpoon with barb left from bar
    "leftharpoonsupdown" => Symbol { unicode: 0x2962, atom_type: AtomType::Relation }, // Unicode: 0x2962, leftwards harpoon with barb up above leftwards harpoon with barb down
    "upharpoonsleftright" => Symbol { unicode: 0x2963, atom_type: AtomType::Relation }, // Unicode: 0x2963, upwards harpoon with barb left beside upwards harpoon with barb right
    "rightharpoonsupdown" => Symbol { unicode: 0x2964, atom_type: AtomType::Relation }, // Unicode: 0x2964, rightwards harpoon with barb up above rightwards harpoon with barb down
    "downharpoonsleftright" => Symbol { unicode: 0x2965, atom_type: AtomType::Relation }, // Unicode: 0x2965, downwards harpoon with barb left beside downwards harpoon with barb right
    "leftrightharpoonsup" => Symbol { unicode: 0x2966, atom_type: AtomType::Relation }, // Unicode: 0x2966, leftwards harpoon with barb up above rightwards harpoon with barb up
    "leftrightharpoonsdown" => Symbol { unicode: 0x2967, atom_type: AtomType::Relation }, // Unicode: 0x2967, leftwards harpoon with barb down above rightwards harpoon with barb down
    "rightleftharpoonsup" => Symbol { unicode: 0x2968, atom_type: AtomType::Relation }, // Unicode: 0x2968, rightwards harpoon with barb up above leftwards harpoon with barb up
    "rightleftharpoonsdown" => Symbol { unicode: 0x2969, atom_type: AtomType::Relation }, // Unicode: 0x2969, rightwards harpoon with barb down above leftwards harpoon with barb down
    "leftharpoonupdash" => Symbol { unicode: 0x296A, atom_type: AtomType::Relation }, // Unicode: 0x296A, leftwards harpoon with barb up above long dash
    "dashleftharpoondown" => Symbol { unicode: 0x296B, atom_type: AtomType::Relation }, // Unicode: 0x296B, leftwards harpoon with barb down below long dash
    "rightharpoonupdash" => Symbol { unicode: 0x296C, atom_type: AtomType::Relation }, // Unicode: 0x296C, rightwards harpoon with barb up above long dash
    "dashrightharpoondown" => Symbol { unicode: 0x296D, atom_type: AtomType::Relation }, // Unicode: 0x296D, rightwards harpoon with barb down below long dash
    "updownharpoonsleftright" => Symbol { unicode: 0x296E, atom_type: AtomType::Relation }, // Unicode: 0x296E, upwards harpoon with barb left beside downwards harpoon with barb right
    "downupharpoonsleftright" => Symbol { unicode: 0x296F, atom_type: AtomType::Relation }, // Unicode: 0x296F, downwards harpoon with barb left beside upwards harpoon with barb right
    "rightimply" => Symbol { unicode: 0x2970, atom_type: AtomType::Relation }, // Unicode: 0x2970, right double arrow with rounded head
    "equalrightarrow" => Symbol { unicode: 0x2971, atom_type: AtomType::Relation }, // Unicode: 0x2971, equals sign above rightwards arrow
    "similarrightarrow" => Symbol { unicode: 0x2972, atom_type: AtomType::Relation }, // Unicode: 0x2972, tilde operator above rightwards arrow
    "leftarrowsimilar" => Symbol { unicode: 0x2973, atom_type: AtomType::Relation }, // Unicode: 0x2973, leftwards arrow above tilde operator
    "rightarrowsimilar" => Symbol { unicode: 0x2974, atom_type: AtomType::Relation }, // Unicode: 0x2974, rightwards arrow above tilde operator
    "rightarrowapprox" => Symbol { unicode: 0x2975, atom_type: AtomType::Relation }, // Unicode: 0x2975, rightwards arrow above almost equal to
    "ltlarr" => Symbol { unicode: 0x2976, atom_type: AtomType::Relation }, // Unicode: 0x2976, less-than above leftwards arrow
    "leftarrowless" => Symbol { unicode: 0x2977, atom_type: AtomType::Relation }, // Unicode: 0x2977, leftwards arrow through less-than
    "gtrarr" => Symbol { unicode: 0x2978, atom_type: AtomType::Relation }, // Unicode: 0x2978, greater-than above rightwards arrow
    "subrarr" => Symbol { unicode: 0x2979, atom_type: AtomType::Relation }, // Unicode: 0x2979, subset above rightwards arrow
    "leftarrowsubset" => Symbol { unicode: 0x297A, atom_type: AtomType::Relation }, // Unicode: 0x297A, leftwards arrow through subset
    "suplarr" => Symbol { unicode: 0x297B, atom_type: AtomType::Relation }, // Unicode: 0x297B, superset above leftwards arrow
    "leftfishtail" => Symbol { unicode: 0x297C, atom_type: AtomType::Relation }, // Unicode: 0x297C, left fish tail
    "rightfishtail" => Symbol { unicode: 0x297D, atom_type: AtomType::Relation }, // Unicode: 0x297D, right fish tail
    "upfishtail" => Symbol { unicode: 0x297E, atom_type: AtomType::Relation }, // Unicode: 0x297E, up fish tail
    "downfishtail" => Symbol { unicode: 0x297F, atom_type: AtomType::Relation }, // Unicode: 0x297F, down fish tail
    "Vvert" => Symbol { unicode: 0x2980, atom_type: AtomType::Fence }, // Unicode: 0x2980, triple vertical bar delimiter
    "mdsmblkcircle" => Symbol { unicode: 0x2981, atom_type: AtomType::Alpha }, // Unicode: 0x2981, z notation spot
    "typecolon" => Symbol { unicode: 0x2982, atom_type: AtomType::Binary }, // Unicode: 0x2982, z notation type colon
    "lBrace" => Symbol { unicode: 0x2983, atom_type: AtomType::Open }, // Unicode: 0x2983, left white curly bracket
    "rBrace" => Symbol { unicode: 0x2984, atom_type: AtomType::Close }, // Unicode: 0x2984, right white curly bracket
    "lParen" => Symbol { unicode: 0x2985, atom_type: AtomType::Open }, // Unicode: 0x2985, left white parenthesis
    "rParen" => Symbol { unicode: 0x2986, atom_type: AtomType::Close }, // Unicode: 0x2986, right white parenthesis
    "llparenthesis" => Symbol { unicode: 0x2987, atom_type: AtomType::Open }, // Unicode: 0x2987, z notation left image bracket
    "rrparenthesis" => Symbol { unicode: 0x2988, atom_type: AtomType::Close }, // Unicode: 0x2988, z notation right image bracket
    "llangle" => Symbol { unicode: 0x2989, atom_type: AtomType::Open }, // Unicode: 0x2989, z notation left binding bracket
    "rrangle" => Symbol { unicode: 0x298A, atom_type: AtomType::Close }, // Unicode: 0x298A, z notation right binding bracket
    "lbrackubar" => Symbol { unicode: 0x298B, atom_type: AtomType::Open }, // Unicode: 0x298B, left square bracket with underbar
    "rbrackubar" => Symbol { unicode: 0x298C, atom_type: AtomType::Close }, // Unicode: 0x298C, right square bracket with underbar
    "lbrackultick" => Symbol { unicode: 0x298D, atom_type: AtomType::Open }, // Unicode: 0x298D, left square bracket with tick in top corner
    "rbracklrtick" => Symbol { unicode: 0x298E, atom_type: AtomType::Close }, // Unicode: 0x298E, right square bracket with tick in bottom corner
    "lbracklltick" => Symbol { unicode: 0x298F, atom_type: AtomType::Open }, // Unicode: 0x298F, left square bracket with tick in bottom corner
    "rbrackurtick" => Symbol { unicode: 0x2990, atom_type: AtomType::Close }, // Unicode: 0x2990, right square bracket with tick in top corner
    "langledot" => Symbol { unicode: 0x2991, atom_type: AtomType::Open }, // Unicode: 0x2991, left angle bracket with dot
    "rangledot" => Symbol { unicode: 0x2992, atom_type: AtomType::Close }, // Unicode: 0x2992, right angle bracket with dot
    "lparenless" => Symbol { unicode: 0x2993, atom_type: AtomType::Open }, // Unicode: 0x2993, left arc less-than bracket
    "rparengtr" => Symbol { unicode: 0x2994, atom_type: AtomType::Close }, // Unicode: 0x2994, right arc greater-than bracket
    "Lparengtr" => Symbol { unicode: 0x2995, atom_type: AtomType::Open }, // Unicode: 0x2995, double left arc greater-than bracket
    "Rparenless" => Symbol { unicode: 0x2996, atom_type: AtomType::Close }, // Unicode: 0x2996, double right arc less-than bracket
    "lblkbrbrak" => Symbol { unicode: 0x2997, atom_type: AtomType::Open }, // Unicode: 0x2997, left black tortoise shell bracket
    "rblkbrbrak" => Symbol { unicode: 0x2998, atom_type: AtomType::Close }, // Unicode: 0x2998, right black tortoise shell bracket
    "fourvdots" => Symbol { unicode: 0x2999, atom_type: AtomType::Alpha }, // Unicode: 0x2999, dotted fence
    "vzigzag" => Symbol { unicode: 0x299A, atom_type: AtomType::Alpha }, // Unicode: 0x299A, vertical zigzag line
    "measuredangleleft" => Symbol { unicode: 0x299B, atom_type: AtomType::Alpha }, // Unicode: 0x299B, measured angle opening left
    "rightanglesqr" => Symbol { unicode: 0x299C, atom_type: AtomType::Alpha }, // Unicode: 0x299C, right angle variant with square
    "rightanglemdot" => Symbol { unicode: 0x299D, atom_type: AtomType::Alpha }, // Unicode: 0x299D, measured right angle with dot
    "angles" => Symbol { unicode: 0x299E, atom_type: AtomType::Alpha }, // Unicode: 0x299E, angle with s inside
    "angdnr" => Symbol { unicode: 0x299F, atom_type: AtomType::Alpha }, // Unicode: 0x299F, acute angle
    "gtlpar" => Symbol { unicode: 0x29A0, atom_type: AtomType::Alpha }, // Unicode: 0x29A0, spherical angle opening left
    "sphericalangleup" => Symbol { unicode: 0x29A1, atom_type: AtomType::Alpha }, // Unicode: 0x29A1, spherical angle opening up
    "turnangle" => Symbol { unicode: 0x29A2, atom_type: AtomType::Alpha }, // Unicode: 0x29A2, turned angle
    "revangle" => Symbol { unicode: 0x29A3, atom_type: AtomType::Alpha }, // Unicode: 0x29A3, reversed angle
    "angleubar" => Symbol { unicode: 0x29A4, atom_type: AtomType::Alpha }, // Unicode: 0x29A4, angle with underbar
    "revangleubar" => Symbol { unicode: 0x29A5, atom_type: AtomType::Alpha }, // Unicode: 0x29A5, reversed angle with underbar
    "wideangledown" => Symbol { unicode: 0x29A6, atom_type: AtomType::Alpha }, // Unicode: 0x29A6, oblique angle opening up
    "wideangleup" => Symbol { unicode: 0x29A7, atom_type: AtomType::Alpha }, // Unicode: 0x29A7, oblique angle opening down
    "measanglerutone" => Symbol { unicode: 0x29A8, atom_type: AtomType::Alpha }, // Unicode: 0x29A8, measured angle with open arm ending in arrow pointing up and right
    "measanglelutonw" => Symbol { unicode: 0x29A9, atom_type: AtomType::Alpha }, // Unicode: 0x29A9, measured angle with open arm ending in arrow pointing up and left
    "measanglerdtose" => Symbol { unicode: 0x29AA, atom_type: AtomType::Alpha }, // Unicode: 0x29AA, measured angle with open arm ending in arrow pointing down and right
    "measangleldtosw" => Symbol { unicode: 0x29AB, atom_type: AtomType::Alpha }, // Unicode: 0x29AB, measured angle with open arm ending in arrow pointing down and left
    "measangleurtone" => Symbol { unicode: 0x29AC, atom_type: AtomType::Alpha }, // Unicode: 0x29AC, measured angle with open arm ending in arrow pointing right and up
    "measangleultonw" => Symbol { unicode: 0x29AD, atom_type: AtomType::Alpha }, // Unicode: 0x29AD, measured angle with open arm ending in arrow pointing left and up
    "measangledrtose" => Symbol { unicode: 0x29AE, atom_type: AtomType::Alpha }, // Unicode: 0x29AE, measured angle with open arm ending in arrow pointing right and down
    "measangledltosw" => Symbol { unicode: 0x29AF, atom_type: AtomType::Alpha }, // Unicode: 0x29AF, measured angle with open arm ending in arrow pointing left and down
    "revemptyset" => Symbol { unicode: 0x29B0, atom_type: AtomType::Alpha }, // Unicode: 0x29B0, reversed empty set
    "emptysetobar" => Symbol { unicode: 0x29B1, atom_type: AtomType::Alpha }, // Unicode: 0x29B1, empty set with overbar
    "emptysetocirc" => Symbol { unicode: 0x29B2, atom_type: AtomType::Alpha }, // Unicode: 0x29B2, empty set with small circle above
    "emptysetoarr" => Symbol { unicode: 0x29B3, atom_type: AtomType::Alpha }, // Unicode: 0x29B3, empty set with right arrow above
    "emptysetoarrl" => Symbol { unicode: 0x29B4, atom_type: AtomType::Alpha }, // Unicode: 0x29B4, empty set with left arrow above
    "circlehbar" => Symbol { unicode: 0x29B5, atom_type: AtomType::Binary }, // Unicode: 0x29B5, circle with horizontal bar
    "circledvert" => Symbol { unicode: 0x29B6, atom_type: AtomType::Binary }, // Unicode: 0x29B6, circled vertical bar
    "circledparallel" => Symbol { unicode: 0x29B7, atom_type: AtomType::Binary }, // Unicode: 0x29B7, circled parallel
    "obslash" => Symbol { unicode: 0x29B8, atom_type: AtomType::Binary }, // Unicode: 0x29B8, circled reverse solidus
    "operp" => Symbol { unicode: 0x29B9, atom_type: AtomType::Binary }, // Unicode: 0x29B9, circled perpendicular
    "obot" => Symbol { unicode: 0x29BA, atom_type: AtomType::Alpha }, // Unicode: 0x29BA, circle divided by horizontal bar and top half divided by vertical bar
    "olcross" => Symbol { unicode: 0x29BB, atom_type: AtomType::Alpha }, // Unicode: 0x29BB, circle with superimposed x
    "odotslashdot" => Symbol { unicode: 0x29BC, atom_type: AtomType::Alpha }, // Unicode: 0x29BC, circled anticlockwise-rotated division sign
    "uparrowoncircle" => Symbol { unicode: 0x29BD, atom_type: AtomType::Alpha }, // Unicode: 0x29BD, up arrow through circle
    "circledwhitebullet" => Symbol { unicode: 0x29BE, atom_type: AtomType::Alpha }, // Unicode: 0x29BE, circled white bullet
    "circledbullet" => Symbol { unicode: 0x29BF, atom_type: AtomType::Alpha }, // Unicode: 0x29BF, circled bullet
    "olessthan" => Symbol { unicode: 0x29C0, atom_type: AtomType::Binary }, // Unicode: 0x29C0, circled less-than
    "ogreaterthan" => Symbol { unicode: 0x29C1, atom_type: AtomType::Binary }, // Unicode: 0x29C1, circled greater-than
    "cirscir" => Symbol { unicode: 0x29C2, atom_type: AtomType::Alpha }, // Unicode: 0x29C2, circle with small circle to the right
    "cirE" => Symbol { unicode: 0x29C3, atom_type: AtomType::Alpha }, // Unicode: 0x29C3, circle with two horizontal strokes to the right
    "boxdiag" => Symbol { unicode: 0x29C4, atom_type: AtomType::Binary }, // Unicode: 0x29C4, squared rising diagonal slash
    "boxbslash" => Symbol { unicode: 0x29C5, atom_type: AtomType::Binary }, // Unicode: 0x29C5, squared falling diagonal slash
    "boxast" => Symbol { unicode: 0x29C6, atom_type: AtomType::Binary }, // Unicode: 0x29C6, squared asterisk
    "boxcircle" => Symbol { unicode: 0x29C7, atom_type: AtomType::Binary }, // Unicode: 0x29C7, squared small circle
    "boxbox" => Symbol { unicode: 0x29C8, atom_type: AtomType::Binary }, // Unicode: 0x29C8, squared square
    "boxonbox" => Symbol { unicode: 0x29C9, atom_type: AtomType::Alpha }, // Unicode: 0x29C9, two joined squares
    "triangleodot" => Symbol { unicode: 0x29CA, atom_type: AtomType::Alpha }, // Unicode: 0x29CA, triangle with dot above
    "triangleubar" => Symbol { unicode: 0x29CB, atom_type: AtomType::Alpha }, // Unicode: 0x29CB, triangle with underbar
    "triangles" => Symbol { unicode: 0x29CC, atom_type: AtomType::Alpha }, // Unicode: 0x29CC, s in triangle
    "triangleserifs" => Symbol { unicode: 0x29CD, atom_type: AtomType::Binary }, // Unicode: 0x29CD, triangle with serifs at bottom
    "rtriltri" => Symbol { unicode: 0x29CE, atom_type: AtomType::Relation }, // Unicode: 0x29CE, right triangle above left triangle
    "ltrivb" => Symbol { unicode: 0x29CF, atom_type: AtomType::Relation }, // Unicode: 0x29CF, left triangle beside vertical bar
    "vbrtri" => Symbol { unicode: 0x29D0, atom_type: AtomType::Relation }, // Unicode: 0x29D0, vertical bar beside right triangle
    "lfbowtie" => Symbol { unicode: 0x29D1, atom_type: AtomType::Relation }, // Unicode: 0x29D1, left black bowtie
    "rfbowtie" => Symbol { unicode: 0x29D2, atom_type: AtomType::Relation }, // Unicode: 0x29D2, right black bowtie
    "fbowtie" => Symbol { unicode: 0x29D3, atom_type: AtomType::Relation }, // Unicode: 0x29D3, black bowtie
    "lftimes" => Symbol { unicode: 0x29D4, atom_type: AtomType::Relation }, // Unicode: 0x29D4, left black times
    "rftimes" => Symbol { unicode: 0x29D5, atom_type: AtomType::Relation }, // Unicode: 0x29D5, right black times
    "hourglass" => Symbol { unicode: 0x29D6, atom_type: AtomType::Binary }, // Unicode: 0x29D6, white hourglass
    "blackhourglass" => Symbol { unicode: 0x29D7, atom_type: AtomType::Binary }, // Unicode: 0x29D7, black hourglass
    "lvzigzag" => Symbol { unicode: 0x29D8, atom_type: AtomType::Open }, // Unicode: 0x29D8, left wiggly fence
    "rvzigzag" => Symbol { unicode: 0x29D9, atom_type: AtomType::Close }, // Unicode: 0x29D9, right wiggly fence
    "Lvzigzag" => Symbol { unicode: 0x29DA, atom_type: AtomType::Open }, // Unicode: 0x29DA, left double wiggly fence
    "Rvzigzag" => Symbol { unicode: 0x29DB, atom_type: AtomType::Close }, // Unicode: 0x29DB, right double wiggly fence
    "iinfin" => Symbol { unicode: 0x29DC, atom_type: AtomType::Alpha }, // Unicode: 0x29DC, incomplete infinity
    "tieinfty" => Symbol { unicode: 0x29DD, atom_type: AtomType::Alpha }, // Unicode: 0x29DD, tie over infinity
    "nvinfty" => Symbol { unicode: 0x29DE, atom_type: AtomType::Alpha }, // Unicode: 0x29DE, infinity negated with vertical bar
    "dualmap" => Symbol { unicode: 0x29DF, atom_type: AtomType::Relation }, // Unicode: 0x29DF, double-ended multimap
    "laplac" => Symbol { unicode: 0x29E0, atom_type: AtomType::Alpha }, // Unicode: 0x29E0, square with contoured outline
    "lrtriangleeq" => Symbol { unicode: 0x29E1, atom_type: AtomType::Relation }, // Unicode: 0x29E1, increases as
    "shuffle" => Symbol { unicode: 0x29E2, atom_type: AtomType::Binary }, // Unicode: 0x29E2, shuffle product
    "eparsl" => Symbol { unicode: 0x29E3, atom_type: AtomType::Relation }, // Unicode: 0x29E3, equals sign and slanted parallel
    "smeparsl" => Symbol { unicode: 0x29E4, atom_type: AtomType::Relation }, // Unicode: 0x29E4, equals sign and slanted parallel with tilde above
    "eqvparsl" => Symbol { unicode: 0x29E5, atom_type: AtomType::Relation }, // Unicode: 0x29E5, identical to and slanted parallel
    "gleichstark" => Symbol { unicode: 0x29E6, atom_type: AtomType::Relation }, // Unicode: 0x29E6, gleich stark
    "thermod" => Symbol { unicode: 0x29E7, atom_type: AtomType::Alpha }, // Unicode: 0x29E7, thermodynamic
    "downtriangleleftblack" => Symbol { unicode: 0x29E8, atom_type: AtomType::Alpha }, // Unicode: 0x29E8, down-pointing triangle with left half black
    "downtrianglerightblack" => Symbol { unicode: 0x29E9, atom_type: AtomType::Alpha }, // Unicode: 0x29E9, down-pointing triangle with right half black
    "blackdiamonddownarrow" => Symbol { unicode: 0x29EA, atom_type: AtomType::Alpha }, // Unicode: 0x29EA, black diamond with down arrow
    "mdlgblklozenge" => Symbol { unicode: 0x29EB, atom_type: AtomType::Binary }, // Unicode: 0x29EB, black lozenge
    "circledownarrow" => Symbol { unicode: 0x29EC, atom_type: AtomType::Alpha }, // Unicode: 0x29EC, white circle with down arrow
    "blackcircledownarrow" => Symbol { unicode: 0x29ED, atom_type: AtomType::Alpha }, // Unicode: 0x29ED, black circle with down arrow
    "errbarsquare" => Symbol { unicode: 0x29EE, atom_type: AtomType::Alpha }, // Unicode: 0x29EE, error-barred white square
    "errbarblacksquare" => Symbol { unicode: 0x29EF, atom_type: AtomType::Alpha }, // Unicode: 0x29EF, error-barred black square
    "errbardiamond" => Symbol { unicode: 0x29F0, atom_type: AtomType::Alpha }, // Unicode: 0x29F0, error-barred white diamond
    "errbarblackdiamond" => Symbol { unicode: 0x29F1, atom_type: AtomType::Alpha }, // Unicode: 0x29F1, error-barred black diamond
    "errbarcircle" => Symbol { unicode: 0x29F2, atom_type: AtomType::Alpha }, // Unicode: 0x29F2, error-barred white circle
    "errbarblackcircle" => Symbol { unicode: 0x29F3, atom_type: AtomType::Alpha }, // Unicode: 0x29F3, error-barred black circle
    "ruledelayed" => Symbol { unicode: 0x29F4, atom_type: AtomType::Relation }, // Unicode: 0x29F4, rule-delayed
    "setminus" => Symbol { unicode: 0x29F5, atom_type: AtomType::Binary }, // Unicode: 0x29F5, reverse solidus operator
    "dsol" => Symbol { unicode: 0x29F6, atom_type: AtomType::Binary }, // Unicode: 0x29F6, solidus with overbar
    "rsolbar" => Symbol { unicode: 0x29F7, atom_type: AtomType::Binary }, // Unicode: 0x29F7, reverse solidus with horizontal stroke
    "xsol" => Symbol { unicode: 0x29F8, atom_type: AtomType::Operator(false) }, // Unicode: 0x29F8, big solidus
    "xbsol" => Symbol { unicode: 0x29F9, atom_type: AtomType::Operator(false) }, // Unicode: 0x29F9, big reverse solidus
    "doubleplus" => Symbol { unicode: 0x29FA, atom_type: AtomType::Binary }, // Unicode: 0x29FA, double plus
    "tripleplus" => Symbol { unicode: 0x29FB, atom_type: AtomType::Binary }, // Unicode: 0x29FB, triple plus
    "lcurvyangle" => Symbol { unicode: 0x29FC, atom_type: AtomType::Open }, // Unicode: 0x29FC, left pointing curved angle bracket
    "rcurvyangle" => Symbol { unicode: 0x29FD, atom_type: AtomType::Close }, // Unicode: 0x29FD, right pointing curved angle bracket
    "tplus" => Symbol { unicode: 0x29FE, atom_type: AtomType::Binary }, // Unicode: 0x29FE, tiny
    "tminus" => Symbol { unicode: 0x29FF, atom_type: AtomType::Binary }, // Unicode: 0x29FF, miny
    "bigodot" => Symbol { unicode: 0x2A00, atom_type: AtomType::Operator(true) }, // Unicode: 0x2A00, n-ary circled dot operator
    "bigoplus" => Symbol { unicode: 0x2A01, atom_type: AtomType::Operator(true) }, // Unicode: 0x2A01, n-ary circled plus operator
    "bigotimes" => Symbol { unicode: 0x2A02, atom_type: AtomType::Operator(true) }, // Unicode: 0x2A02, n-ary circled times operator
    "bigcupdot" => Symbol { unicode: 0x2A03, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A03, n-ary union operator with dot
    "biguplus" => Symbol { unicode: 0x2A04, atom_type: AtomType::Operator(true) }, // Unicode: 0x2A04, n-ary union operator with plus
    "bigsqcap" => Symbol { unicode: 0x2A05, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A05, n-ary square intersection operator
    "bigsqcup" => Symbol { unicode: 0x2A06, atom_type: AtomType::Operator(true) }, // Unicode: 0x2A06, n-ary square union operator
    "conjquant" => Symbol { unicode: 0x2A07, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A07, two logical and operator
    "disjquant" => Symbol { unicode: 0x2A08, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A08, two logical or operator
    "bigtimes" => Symbol { unicode: 0x2A09, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A09, n-ary times operator
    "modtwosum" => Symbol { unicode: 0x2A0A, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A0A, modulo two sum
    "sumint" => Symbol { unicode: 0x2A0B, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A0B, summation with integral
    "iiiint" => Symbol { unicode: 0x2A0C, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A0C, quadruple integral operator
    "intbar" => Symbol { unicode: 0x2A0D, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A0D, finite part integral
    "intBar" => Symbol { unicode: 0x2A0E, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A0E, integral with double stroke
    "fint" => Symbol { unicode: 0x2A0F, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A0F, integral average with slash
    "cirfnint" => Symbol { unicode: 0x2A10, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A10, circulation function
    "awint" => Symbol { unicode: 0x2A11, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A11, anticlockwise integration
    "rppolint" => Symbol { unicode: 0x2A12, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A12, line integration with rectangular path around pole
    "scpolint" => Symbol { unicode: 0x2A13, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A13, line integration with semicircular path around pole
    "npolint" => Symbol { unicode: 0x2A14, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A14, line integration not including the pole
    "pointint" => Symbol { unicode: 0x2A15, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A15, integral around a point operator
    "sqint" => Symbol { unicode: 0x2A16, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A16, quaternion integral operator
    "intlarhk" => Symbol { unicode: 0x2A17, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A17, integral with leftwards arrow with hook
    "intx" => Symbol { unicode: 0x2A18, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A18, integral with times sign
    "intcap" => Symbol { unicode: 0x2A19, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A19, integral with intersection
    "intcup" => Symbol { unicode: 0x2A1A, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A1A, integral with union
    "upint" => Symbol { unicode: 0x2A1B, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A1B, integral with overbar
    "lowint" => Symbol { unicode: 0x2A1C, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A1C, integral with underbar
    "Join" => Symbol { unicode: 0x2A1D, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A1D, join
    "bigtriangleleft" => Symbol { unicode: 0x2A1E, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A1E, large left triangle operator
    "zcmp" => Symbol { unicode: 0x2A1F, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A1F, z notation schema composition
    "zpipe" => Symbol { unicode: 0x2A20, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A20, z notation schema piping
    "zproject" => Symbol { unicode: 0x2A21, atom_type: AtomType::Operator(false) }, // Unicode: 0x2A21, z notation schema projection
    "ringplus" => Symbol { unicode: 0x2A22, atom_type: AtomType::Binary }, // Unicode: 0x2A22, plus sign with small circle above
    "plushat" => Symbol { unicode: 0x2A23, atom_type: AtomType::Binary }, // Unicode: 0x2A23, plus sign with circumflex accent above
    "simplus" => Symbol { unicode: 0x2A24, atom_type: AtomType::Binary }, // Unicode: 0x2A24, plus sign with tilde above
    "plusdot" => Symbol { unicode: 0x2A25, atom_type: AtomType::Binary }, // Unicode: 0x2A25, plus sign with dot below
    "plussim" => Symbol { unicode: 0x2A26, atom_type: AtomType::Binary }, // Unicode: 0x2A26, plus sign with tilde below
    "plussubtwo" => Symbol { unicode: 0x2A27, atom_type: AtomType::Binary }, // Unicode: 0x2A27, plus sign with subscript two
    "plustrif" => Symbol { unicode: 0x2A28, atom_type: AtomType::Binary }, // Unicode: 0x2A28, plus sign with black triangle
    "commaminus" => Symbol { unicode: 0x2A29, atom_type: AtomType::Binary }, // Unicode: 0x2A29, minus sign with comma above
    "minusdot" => Symbol { unicode: 0x2A2A, atom_type: AtomType::Binary }, // Unicode: 0x2A2A, minus sign with dot below
    "minusfdots" => Symbol { unicode: 0x2A2B, atom_type: AtomType::Binary }, // Unicode: 0x2A2B, minus sign with falling dots
    "minusrdots" => Symbol { unicode: 0x2A2C, atom_type: AtomType::Binary }, // Unicode: 0x2A2C, minus sign with rising dots
    "opluslhrim" => Symbol { unicode: 0x2A2D, atom_type: AtomType::Binary }, // Unicode: 0x2A2D, plus sign in left half circle
    "oplusrhrim" => Symbol { unicode: 0x2A2E, atom_type: AtomType::Binary }, // Unicode: 0x2A2E, plus sign in right half circle
    "vectimes" => Symbol { unicode: 0x2A2F, atom_type: AtomType::Binary }, // Unicode: 0x2A2F, vector or cross product
    "dottimes" => Symbol { unicode: 0x2A30, atom_type: AtomType::Binary }, // Unicode: 0x2A30, multiplication sign with dot above
    "timesbar" => Symbol { unicode: 0x2A31, atom_type: AtomType::Binary }, // Unicode: 0x2A31, multiplication sign with underbar
    "btimes" => Symbol { unicode: 0x2A32, atom_type: AtomType::Binary }, // Unicode: 0x2A32, semidirect product with bottom closed
    "smashtimes" => Symbol { unicode: 0x2A33, atom_type: AtomType::Binary }, // Unicode: 0x2A33, smash product
    "otimeslhrim" => Symbol { unicode: 0x2A34, atom_type: AtomType::Binary }, // Unicode: 0x2A34, multiplication sign in left half circle
    "otimesrhrim" => Symbol { unicode: 0x2A35, atom_type: AtomType::Binary }, // Unicode: 0x2A35, multiplication sign in right half circle
    "otimeshat" => Symbol { unicode: 0x2A36, atom_type: AtomType::Binary }, // Unicode: 0x2A36, circled multiplication sign with circumflex accent
    "Otimes" => Symbol { unicode: 0x2A37, atom_type: AtomType::Binary }, // Unicode: 0x2A37, multiplication sign in double circle
    "odiv" => Symbol { unicode: 0x2A38, atom_type: AtomType::Binary }, // Unicode: 0x2A38, circled division sign
    "triangleplus" => Symbol { unicode: 0x2A39, atom_type: AtomType::Binary }, // Unicode: 0x2A39, plus sign in triangle
    "triangleminus" => Symbol { unicode: 0x2A3A, atom_type: AtomType::Binary }, // Unicode: 0x2A3A, minus sign in triangle
    "triangletimes" => Symbol { unicode: 0x2A3B, atom_type: AtomType::Binary }, // Unicode: 0x2A3B, multiplication sign in triangle
    "intprod" => Symbol { unicode: 0x2A3C, atom_type: AtomType::Binary }, // Unicode: 0x2A3C, interior product
    "intprodr" => Symbol { unicode: 0x2A3D, atom_type: AtomType::Binary }, // Unicode: 0x2A3D, righthand interior product
    "fcmp" => Symbol { unicode: 0x2A3E, atom_type: AtomType::Binary }, // Unicode: 0x2A3E, z notation relational composition
    "amalg" => Symbol { unicode: 0x2A3F, atom_type: AtomType::Binary }, // Unicode: 0x2A3F, amalgamation or coproduct
    "capdot" => Symbol { unicode: 0x2A40, atom_type: AtomType::Binary }, // Unicode: 0x2A40, intersection with dot
    "uminus" => Symbol { unicode: 0x2A41, atom_type: AtomType::Binary }, // Unicode: 0x2A41, union with minus sign
    "barcup" => Symbol { unicode: 0x2A42, atom_type: AtomType::Binary }, // Unicode: 0x2A42, union with overbar
    "barcap" => Symbol { unicode: 0x2A43, atom_type: AtomType::Binary }, // Unicode: 0x2A43, intersection with overbar
    "capwedge" => Symbol { unicode: 0x2A44, atom_type: AtomType::Binary }, // Unicode: 0x2A44, intersection with logical and
    "cupvee" => Symbol { unicode: 0x2A45, atom_type: AtomType::Binary }, // Unicode: 0x2A45, union with logical or
    "cupovercap" => Symbol { unicode: 0x2A46, atom_type: AtomType::Binary }, // Unicode: 0x2A46, union above intersection
    "capovercup" => Symbol { unicode: 0x2A47, atom_type: AtomType::Binary }, // Unicode: 0x2A47, intersection above union
    "cupbarcap" => Symbol { unicode: 0x2A48, atom_type: AtomType::Binary }, // Unicode: 0x2A48, union above bar above intersection
    "capbarcup" => Symbol { unicode: 0x2A49, atom_type: AtomType::Binary }, // Unicode: 0x2A49, intersection above bar above union
    "twocups" => Symbol { unicode: 0x2A4A, atom_type: AtomType::Binary }, // Unicode: 0x2A4A, union beside and joined with union
    "twocaps" => Symbol { unicode: 0x2A4B, atom_type: AtomType::Binary }, // Unicode: 0x2A4B, intersection beside and joined with intersection
    "closedvarcup" => Symbol { unicode: 0x2A4C, atom_type: AtomType::Binary }, // Unicode: 0x2A4C, closed union with serifs
    "closedvarcap" => Symbol { unicode: 0x2A4D, atom_type: AtomType::Binary }, // Unicode: 0x2A4D, closed intersection with serifs
    "Sqcap" => Symbol { unicode: 0x2A4E, atom_type: AtomType::Binary }, // Unicode: 0x2A4E, double square intersection
    "Sqcup" => Symbol { unicode: 0x2A4F, atom_type: AtomType::Binary }, // Unicode: 0x2A4F, double square union
    "closedvarcupsmashprod" => Symbol { unicode: 0x2A50, atom_type: AtomType::Binary }, // Unicode: 0x2A50, closed union with serifs and smash product
    "wedgeodot" => Symbol { unicode: 0x2A51, atom_type: AtomType::Binary }, // Unicode: 0x2A51, logical and with dot above
    "veeodot" => Symbol { unicode: 0x2A52, atom_type: AtomType::Binary }, // Unicode: 0x2A52, logical or with dot above
    "Wedge" => Symbol { unicode: 0x2A53, atom_type: AtomType::Binary }, // Unicode: 0x2A53, double logical and
    "Vee" => Symbol { unicode: 0x2A54, atom_type: AtomType::Binary }, // Unicode: 0x2A54, double logical or
    "wedgeonwedge" => Symbol { unicode: 0x2A55, atom_type: AtomType::Binary }, // Unicode: 0x2A55, two intersecting logical and
    "veeonvee" => Symbol { unicode: 0x2A56, atom_type: AtomType::Binary }, // Unicode: 0x2A56, two intersecting logical or
    "bigslopedvee" => Symbol { unicode: 0x2A57, atom_type: AtomType::Binary }, // Unicode: 0x2A57, sloping large or
    "bigslopedwedge" => Symbol { unicode: 0x2A58, atom_type: AtomType::Binary }, // Unicode: 0x2A58, sloping large and
    "veeonwedge" => Symbol { unicode: 0x2A59, atom_type: AtomType::Relation }, // Unicode: 0x2A59, logical or overlapping logical and
    "wedgemidvert" => Symbol { unicode: 0x2A5A, atom_type: AtomType::Binary }, // Unicode: 0x2A5A, logical and with middle stem
    "veemidvert" => Symbol { unicode: 0x2A5B, atom_type: AtomType::Binary }, // Unicode: 0x2A5B, logical or with middle stem
    "midbarwedge" => Symbol { unicode: 0x2A5C, atom_type: AtomType::Binary }, // Unicode: 0x2A5C, ogical and with horizontal dash
    "midbarvee" => Symbol { unicode: 0x2A5D, atom_type: AtomType::Binary }, // Unicode: 0x2A5D, logical or with horizontal dash
    "doublebarwedge" => Symbol { unicode: 0x2A5E, atom_type: AtomType::Binary }, // Unicode: 0x2A5E, logical and with double overbar
    "wedgebar" => Symbol { unicode: 0x2A5F, atom_type: AtomType::Binary }, // Unicode: 0x2A5F, logical and with underbar
    "wedgedoublebar" => Symbol { unicode: 0x2A60, atom_type: AtomType::Binary }, // Unicode: 0x2A60, logical and with double underbar
    "varveebar" => Symbol { unicode: 0x2A61, atom_type: AtomType::Binary }, // Unicode: 0x2A61, small vee with underbar
    "doublebarvee" => Symbol { unicode: 0x2A62, atom_type: AtomType::Binary }, // Unicode: 0x2A62, logical or with double overbar
    "veedoublebar" => Symbol { unicode: 0x2A63, atom_type: AtomType::Binary }, // Unicode: 0x2A63, logical or with double underbar
    "dsub" => Symbol { unicode: 0x2A64, atom_type: AtomType::Binary }, // Unicode: 0x2A64, z notation domain antirestriction
    "rsub" => Symbol { unicode: 0x2A65, atom_type: AtomType::Binary }, // Unicode: 0x2A65, z notation range antirestriction
    "eqdot" => Symbol { unicode: 0x2A66, atom_type: AtomType::Relation }, // Unicode: 0x2A66, equals sign with dot below
    "dotequiv" => Symbol { unicode: 0x2A67, atom_type: AtomType::Relation }, // Unicode: 0x2A67, identical with dot above
    "equivVert" => Symbol { unicode: 0x2A68, atom_type: AtomType::Relation }, // Unicode: 0x2A68, triple horizontal bar with double vertical stroke
    "equivVvert" => Symbol { unicode: 0x2A69, atom_type: AtomType::Relation }, // Unicode: 0x2A69, triple horizontal bar with triple vertical stroke
    "dotsim" => Symbol { unicode: 0x2A6A, atom_type: AtomType::Relation }, // Unicode: 0x2A6A, tilde operator with dot above
    "simrdots" => Symbol { unicode: 0x2A6B, atom_type: AtomType::Relation }, // Unicode: 0x2A6B, tilde operator with rising dots
    "simminussim" => Symbol { unicode: 0x2A6C, atom_type: AtomType::Relation }, // Unicode: 0x2A6C, similar minus similar
    "congdot" => Symbol { unicode: 0x2A6D, atom_type: AtomType::Relation }, // Unicode: 0x2A6D, congruent with dot above
    "asteq" => Symbol { unicode: 0x2A6E, atom_type: AtomType::Relation }, // Unicode: 0x2A6E, equals with asterisk
    "hatapprox" => Symbol { unicode: 0x2A6F, atom_type: AtomType::Relation }, // Unicode: 0x2A6F, almost equal to with circumflex accent
    "approxeqq" => Symbol { unicode: 0x2A70, atom_type: AtomType::Relation }, // Unicode: 0x2A70, approximately equal or equal to
    "eqqplus" => Symbol { unicode: 0x2A71, atom_type: AtomType::Binary }, // Unicode: 0x2A71, equals sign above plus sign
    "pluseqq" => Symbol { unicode: 0x2A72, atom_type: AtomType::Binary }, // Unicode: 0x2A72, plus sign above equals sign
    "eqqsim" => Symbol { unicode: 0x2A73, atom_type: AtomType::Relation }, // Unicode: 0x2A73, equals sign above tilde operator
    "Coloneq" => Symbol { unicode: 0x2A74, atom_type: AtomType::Relation }, // Unicode: 0x2A74, double colon equal
    "eqeq" => Symbol { unicode: 0x2A75, atom_type: AtomType::Relation }, // Unicode: 0x2A75, two consecutive equals signs
    "eqeqeq" => Symbol { unicode: 0x2A76, atom_type: AtomType::Relation }, // Unicode: 0x2A76, three consecutive equals signs
    "ddotseq" => Symbol { unicode: 0x2A77, atom_type: AtomType::Relation }, // Unicode: 0x2A77, equals sign with two dots above and two dots below
    "equivDD" => Symbol { unicode: 0x2A78, atom_type: AtomType::Relation }, // Unicode: 0x2A78, equivalent with four dots above
    "ltcir" => Symbol { unicode: 0x2A79, atom_type: AtomType::Relation }, // Unicode: 0x2A79, less-than with circle inside
    "gtcir" => Symbol { unicode: 0x2A7A, atom_type: AtomType::Relation }, // Unicode: 0x2A7A, greater-than with circle inside
    "ltquest" => Symbol { unicode: 0x2A7B, atom_type: AtomType::Relation }, // Unicode: 0x2A7B, less-than with question mark above
    "gtquest" => Symbol { unicode: 0x2A7C, atom_type: AtomType::Relation }, // Unicode: 0x2A7C, greater-than with question mark above
    "leqslant" => Symbol { unicode: 0x2A7D, atom_type: AtomType::Relation }, // Unicode: 0x2A7D, less-than or slanted equal to
    "geqslant" => Symbol { unicode: 0x2A7E, atom_type: AtomType::Relation }, // Unicode: 0x2A7E, greater-than or slanted equal to
    "lesdot" => Symbol { unicode: 0x2A7F, atom_type: AtomType::Relation }, // Unicode: 0x2A7F, less-than or slanted equal to with dot inside
    "gesdot" => Symbol { unicode: 0x2A80, atom_type: AtomType::Relation }, // Unicode: 0x2A80, greater-than or slanted equal to with dot inside
    "lesdoto" => Symbol { unicode: 0x2A81, atom_type: AtomType::Relation }, // Unicode: 0x2A81, less-than or slanted equal to with dot above
    "gesdoto" => Symbol { unicode: 0x2A82, atom_type: AtomType::Relation }, // Unicode: 0x2A82, greater-than or slanted equal to with dot above
    "lesdotor" => Symbol { unicode: 0x2A83, atom_type: AtomType::Relation }, // Unicode: 0x2A83, less-than or slanted equal to with dot above right
    "gesdotol" => Symbol { unicode: 0x2A84, atom_type: AtomType::Relation }, // Unicode: 0x2A84, greater-than or slanted equal to with dot above left
    "lessapprox" => Symbol { unicode: 0x2A85, atom_type: AtomType::Relation }, // Unicode: 0x2A85, less-than or approximate
    "gtrapprox" => Symbol { unicode: 0x2A86, atom_type: AtomType::Relation }, // Unicode: 0x2A86, greater-than or approximate
    "lneq" => Symbol { unicode: 0x2A87, atom_type: AtomType::Relation }, // Unicode: 0x2A87, less-than and single-line not equal to
    "gneq" => Symbol { unicode: 0x2A88, atom_type: AtomType::Relation }, // Unicode: 0x2A88, greater-than and single-line not equal to
    "lnapprox" => Symbol { unicode: 0x2A89, atom_type: AtomType::Relation }, // Unicode: 0x2A89, less-than and not approximate
    "gnapprox" => Symbol { unicode: 0x2A8A, atom_type: AtomType::Relation }, // Unicode: 0x2A8A, greater-than and not approximate
    "lesseqqgtr" => Symbol { unicode: 0x2A8B, atom_type: AtomType::Relation }, // Unicode: 0x2A8B, less-than above double-line equal above greater-than
    "gtreqqless" => Symbol { unicode: 0x2A8C, atom_type: AtomType::Relation }, // Unicode: 0x2A8C, greater-than above double-line equal above less-than
    "lsime" => Symbol { unicode: 0x2A8D, atom_type: AtomType::Relation }, // Unicode: 0x2A8D, less-than above similar or equal
    "gsime" => Symbol { unicode: 0x2A8E, atom_type: AtomType::Relation }, // Unicode: 0x2A8E, greater-than above similar or equal
    "lsimg" => Symbol { unicode: 0x2A8F, atom_type: AtomType::Relation }, // Unicode: 0x2A8F, less-than above similar above greater-than
    "gsiml" => Symbol { unicode: 0x2A90, atom_type: AtomType::Relation }, // Unicode: 0x2A90, greater-than above similar above less-than
    "lgE" => Symbol { unicode: 0x2A91, atom_type: AtomType::Relation }, // Unicode: 0x2A91, less-than above greater-than above double-line equal
    "glE" => Symbol { unicode: 0x2A92, atom_type: AtomType::Relation }, // Unicode: 0x2A92, greater-than above less-than above double-line equal
    "lesges" => Symbol { unicode: 0x2A93, atom_type: AtomType::Relation }, // Unicode: 0x2A93, less-than above slanted equal above greater-than above slanted equal
    "gesles" => Symbol { unicode: 0x2A94, atom_type: AtomType::Relation }, // Unicode: 0x2A94, greater-than above slanted equal above less-than above slanted equal
    "eqslantless" => Symbol { unicode: 0x2A95, atom_type: AtomType::Relation }, // Unicode: 0x2A95, slanted equal to or less-than
    "eqslantgtr" => Symbol { unicode: 0x2A96, atom_type: AtomType::Relation }, // Unicode: 0x2A96, slanted equal to or greater-than
    "elsdot" => Symbol { unicode: 0x2A97, atom_type: AtomType::Relation }, // Unicode: 0x2A97, slanted equal to or less-than with dot inside
    "egsdot" => Symbol { unicode: 0x2A98, atom_type: AtomType::Relation }, // Unicode: 0x2A98, slanted equal to or greater-than with dot inside
    "eqqless" => Symbol { unicode: 0x2A99, atom_type: AtomType::Relation }, // Unicode: 0x2A99, double-line equal to or less-than
    "eqqgtr" => Symbol { unicode: 0x2A9A, atom_type: AtomType::Relation }, // Unicode: 0x2A9A, double-line equal to or greater-than
    "eqqslantless" => Symbol { unicode: 0x2A9B, atom_type: AtomType::Relation }, // Unicode: 0x2A9B, double-line slanted equal to or less-than
    "eqqslantgtr" => Symbol { unicode: 0x2A9C, atom_type: AtomType::Relation }, // Unicode: 0x2A9C, double-line slanted equal to or greater-than
    "simless" => Symbol { unicode: 0x2A9D, atom_type: AtomType::Relation }, // Unicode: 0x2A9D, similar or less-than
    "simgtr" => Symbol { unicode: 0x2A9E, atom_type: AtomType::Relation }, // Unicode: 0x2A9E, similar or greater-than
    "simlE" => Symbol { unicode: 0x2A9F, atom_type: AtomType::Relation }, // Unicode: 0x2A9F, similar above less-than above equals sign
    "simgE" => Symbol { unicode: 0x2AA0, atom_type: AtomType::Relation }, // Unicode: 0x2AA0, similar above greater-than above equals sign
    "Lt" => Symbol { unicode: 0x2AA1, atom_type: AtomType::Relation }, // Unicode: 0x2AA1, double nested less-than
    "Gt" => Symbol { unicode: 0x2AA2, atom_type: AtomType::Relation }, // Unicode: 0x2AA2, double nested greater-than
    "partialmeetcontraction" => Symbol { unicode: 0x2AA3, atom_type: AtomType::Relation }, // Unicode: 0x2AA3, double less-than with underbar
    "glj" => Symbol { unicode: 0x2AA4, atom_type: AtomType::Relation }, // Unicode: 0x2AA4, greater-than overlapping less-than
    "gla" => Symbol { unicode: 0x2AA5, atom_type: AtomType::Relation }, // Unicode: 0x2AA5, greater-than beside less-than
    "ltcc" => Symbol { unicode: 0x2AA6, atom_type: AtomType::Relation }, // Unicode: 0x2AA6, less-than closed by curve
    "gtcc" => Symbol { unicode: 0x2AA7, atom_type: AtomType::Relation }, // Unicode: 0x2AA7, greater-than closed by curve
    "lescc" => Symbol { unicode: 0x2AA8, atom_type: AtomType::Relation }, // Unicode: 0x2AA8, less-than closed by curve above slanted equal
    "gescc" => Symbol { unicode: 0x2AA9, atom_type: AtomType::Relation }, // Unicode: 0x2AA9, greater-than closed by curve above slanted equal
    "smt" => Symbol { unicode: 0x2AAA, atom_type: AtomType::Relation }, // Unicode: 0x2AAA, smaller than
    "lat" => Symbol { unicode: 0x2AAB, atom_type: AtomType::Relation }, // Unicode: 0x2AAB, larger than
    "smte" => Symbol { unicode: 0x2AAC, atom_type: AtomType::Relation }, // Unicode: 0x2AAC, smaller than or equal to
    "late" => Symbol { unicode: 0x2AAD, atom_type: AtomType::Relation }, // Unicode: 0x2AAD, larger than or equal to
    "bumpeqq" => Symbol { unicode: 0x2AAE, atom_type: AtomType::Relation }, // Unicode: 0x2AAE, equals sign with bumpy above
    "preceq" => Symbol { unicode: 0x2AAF, atom_type: AtomType::Relation }, // Unicode: 0x2AAF, precedes above single-line equals sign
    "succeq" => Symbol { unicode: 0x2AB0, atom_type: AtomType::Relation }, // Unicode: 0x2AB0, succeeds above single-line equals sign
    "precneq" => Symbol { unicode: 0x2AB1, atom_type: AtomType::Relation }, // Unicode: 0x2AB1, precedes above single-line not equal to
    "succneq" => Symbol { unicode: 0x2AB2, atom_type: AtomType::Relation }, // Unicode: 0x2AB2, succeeds above single-line not equal to
    "preceqq" => Symbol { unicode: 0x2AB3, atom_type: AtomType::Relation }, // Unicode: 0x2AB3, precedes above equals sign
    "succeqq" => Symbol { unicode: 0x2AB4, atom_type: AtomType::Relation }, // Unicode: 0x2AB4, succeeds above equals sign
    "precneqq" => Symbol { unicode: 0x2AB5, atom_type: AtomType::Relation }, // Unicode: 0x2AB5, precedes above not equal to
    "succneqq" => Symbol { unicode: 0x2AB6, atom_type: AtomType::Relation }, // Unicode: 0x2AB6, succeeds above not equal to
    "precapprox" => Symbol { unicode: 0x2AB7, atom_type: AtomType::Relation }, // Unicode: 0x2AB7, precedes above almost equal to
    "succapprox" => Symbol { unicode: 0x2AB8, atom_type: AtomType::Relation }, // Unicode: 0x2AB8, succeeds above almost equal to
    "precnapprox" => Symbol { unicode: 0x2AB9, atom_type: AtomType::Relation }, // Unicode: 0x2AB9, precedes above not almost equal to
    "succnapprox" => Symbol { unicode: 0x2ABA, atom_type: AtomType::Relation }, // Unicode: 0x2ABA, succeeds above not almost equal to
    "Prec" => Symbol { unicode: 0x2ABB, atom_type: AtomType::Relation }, // Unicode: 0x2ABB, double precedes
    "Succ" => Symbol { unicode: 0x2ABC, atom_type: AtomType::Relation }, // Unicode: 0x2ABC, double succeeds
    "subsetdot" => Symbol { unicode: 0x2ABD, atom_type: AtomType::Relation }, // Unicode: 0x2ABD, subset with dot
    "supsetdot" => Symbol { unicode: 0x2ABE, atom_type: AtomType::Relation }, // Unicode: 0x2ABE, superset with dot
    "subsetplus" => Symbol { unicode: 0x2ABF, atom_type: AtomType::Relation }, // Unicode: 0x2ABF, subset with plus sign below
    "supsetplus" => Symbol { unicode: 0x2AC0, atom_type: AtomType::Relation }, // Unicode: 0x2AC0, superset with plus sign below
    "submult" => Symbol { unicode: 0x2AC1, atom_type: AtomType::Relation }, // Unicode: 0x2AC1, subset with multiplication sign below
    "supmult" => Symbol { unicode: 0x2AC2, atom_type: AtomType::Relation }, // Unicode: 0x2AC2, superset with multiplication sign below
    "subedot" => Symbol { unicode: 0x2AC3, atom_type: AtomType::Relation }, // Unicode: 0x2AC3, subset of or equal to with dot above
    "supedot" => Symbol { unicode: 0x2AC4, atom_type: AtomType::Relation }, // Unicode: 0x2AC4, superset of or equal to with dot above
    "subseteqq" => Symbol { unicode: 0x2AC5, atom_type: AtomType::Relation }, // Unicode: 0x2AC5, subset of above equals sign
    "supseteqq" => Symbol { unicode: 0x2AC6, atom_type: AtomType::Relation }, // Unicode: 0x2AC6, superset of above equals sign
    "subsim" => Symbol { unicode: 0x2AC7, atom_type: AtomType::Relation }, // Unicode: 0x2AC7, subset of above tilde operator
    "supsim" => Symbol { unicode: 0x2AC8, atom_type: AtomType::Relation }, // Unicode: 0x2AC8, superset of above tilde operator
    "subsetapprox" => Symbol { unicode: 0x2AC9, atom_type: AtomType::Relation }, // Unicode: 0x2AC9, subset of above almost equal to
    "supsetapprox" => Symbol { unicode: 0x2ACA, atom_type: AtomType::Relation }, // Unicode: 0x2ACA, superset of above almost equal to
    "subsetneqq" => Symbol { unicode: 0x2ACB, atom_type: AtomType::Relation }, // Unicode: 0x2ACB, subset of above not equal to
    "supsetneqq" => Symbol { unicode: 0x2ACC, atom_type: AtomType::Relation }, // Unicode: 0x2ACC, superset of above not equal to
    "lsqhook" => Symbol { unicode: 0x2ACD, atom_type: AtomType::Relation }, // Unicode: 0x2ACD, square left open box operator
    "rsqhook" => Symbol { unicode: 0x2ACE, atom_type: AtomType::Relation }, // Unicode: 0x2ACE, square right open box operator
    "csub" => Symbol { unicode: 0x2ACF, atom_type: AtomType::Relation }, // Unicode: 0x2ACF, closed subset
    "csup" => Symbol { unicode: 0x2AD0, atom_type: AtomType::Relation }, // Unicode: 0x2AD0, closed superset
    "csube" => Symbol { unicode: 0x2AD1, atom_type: AtomType::Relation }, // Unicode: 0x2AD1, closed subset or equal to
    "csupe" => Symbol { unicode: 0x2AD2, atom_type: AtomType::Relation }, // Unicode: 0x2AD2, closed superset or equal to
    "subsup" => Symbol { unicode: 0x2AD3, atom_type: AtomType::Relation }, // Unicode: 0x2AD3, subset above superset
    "supsub" => Symbol { unicode: 0x2AD4, atom_type: AtomType::Relation }, // Unicode: 0x2AD4, superset above subset
    "subsub" => Symbol { unicode: 0x2AD5, atom_type: AtomType::Relation }, // Unicode: 0x2AD5, subset above subset
    "supsup" => Symbol { unicode: 0x2AD6, atom_type: AtomType::Relation }, // Unicode: 0x2AD6, superset above superset
    "suphsub" => Symbol { unicode: 0x2AD7, atom_type: AtomType::Relation }, // Unicode: 0x2AD7, superset beside subset
    "supdsub" => Symbol { unicode: 0x2AD8, atom_type: AtomType::Relation }, // Unicode: 0x2AD8, superset beside and joined by dash with subset
    "forkv" => Symbol { unicode: 0x2AD9, atom_type: AtomType::Relation }, // Unicode: 0x2AD9, element of opening downwards
    "topfork" => Symbol { unicode: 0x2ADA, atom_type: AtomType::Relation }, // Unicode: 0x2ADA, pitchfork with tee top
    "mlcp" => Symbol { unicode: 0x2ADB, atom_type: AtomType::Relation }, // Unicode: 0x2ADB, transversal intersection
    "forks" => Symbol { unicode: 0x2ADC, atom_type: AtomType::Relation }, // Unicode: 0x2ADC, forking
    "forksnot" => Symbol { unicode: 0x2ADD, atom_type: AtomType::Relation }, // Unicode: 0x2ADD, nonforking
    "shortlefttack" => Symbol { unicode: 0x2ADE, atom_type: AtomType::Relation }, // Unicode: 0x2ADE, short left tack
    "shortdowntack" => Symbol { unicode: 0x2ADF, atom_type: AtomType::Relation }, // Unicode: 0x2ADF, short down tack
    "shortuptack" => Symbol { unicode: 0x2AE0, atom_type: AtomType::Relation }, // Unicode: 0x2AE0, short up tack
    "perps" => Symbol { unicode: 0x2AE1, atom_type: AtomType::Alpha }, // Unicode: 0x2AE1, perpendicular with s
    "vDdash" => Symbol { unicode: 0x2AE2, atom_type: AtomType::Relation }, // Unicode: 0x2AE2, vertical bar triple right turnstile
    "dashV" => Symbol { unicode: 0x2AE3, atom_type: AtomType::Relation }, // Unicode: 0x2AE3, double vertical bar left turnstile
    "Dashv" => Symbol { unicode: 0x2AE4, atom_type: AtomType::Relation }, // Unicode: 0x2AE4, vertical bar double left turnstile
    "DashV" => Symbol { unicode: 0x2AE5, atom_type: AtomType::Relation }, // Unicode: 0x2AE5, double vertical bar double left turnstile
    "varVdash" => Symbol { unicode: 0x2AE6, atom_type: AtomType::Relation }, // Unicode: 0x2AE6, long dash from left member of double vertical
    "Barv" => Symbol { unicode: 0x2AE7, atom_type: AtomType::Relation }, // Unicode: 0x2AE7, short down tack with overbar
    "vBar" => Symbol { unicode: 0x2AE8, atom_type: AtomType::Relation }, // Unicode: 0x2AE8, short up tack with underbar
    "vBarv" => Symbol { unicode: 0x2AE9, atom_type: AtomType::Relation }, // Unicode: 0x2AE9, short up tack above short down tack
    "barV" => Symbol { unicode: 0x2AEA, atom_type: AtomType::Relation }, // Unicode: 0x2AEA, double down tack
    "Vbar" => Symbol { unicode: 0x2AEB, atom_type: AtomType::Relation }, // Unicode: 0x2AEB, double up tack
    "Not" => Symbol { unicode: 0x2AEC, atom_type: AtomType::Relation }, // Unicode: 0x2AEC, double stroke not sign
    "bNot" => Symbol { unicode: 0x2AED, atom_type: AtomType::Relation }, // Unicode: 0x2AED, reversed double stroke not sign
    "revnmid" => Symbol { unicode: 0x2AEE, atom_type: AtomType::Relation }, // Unicode: 0x2AEE, does not divide with reversed negation slash
    "cirmid" => Symbol { unicode: 0x2AEF, atom_type: AtomType::Relation }, // Unicode: 0x2AEF, vertical line with circle above
    "midcir" => Symbol { unicode: 0x2AF0, atom_type: AtomType::Relation }, // Unicode: 0x2AF0, vertical line with circle below
    "topcir" => Symbol { unicode: 0x2AF1, atom_type: AtomType::Alpha }, // Unicode: 0x2AF1, down tack with circle below
    "nhpar" => Symbol { unicode: 0x2AF2, atom_type: AtomType::Relation }, // Unicode: 0x2AF2, parallel with horizontal stroke
    "parsim" => Symbol { unicode: 0x2AF3, atom_type: AtomType::Relation }, // Unicode: 0x2AF3, parallel with tilde operator
    "interleave" => Symbol { unicode: 0x2AF4, atom_type: AtomType::Binary }, // Unicode: 0x2AF4, triple vertical bar binary relation
    "nhVvert" => Symbol { unicode: 0x2AF5, atom_type: AtomType::Binary }, // Unicode: 0x2AF5, triple vertical bar with horizontal stroke
    "threedotcolon" => Symbol { unicode: 0x2AF6, atom_type: AtomType::Binary }, // Unicode: 0x2AF6, triple colon operator
    "lllnest" => Symbol { unicode: 0x2AF7, atom_type: AtomType::Relation }, // Unicode: 0x2AF7, stacked very much less-than
    "gggnest" => Symbol { unicode: 0x2AF8, atom_type: AtomType::Relation }, // Unicode: 0x2AF8, stacked very much greater-than
    "leqqslant" => Symbol { unicode: 0x2AF9, atom_type: AtomType::Relation }, // Unicode: 0x2AF9, double-line slanted less-than or equal to
    "geqqslant" => Symbol { unicode: 0x2AFA, atom_type: AtomType::Relation }, // Unicode: 0x2AFA, double-line slanted greater-than or equal to
    "trslash" => Symbol { unicode: 0x2AFB, atom_type: AtomType::Binary }, // Unicode: 0x2AFB, triple solidus binary relation
    "biginterleave" => Symbol { unicode: 0x2AFC, atom_type: AtomType::Operator(false) }, // Unicode: 0x2AFC, large triple vertical bar operator
    "sslash" => Symbol { unicode: 0x2AFD, atom_type: AtomType::Binary }, // Unicode: 0x2AFD, double solidus operator
    "talloblong" => Symbol { unicode: 0x2AFE, atom_type: AtomType::Binary }, // Unicode: 0x2AFE, white vertical bar
    "bigtalloblong" => Symbol { unicode: 0x2AFF, atom_type: AtomType::Operator(false) }, // Unicode: 0x2AFF, n-ary white vertical bar
    "squaretopblack" => Symbol { unicode: 0x2B12, atom_type: AtomType::Alpha }, // Unicode: 0x2B12, square with top half black
    "squarebotblack" => Symbol { unicode: 0x2B13, atom_type: AtomType::Alpha }, // Unicode: 0x2B13, square with bottom half black
    "squareurblack" => Symbol { unicode: 0x2B14, atom_type: AtomType::Alpha }, // Unicode: 0x2B14, square with upper right diagonal half black
    "squarellblack" => Symbol { unicode: 0x2B15, atom_type: AtomType::Alpha }, // Unicode: 0x2B15, square with lower left diagonal half black
    "diamondleftblack" => Symbol { unicode: 0x2B16, atom_type: AtomType::Alpha }, // Unicode: 0x2B16, diamond with left half black
    "diamondrightblack" => Symbol { unicode: 0x2B17, atom_type: AtomType::Alpha }, // Unicode: 0x2B17, diamond with right half black
    "diamondtopblack" => Symbol { unicode: 0x2B18, atom_type: AtomType::Alpha }, // Unicode: 0x2B18, diamond with top half black
    "diamondbotblack" => Symbol { unicode: 0x2B19, atom_type: AtomType::Alpha }, // Unicode: 0x2B19, diamond with bottom half black
    "dottedsquare" => Symbol { unicode: 0x2B1A, atom_type: AtomType::Alpha }, // Unicode: 0x2B1A, dotted square
    "lgblksquare" => Symbol { unicode: 0x2B1B, atom_type: AtomType::Alpha }, // Unicode: 0x2B1B, black large square
    "lgwhtsquare" => Symbol { unicode: 0x2B1C, atom_type: AtomType::Alpha }, // Unicode: 0x2B1C, white large square
    "vysmblksquare" => Symbol { unicode: 0x2B1D, atom_type: AtomType::Alpha }, // Unicode: 0x2B1D, black very small square
    "vysmwhtsquare" => Symbol { unicode: 0x2B1E, atom_type: AtomType::Alpha }, // Unicode: 0x2B1E, white very small square
    "pentagonblack" => Symbol { unicode: 0x2B1F, atom_type: AtomType::Alpha }, // Unicode: 0x2B1F, black pentagon
    "pentagon" => Symbol { unicode: 0x2B20, atom_type: AtomType::Alpha }, // Unicode: 0x2B20, white pentagon
    "varhexagon" => Symbol { unicode: 0x2B21, atom_type: AtomType::Alpha }, // Unicode: 0x2B21, white hexagon
    "varhexagonblack" => Symbol { unicode: 0x2B22, atom_type: AtomType::Alpha }, // Unicode: 0x2B22, black hexagon
    "hexagonblack" => Symbol { unicode: 0x2B23, atom_type: AtomType::Alpha }, // Unicode: 0x2B23, horizontal black hexagon
    "lgblkcircle" => Symbol { unicode: 0x2B24, atom_type: AtomType::Alpha }, // Unicode: 0x2B24, black large circle
    "mdblkdiamond" => Symbol { unicode: 0x2B25, atom_type: AtomType::Alpha }, // Unicode: 0x2B25, black medium diamond
    "mdwhtdiamond" => Symbol { unicode: 0x2B26, atom_type: AtomType::Alpha }, // Unicode: 0x2B26, white medium diamond
    "mdblklozenge" => Symbol { unicode: 0x2B27, atom_type: AtomType::Alpha }, // Unicode: 0x2B27, black medium lozenge
    "mdwhtlozenge" => Symbol { unicode: 0x2B28, atom_type: AtomType::Alpha }, // Unicode: 0x2B28, white medium lozenge
    "smblkdiamond" => Symbol { unicode: 0x2B29, atom_type: AtomType::Alpha }, // Unicode: 0x2B29, black small diamond
    "smblklozenge" => Symbol { unicode: 0x2B2A, atom_type: AtomType::Alpha }, // Unicode: 0x2B2A, black small lozenge
    "smwhtlozenge" => Symbol { unicode: 0x2B2B, atom_type: AtomType::Alpha }, // Unicode: 0x2B2B, white small lozenge
    "blkhorzoval" => Symbol { unicode: 0x2B2C, atom_type: AtomType::Alpha }, // Unicode: 0x2B2C, black horizontal ellipse
    "whthorzoval" => Symbol { unicode: 0x2B2D, atom_type: AtomType::Alpha }, // Unicode: 0x2B2D, white horizontal ellipse
    "blkvertoval" => Symbol { unicode: 0x2B2E, atom_type: AtomType::Alpha }, // Unicode: 0x2B2E, black vertical ellipse
    "whtvertoval" => Symbol { unicode: 0x2B2F, atom_type: AtomType::Alpha }, // Unicode: 0x2B2F, white vertical ellipse
    "circleonleftarrow" => Symbol { unicode: 0x2B30, atom_type: AtomType::Relation }, // Unicode: 0x2B30, left arrow with small circle
    "leftthreearrows" => Symbol { unicode: 0x2B31, atom_type: AtomType::Relation }, // Unicode: 0x2B31, three leftwards arrows
    "leftarrowonoplus" => Symbol { unicode: 0x2B32, atom_type: AtomType::Relation }, // Unicode: 0x2B32, left arrow with circled plus
    "longleftsquigarrow" => Symbol { unicode: 0x2B33, atom_type: AtomType::Relation }, // Unicode: 0x2B33, long leftwards squiggle arrow
    "nvtwoheadleftarrow" => Symbol { unicode: 0x2B34, atom_type: AtomType::Relation }, // Unicode: 0x2B34, leftwards two-headed arrow with vertical stroke
    "nVtwoheadleftarrow" => Symbol { unicode: 0x2B35, atom_type: AtomType::Relation }, // Unicode: 0x2B35, leftwards two-headed arrow with double vertical stroke
    "twoheadmapsfrom" => Symbol { unicode: 0x2B36, atom_type: AtomType::Relation }, // Unicode: 0x2B36, leftwards two-headed arrow from bar
    "twoheadleftdbkarrow" => Symbol { unicode: 0x2B37, atom_type: AtomType::Relation }, // Unicode: 0x2B37, leftwards two-headed triple-dash arrow
    "leftdotarrow" => Symbol { unicode: 0x2B38, atom_type: AtomType::Relation }, // Unicode: 0x2B38, leftwards arrow with dotted stem
    "nvleftarrowtail" => Symbol { unicode: 0x2B39, atom_type: AtomType::Relation }, // Unicode: 0x2B39, leftwards arrow with tail with vertical stroke
    "nVleftarrowtail" => Symbol { unicode: 0x2B3A, atom_type: AtomType::Relation }, // Unicode: 0x2B3A, leftwards arrow with tail with double vertical stroke
    "twoheadleftarrowtail" => Symbol { unicode: 0x2B3B, atom_type: AtomType::Relation }, // Unicode: 0x2B3B, leftwards two-headed arrow with tail
    "nvtwoheadleftarrowtail" => Symbol { unicode: 0x2B3C, atom_type: AtomType::Relation }, // Unicode: 0x2B3C, leftwards two-headed arrow with tail with vertical stroke
    "nVtwoheadleftarrowtail" => Symbol { unicode: 0x2B3D, atom_type: AtomType::Relation }, // Unicode: 0x2B3D, leftwards two-headed arrow with tail with double vertical stroke
    "leftarrowx" => Symbol { unicode: 0x2B3E, atom_type: AtomType::Relation }, // Unicode: 0x2B3E, leftwards arrow through x
    "leftcurvedarrow" => Symbol { unicode: 0x2B3F, atom_type: AtomType::Relation }, // Unicode: 0x2B3F, wave arrow pointing directly left
    "equalleftarrow" => Symbol { unicode: 0x2B40, atom_type: AtomType::Relation }, // Unicode: 0x2B40, equals sign above leftwards arrow
    "bsimilarleftarrow" => Symbol { unicode: 0x2B41, atom_type: AtomType::Relation }, // Unicode: 0x2B41, reverse tilde operator above leftwards arrow
    "leftarrowbackapprox" => Symbol { unicode: 0x2B42, atom_type: AtomType::Relation }, // Unicode: 0x2B42, leftwards arrow above reverse almost equal to
    "rightarrowgtr" => Symbol { unicode: 0x2B43, atom_type: AtomType::Relation }, // Unicode: 0x2B43, rightwards arrow through greater-than
    "rightarrowsupset" => Symbol { unicode: 0x2B44, atom_type: AtomType::Relation }, // Unicode: 0x2B44, rightwards arrow through subset
    "LLeftarrow" => Symbol { unicode: 0x2B45, atom_type: AtomType::Relation }, // Unicode: 0x2B45, leftwards quadruple arrow
    "RRightarrow" => Symbol { unicode: 0x2B46, atom_type: AtomType::Relation }, // Unicode: 0x2B46, rightwards quadruple arrow
    "bsimilarrightarrow" => Symbol { unicode: 0x2B47, atom_type: AtomType::Relation }, // Unicode: 0x2B47, reverse tilde operator above rightwards arrow
    "rightarrowbackapprox" => Symbol { unicode: 0x2B48, atom_type: AtomType::Relation }, // Unicode: 0x2B48, rightwards arrow above reverse almost equal to
    "similarleftarrow" => Symbol { unicode: 0x2B49, atom_type: AtomType::Relation }, // Unicode: 0x2B49, tilde operator above leftwards arrow
    "leftarrowapprox" => Symbol { unicode: 0x2B4A, atom_type: AtomType::Relation }, // Unicode: 0x2B4A, leftwards arrow above almost equal to
    "leftarrowbsimilar" => Symbol { unicode: 0x2B4B, atom_type: AtomType::Relation }, // Unicode: 0x2B4B, leftwards arrow above reverse tilde operator
    "rightarrowbsimilar" => Symbol { unicode: 0x2B4C, atom_type: AtomType::Relation }, // Unicode: 0x2B4C, righttwards arrow above reverse tilde operator
    "medwhitestar" => Symbol { unicode: 0x2B50, atom_type: AtomType::Alpha }, // Unicode: 0x2B50, white medium star
    "medblackstar" => Symbol { unicode: 0x2B51, atom_type: AtomType::Alpha }, // Unicode: 0x2B51, black medium star
    "smwhitestar" => Symbol { unicode: 0x2B52, atom_type: AtomType::Alpha }, // Unicode: 0x2B52, white small star
    "rightpentagonblack" => Symbol { unicode: 0x2B53, atom_type: AtomType::Alpha }, // Unicode: 0x2B53, black right-pointing pentagon
    "rightpentagon" => Symbol { unicode: 0x2B54, atom_type: AtomType::Alpha }, // Unicode: 0x2B54, white right-pointing pentagon
    "postalmark" => Symbol { unicode: 0x3012, atom_type: AtomType::Alpha }, // Unicode: 0x3012, postal mark
    "hzigzag" => Symbol { unicode: 0x3030, atom_type: AtomType::Alpha }, // Unicode: 0x3030, zigzag

    // Additional commands from TeX
    "Alpha" => Symbol { unicode: 0x391, atom_type: AtomType::Alpha }, // Unicode: 0x391,
    "Beta" => Symbol { unicode: 0x392, atom_type: AtomType::Alpha }, // Unicode: 0x392,
    "Gamma" => Symbol { unicode: 0x393, atom_type: AtomType::Alpha }, // Unicode: 0x393,
    "Delta" => Symbol { unicode: 0x394, atom_type: AtomType::Alpha }, // Unicode: 0x394,
    "Epsilon" => Symbol { unicode: 0x395, atom_type: AtomType::Alpha }, // Unicode: 0x395,
    "Zeta" => Symbol { unicode: 0x396, atom_type: AtomType::Alpha }, // Unicode: 0x396,
    "Eta" => Symbol { unicode: 0x397, atom_type: AtomType::Alpha }, // Unicode: 0x397,
    "Theta" => Symbol { unicode: 0x398, atom_type: AtomType::Alpha }, // Unicode: 0x398,
    "Iota" => Symbol { unicode: 0x399, atom_type: AtomType::Alpha }, // Unicode: 0x399,
    "Kappa" => Symbol { unicode: 0x39A, atom_type: AtomType::Alpha }, // Unicode: 0x39A,
    "Lambda" => Symbol { unicode: 0x39B, atom_type: AtomType::Alpha }, // Unicode: 0x39B,
    "Mu" => Symbol { unicode: 0x39C, atom_type: AtomType::Alpha }, // Unicode: 0x39C,
    "Nu" => Symbol { unicode: 0x39D, atom_type: AtomType::Alpha }, // Unicode: 0x39D,
    "Xi" => Symbol { unicode: 0x39E, atom_type: AtomType::Alpha }, // Unicode: 0x39E,
    "Omicron" => Symbol { unicode: 0x39F, atom_type: AtomType::Alpha }, // Unicode: 0x39F,
    "Pi" => Symbol { unicode: 0x3A0, atom_type: AtomType::Alpha }, // Unicode: 0x3A0,
    "Rho" => Symbol { unicode: 0x3A1, atom_type: AtomType::Alpha }, // Unicode: 0x3A1,
    "Sigma" => Symbol { unicode: 0x3A3, atom_type: AtomType::Alpha }, // Unicode: 0x3A3,
    "Tau" => Symbol { unicode: 0x3A4, atom_type: AtomType::Alpha }, // Unicode: 0x3A4,
    "Upsilon" => Symbol { unicode: 0x3A5, atom_type: AtomType::Alpha }, // Unicode: 0x3A5,
    "Phi" => Symbol { unicode: 0x3A6, atom_type: AtomType::Alpha }, // Unicode: 0x3A6,
    "Chi" => Symbol { unicode: 0x3A7, atom_type: AtomType::Alpha }, // Unicode: 0x3A7,
    "Psi" => Symbol { unicode: 0x3A8, atom_type: AtomType::Alpha }, // Unicode: 0x3A8,
    "Omega" => Symbol { unicode: 0x3A9, atom_type: AtomType::Alpha }, // Unicode: 0x3A9,
    "alpha" => Symbol { unicode: 0x3B1, atom_type: AtomType::Alpha }, // Unicode: 0x3B1,
    "beta" => Symbol { unicode: 0x3B2, atom_type: AtomType::Alpha }, // Unicode: 0x3B2,
    "gamma" => Symbol { unicode: 0x3B3, atom_type: AtomType::Alpha }, // Unicode: 0x3B3,
    "delta" => Symbol { unicode: 0x3B4, atom_type: AtomType::Alpha }, // Unicode: 0x3B4,
    "epsilon" => Symbol { unicode: 0x3B5, atom_type: AtomType::Alpha }, // Unicode: 0x3B5,
    "zeta" => Symbol { unicode: 0x3B6, atom_type: AtomType::Alpha }, // Unicode: 0x3B6,
    "eta" => Symbol { unicode: 0x3B7, atom_type: AtomType::Alpha }, // Unicode: 0x3B7,
    "theta" => Symbol { unicode: 0x3B8, atom_type: AtomType::Alpha }, // Unicode: 0x3B8,
    "iota" => Symbol { unicode: 0x3B9, atom_type: AtomType::Alpha }, // Unicode: 0x3B9,
    "kappa" => Symbol { unicode: 0x3BA, atom_type: AtomType::Alpha }, // Unicode: 0x3BA,
    "lambda" => Symbol { unicode: 0x3BB, atom_type: AtomType::Alpha }, // Unicode: 0x3BB,
    "mu" => Symbol { unicode: 0x3BC, atom_type: AtomType::Alpha }, // Unicode: 0x3BC,
    "nu" => Symbol { unicode: 0x3BD, atom_type: AtomType::Alpha }, // Unicode: 0x3BD,
    "xi" => Symbol { unicode: 0x3BE, atom_type: AtomType::Alpha }, // Unicode: 0x3BE,
    "omicron" => Symbol { unicode: 0x3BF, atom_type: AtomType::Alpha }, // Unicode: 0x3BF,
    "pi" => Symbol { unicode: 0x3C0, atom_type: AtomType::Alpha }, // Unicode: 0x3C0,
    "rho" => Symbol { unicode: 0x3C1, atom_type: AtomType::Alpha }, // Unicode: 0x3C1,
    "sigma" => Symbol { unicode: 0x3C3, atom_type: AtomType::Alpha }, // Unicode: 0x3C3,
    "tau" => Symbol { unicode: 0x3C4, atom_type: AtomType::Alpha }, // Unicode: 0x3C4,
    "upsilon" => Symbol { unicode: 0x3C5, atom_type: AtomType::Alpha }, // Unicode: 0x3C5,
    "phi" => Symbol { unicode: 0x3C6, atom_type: AtomType::Alpha }, // Unicode: 0x3C6,
    "chi" => Symbol { unicode: 0x3C7, atom_type: AtomType::Alpha }, // Unicode: 0x3C7,
    "psi" => Symbol { unicode: 0x3C8, atom_type: AtomType::Alpha }, // Unicode: 0x3C8,
    "omega" => Symbol { unicode: 0x3C9, atom_type: AtomType::Alpha }, // Unicode: 0x3C9,
    "varphi" => Symbol { unicode: 0x3C6, atom_type: AtomType::Alpha }, // Unicode: 0x3C6, curly or open small phi, greek
    "varsigma" => Symbol { unicode: 0x3C2, atom_type: AtomType::Alpha }, // Unicode: 0x3C2, terminal sigma, greek
    "varbeta" => Symbol { unicode: 0x3D0, atom_type: AtomType::Alpha }, // Unicode: 0x3D0, rounded small beta, greek
    "vartheta" => Symbol { unicode: 0x3D1, atom_type: AtomType::Alpha }, // Unicode: 0x3D1, /vartheta - curly or open theta
    "varpi" => Symbol { unicode: 0x3D6, atom_type: AtomType::Alpha }, // Unicode: 0x3D6, rounded small pi (pomega), greek
    "varkappa" => Symbol { unicode: 0x3F0, atom_type: AtomType::Alpha }, // Unicode: 0x3F0, rounded small kappa, greek
    "varrho" => Symbol { unicode: 0x3F1, atom_type: AtomType::Alpha }, // Unicode: 0x3F1, rounded small rho, greek
    "varTheta" => Symbol { unicode: 0x3F4, atom_type: AtomType::Alpha }, // Unicode: 0x3F4, greek capital theta symbol
    "varepsilon" => Symbol { unicode: 0x3F5, atom_type: AtomType::Alpha }, // Unicode: 0x3F5, greek lunate epsilon symbol
};

static KEYS: &[&str] = &["nvLeftrightarrow",
                         "diamondtopblack",
                         "glj",
                         "nsucccurlyeq",
                         "nVleftrightarrow",
                         "triangles",
                         "wedgedot",
                         "bigsqcup",
                         "xi",
                         "Xi",
                         "approxident",
                         "measangleurtone",
                         "ddotseq",
                         "shortuptack",
                         "underbrace",
                         "lesges",
                         "ccwundercurvearrow",
                         "hexagonblack",
                         "lftimes",
                         "mid",
                         "bumpeq",
                         "tilde",
                         "rrparenthesis",
                         "leftouterjoin",
                         "fisheye",
                         "Theta",
                         "Sqcup",
                         "rightarrowplus",
                         "vDash",
                         "overbar",
                         "midbarwedge",
                         "cirscir",
                         "Doteq",
                         "curvearrowrightminus",
                         "mdblklozenge",
                         "longrightarrow",
                         "lcurvyangle",
                         "smt",
                         "blacktriangle",
                         "lparen",
                         "rbrbrak",
                         "forks",
                         "doubleplus",
                         "leqslant",
                         "Mapsto",
                         "Swarrow",
                         "vrectangle",
                         "subsetplus",
                         "twonotes",
                         "subsup",
                         "measuredangle",
                         "precsim",
                         "leqq",
                         "nvartriangleleft",
                         "lesseqqgtr",
                         "lnapprox",
                         "lbracklend",
                         "gneqq",
                         "operp",
                         "downrightcurvedarrow",
                         "ointctrclockwise",
                         "isinE",
                         "infty",
                         "eqdot",
                         "leftbkarrow",
                         "mho",
                         "blackinwhitediamond",
                         "squareleftblack",
                         "isinvb",
                         "ltlarr",
                         "subsim",
                         "smblksquare",
                         "lesseqgtr",
                         "Gt",
                         "veeonwedge",
                         "nni",
                         "pentagon",
                         "trapezium",
                         "simneqq",
                         "bigblacktriangledown",
                         "barrightharpoondown",
                         "downuparrows",
                         "nlesssim",
                         "congdot",
                         "triangledown",
                         "mfrakZ",
                         "equalparallel",
                         "bardownharpoonleft",
                         "accurrent",
                         "bigtriangledown",
                         "parallel",
                         "APLboxupcaret",
                         "equiv",
                         "Barv",
                         "bigtop",
                         "check",
                         "similarleftarrow",
                         "nvleftarrowtail",
                         "dashrightharpoondown",
                         "nVtwoheadleftarrowtail",
                         "twocups",
                         "lefttail",
                         "mathvisiblespace",
                         "revangle",
                         "bNot",
                         "boxtimes",
                         "eqsim",
                         "llangle",
                         "obslash",
                         "whitesquaretickright",
                         "urblacktriangle",
                         "smallni",
                         "righttail",
                         "forksnot",
                         "veedot",
                         "simeq",
                         "smallin",
                         "twoheadrightarrow",
                         "twolowline",
                         "barcap",
                         "tripleplus",
                         "leftarrowtail",
                         "bdtriplevdash",
                         "parallelogramblack",
                         "smallblacktriangleleft",
                         "Downarrow",
                         "toea",
                         "dotsminusdots",
                         "leftwavearrow",
                         "barleftharpoonup",
                         "wedgemidvert",
                         "mscro",
                         "circleonleftarrow",
                         "multimapinv",
                         "rftimes",
                         "underleftarrow",
                         "bigslopedvee",
                         "downfishtail",
                         "Vvert",
                         "updownharpoonrightleft",
                         "vartriangle",
                         "strns",
                         "nvrightarrow",
                         "precapprox",
                         "ntrianglerighteq",
                         "cwopencirclearrow",
                         "nwovnearrow",
                         "Succ",
                         "rightarrowbar",
                         "boxbslash",
                         "wedge",
                         "nvinfty",
                         "upand",
                         "wedgedoublebar",
                         "Supset",
                         "looparrowright",
                         "blockrighthalf",
                         "whthorzoval",
                         "smwhitestar",
                         "ll",
                         "baruparrow",
                         "mu",
                         "circledast",
                         "uplus",
                         "backcong",
                         "uparrowoncircle",
                         "triangleodot",
                         "coprod",
                         "BbbZ",
                         "Lt",
                         "acwleftarcarrow",
                         "bigstar",
                         "mdsmwhtsquare",
                         "horizbar",
                         "rightrightarrows",
                         "kernelcontraction",
                         "mscrg",
                         "barovernorthwestarrow",
                         "imageof",
                         "updownharpoonsleftright",
                         "ultriangle",
                         "varcarriagereturn",
                         "circledparallel",
                         "urarc",
                         "nVtwoheadrightarrowtail",
                         "circledstar",
                         "swarrow",
                         "leftharpoon",
                         "rbrackuend",
                         "drbkarrow",
                         "lessgtr",
                         "mdsmwhtcircle",
                         "mitBbbD",
                         "circledvert",
                         "barV",
                         "house",
                         "rightharpoondown",
                         "tieinfty",
                         "BbbPi",
                         "wideutilde",
                         "leftarrowonoplus",
                         "blacktriangleright",
                         "rightharpoon",
                         "neuter",
                         "preceq",
                         "invwhiteupperhalfcircle",
                         "oiiint",
                         "stareq",
                         "rbrace",
                         "trianglerighteq",
                         "mdwhtsquare",
                         "leftrightarrow",
                         "cwcirclearrow",
                         "dotsim",
                         "rparenlend",
                         "gtquest",
                         "leftharpoonsupdown",
                         "Zeta",
                         "tona",
                         "whitepointerleft",
                         "nvtwoheadrightarrowtail",
                         "subrarr",
                         "BbbC",
                         "Rho",
                         "squareurblack",
                         "leftharpoonupdash",
                         "Bumpeq",
                         "bigsqcap",
                         "shortdowntack",
                         "acwcirclearrow",
                         "arceq",
                         "rbag",
                         "diamondbotblack",
                         "curvearrowleftplus",
                         "capbarcup",
                         "varniobar",
                         "cirbot",
                         "twocaps",
                         "gtcc",
                         "measangledrtose",
                         "setminus",
                         "rbracelend",
                         "forkv",
                         "nsime",
                         "isindot",
                         "veedoublebar",
                         "nrightarrow",
                         "cupbarcap",
                         "Uuparrow",
                         "supsetplus",
                         "boxbar",
                         "lbracelend",
                         "equivDD",
                         "vartheta",
                         "adots",
                         "wedgeodot",
                         "lgwhtcircle",
                         "trslash",
                         "between",
                         "varepsilon",
                         "rparen",
                         "squarevfill",
                         "rightleftharpoonsdown",
                         "nvLeftarrow",
                         "Ddownarrow",
                         "beta",
                         "blockuphalf",
                         "Nu",
                         "measangleultonw",
                         "theta",
                         "lnsim",
                         "plussim",
                         "fdiagovnearrow",
                         "barwedge",
                         "to",
                         "backslash",
                         "nVrightarrowtail",
                         "suphsol",
                         "questeq",
                         "leftrightharpoonupdown",
                         "gleichstark",
                         "bar",
                         "Vbar",
                         "intbar",
                         "typecolon",
                         "nsim",
                         "iint",
                         "pointint",
                         "supsub",
                         "cong",
                         "ddot",
                         "ncong",
                         "interleave",
                         "rightharpoonupdash",
                         "underrightarrow",
                         "rrangle",
                         "obot",
                         "napprox",
                         "mscrB",
                         "Bbbsum",
                         "DashVDash",
                         "hermitmatrix",
                         "vBar",
                         "leftarrowless",
                         "rightdbltail",
                         "pluseqq",
                         "rsub",
                         "supedot",
                         "rmoustache",
                         "lbrack",
                         "threedotcolon",
                         "times",
                         "squareulquad",
                         "chi",
                         "dottimes",
                         "ni",
                         "zpipe",
                         "rightarrowsimilar",
                         "sum",
                         "varbeta",
                         "obar",
                         "minusrdots",
                         "rceil",
                         "intcap",
                         "mdblksquare",
                         "rightarrowgtr",
                         "gimel",
                         "bullseye",
                         "leftrightharpoonsdown",
                         "circledbullet",
                         "triangleminus",
                         "blackrighthalfcircle",
                         "sqsubset",
                         "underbracket",
                         "upharpoonleftbar",
                         "overleftarrow",
                         "concavediamondtickright",
                         "lsime",
                         "circlevertfill",
                         "increment",
                         "Sqcap",
                         "langledot",
                         "squarellquad",
                         "whitesquaretickleft",
                         "breve",
                         "backtrprime",
                         "rangledownzigzagarrow",
                         "lllnest",
                         "leftrightharpoondowndown",
                         "diceiii",
                         "sansLturned",
                         "trianglelefteq",
                         "Kappa",
                         "fint",
                         "underrightharpoondown",
                         "bigvee",
                         "mdwhtcircle",
                         "eqless",
                         "bigcup",
                         "Yup",
                         "profsurf",
                         "veeodot",
                         "nvtwoheadleftarrow",
                         "succ",
                         "curvearrowright",
                         "eighthnote",
                         "prurel",
                         "downwhitearrow",
                         "nVleftarrowtail",
                         "ogreaterthan",
                         "leftarrow",
                         "DDownarrow",
                         "cupovercap",
                         "diceii",
                         "rightdotarrow",
                         "nHuparrow",
                         "triangleplus",
                         "squarehfill",
                         "leftrightharpoonsup",
                         "rbrackurtick",
                         "blockthreeqtrshaded",
                         "xsol",
                         "lowint",
                         "Re",
                         "nabla",
                         "grave",
                         "llblacktriangle",
                         "iiiint",
                         "bsimilarleftarrow",
                         "Lbrbrak",
                         "varhexagonblack",
                         "leftcurvedarrow",
                         "mitBbbj",
                         "updownarrows",
                         "gamma",
                         "varisins",
                         "subsetcirc",
                         "ntrianglelefteq",
                         "underparen",
                         "smwhtdiamond",
                         "capovercup",
                         "emptysetocirc",
                         "hexagon",
                         "lfloor",
                         "beth",
                         "cup",
                         "twoheaduparrowcircle",
                         "gescc",
                         "cwundercurvearrow",
                         "conjquant",
                         "intx",
                         "longrightsquigarrow",
                         "Rightarrow",
                         "wideangleup",
                         "npreccurlyeq",
                         "asteraccent",
                         "twoheadmapsto",
                         "rdiagovfdiag",
                         "vec",
                         "midcir",
                         "downharpoonright",
                         "leftarrowsimilar",
                         "urcorner",
                         "twoheadleftarrowtail",
                         "subsetneq",
                         "rightimply",
                         "mdlgwhtdiamond",
                         "geqq",
                         "lesdot",
                         "ngeq",
                         "rparengtr",
                         "leftrightarrowtriangle",
                         "iota",
                         "Gamma",
                         "blackdiamonddownarrow",
                         "rightwhitearrow",
                         "neovsearrow",
                         "unicodeellipsis",
                         "Vdash",
                         "leftdasharrow",
                         "downharpoonrightbar",
                         "blacksmiley",
                         "simgtr",
                         "lrtriangleeq",
                         "succeqq",
                         "diamondsuit",
                         "rightharpoonupbar",
                         "circledequal",
                         "doteq",
                         "lmoustache",
                         "Eta",
                         "nis",
                         "twoheaddownarrow",
                         "squarelrblack",
                         "tplus",
                         "neg",
                         "rightbkarrow",
                         "cwrightarcarrow",
                         "minus",
                         "bigotimes",
                         "llcorner",
                         "Nwarrow",
                         "varheartsuit",
                         "rbracemid",
                         "varVdash",
                         "mdlgwhtlozenge",
                         "ularc",
                         "parallelogram",
                         "neovnwarrow",
                         "Dashv",
                         "leftrightharpoons",
                         "Rvzigzag",
                         "varrho",
                         "Sigma",
                         "eta",
                         "Beta",
                         "revemptyset",
                         "Cap",
                         "lBrace",
                         "cap",
                         "capwedge",
                         "hzigzag",
                         "rsolbar",
                         "lrcorner",
                         "rightouterjoin",
                         "dashVdash",
                         "gesdotol",
                         "ltcir",
                         "simrdots",
                         "smwhtsquare",
                         "leftrightharpoonupup",
                         "int",
                         "sqsupset",
                         "lbrbrak",
                         "rbracklrtick",
                         "nleq",
                         "Eulerconst",
                         "uprightcurvearrow",
                         "Chi",
                         "rfbowtie",
                         "nsucc",
                         "errbarcircle",
                         "otimeslhrim",
                         "dashv",
                         "succcurlyeq",
                         "candra",
                         "dashcolon",
                         "rvboxline",
                         "btimes",
                         "backsim",
                         "squaretopblack",
                         "seovnearrow",
                         "supmult",
                         "leftharpoonup",
                         "Phi",
                         "BbbN",
                         "squarebotblack",
                         "bigwedge",
                         "hrectangleblack",
                         "towa",
                         "mdsmblkcircle",
                         "partialmeetcontraction",
                         "leftrightharpoondownup",
                         "supseteq",
                         "talloblong",
                         "hat",
                         "intbottom",
                         "nearrow",
                         "lgE",
                         "eqdef",
                         "dashV",
                         "twoheadrightarrowtail",
                         "circlehbar",
                         "langle",
                         "boxbox",
                         "nVtwoheadrightarrow",
                         "nsqsupseteq",
                         "Longrightarrow",
                         "downupharpoonsleftright",
                         "eqqless",
                         "barleftarrow",
                         "rightarrowonoplus",
                         "mdlgblksquare",
                         "rightarrowsupset",
                         "varspadesuit",
                         "varhexagon",
                         "inversebullet",
                         "hatapprox",
                         "rbracklend",
                         "caretinsert",
                         "mathunderbar",
                         "twoheaduparrow",
                         "rightarrowbackapprox",
                         "pm",
                         "approxeqq",
                         "cdots",
                         "elinters",
                         "plustrif",
                         "dicev",
                         "lvzigzag",
                         "nVdash",
                         "trianglerightblack",
                         "squareneswfill",
                         "Lparengtr",
                         "precneq",
                         "dottedsquare",
                         "smallblacktriangleright",
                         "amalg",
                         "varclubsuit",
                         "varstar",
                         "Prec",
                         "pi",
                         "lsimg",
                         "vbraceextender",
                         "spadesuit",
                         "divslash",
                         "prec",
                         "lozengeminus",
                         "updownarrow",
                         "barcup",
                         "bagmember",
                         "downarrowbarred",
                         "smblklozenge",
                         "egsdot",
                         "ovhook",
                         "carriagereturn",
                         "succnsim",
                         "veemidvert",
                         "mscrH",
                         "exists",
                         "measuredrightangle",
                         "top",
                         "mscrM",
                         "downharpoonleft",
                         "leftarrowbsimilar",
                         "acidfree",
                         "intBar",
                         "Angstrom",
                         "obrbrak",
                         "elsdot",
                         "overrightharpoon",
                         "lat",
                         "llarc",
                         "cwgapcirclearrow",
                         "errbarblacksquare",
                         "vlongdash",
                         "Delta",
                         "lneq",
                         "linefeed",
                         "glE",
                         "varlrtriangle",
                         "geqslant",
                         "blackinwhitesquare",
                         "diamondleftblack",
                         "gesles",
                         "widehat",
                         "leftsquigarrow",
                         "circleddash",
                         "nisd",
                         "daleth",
                         "rfloor",
                         "intprod",
                         "varTheta",
                         "rightanglesqr",
                         "circledwhitebullet",
                         "blockfull",
                         "Longmapsfrom",
                         "bardownharpoonright",
                         "bsolhsub",
                         "doublebarvee",
                         "nhpar",
                         "smalltriangleleft",
                         "wp",
                         "blackpointerright",
                         "fcmp",
                         "eqslantgtr",
                         "rightdowncurvedarrow",
                         "Uparrow",
                         "Subset",
                         "sphericalangle",
                         "triangleserifs",
                         "leqqslant",
                         "leftarrowshortrightarrow",
                         "vzigzag",
                         "rbraceuend",
                         "nprec",
                         "cdot",
                         "hknearrow",
                         "rightarrowx",
                         "measuredangleleft",
                         "Psi",
                         "zcmp",
                         "simplus",
                         "rightarrowshortleftarrow",
                         "cupleftarrow",
                         "upharpoonrightbar",
                         "lbrackextender",
                         "sqcap",
                         "plusdot",
                         "barupharpoonleft",
                         "lambda",
                         "APLboxquestion",
                         "dagger",
                         "vardiamondsuit",
                         "Longmapsto",
                         "eqgtr",
                         "acwoverarcarrow",
                         "circleurquadblack",
                         "lbag",
                         "rightleftharpoons",
                         "measanglerdtose",
                         "Alpha",
                         "pushout",
                         "blockhalfshaded",
                         "Game",
                         "nVrightarrow",
                         "longdashv",
                         "bigtimes",
                         "maltese",
                         "threedangle",
                         "lbrackubar",
                         "ltcc",
                         "neswarrow",
                         "nHdownarrow",
                         "diameter",
                         "leftarrowx",
                         "lescc",
                         "gnsim",
                         "lgwhtsquare",
                         "topfork",
                         "errbardiamond",
                         "ast",
                         "disjquant",
                         "perp",
                         "mitBbbi",
                         "closedvarcap",
                         "bot",
                         "vdots",
                         "gtrapprox",
                         "dotequiv",
                         "natural",
                         "vysmblkcircle",
                         "leftdotarrow",
                         "circledtwodots",
                         "hslash",
                         "dotminus",
                         "lesssim",
                         "downdasharrow",
                         "curvearrowleft",
                         "rightangle",
                         "boxminus",
                         "gtrarr",
                         "multimap",
                         "eparsl",
                         "leftfishtail",
                         "upharpoonsleftright",
                         "rtimes",
                         "vDdash",
                         "bigoplus",
                         "Hermaphrodite",
                         "Rbrbrak",
                         "mdlgwhtsquare",
                         "cupvee",
                         "measanglelutonw",
                         "invlazys",
                         "mscre",
                         "rcurvyangle",
                         "gtrsim",
                         "harrowextender",
                         "leftrightsquigarrow",
                         "ringplus",
                         "quarternote",
                         "eqqplus",
                         "phi",
                         "rightpentagonblack",
                         "curlywedge",
                         "triangleright",
                         "concavediamondtickleft",
                         "eqeq",
                         "Upsilon",
                         "varnis",
                         "sqsubsetneq",
                         "rightarrowdiamond",
                         "measeq",
                         "rho",
                         "geqqslant",
                         "scurel",
                         "revangleubar",
                         "upin",
                         "leftrightarrowcircle",
                         "div",
                         "coloneq",
                         "subedot",
                         "otimes",
                         "nLeftrightarrow",
                         "veebar",
                         "whiteinwhitetriangle",
                         "hknwarrow",
                         "Rsh",
                         "UUparrow",
                         "mp",
                         "sumint",
                         "blkhorzoval",
                         "lgblkcircle",
                         "intclockwise",
                         "fourvdots",
                         "nwsearrow",
                         "subset",
                         "lblkbrbrak",
                         "hookleftarrow",
                         "vysmblksquare",
                         "triangletimes",
                         "equivVert",
                         "gtcir",
                         "capdot",
                         "shuffle",
                         "nleftarrow",
                         "vertoverlay",
                         "smwhtlozenge",
                         "nvDash",
                         "urtriangle",
                         "twoheadleftdbkarrow",
                         "matheth",
                         "gtreqqless",
                         "leftdbltail",
                         "RRightarrow",
                         "longdivision",
                         "rdiagovsearrow",
                         "sqrt",
                         "pentagonblack",
                         "bigwhitestar",
                         "csube",
                         "Vvdash",
                         "leftarrowtriangle",
                         "APLnotslash",
                         "nsupseteq",
                         "vee",
                         "boxast",
                         "olessthan",
                         "mdwhtdiamond",
                         "Bbbgamma",
                         "succeq",
                         "rppolint",
                         "turnediota",
                         "nvRightarrow",
                         "circlellquad",
                         "bigcap",
                         "sqcup",
                         "upharpoonright",
                         "checkmark",
                         "supseteqq",
                         "ocommatopright",
                         "eqcirc",
                         "sslash",
                         "overparen",
                         "concavediamond",
                         "downarrow",
                         "widebridgeabove",
                         "nsubset",
                         "invwhitelowerhalfcircle",
                         "bigtalloblong",
                         "triangleleft",
                         "acwunderarcarrow",
                         "nvtwoheadleftarrowtail",
                         "veeeq",
                         "varhexagonlrbonds",
                         "inttop",
                         "leftwhitearrow",
                         "bigslopedwedge",
                         "rgroup",
                         "nless",
                         "odot",
                         "succnapprox",
                         "iinfin",
                         "looparrowleft",
                         "succneqq",
                         "whitepointerright",
                         "gnapprox",
                         "varbarwedge",
                         "dicei",
                         "late",
                         "fbowtie",
                         "similarrightarrow",
                         "rAngle",
                         "viewdata",
                         "curlyvee",
                         "Zbar",
                         "minusdot",
                         "oiint",
                         "intlarhk",
                         "circlerighthalfblack",
                         "mdblkcircle",
                         "smallsetminus",
                         "biguplus",
                         "diamondleftarrow",
                         "squarecrossfill",
                         "squarehvfill",
                         "mitBbbe",
                         "perps",
                         "updownharpoonleftright",
                         "simgE",
                         "lceil",
                         "boxplus",
                         "lessdot",
                         "supsetcirc",
                         "niobar",
                         "emptysetoarr",
                         "therefore",
                         "unicodecdots",
                         "barrightharpoonup",
                         "blacktriangleleft",
                         "triangleq",
                         "varisinobar",
                         "lbracklltick",
                         "sumbottom",
                         "Epsilon",
                         "vrectangleblack",
                         "ltrivb",
                         "Iota",
                         "dsol",
                         "assert",
                         "twoheadleftarrow",
                         "subsub",
                         "circletophalfblack",
                         "leftdbkarrow",
                         "simminussim",
                         "psi",
                         "flat",
                         "mfrakC",
                         "ocirc",
                         "downharpoonleftbar",
                         "sphericalangleup",
                         "fourthroot",
                         "in",
                         "nsqsubseteq",
                         "lbrace",
                         "sqsupsetneq",
                         "acute",
                         "lparenuend",
                         "angles",
                         "bigodot",
                         "bigblacktriangleup",
                         "barleftharpoondown",
                         "lbraceuend",
                         "notin",
                         "gesdot",
                         "rightarrowapprox",
                         "lrarc",
                         "Updownarrow",
                         "not",
                         "bigcupdot",
                         "sigma",
                         "lessapprox",
                         "blocklowhalf",
                         "supsup",
                         "leftarrowsubset",
                         "mathratio",
                         "aleph",
                         "Coloneq",
                         "lbrackultick",
                         "Ldsh",
                         "zeta",
                         "eqeqeq",
                         "supset",
                         "ddddot",
                         "leftharpoondownbar",
                         "supsetdot",
                         "ubrbrak",
                         "Planckconst",
                         "equalrightarrow",
                         "mlcp",
                         "plussubtwo",
                         "cdotp",
                         "pitchfork",
                         "dualmap",
                         "downharpoonsleftright",
                         "dotplus",
                         "intercal",
                         "opluslhrim",
                         "errbarblackdiamond",
                         "lAngle",
                         "female",
                         "frown",
                         "trprime",
                         "hrectangle",
                         "thermod",
                         "ne",
                         "rightarrowbsimilar",
                         "delta",
                         "Nearrow",
                         "asteq",
                         "QED",
                         "ell",
                         "smalltriangleright",
                         "lsqhook",
                         "cirfnint",
                         "sqrtbottom",
                         "dddot",
                         "leftharpoondown",
                         "Longleftrightarrow",
                         "diamondleftarrowbar",
                         "downtrianglerightblack",
                         "isinobar",
                         "ddagger",
                         "bowtie",
                         "wedgebar",
                         "cirE",
                         "biginterleave",
                         "kappa",
                         "nvleftarrow",
                         "rightmoon",
                         "vectimes",
                         "angleubar",
                         "ddots",
                         "Not",
                         "prime",
                         "ulblacktriangle",
                         "iiint",
                         "barvee",
                         "approx",
                         "alpha",
                         "supsetneq",
                         "droang",
                         "subsetdot",
                         "Im",
                         "supsetneqq",
                         "BbbR",
                         "sqlozenge",
                         "varpi",
                         "smblkcircle",
                         "downzigzagarrow",
                         "blacklefthalfcircle",
                         "gtrless",
                         "nvartriangleright",
                         "models",
                         "otimeshat",
                         "ngtrsim",
                         "gneq",
                         "rightharpoondownbar",
                         "Rdsh",
                         "Leftrightarrow",
                         "varphi",
                         "longmapsfrom",
                         "lBrack",
                         "rightsquigarrow",
                         "oplusrhrim",
                         "otimesrhrim",
                         "gsiml",
                         "csupe",
                         "lltriangle",
                         "llparenthesis",
                         "parsim",
                         "ruledelayed",
                         "precnsim",
                         "intcup",
                         "widetilde",
                         "preccurlyeq",
                         "succneq",
                         "DashV",
                         "Rparenless",
                         "mscrR",
                         "errbarblackcircle",
                         "mapsto",
                         "updownharpoonrightright",
                         "tminus",
                         "astrosun",
                         "rbrackubar",
                         "diceiv",
                         "curlyeqsucc",
                         "subsetneqq",
                         "underleftrightarrow",
                         "gtrdot",
                         "lesdoto",
                         "BbbGamma",
                         "nequiv",
                         "hourglass",
                         "rParen",
                         "preceqq",
                         "suphsub",
                         "precnapprox",
                         "smashtimes",
                         "squareulblack",
                         "circlelefthalfblack",
                         "topcir",
                         "sun",
                         "bbrktbrk",
                         "acwgapcirclearrow",
                         "blackcircleulquadwhite",
                         "wr",
                         "rangledot",
                         "topsemicircle",
                         "upfishtail",
                         "smte",
                         "blockqtrshaded",
                         "pullback",
                         "squareurquad",
                         "eqqsim",
                         "nhVvert",
                         "bsimilarrightarrow",
                         "minusfdots",
                         "shortrightarrowleftarrow",
                         "rparenextender",
                         "mdlgblkcircle",
                         "oslash",
                         "nvrightarrowtail",
                         "updownarrowbar",
                         "squoval",
                         "draftingarrow",
                         "vdash",
                         "fltns",
                         "lParen",
                         "downdownarrows",
                         "backsimeq",
                         "hksearrow",
                         "dprime",
                         "Mapsfrom",
                         "overrightarrow",
                         "eqqgtr",
                         "lgblksquare",
                         "closedvarcup",
                         "bigtriangleup",
                         "updasharrow",
                         "backdprime",
                         "triangleleftblack",
                         "longleftarrow",
                         "rightthreetimes",
                         "circledcirc",
                         "mitBbbd",
                         "dsub",
                         "mdblkdiamond",
                         "uparrow",
                         "fdiagovrdiag",
                         "barrightarrowdiamond",
                         "leftthreetimes",
                         "Bbbpi",
                         "subseteq",
                         "gg",
                         "leftthreearrows",
                         "cuberoot",
                         "boxcircle",
                         "rightarrowtail",
                         "dbkarrow",
                         "propto",
                         "Omega",
                         "subseteqq",
                         "mapsup",
                         "nlessgtr",
                         "circeq",
                         "PropertyLine",
                         "rvzigzag",
                         "circleurquad",
                         "eqcolon",
                         "revnmid",
                         "danger",
                         "leftarrowapprox",
                         "trianglecdot",
                         "star",
                         "equalleftarrow",
                         "circleonrightarrow",
                         "varointclockwise",
                         "forall",
                         "underleftharpoondown",
                         "sqsupseteq",
                         "diamondrightblack",
                         "benzenr",
                         "upsilon",
                         "twoheadmapsfrom",
                         "leftmoon",
                         "angle",
                         "invnot",
                         "epsilon",
                         "mdlgwhtcircle",
                         "emptysetobar",
                         "upint",
                         "squarellblack",
                         "turnangle",
                         "succapprox",
                         "upuparrows",
                         "odiv",
                         "varsigma",
                         "Join",
                         "Equiv",
                         "smwhtcircle",
                         "barupharpoonright",
                         "medblackstar",
                         "LLeftarrow",
                         "blkvertoval",
                         "supsetapprox",
                         "lbracemid",
                         "circlelrquad",
                         "circledownarrow",
                         "dingasterisk",
                         "rightarrowtriangle",
                         "gggnest",
                         "rightanglemdot",
                         "rBrace",
                         "cupdot",
                         "partial",
                         "suplarr",
                         "ulcorner",
                         "whtvertoval",
                         "intprodr",
                         "rightpentagon",
                         "shortlefttack",
                         "conictaper",
                         "lbrackuend",
                         "blackcircledownarrow",
                         "leftarrowplus",
                         "emptysetoarrl",
                         "csub",
                         "nVtwoheadleftarrow",
                         "rightharpoonup",
                         "simlE",
                         "varkappa",
                         "ggg",
                         "nvtwoheadrightarrow",
                         "wideangledown",
                         "nwarrow",
                         "searrow",
                         "leftdowncurvedarrow",
                         "Searrow",
                         "overbracket",
                         "mapsdown",
                         "nexists",
                         "doublebarwedge",
                         "prod",
                         "dottedcircle",
                         "bigbot",
                         "fullouterjoin",
                         "rightthreearrows",
                         "male",
                         "downtriangleleftblack",
                         "mapsfrom",
                         "circleulquad",
                         "origof",
                         "clubsuit",
                         "dot",
                         "eqqslantgtr",
                         "Longleftarrow",
                         "blacktriangledown",
                         "angdnr",
                         "mdlgblkdiamond",
                         "equivVvert",
                         "lgroup",
                         "Lvzigzag",
                         "lesdotor",
                         "scpolint",
                         "mscrF",
                         "leftharpoonupbar",
                         "leftarrowbackapprox",
                         "vysmwhtsquare",
                         "inversewhitecircle",
                         "intextender",
                         "BbbH",
                         "measangleldtosw",
                         "vartriangleright",
                         "longmapsto",
                         "bigtriangleleft",
                         "lll",
                         "Finv",
                         "smeparsl",
                         "risingdotseq",
                         "medwhitestar",
                         "topbot",
                         "lparenless",
                         "nsubseteq",
                         "squarelrquad",
                         "downarrowbar",
                         "BbbP",
                         "rbrackextender",
                         "boxdot",
                         "commaminus",
                         "upharpoonleft",
                         "nleftrightarrow",
                         "nLeftarrow",
                         "xbsol",
                         "dashleftharpoondown",
                         "Cup",
                         "divideontimes",
                         "Tau",
                         "eqqslantless",
                         "Omicron",
                         "enleadertwodots",
                         "gesdoto",
                         "updownharpoonleftleft",
                         "leq",
                         "nu",
                         "nvleftrightarrow",
                         "submult",
                         "lparenlend",
                         "gtlpar",
                         "varveebar",
                         "botsemicircle",
                         "because",
                         "omicron",
                         "Mu",
                         "eqvparsl",
                         "smile",
                         "Colon",
                         "rtriltri",
                         "disin",
                         "simless",
                         "Vee",
                         "rightwavearrow",
                         "ltquest",
                         "tau",
                         "subsetapprox",
                         "olcross",
                         "sqint",
                         "blackcircledtwodots",
                         "mscrL",
                         "oturnedcomma",
                         "Lambda",
                         "rightleftarrows",
                         "heartsuit",
                         "sim",
                         "rBrack",
                         "nvdash",
                         "midbarvee",
                         "bumpeqq",
                         "rbrack",
                         "blocklefthalf",
                         "lvboxline",
                         "lneqq",
                         "rblkbrbrak",
                         "npolint",
                         "vert",
                         "rightarrow",
                         "Vert",
                         "ominus",
                         "circlebottomhalfblack",
                         "mscrE",
                         "approxeq",
                         "vartriangleleft",
                         "Rrightarrow",
                         "rightcurvedarrow",
                         "gtreqless",
                         "fallingdotseq",
                         "mfrakH",
                         "cirmid",
                         "backprime",
                         "blackcircledrightdot",
                         "nVDash",
                         "isins",
                         "blackhourglass",
                         "postalmark",
                         "barleftarrowrightarrowbar",
                         "circledrightdot",
                         "turnednot",
                         "oplus",
                         "modtwosum",
                         "ngtrless",
                         "boxdiag",
                         "nRightarrow",
                         "oint",
                         "omega",
                         "mdsmblksquare",
                         "Leftarrow",
                         "sharp",
                         "lrtriangle",
                         "succsim",
                         "profline",
                         "nmid",
                         "rightfishtail",
                         "eqslantless",
                         "supsim",
                         "supdsub",
                         "uparrowbarred",
                         "hookrightarrow",
                         "measanglerutone",
                         "leftrightarrows",
                         "rparenuend",
                         "dicevi",
                         "longleftrightarrow",
                         "uminus",
                         "vysmwhtcircle",
                         "Wedge",
                         "squarerightblack",
                         "squarenwsefill",
                         "vBarv",
                         "ltimes",
                         "rightdasharrow",
                         "longleftsquigarrow",
                         "acwopencirclearrow",
                         "tosa",
                         "Lsh",
                         "nparallel",
                         "vbrtri",
                         "boxonbox",
                         "Pi",
                         "errbarsquare",
                         "overbrace",
                         "leftleftarrows",
                         "APLnotbackslash",
                         "precneqq",
                         "nVleftarrow",
                         "rangle",
                         "blackpointerleft",
                         "laplac",
                         "plushat",
                         "sinewave",
                         "sqsubseteq",
                         "curlyeqprec",
                         "rsqhook",
                         "measangledltosw",
                         "lrblacktriangle",
                         "rightharpoonsupdown",
                         "smblkdiamond",
                         "zproject",
                         "complement",
                         "mscrI",
                         "closedvarcupsmashprod",
                         "VDash",
                         "wedgeonwedge",
                         "awint",
                         "sansLmirrored",
                         "overleftharpoon",
                         "nsupset",
                         "whitearrowupfrombar",
                         "lfbowtie",
                         "Lleftarrow",
                         "hkswarrow",
                         "nasymp",
                         "gla",
                         "triangleubar",
                         "asymp",
                         "sumtop",
                         "ngtr",
                         "vardoublebarwedge",
                         "mdwhtlozenge",
                         "lparenextender",
                         "gsime",
                         "geq",
                         "timesbar",
                         "upwhitearrow",
                         "BbbQ",
                         "wedgeq",
                         "diamondcdot",
                         "odotslashdot",
                         "Otimes",
                         "csup",
                         "veeonvee",
                         "rightleftharpoonsup",
                         "varnothing",
                         "mdlgblklozenge"];

#[bench]
fn bench_phf(b: &mut Bencher) {
    let keys = black_box(KEYS);

    b.iter(|| for key in keys {
               SYMBOLS_PHF
                   .get(key)
                   .expect("PHF failed to find a valid key.");
           });
}

#[bench]
fn bench_static_map(b: &mut Bencher) {
    let keys = black_box(KEYS);

    b.iter(|| for key in keys {
               SYMBOLS_STATIC_MAP
                   .get(key)
                   .expect("static_map failed to find a valid key.");
           });
}