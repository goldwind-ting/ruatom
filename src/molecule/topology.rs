use super::configuration::*;
use crate::error::RuatomError;

#[derive(PartialEq, Eq, Debug)]
pub enum TopologySeq {
    Tetrahedral,
    ExtendedTetrahedral,
    UnknownTopology,
    Trigonal,
    SquarePlanar,
    TrigonalBipyramidal,
    Octahedral,
}

pub trait Topology {
    fn new_topology(u: u8, conf: Configuration, vs: Vec<i8>) -> Result<Self, RuatomError>
    where
        Self: Sized;
    fn configuration(&self) -> Result<Configuration, RuatomError> {
        return Ok(UNKNOWN);
    }
    fn atom(&self) -> i8;
    fn seq(&self) -> TopologySeq;
}

#[derive(PartialEq, Eq, Clone)]
struct BaseTopology {
    u: u8,
    p: u8,
    vs: Vec<i8>,
}

impl BaseTopology {
    fn new(u: u8, p: u8, vs: Vec<i8>) -> Self {
        Self { u, p, vs }
    }
}

pub struct Tetrahedral(BaseTopology);

impl Topology for Tetrahedral {
    fn new_topology(u: u8, conf: Configuration, vs: Vec<i8>) -> Result<Self, RuatomError> {
        if !conf.is_implict() && !conf.is_tetrahedral() {
            return Err(RuatomError::IllegalMolecule(
                "invalid Tetrahedral configuration",
            ));
        }
        Ok(Self(BaseTopology::new(u, conf.seq(), vs)))
    }

    fn configuration(&self) -> Result<Configuration, RuatomError> {
        if self.0.p == 1 {
            return Ok(TH1);
        } else if self.0.p == 2 {
            return Ok(TH2);
        } else {
            return Err(RuatomError::IllegalMolecule(
                "invalid Tetrahedral configuration",
            ));
        }
    }
    fn atom(&self) -> i8 {
        return self.0.u.clone() as i8;
    }
    fn seq(&self) -> TopologySeq {
        TopologySeq::Tetrahedral
    }
}

pub struct Trigonal(BaseTopology);

impl Topology for Trigonal {
    fn new_topology(u: u8, conf: Configuration, vs: Vec<i8>) -> Result<Self, RuatomError> {
        if !conf.is_implict() && !conf.is_trigonal() {
            return Err(RuatomError::IllegalMolecule(
                "invalid Trigonal configuration",
            ));
        }
        Ok(Self(BaseTopology::new(u, conf.seq(), vs)))
    }

    fn configuration(&self) -> Result<Configuration, RuatomError> {
        if self.0.p == 1 {
            return Ok(DB1);
        } else if self.0.p == 2 {
            return Ok(DB2);
        } else {
            return Err(RuatomError::IllegalMolecule(
                "invalid Trigonal configuration",
            ));
        }
    }
    fn atom(&self) -> i8 {
        return self.0.u.clone() as i8;
    }

    fn seq(&self) -> TopologySeq {
        TopologySeq::Trigonal
    }
}

pub struct ExtendedTetrahedral(BaseTopology);

impl Topology for ExtendedTetrahedral {
    fn new_topology(u: u8, conf: Configuration, vs: Vec<i8>) -> Result<Self, RuatomError> {
        if !conf.is_implict() && !conf.is_extend_tetrahedral() {
            return Err(RuatomError::IllegalMolecule(
                "invalid ExtendedTetrahedral configuration",
            ));
        }
        Ok(Self(BaseTopology::new(u, conf.seq(), vs)))
    }

    fn configuration(&self) -> Result<Configuration, RuatomError> {
        if self.0.p == 1 {
            return Ok(AL1);
        } else if self.0.p == 2 {
            return Ok(AL2);
        } else {
            return Err(RuatomError::IllegalMolecule(
                "invalid ExtendedTetrahedral configuration",
            ));
        }
    }
    fn atom(&self) -> i8 {
        return self.0.u.clone() as i8;
    }

    fn seq(&self) -> TopologySeq {
        TopologySeq::ExtendedTetrahedral
    }
}

pub struct SquarePlanar(BaseTopology);

