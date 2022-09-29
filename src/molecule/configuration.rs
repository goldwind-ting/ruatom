use phf::phf_map;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Configuration {
    symbol: &'static str,
    order: Option<ConfigOrder>,
    kind: ConfigKind,
    seq: u8,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ConfigKind {
    None,
    Implicit,
    Tetrahedral,
    DoubleBond,
    ExtendedTetrahedral,
    SquarePlanar,
    TrigonalBipyramidal,
    Octahedral,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub(crate) enum ConfigOrder {
    AntiClockwise(&'static str),
    Clockwise(&'static str),
}

impl Configuration {
    pub(crate) const fn new(
        symbol: &'static str,
        order: Option<ConfigOrder>,
        kind: ConfigKind,
        seq: u8,
    ) -> Self {
        Self {
            symbol,
            order,
            kind,
            seq,
        }
    }

    pub(crate) fn is_implict(&self) -> bool {
        self.kind == ConfigKind::Implicit
    }

    pub(crate) fn is_anti_clockwise(&self) -> bool {
        self.order.clone().map_or(false, |o| match o {
            ConfigOrder::AntiClockwise(_) => true,
            _ => false,
        })
    }

    pub fn is_tetrahedral(&self) -> bool {
        self.kind == ConfigKind::Tetrahedral
    }

    pub fn is_extend_tetrahedral(&self) -> bool {
        self.kind == ConfigKind::ExtendedTetrahedral
    }

    pub fn is_square_plannar(&self) -> bool {
        self.kind == ConfigKind::SquarePlanar
    }

    pub(crate) fn seq(&self) -> u8 {
        self.seq
    }

    pub fn symbol(&self) -> &str {
        self.symbol
    }

    pub fn shorthand(&self) -> &str {
        if self.is_trigonal() {
            self.order.as_ref().map_or("", |o| match o {
                ConfigOrder::AntiClockwise(s) => s,
                ConfigOrder::Clockwise(s) => s,
            })
        } else {
            self.symbol()
        }
    }

    pub fn is_trigonal_bipyramidal(&self) -> bool {
        self.kind == ConfigKind::TrigonalBipyramidal
    }

    pub fn is_octahedral(&self) -> bool {
        self.kind == ConfigKind::Octahedral
    }

    pub fn is_trigonal(&self) -> bool {
        self.kind == ConfigKind::DoubleBond
    }
}

#[macro_export]
macro_rules! to_config {
    ($variable:ident, $sym:expr, $order:expr, $kind:expr, $seq:expr) => {
        pub const $variable: Configuration = Configuration::new($sym, $order, $kind, $seq);
    };
}

to_config!(UNKNOWN, "", None, ConfigKind::None, 0);
to_config!(
    TH1,
    "@TH1",
    Some(ConfigOrder::AntiClockwise("@")),
    ConfigKind::Tetrahedral,
    1
);
to_config!(
    TH2,
    "@TH2",
    Some(ConfigOrder::Clockwise("@@")),
    ConfigKind::Tetrahedral,
    2
);
to_config!(
    DB1,
    "@DB1",
    Some(ConfigOrder::AntiClockwise("@")),
    ConfigKind::DoubleBond,
    1
);
to_config!(
    DB2,
    "@DB2",
    Some(ConfigOrder::Clockwise("@@")),
    ConfigKind::DoubleBond,
    2
);
to_config!(
    AL1,
    "@AL1",
    Some(ConfigOrder::AntiClockwise("@")),
    ConfigKind::ExtendedTetrahedral,
    1
);
to_config!(
    AL2,
    "@AL2",
    Some(ConfigOrder::Clockwise("@@")),
    ConfigKind::ExtendedTetrahedral,
    2
);
to_config!(SP1, "@SP1", None, ConfigKind::SquarePlanar, 1);
to_config!(SP2, "@SP2", None, ConfigKind::SquarePlanar, 2);
to_config!(SP3, "@SP3", None, ConfigKind::SquarePlanar, 3);
to_config!(
    TB1,
    "@TB1",
    Some(ConfigOrder::AntiClockwise("@")),
    ConfigKind::TrigonalBipyramidal,
    1
);
to_config!(
    TB2,
    "@TB2",
    Some(ConfigOrder::Clockwise("@@")),
    ConfigKind::TrigonalBipyramidal,
    1
);
to_config!(TB3, "@TB3", None, ConfigKind::TrigonalBipyramidal, 3);
to_config!(TB4, "@TB4", None, ConfigKind::TrigonalBipyramidal, 4);
to_config!(TB5, "@TB5", None, ConfigKind::TrigonalBipyramidal, 5);
to_config!(TB6, "@TB6", None, ConfigKind::TrigonalBipyramidal, 6);
to_config!(TB7, "@TB7", None, ConfigKind::TrigonalBipyramidal, 7);
to_config!(TB8, "@TB8", None, ConfigKind::TrigonalBipyramidal, 8);
to_config!(TB9, "@TB9", None, ConfigKind::TrigonalBipyramidal, 9);
to_config!(TB10, "@TB10", None, ConfigKind::TrigonalBipyramidal, 10);
to_config!(TB11, "@TB11", None, ConfigKind::TrigonalBipyramidal, 11);
to_config!(TB12, "@TB12", None, ConfigKind::TrigonalBipyramidal, 12);
to_config!(TB13, "@TB13", None, ConfigKind::TrigonalBipyramidal, 13);
to_config!(TB14, "@TB14", None, ConfigKind::TrigonalBipyramidal, 14);
to_config!(TB15, "@TB15", None, ConfigKind::TrigonalBipyramidal, 15);
to_config!(TB16, "@TB16", None, ConfigKind::TrigonalBipyramidal, 16);
to_config!(TB17, "@TB17", None, ConfigKind::TrigonalBipyramidal, 17);
to_config!(TB18, "@TB18", None, ConfigKind::TrigonalBipyramidal, 18);
to_config!(TB19, "@TB19", None, ConfigKind::TrigonalBipyramidal, 19);
to_config!(TB20, "@TB20", None, ConfigKind::TrigonalBipyramidal, 20);
to_config!(
    OH1,
    "@OH1",
    Some(ConfigOrder::AntiClockwise("@")),
    ConfigKind::Octahedral,
    1
);
to_config!(
    OH2,
    "@OH2",
    Some(ConfigOrder::Clockwise("@@")),
    ConfigKind::Octahedral,
    2
);
to_config!(OH3, "@OH3", None, ConfigKind::Octahedral, 3);
to_config!(OH4, "@OH4", None, ConfigKind::Octahedral, 4);
to_config!(OH5, "@OH5", None, ConfigKind::Octahedral, 5);
to_config!(OH6, "@OH6", None, ConfigKind::Octahedral, 6);
to_config!(OH7, "@OH7", None, ConfigKind::Octahedral, 7);
to_config!(OH8, "@OH8", None, ConfigKind::Octahedral, 8);
to_config!(OH9, "@OH9", None, ConfigKind::Octahedral, 9);
to_config!(OH10, "@OH10", None, ConfigKind::Octahedral, 10);
to_config!(OH11, "@OH11", None, ConfigKind::Octahedral, 11);
to_config!(OH12, "@OH12", None, ConfigKind::Octahedral, 12);
to_config!(OH13, "@OH13", None, ConfigKind::Octahedral, 13);
to_config!(OH14, "@OH14", None, ConfigKind::Octahedral, 14);
to_config!(OH15, "@OH15", None, ConfigKind::Octahedral, 15);
to_config!(OH16, "@OH16", None, ConfigKind::Octahedral, 16);
to_config!(OH17, "@OH17", None, ConfigKind::Octahedral, 17);
to_config!(OH18, "@OH18", None, ConfigKind::Octahedral, 18);
to_config!(OH19, "@OH19", None, ConfigKind::Octahedral, 19);
to_config!(OH20, "@OH20", None, ConfigKind::Octahedral, 20);
to_config!(OH21, "@OH21", None, ConfigKind::Octahedral, 21);
to_config!(OH22, "@OH22", None, ConfigKind::Octahedral, 22);
to_config!(OH23, "@OH23", None, ConfigKind::Octahedral, 23);
to_config!(OH24, "@OH24", None, ConfigKind::Octahedral, 24);
to_config!(OH25, "@OH25", None, ConfigKind::Octahedral, 25);
to_config!(OH26, "@OH26", None, ConfigKind::Octahedral, 26);
to_config!(OH27, "@OH27", None, ConfigKind::Octahedral, 27);
to_config!(OH28, "@OH28", None, ConfigKind::Octahedral, 28);
to_config!(OH29, "@OH29", None, ConfigKind::Octahedral, 29);
to_config!(OH30, "@OH30", None, ConfigKind::Octahedral, 30);
to_config!(
    CLOCKWISE,
    "@@",
    Some(ConfigOrder::Clockwise("@@")),
    ConfigKind::Implicit,
    2
);
to_config!(
    ANTICLOCKWISE,
    "@",
    Some(ConfigOrder::AntiClockwise("@")),
    ConfigKind::Implicit,
    1
);

pub static TB_MAP: phf::Map<&'static str, &'static Configuration> = phf_map! {
    "1" => &TB1,
    "2" => &TB2,
    "3" => &TB3,
    "4" => &TB4,
    "5" => &TB5,
    "6" => &TB6,
    "7" => &TB7,
    "8" => &TB8,
    "9" => &TB9,
    "10" => &TB10,
    "11" => &TB11,
    "12" => &TB12,
    "13" => &TB13,
    "14" => &TB14,
    "15" => &TB15,
    "16" => &TB16,
    "17" => &TB17,
    "18" => &TB18,
    "19" => &TB19,
    "20" => &TB20,
};

pub static OH_MAP: phf::Map<&'static str, &'static Configuration> = phf_map! {
    "1" => &OH1,
    "2" => &OH2,
    "3" => &OH3,
    "4" => &OH4,
    "5" => &OH5,
    "6" => &OH6,
    "7" => &OH7,
    "8" => &OH8,
    "9" => &OH9,
    "10" => &OH10,
    "11" => &OH11,
    "12" => &OH12,
    "13" => &OH13,
    "14" => &OH14,
    "15" => &OH15,
    "16" => &OH16,
    "17" => &OH17,
    "18" => &OH18,
    "19" => &OH19,
    "20" => &OH20,
    "21" => &OH21,
    "22" => &OH22,
    "23" => &OH23,
    "24" => &OH24,
    "25" => &OH25,
    "26" => &OH26,
    "27" => &OH27,
    "28" => &OH28,
    "29" => &OH29,
    "30" => &OH30,
};
