#[macro_export]
macro_rules! to_bond {
    ($variable:ident, $t:expr, $ele:expr, $direct:expr, $kind: expr) => {
        pub const $variable: Bond = Bond::new($t, $ele, $direct, $kind);
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BondKind {
    Dot,
    Implict,
    ImplictAromatic,
    Single,
    Double,
    DoubleAromatic,
    Triple,
    Quadruple,
    Aromatic,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bond {
    token: &'static str,
    electron: u8,
    directional: bool,
    kind: BondKind,
    ring_size: u8,
    ring_membership: u8,
}

impl Bond {
    pub const fn new(t: &'static str, ele: u8, direct: bool, kind: BondKind) -> Self {
        Self {
            token: t,
            electron: ele,
            directional: direct,
            kind,
            ring_size: 0,
            ring_membership: 0,
        }
    }

    pub fn token(&self) -> &str {
        return self.token;
    }

    pub(crate) fn electron(&self) -> u8 {
        return self.electron;
    }

    pub fn direction(&self) -> bool {
        self.directional
    }

    pub fn inverse(&self) -> Self {
        match self.kind {
            BondKind::Up => DOWN,
            BondKind::Down => UP,
            _ => *self,
        }
    }

    pub fn is(&self, tar: &str) -> bool {
        self.token() == tar
    }

    pub fn is_aromatic(&self) -> bool {
        self.kind == BondKind::Aromatic
    }

    pub(crate) fn set_ring_membership(&mut self, rm: u8){
        self.ring_membership = rm;
    }
    pub(crate) fn set_ring_size(&mut self, rs: u8){
        self.ring_size = rs;
    }
}

to_bond!(DOT, ".", 0, false, BondKind::Dot);
to_bond!(IMPLICTAROMATIC, "", 1, false, BondKind::ImplictAromatic);
to_bond!(IMPLICT, "", 1, false, BondKind::Implict);
to_bond!(SINGLE, "-", 1, false, BondKind::Single);
to_bond!(DOUBLE, "=", 2, false, BondKind::Double);
to_bond!(DOUBLEAROMATIC, "=", 2, false, BondKind::DoubleAromatic);
to_bond!(TRIPLE, "#", 3, false, BondKind::Triple);
to_bond!(QUADRUPLE, "$", 4, false, BondKind::Quadruple);
to_bond!(AROMATIC, ":", 1, false, BondKind::Aromatic);
to_bond!(UP, "/", 1, true, BondKind::Up);
to_bond!(DOWN, "\\", 1, true, BondKind::Down);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RingBond {
    bond: Bond,
    vertex: u8,
    pos: Option<u8>,
}

impl RingBond {
    #[inline]
    pub fn new(bond: Bond, vertex: u8, pos: Option<u8>) -> Self {
        Self { bond, vertex, pos }
    }

    #[inline]
    pub fn vertex(&self) -> u8 {
        self.vertex
    }

    #[inline]
    pub fn bond(&self) -> Bond {
        self.bond
    }
}
