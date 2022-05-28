use phf::phf_map;

#[derive(PartialEq, Eq, Hash)]
pub enum Specification {
    DayLight,
    General,
    OpenSMILES,
}

#[derive(Clone, Debug)]
pub struct Element {
    atomic_number: u8,
    symbol: &'static str,
    valence: [u8; 3],
}
impl Element {
    pub const fn new(symbol: &'static str, atomic_number: u8, valence: [u8; 3]) -> Self {
        Self {
            atomic_number,
            symbol,
            valence,
        }
    }

    pub fn read(c: &str) -> Option<Self> {
        ELEMENT_MAP.get(c).map_or(None, |e| Some(e.clone()))
    }

    pub fn atomic_number(&self) -> u8 {
        self.atomic_number
    }

    pub fn symbol(&self) -> &str {
        self.symbol
    }

    pub fn is_tritium(&self) -> bool {
        return self.symbol == "T" && self.atomic_number == 3;
    }

    pub fn is_deuterium(&self) -> bool {
        return self.symbol == "D" && self.atomic_number == 2;
    }

    pub(crate) fn implict_hydrogen_amount(&self, valence: u8) -> u8 {
        self.valence
            .iter()
            .find(|&&i| i >= valence)
            .map_or(0, |v| v - valence)
    }

    pub(crate) fn implict_atom_hydrogen(&self, valence: u8) -> u8 {
        if self.valence[0] - valence > 0 {
            return self.valence[0] - valence;
        } else {
            return 0;
        }
    }

    pub fn is_aromatic(&self, spec: Specification) -> bool {
        match spec {
            Specification::DayLight => DAYLIGHT_AROMATIC_ELEMENT.contains(&self.atomic_number),
            Specification::General => GENERAL_AROMATIC_ELEMENT.contains(&self.atomic_number),
            Specification::OpenSMILES => OPENSMILE_AROMATIC_ELEMENT.contains(&self.atomic_number),
        }
    }
}

#[macro_export]
macro_rules! to_element {
    ($variable:ident, $sym:expr, $num:expr, $val:expr) => {
        pub const $variable: Element = Element::new($sym, $num, $val);
    };
}

