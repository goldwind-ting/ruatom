use super::configuration::*;
use crate::error::RuatomError;

const SQUAREPLANARPERMUTATIONS: [[i8; 32]; 3] = [
    [
        1, 2, 3, 4, 1, 4, 3, 2, 2, 3, 4, 1, 2, 1, 4, 3, 3, 4, 1, 2, 3, 2, 1, 4, 4, 3, 2, 1, 4, 1,
        2, 3,
    ], // SP1 (U)
    [
        1, 3, 2, 4, 1, 4, 2, 3, 2, 4, 1, 3, 2, 3, 1, 4, 3, 1, 4, 2, 3, 2, 4, 1, 4, 2, 3, 1, 4, 1,
        3, 2,
    ], // SP2 (D)
    [
        1, 2, 4, 3, 1, 3, 4, 2, 2, 1, 3, 4, 2, 4, 3, 1, 3, 4, 2, 1, 3, 1, 2, 4, 4, 3, 1, 2, 4, 2,
        1, 3,
    ], // SP3 (Z)
];

const TRIGONALBIPYRAMIDALPERMUTATIONS: [[i8; 30]; 20] = [
    [
        1, 2, 3, 4, 5, 1, 3, 4, 2, 5, 1, 4, 2, 3, 5, 5, 4, 3, 2, 1, 5, 2, 4, 3, 1, 5, 3, 2, 4, 1,
    ], // T21 a -> e @
    [
        1, 4, 3, 2, 5, 1, 3, 2, 4, 5, 1, 2, 4, 3, 5, 5, 2, 3, 4, 1, 5, 4, 2, 3, 1, 5, 3, 4, 2, 1,
    ], // T22 a -> e @@
    [
        1, 2, 3, 5, 4, 1, 3, 5, 2, 4, 1, 5, 2, 3, 4, 4, 5, 3, 2, 1, 4, 2, 5, 3, 1, 4, 3, 2, 5, 1,
    ], // T23 a -> d @
    [
        1, 5, 3, 2, 4, 1, 3, 2, 5, 4, 1, 2, 5, 3, 4, 4, 2, 3, 5, 1, 4, 5, 2, 3, 1, 4, 3, 5, 2, 1,
    ], // T24 a -> d @@
    [
        1, 2, 4, 5, 3, 1, 4, 5, 2, 3, 1, 5, 2, 4, 3, 3, 5, 4, 2, 1, 3, 2, 5, 4, 1, 3, 4, 2, 5, 1,
    ], // T25 a -> c @
    [
        1, 5, 4, 2, 3, 1, 4, 2, 5, 3, 1, 2, 5, 4, 3, 3, 2, 4, 5, 1, 3, 5, 2, 4, 1, 3, 4, 5, 2, 1,
    ], // T26 a -> c @@
    [
        1, 3, 4, 5, 2, 1, 4, 5, 3, 2, 1, 5, 3, 4, 2, 2, 5, 4, 3, 1, 2, 3, 5, 4, 1, 2, 4, 3, 5, 1,
    ], // T27 a -> b @
    [
        1, 5, 4, 3, 2, 1, 4, 3, 5, 2, 1, 3, 5, 4, 2, 2, 3, 4, 5, 1, 2, 5, 3, 4, 1, 2, 4, 5, 3, 1,
    ], // T28 a -> b @@
    [
        2, 1, 3, 4, 5, 2, 3, 4, 1, 5, 2, 4, 1, 3, 5, 5, 4, 3, 1, 2, 5, 1, 4, 3, 2, 5, 3, 1, 4, 2,
    ], // T29 b -> e @
    [
        2, 1, 3, 5, 4, 2, 3, 5, 1, 4, 2, 5, 1, 3, 4, 4, 5, 3, 1, 2, 4, 1, 5, 3, 2, 4, 3, 1, 5, 2,
    ], // T210 b -> d @
    [
        2, 4, 3, 1, 5, 2, 3, 1, 4, 5, 2, 1, 4, 3, 5, 5, 1, 3, 4, 2, 5, 4, 1, 3, 2, 5, 3, 4, 1, 2,
    ], // T211 b -> e @@
    [
        2, 5, 3, 1, 4, 2, 3, 1, 5, 4, 2, 1, 5, 3, 4, 4, 1, 3, 5, 2, 4, 5, 1, 3, 2, 4, 3, 5, 1, 2,
    ], // T212 b -> d @@
    [
        2, 1, 4, 5, 3, 2, 4, 5, 1, 3, 2, 5, 1, 4, 3, 3, 5, 4, 1, 2, 3, 1, 5, 4, 2, 3, 4, 1, 5, 2,
    ], // T213 b -> c @
    [
        2, 5, 4, 1, 3, 2, 4, 1, 5, 3, 2, 1, 5, 4, 3, 3, 1, 4, 5, 2, 3, 5, 1, 4, 2, 3, 4, 5, 1, 2,
    ], // T214 b -> c @@
    [
        3, 1, 2, 4, 5, 3, 2, 4, 1, 5, 3, 4, 1, 2, 5, 5, 4, 2, 1, 3, 5, 1, 4, 2, 3, 5, 2, 1, 4, 3,
    ], // T215 c -> e @
    [
        3, 1, 2, 5, 4, 3, 2, 5, 1, 4, 3, 5, 1, 2, 4, 4, 5, 2, 1, 3, 4, 1, 5, 2, 3, 4, 2, 1, 5, 3,
    ], // T216 c -> d @
    [
        4, 1, 2, 3, 5, 4, 2, 3, 1, 5, 4, 3, 1, 2, 5, 5, 3, 2, 1, 4, 5, 1, 3, 2, 4, 5, 2, 1, 3, 4,
    ], // T217 d -> e @
    [
        4, 3, 2, 1, 5, 4, 2, 1, 3, 5, 4, 1, 3, 2, 5, 5, 1, 2, 3, 4, 5, 3, 1, 2, 4, 5, 2, 3, 1, 4,
    ], // T218 d -> e @@
    [
        3, 5, 2, 1, 4, 3, 2, 1, 5, 4, 3, 1, 5, 2, 4, 4, 1, 2, 5, 3, 4, 5, 1, 2, 3, 4, 2, 5, 1, 3,
    ], // T219 c -> d @@
    [
        3, 4, 2, 1, 5, 3, 2, 1, 4, 5, 3, 1, 4, 2, 5, 5, 1, 2, 4, 3, 5, 4, 1, 2, 3, 5, 2, 4, 1, 3,
    ], // SP3 (Z)
];

