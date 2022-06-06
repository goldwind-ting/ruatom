use crate::element::Element;
use crate::element::Specification;
use crate::error::MoleculeError;

#[derive(PartialEq, Eq, Clone, Debug)]
pub(crate) enum AtomKind {
    Any,
    Aliphatic,
    Aromatic,
    Bracket(bool),
}

#[derive(Clone, Debug)]
pub struct Atom {
    element: Element,
    kind: AtomKind,
    hydrogen_count: u8,
    charge: i8,
    isotope: i16,
}

impl Atom {
    const fn new(e: Element, kind: AtomKind, isotope: i16) -> Self {
        Self {
            element: e,
            kind,
            hydrogen_count: 0,
            charge: 0,
            isotope,
        }
    }

    pub fn new_aromatic(e: Element) -> Self {
        Atom::new(e, AtomKind::Aromatic, -1)
    }

    pub fn new_aliphatic(e: Element) -> Self {
        Atom::new(e, AtomKind::Aliphatic, -1)
    }

    pub const fn new_bracket(
        e: Element,
        isotope: i16,
        hydrogens: u8,
        charge: i8,
        is_aromatic: bool,
    ) -> Self {
        Self {
            element: e,
            kind: AtomKind::Bracket(is_aromatic),
            hydrogen_count: hydrogens,
            charge,
            isotope,
        }
    }

    pub fn is(&self, tar: &str) -> bool {
        self.element.symbol() == tar
    }

    pub fn new_any(e: Element) -> Self {
        Atom::new(e, AtomKind::Any, -1)
    }

    pub fn isotope(&self) -> i16 {
        return self.isotope;
    }

    #[inline]
    pub fn is_aromatic(&self) -> bool {
        match self.kind {
            AtomKind::Bracket(is_aromatic) => is_aromatic,
            AtomKind::Aromatic => true,
            _ => false,
        }
    }

    pub fn is_aliphatic(&self) -> bool {
        self.kind == AtomKind::Aliphatic
    }

    pub fn is_bracket_atom(&self) -> bool {
        match self.kind {
            AtomKind::Bracket(_) => true,
            _ => false,
        }
    }

    pub fn ele_is_any(&self) -> bool {
        self.element.symbol() == "*"
    }

    pub(crate) fn kind(&self) -> AtomKind {
        self.kind.clone()
    }

    pub fn charge(&self) -> i8 {
        return self.charge;
    }

    pub fn hydrogens(&self) -> u8 {
        return self.hydrogen_count;
    }

    pub(crate) fn element(&self) -> Element {
        self.element.clone()
    }

    pub(crate) fn to_aromatic(&self, spec: Specification) -> Option<Self> {
        if self.is_aromatic() || self.element.is_aromatic(spec) {
            return None;
        }
        Some(Self {
            element: self.element.clone(),
            kind: AtomKind::Aromatic,
            hydrogen_count: self.hydrogen_count,
            charge: self.charge,
            isotope: self.isotope,
        })
    }

    pub(crate) fn to_bracket(&self) -> Option<Self> {
        if self.is_bracket_atom() {
            return None;
        }
        Some(Self {
            element: self.element.clone(),
            kind: AtomKind::Bracket(self.is_aromatic()),
            hydrogen_count: self.hydrogen_count,
            charge: self.charge,
            isotope: self.isotope,
        })
    }

    pub(crate) fn implict_hydrogen_amount(&self, valence: u8) -> u8 {
        match self.kind {
            AtomKind::Aromatic => self.element.implict_atom_hydrogen(valence),
            AtomKind::Aliphatic => self.element.implict_hydrogen_amount(valence),
            _ => self.hydrogen_count,
        }
    }

    #[inline]
    pub(crate) fn get_mass(&self) -> Result<f64, MoleculeError> {
        self.element.get_mass(self.isotope)
    }

    // pub(crate) fn to_aliphatic(&self) -> Option<Self> {
    //     if self.is_aliphatic() {
    //         return None;
    //     }
    //     Some(Self {
    //         element: self.element.clone(),
    //         kind: AtomKind::Aliphatic,
    //         hydrogen_count: self.hydrogen_count,
    //         charge: self.charge,
    //         isotope: self.isotope,
    //     })
    // }
}