to_element!(ANY, "*", 0, [0, 0, 0]);
to_element!(H, "H", 1, [0, 0, 0]);
to_element!(D, "D", 2, [0, 0, 0]);
to_element!(T, "T", 3, [0, 0, 0]);
to_element!(HE, "He", 2, [0, 0, 0]);
to_element!(LI, "Li", 3, [0, 0, 0]);
to_element!(BE, "Be", 4, [0, 0, 0]);
to_element!(B, "B", 5, [3, 0, 0]);
to_element!(C, "C", 6, [4, 0, 0]);
to_element!(N, "N", 7, [3, 5, 0]);
to_element!(O, "O", 8, [2, 0, 0]);
to_element!(F, "F", 9, [1, 0, 0]);
to_element!(NE, "Ne", 10, [0, 0, 0]);
to_element!(NA, "Na", 11, [0, 0, 0]);
to_element!(MG, "Mg", 12, [0, 0, 0]);
to_element!(AL, "Al", 13, [0, 0, 0]);
to_element!(SI, "Si", 14, [0, 0, 0]);
to_element!(P, "P", 15, [3, 5, 0]);
to_element!(S, "S", 16, [2, 4, 6]);
to_element!(CL, "Cl", 17, [1, 0, 0]);
to_element!(AR, "Ar", 18, [0, 0, 0]);
to_element!(K, "K", 19, [0, 0, 0]);
to_element!(CA, "Ca", 20, [0, 0, 0]);
to_element!(SC, "Sc", 21, [0, 0, 0]);
to_element!(TI, "Ti", 22, [0, 0, 0]);
to_element!(V, "V", 23, [0, 0, 0]);
to_element!(CR, "Cr", 24, [0, 0, 0]);
to_element!(MN, "Mn", 25, [0, 0, 0]);
to_element!(FE, "Fe", 26, [0, 0, 0]);
to_element!(CO, "Co", 27, [0, 0, 0]);
to_element!(NI, "Ni", 28, [0, 0, 0]);
to_element!(CU, "Cu", 29, [0, 0, 0]);
to_element!(ZN, "Zn", 30, [0, 0, 0]);
to_element!(GA, "Ga", 31, [0, 0, 0]);
to_element!(GE, "Ge", 32, [0, 0, 0]);
to_element!(AS, "As", 33, [0, 0, 0]);
to_element!(SE, "Se", 34, [0, 0, 0]);
to_element!(BR, "Br", 35, [1, 0, 0]);
to_element!(KR, "Kr", 36, [0, 0, 0]);
to_element!(RB, "Rb", 37, [0, 0, 0]);
to_element!(SR, "Sr", 38, [0, 0, 0]);
to_element!(Y, "Y", 39, [0, 0, 0]);
to_element!(ZR, "Zr", 40, [0, 0, 0]);
to_element!(NB, "Nb", 41, [0, 0, 0]);
to_element!(MO, "Mo", 42, [0, 0, 0]);
to_element!(TC, "Tc", 43, [0, 0, 0]);
to_element!(RU, "Ru", 44, [0, 0, 0]);
to_element!(RH, "Rh", 45, [0, 0, 0]);
to_element!(PD, "Pd", 46, [0, 0, 0]);
to_element!(AG, "Ag", 47, [0, 0, 0]);
to_element!(CD, "Cd", 48, [0, 0, 0]);
to_element!(IN, "In", 49, [0, 0, 0]);
to_element!(SN, "Sn", 50, [0, 0, 0]);
to_element!(SB, "Sb", 51, [0, 0, 0]);
to_element!(TE, "Te", 52, [0, 0, 0]);
to_element!(I, "I", 53, [1, 0, 0]);
to_element!(XE, "Xe", 54, [0, 0, 0]);
to_element!(CS, "Cs", 55, [0, 0, 0]);
to_element!(BA, "Ba", 56, [0, 0, 0]);
to_element!(LA, "La", 57, [0, 0, 0]);
to_element!(CE, "Ce", 58, [0, 0, 0]);
to_element!(PR, "Pr", 59, [0, 0, 0]);
to_element!(ND, "Nd", 60, [0, 0, 0]);
to_element!(PM, "Pm", 61, [0, 0, 0]);
to_element!(SM, "Sm", 62, [0, 0, 0]);
to_element!(EU, "Eu", 63, [0, 0, 0]);
to_element!(GD, "Gd", 64, [0, 0, 0]);
to_element!(TB, "Tb", 65, [0, 0, 0]);
to_element!(DY, "Dy", 66, [0, 0, 0]);
to_element!(HO, "Ho", 67, [0, 0, 0]);
to_element!(ER, "Er", 68, [0, 0, 0]);
to_element!(TM, "Tm", 69, [0, 0, 0]);
to_element!(YB, "Yb", 70, [0, 0, 0]);
to_element!(LU, "Lu", 71, [0, 0, 0]);
to_element!(HF, "Hf", 72, [0, 0, 0]);
to_element!(TA, "Ta", 73, [0, 0, 0]);
to_element!(W, "W", 74, [0, 0, 0]);
to_element!(RE, "Re", 75, [0, 0, 0]);
to_element!(OS, "Os", 76, [0, 0, 0]);
to_element!(IR, "Ir", 77, [0, 0, 0]);
to_element!(PT, "Pt", 78, [0, 0, 0]);
to_element!(AU, "Au", 79, [0, 0, 0]);
to_element!(HG, "Hg", 80, [0, 0, 0]);
to_element!(TL, "Tl", 81, [0, 0, 0]);
to_element!(PB, "Pb", 82, [0, 0, 0]);
to_element!(BI, "Bi", 83, [0, 0, 0]);
to_element!(PO, "Po", 84, [0, 0, 0]);
to_element!(AT, "At", 85, [0, 0, 0]);
to_element!(RN, "Rn", 86, [0, 0, 0]);
to_element!(FR, "Fr", 87, [0, 0, 0]);
to_element!(RA, "Ra", 88, [0, 0, 0]);
to_element!(AC, "Ac", 89, [0, 0, 0]);
to_element!(TH, "Th", 90, [0, 0, 0]);
to_element!(PA, "Pa", 91, [0, 0, 0]);
to_element!(U, "U", 92, [0, 0, 0]);
to_element!(NP, "Np", 93, [0, 0, 0]);
to_element!(PU, "Pu", 94, [0, 0, 0]);
to_element!(AM, "Am", 95, [0, 0, 0]);
to_element!(CM, "Cm", 96, [0, 0, 0]);
to_element!(BK, "Bk", 97, [0, 0, 0]);
to_element!(CF, "Cf", 98, [0, 0, 0]);
to_element!(ES, "Es", 99, [0, 0, 0]);
to_element!(FM, "Fm", 100, [0, 0, 0]);
to_element!(MD, "Md", 101, [0, 0, 0]);
to_element!(NO, "No", 102, [0, 0, 0]);
to_element!(LR, "Lr", 103, [0, 0, 0]);
to_element!(RF, "Rf", 104, [0, 0, 0]);
to_element!(DB, "Db", 105, [0, 0, 0]);
to_element!(SG, "Sg", 106, [0, 0, 0]);
to_element!(BH, "Bh", 107, [0, 0, 0]);
to_element!(HS, "Hs", 108, [0, 0, 0]);
to_element!(MT, "Mt", 109, [0, 0, 0]);
to_element!(DS, "Ds", 110, [0, 0, 0]);
to_element!(RG, "Rg", 111, [0, 0, 0]);
to_element!(CN, "Cn", 112, [0, 0, 0]);
to_element!(NH, "Nh", 113, [0, 0, 0]);
to_element!(FL, "Fl", 114, [0, 0, 0]);
to_element!(MC, "Mc", 115, [0, 0, 0]);
to_element!(LV, "Lv", 116, [0, 0, 0]);
to_element!(TS, "Ts", 117, [0, 0, 0]);
to_element!(OG, "Og", 118, [0, 0, 0]);