const OCTAHEDRALPERMUTATIONS: [[i8; 144]; 30] = [
    [
        1, 2, 3, 4, 5, 6, 1, 3, 4, 5, 2, 6, 1, 4, 5, 2, 3, 6, 1, 5, 2, 3, 4, 6, 2, 1, 5, 6, 3, 4,
        2, 3, 1, 5, 6, 4, 2, 5, 6, 3, 1, 4, 2, 6, 3, 1, 5, 4, 3, 1, 2, 6, 4, 5, 3, 2, 6, 4, 1, 5,
        3, 4, 1, 2, 6, 5, 3, 6, 4, 1, 2, 5, 4, 1, 3, 6, 5, 2, 4, 3, 6, 5, 1, 2, 4, 5, 1, 3, 6, 2,
        4, 6, 5, 1, 3, 2, 5, 1, 4, 6, 2, 3, 5, 2, 1, 4, 6, 3, 5, 4, 6, 2, 1, 3, 5, 6, 2, 1, 4, 3,
        6, 2, 5, 4, 3, 1, 6, 3, 2, 5, 4, 1, 6, 4, 3, 2, 5, 1, 6, 5, 4, 3, 2, 1,
    ],
    [
        1, 2, 5, 4, 3, 6, 1, 3, 2, 5, 4, 6, 1, 4, 3, 2, 5, 6, 1, 5, 4, 3, 2, 6, 2, 1, 3, 6, 5, 4,
        2, 3, 6, 5, 1, 4, 2, 5, 1, 3, 6, 4, 2, 6, 5, 1, 3, 4, 3, 1, 4, 6, 2, 5, 3, 2, 1, 4, 6, 5,
        3, 4, 6, 2, 1, 5, 3, 6, 2, 1, 4, 5, 4, 1, 5, 6, 3, 2, 4, 3, 1, 5, 6, 2, 4, 5, 6, 3, 1, 2,
        4, 6, 3, 1, 5, 2, 5, 1, 2, 6, 4, 3, 5, 2, 6, 4, 1, 3, 5, 4, 1, 2, 6, 3, 5, 6, 4, 1, 2, 3,
        6, 2, 3, 4, 5, 1, 6, 3, 4, 5, 2, 1, 6, 4, 5, 2, 3, 1, 6, 5, 2, 3, 4, 1,
    ],
    // @OH3
    [
        1, 2, 3, 4, 6, 5, 1, 3, 4, 6, 2, 5, 1, 4, 6, 2, 3, 5, 1, 6, 2, 3, 4, 5, 2, 1, 6, 5, 3, 4,
        2, 3, 1, 6, 5, 4, 2, 5, 3, 1, 6, 4, 2, 6, 5, 3, 1, 4, 3, 1, 2, 5, 4, 6, 3, 2, 5, 4, 1, 6,
        3, 4, 1, 2, 5, 6, 3, 5, 4, 1, 2, 6, 4, 1, 3, 5, 6, 2, 4, 3, 5, 6, 1, 2, 4, 5, 6, 1, 3, 2,
        4, 6, 1, 3, 5, 2, 5, 2, 6, 4, 3, 1, 5, 3, 2, 6, 4, 1, 5, 4, 3, 2, 6, 1, 5, 6, 4, 3, 2, 1,
        6, 1, 4, 5, 2, 3, 6, 2, 1, 4, 5, 3, 6, 4, 5, 2, 1, 3, 6, 5, 2, 1, 4, 3,
    ],
    // @OH4
    [
        1, 2, 3, 5, 4, 6, 1, 3, 5, 4, 2, 6, 1, 4, 2, 3, 5, 6, 1, 5, 4, 2, 3, 6, 2, 1, 4, 6, 3, 5,
        2, 3, 1, 4, 6, 5, 2, 4, 6, 3, 1, 5, 2, 6, 3, 1, 4, 5, 3, 1, 2, 6, 5, 4, 3, 2, 6, 5, 1, 4,
        3, 5, 1, 2, 6, 4, 3, 6, 5, 1, 2, 4, 4, 1, 5, 6, 2, 3, 4, 2, 1, 5, 6, 3, 4, 5, 6, 2, 1, 3,
        4, 6, 2, 1, 5, 3, 5, 1, 3, 6, 4, 2, 5, 3, 6, 4, 1, 2, 5, 4, 1, 3, 6, 2, 5, 6, 4, 1, 3, 2,
        6, 2, 4, 5, 3, 1, 6, 3, 2, 4, 5, 1, 6, 4, 5, 3, 2, 1, 6, 5, 3, 2, 4, 1,
    ],
    // @OH5
    [
        1, 2, 3, 6, 4, 5, 1, 3, 6, 4, 2, 5, 1, 4, 2, 3, 6, 5, 1, 6, 4, 2, 3, 5, 2, 1, 4, 5, 3, 6,
        2, 3, 1, 4, 5, 6, 2, 4, 5, 3, 1, 6, 2, 5, 3, 1, 4, 6, 3, 1, 2, 5, 6, 4, 3, 2, 5, 6, 1, 4,
        3, 5, 6, 1, 2, 4, 3, 6, 1, 2, 5, 4, 4, 1, 6, 5, 2, 3, 4, 2, 1, 6, 5, 3, 4, 5, 2, 1, 6, 3,
        4, 6, 5, 2, 1, 3, 5, 2, 4, 6, 3, 1, 5, 3, 2, 4, 6, 1, 5, 4, 6, 3, 2, 1, 5, 6, 3, 2, 4, 1,
        6, 1, 3, 5, 4, 2, 6, 3, 5, 4, 1, 2, 6, 4, 1, 3, 5, 2, 6, 5, 4, 1, 3, 2,
    ],
    // @OH6
    [
        1, 2, 3, 5, 6, 4, 1, 3, 5, 6, 2, 4, 1, 5, 6, 2, 3, 4, 1, 6, 2, 3, 5, 4, 2, 1, 6, 4, 3, 5,
        2, 3, 1, 6, 4, 5, 2, 4, 3, 1, 6, 5, 2, 6, 4, 3, 1, 5, 3, 1, 2, 4, 5, 6, 3, 2, 4, 5, 1, 6,
        3, 4, 5, 1, 2, 6, 3, 5, 1, 2, 4, 6, 4, 2, 6, 5, 3, 1, 4, 3, 2, 6, 5, 1, 4, 5, 3, 2, 6, 1,
        4, 6, 5, 3, 2, 1, 5, 1, 3, 4, 6, 2, 5, 3, 4, 6, 1, 2, 5, 4, 6, 1, 3, 2, 5, 6, 1, 3, 4, 2,
        6, 1, 5, 4, 2, 3, 6, 2, 1, 5, 4, 3, 6, 4, 2, 1, 5, 3, 6, 5, 4, 2, 1, 3,
    ],
    // @OH7
    [
        1, 2, 3, 6, 5, 4, 1, 3, 6, 5, 2, 4, 1, 5, 2, 3, 6, 4, 1, 6, 5, 2, 3, 4, 2, 1, 5, 4, 3, 6,
        2, 3, 1, 5, 4, 6, 2, 4, 3, 1, 5, 6, 2, 5, 4, 3, 1, 6, 3, 1, 2, 4, 6, 5, 3, 2, 4, 6, 1, 5,
        3, 4, 6, 1, 2, 5, 3, 6, 1, 2, 4, 5, 4, 2, 5, 6, 3, 1, 4, 3, 2, 5, 6, 1, 4, 5, 6, 3, 2, 1,
        4, 6, 3, 2, 5, 1, 5, 1, 6, 4, 2, 3, 5, 2, 1, 6, 4, 3, 5, 4, 2, 1, 6, 3, 5, 6, 4, 2, 1, 3,
        6, 1, 3, 4, 5, 2, 6, 3, 4, 5, 1, 2, 6, 4, 5, 1, 3, 2, 6, 5, 1, 3, 4, 2,
    ],
    // @OH8
    [
        1, 2, 4, 3, 5, 6, 1, 3, 5, 2, 4, 6, 1, 4, 3, 5, 2, 6, 1, 5, 2, 4, 3, 6, 2, 1, 5, 6, 4, 3,
        2, 4, 1, 5, 6, 3, 2, 5, 6, 4, 1, 3, 2, 6, 4, 1, 5, 3, 3, 1, 4, 6, 5, 2, 3, 4, 6, 5, 1, 2,
        3, 5, 1, 4, 6, 2, 3, 6, 5, 1, 4, 2, 4, 1, 2, 6, 3, 5, 4, 2, 6, 3, 1, 5, 4, 3, 1, 2, 6, 5,
        4, 6, 3, 1, 2, 5, 5, 1, 3, 6, 2, 4, 5, 2, 1, 3, 6, 4, 5, 3, 6, 2, 1, 4, 5, 6, 2, 1, 3, 4,
        6, 2, 5, 3, 4, 1, 6, 3, 4, 2, 5, 1, 6, 4, 2, 5, 3, 1, 6, 5, 3, 4, 2, 1,
    ],
    // @OH9
    [
        1, 2, 4, 3, 6, 5, 1, 3, 6, 2, 4, 5, 1, 4, 3, 6, 2, 5, 1, 6, 2, 4, 3, 5, 2, 1, 6, 5, 4, 3,
        2, 4, 1, 6, 5, 3, 2, 5, 4, 1, 6, 3, 2, 6, 5, 4, 1, 3, 3, 1, 4, 5, 6, 2, 3, 4, 5, 6, 1, 2,
        3, 5, 6, 1, 4, 2, 3, 6, 1, 4, 5, 2, 4, 1, 2, 5, 3, 6, 4, 2, 5, 3, 1, 6, 4, 3, 1, 2, 5, 6,
        4, 5, 3, 1, 2, 6, 5, 2, 6, 3, 4, 1, 5, 3, 4, 2, 6, 1, 5, 4, 2, 6, 3, 1, 5, 6, 3, 4, 2, 1,
        6, 1, 3, 5, 2, 4, 6, 2, 1, 3, 5, 4, 6, 3, 5, 2, 1, 4, 6, 5, 2, 1, 3, 4,
    ],
    // @OH10
    [
        1, 2, 5, 3, 4, 6, 1, 3, 4, 2, 5, 6, 1, 4, 2, 5, 3, 6, 1, 5, 3, 4, 2, 6, 2, 1, 4, 6, 5, 3,
        2, 4, 6, 5, 1, 3, 2, 5, 1, 4, 6, 3, 2, 6, 5, 1, 4, 3, 3, 1, 5, 6, 4, 2, 3, 4, 1, 5, 6, 2,
        3, 5, 6, 4, 1, 2, 3, 6, 4, 1, 5, 2, 4, 1, 3, 6, 2, 5, 4, 2, 1, 3, 6, 5, 4, 3, 6, 2, 1, 5,
        4, 6, 2, 1, 3, 5, 5, 1, 2, 6, 3, 4, 5, 2, 6, 3, 1, 4, 5, 3, 1, 2, 6, 4, 5, 6, 3, 1, 2, 4,
        6, 2, 4, 3, 5, 1, 6, 3, 5, 2, 4, 1, 6, 4, 3, 5, 2, 1, 6, 5, 2, 4, 3, 1,
    ],
    // @OH11
    [
        1, 2, 6, 3, 4, 5, 1, 3, 4, 2, 6, 5, 1, 4, 2, 6, 3, 5, 1, 6, 3, 4, 2, 5, 2, 1, 4, 5, 6, 3,
        2, 4, 5, 6, 1, 3, 2, 5, 6, 1, 4, 3, 2, 6, 1, 4, 5, 3, 3, 1, 6, 5, 4, 2, 3, 4, 1, 6, 5, 2,
        3, 5, 4, 1, 6, 2, 3, 6, 5, 4, 1, 2, 4, 1, 3, 5, 2, 6, 4, 2, 1, 3, 5, 6, 4, 3, 5, 2, 1, 6,
        4, 5, 2, 1, 3, 6, 5, 2, 4, 3, 6, 1, 5, 3, 6, 2, 4, 1, 5, 4, 3, 6, 2, 1, 5, 6, 2, 4, 3, 1,
        6, 1, 2, 5, 3, 4, 6, 2, 5, 3, 1, 4, 6, 3, 1, 2, 5, 4, 6, 5, 3, 1, 2, 4,
    ],
    // @OH12
    [
        1, 2, 5, 3, 6, 4, 1, 3, 6, 2, 5, 4, 1, 5, 3, 6, 2, 4, 1, 6, 2, 5, 3, 4, 2, 1, 6, 4, 5, 3,
        2, 4, 5, 1, 6, 3, 2, 5, 1, 6, 4, 3, 2, 6, 4, 5, 1, 3, 3, 1, 5, 4, 6, 2, 3, 4, 6, 1, 5, 2,
        3, 5, 4, 6, 1, 2, 3, 6, 1, 5, 4, 2, 4, 2, 6, 3, 5, 1, 4, 3, 5, 2, 6, 1, 4, 5, 2, 6, 3, 1,
        4, 6, 3, 5, 2, 1, 5, 1, 2, 4, 3, 6, 5, 2, 4, 3, 1, 6, 5, 3, 1, 2, 4, 6, 5, 4, 3, 1, 2, 6,
        6, 1, 3, 4, 2, 5, 6, 2, 1, 3, 4, 5, 6, 3, 4, 2, 1, 5, 6, 4, 2, 1, 3, 5,
    ],
    // @OH13
    [
        1, 2, 6, 3, 5, 4, 1, 3, 5, 2, 6, 4, 1, 5, 2, 6, 3, 4, 1, 6, 3, 5, 2, 4, 2, 1, 5, 4, 6, 3,
        2, 4, 6, 1, 5, 3, 2, 5, 4, 6, 1, 3, 2, 6, 1, 5, 4, 3, 3, 1, 6, 4, 5, 2, 3, 4, 5, 1, 6, 2,
        3, 5, 1, 6, 4, 2, 3, 6, 4, 5, 1, 2, 4, 2, 5, 3, 6, 1, 4, 3, 6, 2, 5, 1, 4, 5, 3, 6, 2, 1,
        4, 6, 2, 5, 3, 1, 5, 1, 3, 4, 2, 6, 5, 2, 1, 3, 4, 6, 5, 3, 4, 2, 1, 6, 5, 4, 2, 1, 3, 6,
        6, 1, 2, 4, 3, 5, 6, 2, 4, 3, 1, 5, 6, 3, 1, 2, 4, 5, 6, 4, 3, 1, 2, 5,
    ],
    // @OH14
    [
        1, 2, 4, 5, 3, 6, 1, 3, 2, 4, 5, 6, 1, 4, 5, 3, 2, 6, 1, 5, 3, 2, 4, 6, 2, 1, 3, 6, 4, 5,
        2, 3, 6, 4, 1, 5, 2, 4, 1, 3, 6, 5, 2, 6, 4, 1, 3, 5, 3, 1, 5, 6, 2, 4, 3, 2, 1, 5, 6, 4,
        3, 5, 6, 2, 1, 4, 3, 6, 2, 1, 5, 4, 4, 1, 2, 6, 5, 3, 4, 2, 6, 5, 1, 3, 4, 5, 1, 2, 6, 3,
        4, 6, 5, 1, 2, 3, 5, 1, 4, 6, 3, 2, 5, 3, 1, 4, 6, 2, 5, 4, 6, 3, 1, 2, 5, 6, 3, 1, 4, 2,
        6, 2, 3, 5, 4, 1, 6, 3, 5, 4, 2, 1, 6, 4, 2, 3, 5, 1, 6, 5, 4, 2, 3, 1,
    ],
    // @OH15
    [
        1, 2, 4, 6, 3, 5, 1, 3, 2, 4, 6, 5, 1, 4, 6, 3, 2, 5, 1, 6, 3, 2, 4, 5, 2, 1, 3, 5, 4, 6,
        2, 3, 5, 4, 1, 6, 2, 4, 1, 3, 5, 6, 2, 5, 4, 1, 3, 6, 3, 1, 6, 5, 2, 4, 3, 2, 1, 6, 5, 4,
        3, 5, 2, 1, 6, 4, 3, 6, 5, 2, 1, 4, 4, 1, 2, 5, 6, 3, 4, 2, 5, 6, 1, 3, 4, 5, 6, 1, 2, 3,
        4, 6, 1, 2, 5, 3, 5, 2, 3, 6, 4, 1, 5, 3, 6, 4, 2, 1, 5, 4, 2, 3, 6, 1, 5, 6, 4, 2, 3, 1,
        6, 1, 4, 5, 3, 2, 6, 3, 1, 4, 5, 2, 6, 4, 5, 3, 1, 2, 6, 5, 3, 1, 4, 2,
    ],
    // @OH16
    [
        1, 2, 6, 4, 3, 5, 1, 3, 2, 6, 4, 5, 1, 4, 3, 2, 6, 5, 1, 6, 4, 3, 2, 5, 2, 1, 3, 5, 6, 4,
        2, 3, 5, 6, 1, 4, 2, 5, 6, 1, 3, 4, 2, 6, 1, 3, 5, 4, 3, 1, 4, 5, 2, 6, 3, 2, 1, 4, 5, 6,
        3, 4, 5, 2, 1, 6, 3, 5, 2, 1, 4, 6, 4, 1, 6, 5, 3, 2, 4, 3, 1, 6, 5, 2, 4, 5, 3, 1, 6, 2,
        4, 6, 5, 3, 1, 2, 5, 2, 3, 4, 6, 1, 5, 3, 4, 6, 2, 1, 5, 4, 6, 2, 3, 1, 5, 6, 2, 3, 4, 1,
        6, 1, 2, 5, 4, 3, 6, 2, 5, 4, 1, 3, 6, 4, 1, 2, 5, 3, 6, 5, 4, 1, 2, 3,
    ],
    // @OH17
    [
        1, 2, 5, 6, 3, 4, 1, 3, 2, 5, 6, 4, 1, 5, 6, 3, 2, 4, 1, 6, 3, 2, 5, 4, 2, 1, 3, 4, 5, 6,
        2, 3, 4, 5, 1, 6, 2, 4, 5, 1, 3, 6, 2, 5, 1, 3, 4, 6, 3, 1, 6, 4, 2, 5, 3, 2, 1, 6, 4, 5,
        3, 4, 2, 1, 6, 5, 3, 6, 4, 2, 1, 5, 4, 2, 3, 6, 5, 1, 4, 3, 6, 5, 2, 1, 4, 5, 2, 3, 6, 1,
        4, 6, 5, 2, 3, 1, 5, 1, 2, 4, 6, 3, 5, 2, 4, 6, 1, 3, 5, 4, 6, 1, 2, 3, 5, 6, 1, 2, 4, 3,
        6, 1, 5, 4, 3, 2, 6, 3, 1, 5, 4, 2, 6, 4, 3, 1, 5, 2, 6, 5, 4, 3, 1, 2,
    ],
    // @OH18
    [
        1, 2, 6, 5, 3, 4, 1, 3, 2, 6, 5, 4, 1, 5, 3, 2, 6, 4, 1, 6, 5, 3, 2, 4, 2, 1, 3, 4, 6, 5,
        2, 3, 4, 6, 1, 5, 2, 4, 6, 1, 3, 5, 2, 6, 1, 3, 4, 5, 3, 1, 5, 4, 2, 6, 3, 2, 1, 5, 4, 6,
        3, 4, 2, 1, 5, 6, 3, 5, 4, 2, 1, 6, 4, 2, 3, 5, 6, 1, 4, 3, 5, 6, 2, 1, 4, 5, 6, 2, 3, 1,
        4, 6, 2, 3, 5, 1, 5, 1, 6, 4, 3, 2, 5, 3, 1, 6, 4, 2, 5, 4, 3, 1, 6, 2, 5, 6, 4, 3, 1, 2,
        6, 1, 2, 4, 5, 3, 6, 2, 4, 5, 1, 3, 6, 4, 5, 1, 2, 3, 6, 5, 1, 2, 4, 3,
    ],
    // @OH19
    [
        1, 2, 4, 5, 6, 3, 1, 4, 5, 6, 2, 3, 1, 5, 6, 2, 4, 3, 1, 6, 2, 4, 5, 3, 2, 1, 6, 3, 4, 5,
        2, 3, 4, 1, 6, 5, 2, 4, 1, 6, 3, 5, 2, 6, 3, 4, 1, 5, 3, 2, 6, 5, 4, 1, 3, 4, 2, 6, 5, 1,
        3, 5, 4, 2, 6, 1, 3, 6, 5, 4, 2, 1, 4, 1, 2, 3, 5, 6, 4, 2, 3, 5, 1, 6, 4, 3, 5, 1, 2, 6,
        4, 5, 1, 2, 3, 6, 5, 1, 4, 3, 6, 2, 5, 3, 6, 1, 4, 2, 5, 4, 3, 6, 1, 2, 5, 6, 1, 4, 3, 2,
        6, 1, 5, 3, 2, 4, 6, 2, 1, 5, 3, 4, 6, 3, 2, 1, 5, 4, 6, 5, 3, 2, 1, 4,
    ],
    // @OH20
    [
        1, 2, 4, 6, 5, 3, 1, 4, 6, 5, 2, 3, 1, 5, 2, 4, 6, 3, 1, 6, 5, 2, 4, 3, 2, 1, 5, 3, 4, 6,
        2, 3, 4, 1, 5, 6, 2, 4, 1, 5, 3, 6, 2, 5, 3, 4, 1, 6, 3, 2, 5, 6, 4, 1, 3, 4, 2, 5, 6, 1,
        3, 5, 6, 4, 2, 1, 3, 6, 4, 2, 5, 1, 4, 1, 2, 3, 6, 5, 4, 2, 3, 6, 1, 5, 4, 3, 6, 1, 2, 5,
        4, 6, 1, 2, 3, 5, 5, 1, 6, 3, 2, 4, 5, 2, 1, 6, 3, 4, 5, 3, 2, 1, 6, 4, 5, 6, 3, 2, 1, 4,
        6, 1, 4, 3, 5, 2, 6, 3, 5, 1, 4, 2, 6, 4, 3, 5, 1, 2, 6, 5, 1, 4, 3, 2,
    ],
    // @OH21
    [
        1, 2, 5, 4, 6, 3, 1, 4, 6, 2, 5, 3, 1, 5, 4, 6, 2, 3, 1, 6, 2, 5, 4, 3, 2, 1, 6, 3, 5, 4,
        2, 3, 5, 1, 6, 4, 2, 5, 1, 6, 3, 4, 2, 6, 3, 5, 1, 4, 3, 2, 6, 4, 5, 1, 3, 4, 5, 2, 6, 1,
        3, 5, 2, 6, 4, 1, 3, 6, 4, 5, 2, 1, 4, 1, 5, 3, 6, 2, 4, 3, 6, 1, 5, 2, 4, 5, 3, 6, 1, 2,
        4, 6, 1, 5, 3, 2, 5, 1, 2, 3, 4, 6, 5, 2, 3, 4, 1, 6, 5, 3, 4, 1, 2, 6, 5, 4, 1, 2, 3, 6,
        6, 1, 4, 3, 2, 5, 6, 2, 1, 4, 3, 5, 6, 3, 2, 1, 4, 5, 6, 4, 3, 2, 1, 5,
    ],
    // @OH22
    [
        1, 2, 6, 4, 5, 3, 1, 4, 5, 2, 6, 3, 1, 5, 2, 6, 4, 3, 1, 6, 4, 5, 2, 3, 2, 1, 5, 3, 6, 4,
        2, 3, 6, 1, 5, 4, 2, 5, 3, 6, 1, 4, 2, 6, 1, 5, 3, 4, 3, 2, 5, 4, 6, 1, 3, 4, 6, 2, 5, 1,
        3, 5, 4, 6, 2, 1, 3, 6, 2, 5, 4, 1, 4, 1, 6, 3, 5, 2, 4, 3, 5, 1, 6, 2, 4, 5, 1, 6, 3, 2,
        4, 6, 3, 5, 1, 2, 5, 1, 4, 3, 2, 6, 5, 2, 1, 4, 3, 6, 5, 3, 2, 1, 4, 6, 5, 4, 3, 2, 1, 6,
        6, 1, 2, 3, 4, 5, 6, 2, 3, 4, 1, 5, 6, 3, 4, 1, 2, 5, 6, 4, 1, 2, 3, 5,
    ],
    // @OH23
    [
        1, 2, 5, 6, 4, 3, 1, 4, 2, 5, 6, 3, 1, 5, 6, 4, 2, 3, 1, 6, 4, 2, 5, 3, 2, 1, 4, 3, 5, 6,
        2, 3, 5, 1, 4, 6, 2, 4, 3, 5, 1, 6, 2, 5, 1, 4, 3, 6, 3, 2, 4, 6, 5, 1, 3, 4, 6, 5, 2, 1,
        3, 5, 2, 4, 6, 1, 3, 6, 5, 2, 4, 1, 4, 1, 6, 3, 2, 5, 4, 2, 1, 6, 3, 5, 4, 3, 2, 1, 6, 5,
        4, 6, 3, 2, 1, 5, 5, 1, 2, 3, 6, 4, 5, 2, 3, 6, 1, 4, 5, 3, 6, 1, 2, 4, 5, 6, 1, 2, 3, 4,
        6, 1, 5, 3, 4, 2, 6, 3, 4, 1, 5, 2, 6, 4, 1, 5, 3, 2, 6, 5, 3, 4, 1, 2,
    ],
    // @OH24
    [
        1, 2, 6, 5, 4, 3, 1, 4, 2, 6, 5, 3, 1, 5, 4, 2, 6, 3, 1, 6, 5, 4, 2, 3, 2, 1, 4, 3, 6, 5,
        2, 3, 6, 1, 4, 5, 2, 4, 3, 6, 1, 5, 2, 6, 1, 4, 3, 5, 3, 2, 4, 5, 6, 1, 3, 4, 5, 6, 2, 1,
        3, 5, 6, 2, 4, 1, 3, 6, 2, 4, 5, 1, 4, 1, 5, 3, 2, 6, 4, 2, 1, 5, 3, 6, 4, 3, 2, 1, 5, 6,
        4, 5, 3, 2, 1, 6, 5, 1, 6, 3, 4, 2, 5, 3, 4, 1, 6, 2, 5, 4, 1, 6, 3, 2, 5, 6, 3, 4, 1, 2,
        6, 1, 2, 3, 5, 4, 6, 2, 3, 5, 1, 4, 6, 3, 5, 1, 2, 4, 6, 5, 1, 2, 3, 4,
    ],
    // @OH25
    [
        1, 3, 4, 5, 6, 2, 1, 4, 5, 6, 3, 2, 1, 5, 6, 3, 4, 2, 1, 6, 3, 4, 5, 2, 2, 3, 6, 5, 4, 1,
        2, 4, 3, 6, 5, 1, 2, 5, 4, 3, 6, 1, 2, 6, 5, 4, 3, 1, 3, 1, 6, 2, 4, 5, 3, 2, 4, 1, 6, 5,
        3, 4, 1, 6, 2, 5, 3, 6, 2, 4, 1, 5, 4, 1, 3, 2, 5, 6, 4, 2, 5, 1, 3, 6, 4, 3, 2, 5, 1, 6,
        4, 5, 1, 3, 2, 6, 5, 1, 4, 2, 6, 3, 5, 2, 6, 1, 4, 3, 5, 4, 2, 6, 1, 3, 5, 6, 1, 4, 2, 3,
        6, 1, 5, 2, 3, 4, 6, 2, 3, 1, 5, 4, 6, 3, 1, 5, 2, 4, 6, 5, 2, 3, 1, 4,
    ],
    // @OH26
    [
        1, 3, 4, 6, 5, 2, 1, 4, 6, 5, 3, 2, 1, 5, 3, 4, 6, 2, 1, 6, 5, 3, 4, 2, 2, 3, 5, 6, 4, 1,
        2, 4, 3, 5, 6, 1, 2, 5, 6, 4, 3, 1, 2, 6, 4, 3, 5, 1, 3, 1, 5, 2, 4, 6, 3, 2, 4, 1, 5, 6,
        3, 4, 1, 5, 2, 6, 3, 5, 2, 4, 1, 6, 4, 1, 3, 2, 6, 5, 4, 2, 6, 1, 3, 5, 4, 3, 2, 6, 1, 5,
        4, 6, 1, 3, 2, 5, 5, 1, 6, 2, 3, 4, 5, 2, 3, 1, 6, 4, 5, 3, 1, 6, 2, 4, 5, 6, 2, 3, 1, 4,
        6, 1, 4, 2, 5, 3, 6, 2, 5, 1, 4, 3, 6, 4, 2, 5, 1, 3, 6, 5, 1, 4, 2, 3,
    ],
    // @OH27
    [
        1, 3, 5, 4, 6, 2, 1, 4, 6, 3, 5, 2, 1, 5, 4, 6, 3, 2, 1, 6, 3, 5, 4, 2, 2, 3, 6, 4, 5, 1,
        2, 4, 5, 3, 6, 1, 2, 5, 3, 6, 4, 1, 2, 6, 4, 5, 3, 1, 3, 1, 6, 2, 5, 4, 3, 2, 5, 1, 6, 4,
        3, 5, 1, 6, 2, 4, 3, 6, 2, 5, 1, 4, 4, 1, 5, 2, 6, 3, 4, 2, 6, 1, 5, 3, 4, 5, 2, 6, 1, 3,
        4, 6, 1, 5, 2, 3, 5, 1, 3, 2, 4, 6, 5, 2, 4, 1, 3, 6, 5, 3, 2, 4, 1, 6, 5, 4, 1, 3, 2, 6,
        6, 1, 4, 2, 3, 5, 6, 2, 3, 1, 4, 5, 6, 3, 1, 4, 2, 5, 6, 4, 2, 3, 1, 5,
    ],
    // @OH28
    [
        1, 3, 6, 4, 5, 2, 1, 4, 5, 3, 6, 2, 1, 5, 3, 6, 4, 2, 1, 6, 4, 5, 3, 2, 2, 3, 5, 4, 6, 1,
        2, 4, 6, 3, 5, 1, 2, 5, 4, 6, 3, 1, 2, 6, 3, 5, 4, 1, 3, 1, 5, 2, 6, 4, 3, 2, 6, 1, 5, 4,
        3, 5, 2, 6, 1, 4, 3, 6, 1, 5, 2, 4, 4, 1, 6, 2, 5, 3, 4, 2, 5, 1, 6, 3, 4, 5, 1, 6, 2, 3,
        4, 6, 2, 5, 1, 3, 5, 1, 4, 2, 3, 6, 5, 2, 3, 1, 4, 6, 5, 3, 1, 4, 2, 6, 5, 4, 2, 3, 1, 6,
        6, 1, 3, 2, 4, 5, 6, 2, 4, 1, 3, 5, 6, 3, 2, 4, 1, 5, 6, 4, 1, 3, 2, 5,
    ],
    // @OH29
    [
        1, 3, 5, 6, 4, 2, 1, 4, 3, 5, 6, 2, 1, 5, 6, 4, 3, 2, 1, 6, 4, 3, 5, 2, 2, 3, 4, 6, 5, 1,
        2, 4, 6, 5, 3, 1, 2, 5, 3, 4, 6, 1, 2, 6, 5, 3, 4, 1, 3, 1, 4, 2, 5, 6, 3, 2, 5, 1, 4, 6,
        3, 4, 2, 5, 1, 6, 3, 5, 1, 4, 2, 6, 4, 1, 6, 2, 3, 5, 4, 2, 3, 1, 6, 5, 4, 3, 1, 6, 2, 5,
        4, 6, 2, 3, 1, 5, 5, 1, 3, 2, 6, 4, 5, 2, 6, 1, 3, 4, 5, 3, 2, 6, 1, 4, 5, 6, 1, 3, 2, 4,
        6, 1, 5, 2, 4, 3, 6, 2, 4, 1, 5, 3, 6, 4, 1, 5, 2, 3, 6, 5, 2, 4, 1, 3,
    ],
    // @OH30
    [
        1, 3, 6, 5, 4, 2, 1, 4, 3, 6, 5, 2, 1, 5, 4, 3, 6, 2, 1, 6, 5, 4, 3, 2, 2, 3, 4, 5, 6, 1,
        2, 4, 5, 6, 3, 1, 2, 5, 6, 3, 4, 1, 2, 6, 3, 4, 5, 1, 3, 1, 4, 2, 6, 5, 3, 2, 6, 1, 4, 5,
        3, 4, 2, 6, 1, 5, 3, 6, 1, 4, 2, 5, 4, 1, 5, 2, 3, 6, 4, 2, 3, 1, 5, 6, 4, 3, 1, 5, 2, 6,
        4, 5, 2, 3, 1, 6, 5, 1, 6, 2, 4, 3, 5, 2, 4, 1, 6, 3, 5, 4, 1, 6, 2, 3, 5, 6, 2, 4, 1, 3,
        6, 1, 3, 2, 5, 4, 6, 2, 5, 1, 3, 4, 6, 3, 2, 5, 1, 4, 6, 5, 1, 3, 2, 4,
    ],
];

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
    fn order_by(&self, ranks: &Vec<i8>) -> Option<Box<dyn Topology>>;
    fn configuration_of(&self, ranks: &Vec<i8>) -> Result<Configuration, RuatomError> {
        let topology = self.order_by(ranks);
        return match topology {
            None => Ok(UNKNOWN),
            Some(t) => t.configuration(),
        };
    }
    fn parity(&self, atoms: Vec<u8>, ranks: Vec<u8>) -> i8 {
        let mut count = 0;
        for (ix, atom) in atoms.iter().enumerate() {
            let mut j = ix + 1;
            while j < atoms.len() {
                if ranks[*atom as usize] < ranks[atoms[j] as usize] {
                    count += 1;
                }
                j += 1;
            }
        }
        if count & 0x1 == 1 {
            return -1;
        }
        return 1;
    }

    fn parity4(&self, atoms: &Vec<i8>, ranks: &Vec<i8>) -> i8 {
        let mut count = 0;
        let mut ix = 0;
        while ix < 4 {
            let mut jx = ix + 1;
            while jx < 4 {
                if ranks[atoms[jx] as usize] < ranks[atoms[jx] as usize] {
                    count += 1;
                }
                jx += 1;
            }
            ix += 1;
        }
        if count & 0x1 == 1 {
            return -1;
        }
        return 1;
    }

    fn sort(&self, atoms: &mut Vec<i8>, ranks: &Vec<i8>) {
        let mut ix = 0;
        let mut jx = 0;
        let leg = atoms.len() - 1;
        let mut is_breaked = false;
        while ix < leg {
            let v = atoms[ix + 1];
            while ranks[v as usize] < ranks[atoms[jx] as usize] {
                atoms[jx + 1] = atoms[jx];
                if jx == 0 {
                    is_breaked = true;
                    break;
                }
                jx -= 1;
            }
            if is_breaked {
                is_breaked = false;
                atoms[jx] = v;
            } else {
                atoms[jx + 1] = v;
            }
            ix += 1;
            jx = ix;
        }
    }

    fn apply_inv(&self, src: &Vec<i8>, perm: &[i8]) -> Vec<i8> {
        let mut ix = 0;
        let mut res = Vec::new();
        while ix < src.len() {
            res[ix] = src[perm[ix] as usize];
            ix += 1;
        }
        return res;
    }

    fn indirect_sort(&self, dst: &mut Vec<i8>, rank: &Vec<i8>) {
        let mut ix = 0;
        while ix < dst.len() {
            let mut jx = ix;
            while jx > 0 && rank[dst[jx - 1] as usize] > rank[dst[jx] as usize] {
                dst.swap(jx, jx - 1);
                if jx == 0 {
                    break;
                }
                jx -= 1;
            }
            ix += 1;
        }
    }

    fn check(&self, dst: &Vec<i8>, src: &Vec<i8>, perm: &[i8], step: usize, skip: usize) -> bool {
        let mut ix = 0;
        while ix < perm.len() {
            let mut jx = 0;
            while jx < step {
                if dst[perm[ix + jx] as usize] != src[jx] {
                    break;
                }
                jx += 1;
            }
            if jx == 0 {
                ix += step * skip
            } else if jx == step {
                return true;
            } else {
                ix += step;
            }
        }
        return false;
    }
}

