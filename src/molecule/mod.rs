#[macro_use]
pub mod element;
pub mod atom;
pub mod bond;
mod canon;
pub mod configuration;
pub mod molecule;
pub mod topology;

use std::borrow::Borrow;
use std::borrow::Cow;

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

pub fn leftpad_with<'a, S>(string: S, codepoints: usize, pad_char: char) -> Cow<'a, str>
where
    S: Into<Cow<'a, str>>,
{
    let cow = string.into();

    let cow_codepoints = cow.chars().count();
    if codepoints <= cow_codepoints {
        return cow;
    }

    let to_pad = codepoints - cow_codepoints;
    let mut padded = String::with_capacity(cow.len() + to_pad);

    for _ in 0..to_pad {
        padded.push(pad_char);
    }

    padded.push_str(cow.borrow());

    padded.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