const DAYLIGHT_AROMATIC_ELEMENT: [u8; 8] = [0, 6, 7, 8, 16, 15, 33, 34];
const GENERAL_AROMATIC_ELEMENT: [u8; 9] = [0, 5, 6, 7, 8, 16, 15, 33, 34];
const OPENSMILE_AROMATIC_ELEMENT: [u8; 15] =
    [0, 5, 6, 7, 8, 14, 16, 15, 32, 33, 34, 50, 51, 52, 83];

static ELEMENT_MAP: phf::Map<&'static str, Element> = phf_map! {
    "*" => ANY,
    "H" => H,
    "D" => D,
    "T" => T,
    "He" => HE,
    "Li" => LI,
    "Be" => BE,
    "B" => B,
    "C" => C,
    "N" => N,
    "O" => O,
    "F" => F,
    "Ne" => NE,
    "Na" => NA,
    "Mg" => MG,
    "Al" => AL,
    "Si" => SI,
    "P" => P,
    "S" => S,
    "Cl" => CL,
    "Ar" => AR,
    "K" => K,
    "Ca" => CA,
    "Sc" => SC,
    "Ti" => TI,
    "V" => V,
    "Cr" => CR,
    "Mn" => MN,
    "Fe" => FE,
    "Co" => CO,
    "Ni" => NI,
    "Cu" => CU,
    "Zn" => ZN,
    "Ga" => GA,
    "Ge" => GE,
    "As" => AS,
    "Se" => SE,
    "Br" => BR,
    "Kr" => KR,
    "Rb" => RB,
    "Sr" => SR,
    "Y" => Y,
    "Zr" => ZR,
    "Nb" => NB,
    "Mo" => MO,
    "Tc" => TC,
    "Ru" => RU,
    "Rh" => RH,
    "Pd" => PD,
    "Ag" => AG,
    "Cd" => CD,
    "In" => IN,
    "Sn" => SN,
    "Sb" => SB,
    "Te" => TE,
    "I" => I,
    "Xe" => XE,
    "Cs" => CS,
    "Ba"=> BA,
    "La" => LA,
    "Ce" => CE,
    "Pr" => PR,
    "Nd" => ND,
    "Pm" => PM,
    "Sm" => SM,
    "Eu" => EU,
    "Gd" => GD,
    "Tb" => TB,
    "Dy" => DY,
    "Ho" => HO,
    "Er" => ER,
    "Tm" => TM,
    "Yb" => YB,
    "Lu" => LU,
    "Hf" => HF,
    "Ta" => TA,
    "W" => W,
    "Re" => RE,
    "Os" => OS,
    "Ir" => IR,
    "Pt" => PT,
    "Au" => AU,
    "Hg" => HG,
    "Tl" => TL,
    "Pb" => PB,
    "Bi" => BI,
    "Po" => PO,
    "At" => AT,
    "Rn" => RN,
    "Fr" => FR,
    "Ra" => RA,
    "Ac" => AC,
    "Th" => TH,
    "Pa" => PA,
    "U" => U,
    "Np" => NP,
    "Pu" => PU,
    "Am" => AM,
    "Cm" => CM,
    "Bk" => BK,
    "Cf" => CF,
    "Es" => ES,
    "Fm" => FM,
    "Md" => MD,
    "No" => NO,
    "Lr" => LR,
    "Rf" => RF,
    "Db" => DB,
    "Sg" => SG,
    "Bh" => BH,
    "Hs" => HS,
    "Mt" => MT,
    "Ds" => DS,
    "Rg" => RG,
    "Cn" => CN,
    "Nh" => NH,
    "Fl" => FL,
    "Mc" => MC,
    "Lv" => LV,
    "Ts" => TS,
    "Og" => OG,
    "h" => H,
    "d" => D,
    "t" => T,
    "he" => HE,
    "li" => LI,
    "be" => BE,
    "b" => B,
    "c" => C,
    "n" => N,
    "o" => O,
    "f" => F,
    "ne" => NE,
    "na" => NA,
    "mg" => MG,
    "al" => AL,
    "si" => SI,
    "p" => P,
    "s" => S,
    "cl" => CL,
    "ar" => AR,
    "k" => K,
    "ca" => CA,
    "sc" => SC,
    "ti" => TI,
    "v" => V,
    "cr" => CR,
    "mn" => MN,
    "fe" => FE,
    "co" => CO,
    "ni" => NI,
    "cu" => CU,
    "zn" => ZN,
    "ga" => GA,
    "ge" => GE,
    "as" => AS,
    "se" => SE,
    "br" => BR,
    "kr" => KR,
    "rb" => RB,
    "sr" => SR,
    "y" => Y,
    "zr" => ZR,
    "nb" => NB,
    "mo" => MO,
    "tc" => TC,
    "ru" => RU,
    "rh" => RH,
    "pd" => PD,
    "ag" => AG,
    "cd" => CD,
    "in" => IN,
    "sn" => SN,
    "sb" => SB,
    "te" => TE,
    "i" => I,
    "xe" => XE,
    "cs" => CS,
    "ba"=> BA,
    "la" => LA,
    "ce" => CE,
    "pr" => PR,
    "nd" => ND,
    "pm" => PM,
    "sm" => SM,
    "eu" => EU,
    "gd" => GD,
    "tb" => TB,
    "dy" => DY,
    "ho" => HO,
    "er" => ER,
    "tm" => TM,
    "yb" => YB,
    "lu" => LU,
    "hf" => HF,
    "ta" => TA,
    "w" => W,
    "re" => RE,
    "os" => OS,
    "ir" => IR,
    "pt" => PT,
    "au" => AU,
    "hg" => HG,
    "tl" => TL,
    "pb" => PB,
    "bi" => BI,
    "po" => PO,
    "at" => AT,
    "rn" => RN,
    "fr" => FR,
    "ra" => RA,
    "ac" => AC,
    "th" => TH,
    "pa" => PA,
    "u" => U,
    "np" => NP,
    "pu" => PU,
    "am" => AM,
    "cm" => CM,
    "bk" => BK,
    "cf" => CF,
    "es" => ES,
    "fm" => FM,
    "md" => MD,
    "no" => NO,
    "lr" => LR,
    "rf" => RF,
    "db" => DB,
    "sg" => SG,
    "bh" => BH,
    "hs" => HS,
    "mt" => MT,
    "ds" => DS,
    "rg" => RG,
    "cn" => CN,
    "nh" => NH,
    "fl" => FL,
    "mc" => MC,
    "lv" => LV,
    "ts" => TS,
    "og" => OG
};