#[derive(PartialEq, Eq, Clone)]
struct BaseTopology {
    u: u8,
    p: i8,
    vs: Vec<i8>,
}

impl BaseTopology {
    fn new(u: u8, p: i8, vs: Vec<i8>) -> Self {
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
        let p;
        if conf.is_anti_clockwise() {
            p = -1;
        } else {
            p = 1;
        }
        Ok(Self(BaseTopology::new(u, p, vs)))
    }

    fn configuration(&self) -> Result<Configuration, RuatomError> {
        if self.0.p == -1 {
            return Ok(TH1);
        } else if self.0.p == 1 {
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
    fn order_by(&self, ranks: &Vec<i8>) -> Option<Box<dyn Topology>> {
        let mut ams = self.0.vs.clone();
        self.sort(&mut ams, ranks);
        return Some(Box::new(Self(BaseTopology::new(
            self.0.u,
            self.0.p * self.parity4(&self.0.vs, ranks),
            ams,
        ))));
    }

    fn configuration_of(&self, ranks: &Vec<i8>) -> Result<Configuration, RuatomError> {
        let c = self.0.p * self.parity4(&self.0.vs, ranks);
        if c < 0 {
            return Ok(TH1);
        } else {
            return Ok(TH2);
        }
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
        let p;
        if conf.is_anti_clockwise() {
            p = -1;
        } else {
            p = 1;
        }
        Ok(Self(BaseTopology::new(u, p, vs)))
    }

    fn configuration(&self) -> Result<Configuration, RuatomError> {
        if self.0.p == -1 {
            return Ok(DB1);
        } else if self.0.p == 1 {
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
    fn order_by(&self, ranks: &Vec<i8>) -> Option<Box<dyn Topology>> {
        let mut ams = self.0.vs.clone();
        self.sort(&mut ams, ranks);
        return Some(Box::new(Self(BaseTopology::new(
            self.0.u,
            self.0.p * self.parity4(&self.0.vs, ranks),
            ams,
        ))));
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
        let p;
        if conf.is_anti_clockwise() {
            p = -1;
        } else {
            p = 1;
        }
        Ok(Self(BaseTopology::new(u, p, vs)))
    }

    fn configuration(&self) -> Result<Configuration, RuatomError> {
        if self.0.p == -1 {
            return Ok(AL1);
        } else if self.0.p == 1 {
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

    fn order_by(&self, ranks: &Vec<i8>) -> Option<Box<dyn Topology>> {
        let mut ams = self.0.vs.clone();
        self.sort(&mut ams, ranks);
        return Some(Box::new(Self(BaseTopology::new(
            self.0.u,
            self.0.p * self.parity4(&self.0.vs, ranks),
            ams,
        ))));
    }
}

pub struct SquarePlanar(BaseTopology, u8);

impl Topology for SquarePlanar {
    fn new_topology(u: u8, conf: Configuration, vs: Vec<i8>) -> Result<Self, RuatomError> {
        if !conf.is_square_plannar() {
            return Err(RuatomError::IllegalMolecule(
                "invalid SquarePlanar configuration",
            ));
        }
        let p;
        if conf.is_anti_clockwise() {
            p = -1;
        } else {
            p = 1;
        }
        Ok(Self(BaseTopology::new(u, p, vs), conf.seq()))
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

    fn order_by(&self, ranks: &Vec<i8>) -> Option<Box<dyn Topology>> {
        if self.1 < 1 || self.1 > 20 {
            return None;
        }
        let src = self.apply_inv(
            &self.0.vs,
            SQUAREPLANARPERMUTATIONS[(self.1 - 1) as usize].as_slice(),
        );
        let mut dst = src.clone();
        self.indirect_sort(&mut dst, ranks);
        let mut ix = 0;
        while ix < 4 {
            if self.check(
                &dst,
                &src,
                SQUAREPLANARPERMUTATIONS[ix - 1].as_slice(),
                4,
                2,
            ) {
                return Some(Box::new(Self(
                    BaseTopology::new(self.0.u, self.0.p, self.0.vs.clone()),
                    self.1,
                )));
            }
            ix += 1;
        }
        return None;
    }
}

pub struct TrigonalBipyramidal(BaseTopology, u8);

impl Topology for TrigonalBipyramidal {
    fn new_topology(u: u8, conf: Configuration, vs: Vec<i8>) -> Result<Self, RuatomError> {
        if conf.seq() < 1 && conf.seq() > 20 {
            return Err(RuatomError::IllegalMolecule(
                "invalid TrigonalBipyramidal configuration",
            ));
        }
        let p;
        if conf.is_anti_clockwise() {
            p = -1;
        } else {
            p = 1;
        }
        Ok(Self(BaseTopology::new(u, p, vs), conf.seq()))
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

    fn order_by(&self, ranks: &Vec<i8>) -> Option<Box<dyn Topology>> {
        let src = self.apply_inv(
            &self.0.vs,
            TRIGONALBIPYRAMIDALPERMUTATIONS[(self.1 - 1) as usize].as_slice(),
        );
        let mut dst = src.clone();
        self.indirect_sort(&mut dst, ranks);
        let mut ix = 0;
        while ix < 20 {
            if self.check(
                &dst,
                &src,
                TRIGONALBIPYRAMIDALPERMUTATIONS[ix - 1].as_slice(),
                5,
                3,
            ) {
                return Some(Box::new(Self(
                    BaseTopology::new(self.0.u, self.0.p, self.0.vs.clone()),
                    self.1,
                )));
            }
            ix += 1;
        }
        return None;
    }
}

pub struct Octahedral(BaseTopology, u8);

impl Topology for Octahedral {
    fn new_topology(u: u8, conf: Configuration, vs: Vec<i8>) -> Result<Self, RuatomError> {
        if conf.seq() < 1 && conf.seq() > 30 {
            return Err(RuatomError::IllegalMolecule(
                "invalid Octahedral configuration",
            ));
        }
        let p;
        if conf.is_anti_clockwise() {
            p = -1;
        } else {
            p = 1;
        }
        Ok(Self(BaseTopology::new(u, p, vs), conf.seq()))
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

    fn order_by(&self, ranks: &Vec<i8>) -> Option<Box<dyn Topology>> {
        let src = self.apply_inv(
            &self.0.vs,
            OCTAHEDRALPERMUTATIONS[(self.1 - 1) as usize].as_slice(),
        );
        let mut dst = src.clone();
        self.indirect_sort(&mut dst, ranks);
        let mut ix = 0;
        while ix < 30 {
            if self.check(&dst, &src, OCTAHEDRALPERMUTATIONS[ix - 1].as_slice(), 6, 4) {
                return Some(Box::new(Self(
                    BaseTopology::new(self.0.u, self.0.p, self.0.vs.clone()),
                    self.1,
                )));
            }
            ix += 1;
        }
        return None;
    }
}

pub struct UnknownTopology(u8);

impl Topology for UnknownTopology {
    fn new_topology(u: u8, _conf: Configuration, _vs: Vec<i8>) -> Result<Self, RuatomError> {
        Ok(Self(u))
    }
    fn atom(&self) -> i8 {
        return self.0 as i8;
    }

    fn seq(&self) -> TopologySeq {
        TopologySeq::UnknownTopology
    }
    fn order_by(&self, _ranks: &Vec<i8>) -> Option<Box<dyn Topology>> {
        return Some(Box::new(Self(self.0)));
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