impl Topology for SquarePlanar {
    fn new_topology(u: u8, conf: Configuration, vs: Vec<i8>) -> Result<Self, RuatomError> {
        if !conf.is_square_plannar() {
            return Err(RuatomError::IllegalMolecule(
                "invalid SquarePlanar configuration",
            ));
        }

        Ok(Self(BaseTopology::new(u, conf.seq(), vs)))
    }

    fn configuration(&self) -> Result<Configuration, RuatomError> {
        match self.0.p {
            1 => Ok(SP1),
            2 => Ok(SP2),
            3 => Ok(SP3),
            _ => Err(RuatomError::IllegalMolecule(
                "invalid SquarePlanar configuration",
            )),
        }
    }
    fn atom(&self) -> i8 {
        return self.0.u.clone() as i8;
    }

    fn seq(&self) -> TopologySeq {
        TopologySeq::SquarePlanar
    }
}

pub struct TrigonalBipyramidal(BaseTopology);

impl Topology for TrigonalBipyramidal {
    fn new_topology(u: u8, conf: Configuration, vs: Vec<i8>) -> Result<Self, RuatomError> {
        if conf.seq() < 1 && conf.seq() > 20 {
            return Err(RuatomError::IllegalMolecule(
                "invalid TrigonalBipyramidal configuration",
            ));
        }
        Ok(Self(BaseTopology::new(u, conf.seq(), vs)))
    }

    fn configuration(&self) -> Result<Configuration, RuatomError> {
        Ok(TB_MAP
            .get(&self.0.p.to_string())
            .ok_or(RuatomError::IllegalMolecule(
                "invalid TrigonalBipyramidal configuration",
            ))?
            .clone()
            .to_owned())
    }
    fn atom(&self) -> i8 {
        return self.0.u.clone() as i8;
    }

    fn seq(&self) -> TopologySeq {
        TopologySeq::TrigonalBipyramidal
    }
}

pub struct Octahedral(BaseTopology);

impl Topology for Octahedral {
    fn new_topology(u: u8, conf: Configuration, vs: Vec<i8>) -> Result<Self, RuatomError> {
        if conf.seq() < 1 && conf.seq() > 30 {
            return Err(RuatomError::IllegalMolecule(
                "invalid Octahedral configuration",
            ));
        }
        Ok(Self(BaseTopology::new(u, conf.seq(), vs)))
    }

    fn configuration(&self) -> Result<Configuration, RuatomError> {
        Ok(OH_MAP
            .get(&self.0.p.to_string())
            .ok_or(RuatomError::IllegalMolecule(
                "invalid Octahedral configuration",
            ))?
            .clone()
            .to_owned())
    }
    fn atom(&self) -> i8 {
        return self.0.u.clone() as i8;
    }

    fn seq(&self) -> TopologySeq {
        TopologySeq::Octahedral
    }
}

pub struct UnknownTopology(BaseTopology);

impl Topology for UnknownTopology {
    fn new_topology(u: u8, conf: Configuration, vs: Vec<i8>) -> Result<Self, RuatomError> {
        Ok(Self(BaseTopology::new(u, conf.seq(), vs)))
    }
    fn atom(&self) -> i8 {
        return self.0.u.clone() as i8;
    }

    fn seq(&self) -> TopologySeq {
        TopologySeq::UnknownTopology
    }
}

pub fn create(u: u8, conf: Configuration, vs: Vec<i8>) -> Result<Box<dyn Topology>, RuatomError> {
    if conf.is_tetrahedral() {
        return Ok(Box::new(Tetrahedral::new_topology(u, conf, vs)?));
    } else if conf.is_trigonal() {
        return Ok(Box::new(Trigonal::new_topology(u, conf, vs)?));
    } else if conf.is_extend_tetrahedral() {
        return Ok(Box::new(ExtendedTetrahedral::new_topology(u, conf, vs)?));
    } else if conf.is_square_plannar() {
        return Ok(Box::new(SquarePlanar::new_topology(u, conf, vs)?));
    } else if conf.is_trigonal_bipyramidal() {
        return Ok(Box::new(TrigonalBipyramidal::new_topology(u, conf, vs)?));
    } else if conf.is_octahedral() {
        return Ok(Box::new(Octahedral::new_topology(u, conf, vs)?));
    }
    return Ok(Box::new(UnknownTopology::new_topology(u, conf, vs)?));
}
