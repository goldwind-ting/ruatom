use crate::error::RuatomError;

use super::element::Element;
use super::element::Specification;

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
    bond_degree: u8,
    ring_size: u8,
    ring_membership: u8,
    ring_connectivity: u8,
    max_bonds_ringsize: u8,
    chirality: u8,
}

impl Atom {
    const fn new(e: Element, kind: AtomKind, isotope: i16) -> Self {
        Self {
            element: e,
            kind,
            hydrogen_count: 0,
            charge: 0,
            isotope,
            bond_degree: 0,
            ring_membership: 0,
            ring_size: 0,
            ring_connectivity: 0,
            max_bonds_ringsize: 0,
            chirality: 0
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
            bond_degree: 0,
            ring_membership: 0,
            ring_size: 0,
            ring_connectivity: 0,
            max_bonds_ringsize: 0,
            chirality: 0,
        }
    }

    pub fn is(&self, tar: &str) -> bool {
        self.element.symbol() == tar
    }

    #[inline]
    pub fn new_any(e: Element) -> Self {
        Atom::new(e, AtomKind::Any, -1)
    }

    #[inline]
    pub fn isotope(&self) -> i16 {
        return self.isotope;
    }

    pub fn bond_degree(&self) -> u8 {
        self.bond_degree
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

    #[inline]
    pub(crate) fn set_membership(&mut self, rm: u8) {
        self.ring_membership = rm;
    }

    #[inline]
    pub(crate) fn set_ring_size(&mut self, rs: u8){
        self.ring_size = rs;
    }

    pub(crate) fn ring_membership(&self) -> u8 {
        self.ring_membership
    }

    #[inline]
    pub(crate) fn max_bonds_ringsize(&self) -> u8 {
        self.max_bonds_ringsize
    }

    #[inline]
    pub(crate) fn set_max_bonds_ringsize(&mut self, mbr: u8) {
        self.max_bonds_ringsize = mbr;
    }

    pub(crate) fn incr_ring_connectivity(&mut self, con: u8) {
        self.ring_connectivity += con;
    }

    pub(crate) fn ring_connectivity(&self) -> u8 {
        self.ring_connectivity
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
            bond_degree: self.bond_degree,
            ring_membership: self.ring_membership,
            ring_size: self.ring_size,
            max_bonds_ringsize: self.max_bonds_ringsize,
            ring_connectivity: self.ring_connectivity,
            chirality: self.chirality,
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
            bond_degree: self.bond_degree,
            ring_membership: self.ring_membership,
            ring_size: self.ring_size,
            max_bonds_ringsize: self.max_bonds_ringsize,
            ring_connectivity: self.ring_connectivity,
            chirality: self.chirality
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
    pub(crate) fn get_mass(&self) -> f64 {
        self.element.get_mass()
    }

    #[inline]
    pub(crate) fn get_exact_mass(&self) -> Result<f64, RuatomError> {
        self.element.get_exact_mass(self.isotope)
    }

    #[inline]
    pub(crate) fn incr_degree(&mut self, var: u8) {
        self.bond_degree += var;
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
