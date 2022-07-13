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
    explicit_hydrogens: u8,
    charge: i8,
    isotope: i16,
    bond_degree: u8,
    ring_size: u8,
    ring_membership: u8,
    ring_connectivity: u8,
    max_bonds_ringsize: u8,
    isorganogen: bool,
    rank: Option<u128>,
    symmetry_class: Option<usize>,
    is_stereocenter: bool,
}

impl Atom {
    #[inline]
    const fn new(e: Element, kind: AtomKind, isotope: i16, isorganogen: bool) -> Self {
        Self {
            element: e,
            kind,
            explicit_hydrogens: 0,
            charge: 0,
            isotope,
            bond_degree: 0,
            ring_membership: 0,
            ring_size: 0,
            ring_connectivity: 0,
            max_bonds_ringsize: 0,
            isorganogen,
            rank: None,
            symmetry_class: None,
            is_stereocenter: false,
        }
    }

    pub(crate) fn new_aromatic(e: Element, isorganogen: bool) -> Self {
        Atom::new(e, AtomKind::Aromatic, -1, isorganogen)
    }

    pub fn new_aliphatic(e: Element, isorganogen: bool) -> Self {
        Atom::new(e, AtomKind::Aliphatic, -1, isorganogen)
    }

    pub(crate) const fn new_bracket(
        e: Element,
        isotope: i16,
        hydrogens: u8,
        charge: i8,
        is_aromatic: bool,
        is_organogen: bool,
    ) -> Self {
        Self {
            element: e,
            kind: AtomKind::Bracket(is_aromatic),
            explicit_hydrogens: hydrogens,
            charge,
            isotope,
            bond_degree: 0,
            ring_membership: 0,
            ring_size: 0,
            ring_connectivity: 0,
            max_bonds_ringsize: 0,
            isorganogen: is_organogen,
            rank: None,
            symmetry_class: None,
            is_stereocenter: false,
        }
    }

    #[inline]
    pub fn is(&self, tar: &str) -> bool {
        self.element.symbol() == tar
    }

    #[inline]
    pub fn new_any(e: Element, isorganogen: bool) -> Self {
        Atom::new(e, AtomKind::Any, -1, isorganogen)
    }

    #[inline]
    pub fn isotope(&self) -> i16 {
        return self.isotope;
    }

    #[inline]
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

    #[inline]
    pub(crate) fn set_aromatic(&mut self) {
        match &mut self.kind {
            AtomKind::Bracket(is_aromatic) => *is_aromatic = true,
            _ => self.kind = AtomKind::Aromatic,
        };
    }

    #[inline]
    pub(crate) fn set_stereocenter(&mut self) {
        if !self.is_stereocenter {
            self.is_stereocenter = true;
        }
    }

    #[inline]
    pub fn is_stereocenter(&self) -> bool {
        self.is_stereocenter
    }

    #[inline]
    pub fn is_organogen(&self) -> bool {
        self.isorganogen
    }

    #[inline]
    pub fn is_aliphatic(&self) -> bool {
        self.kind == AtomKind::Aliphatic
    }

    #[inline]
    pub fn is_bracket_atom(&self) -> bool {
        match self.kind {
            AtomKind::Bracket(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn ele_is_any(&self) -> bool {
        self.element.symbol() == "*"
    }

    #[inline]
    pub(crate) fn kind(&self) -> AtomKind {
        self.kind.clone()
    }

    #[inline]
    pub fn charge(&self) -> i8 {
        return self.charge;
    }

    #[inline]
    pub fn explicit_hydrogens(&self) -> u8 {
        return self.explicit_hydrogens;
    }

    #[inline]
    pub(crate) fn element(&self) -> Element {
        self.element.clone()
    }

    #[inline]
    pub(crate) fn set_membership(&mut self, rm: u8) {
        self.ring_membership = rm;
    }

    #[inline]
    pub(crate) fn set_ring_size(&mut self, rs: u8) {
        self.ring_size = rs;
    }

    #[inline]
    pub(crate) fn ring_size(&self) -> u8 {
        self.ring_size
    }

    #[inline]
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

    #[inline]
    pub(crate) fn incr_ring_connectivity(&mut self, con: u8) {
        self.ring_connectivity += con;
    }

    #[inline]
    pub(crate) fn rank(&self) -> u128 {
        self.rank.unwrap()
    }

    #[inline]
    pub(crate) fn set_rank(&mut self, r: u128) {
        self.rank = Some(r);
    }

    #[inline]
    pub(crate) fn ring_connectivity(&self) -> u8 {
        self.ring_connectivity
    }

    #[inline]
    pub(crate) fn symmetry_class(&self) -> usize {
        self.symmetry_class.unwrap()
    }

    #[inline]
    pub(crate) fn set_symmetry_class(&mut self) {
        self.symmetry_class = Some(self.rank.unwrap() as usize);
    }

    pub(crate) fn to_aromatic(&self, spec: Specification) -> Option<Self> {
        if self.is_aromatic() || self.element.is_aromatic(spec) {
            return None;
        }
        Some(Self {
            element: self.element.clone(),
            kind: AtomKind::Aromatic,
            explicit_hydrogens: self.explicit_hydrogens,
            charge: self.charge,
            isotope: self.isotope,
            bond_degree: self.bond_degree,
            ring_membership: self.ring_membership,
            ring_size: self.ring_size,
            max_bonds_ringsize: self.max_bonds_ringsize,
            ring_connectivity: self.ring_connectivity,
            isorganogen: self.isorganogen,
            rank: self.rank,
            symmetry_class: self.symmetry_class,
            is_stereocenter: self.is_stereocenter,
        })
    }

    pub(crate) fn to_bracket(&self) -> Option<Self> {
        if self.is_bracket_atom() {
            return None;
        }
        Some(Self {
            element: self.element.clone(),
            kind: AtomKind::Bracket(self.is_aromatic()),
            explicit_hydrogens: self.explicit_hydrogens,
            charge: self.charge,
            isotope: self.isotope,
            bond_degree: self.bond_degree,
            ring_membership: self.ring_membership,
            ring_size: self.ring_size,
            max_bonds_ringsize: self.max_bonds_ringsize,
            ring_connectivity: self.ring_connectivity,
            isorganogen: self.isorganogen,
            rank: self.rank,
            symmetry_class: self.symmetry_class,
            is_stereocenter: self.is_stereocenter,
        })
    }

    #[inline]
    pub(crate) fn implict_hydrogen_amount(&self, valence: u8) -> u8 {
        match self.kind {
            AtomKind::Aromatic => self.element.implict_atom_hydrogen(valence),
            AtomKind::Aliphatic => self.element.implict_hydrogen_amount(valence),
            _ => 0,
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
}
