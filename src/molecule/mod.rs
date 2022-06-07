#[macro_use]
pub mod element;
pub mod atom;
pub mod bond;
pub mod configuration;
pub mod molecule;
pub mod topology;

pub(crate) use atom::Atom;
pub(crate) use bond::RingBond;
pub use configuration::*;
pub(crate) use element::H;
pub use molecule::Molecule;
pub use topology::{create, Topology, TopologySeq};

pub const HAS_AROM: u8 = 0x1;
pub const HAS_EXT_STRO: u8 = 0x4;
pub const HAS_ATM_STRO: u8 = 0x4;
pub const HAS_BND_STRO: u8 = 0x4;
pub const HAS_STRO: u8 = HAS_BND_STRO | HAS_ATM_STRO | HAS_EXT_STRO;
// pub(crate) const BRACKET_HYDROGEN: Atom = Atom::new_bracket(H, 1, 1, 0);
// pub(crate) const BRACKET_DEUTERIUM: Atom = Atom::new_bracket(H, 2, 1, 0);
// pub(crate) const BRACKET_TRITIUM: Atom = Atom::new_bracket(H, 3, 1, 0);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
