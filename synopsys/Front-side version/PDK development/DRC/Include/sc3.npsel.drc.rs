

rNSEL_1 @={ @ "NSEL.1 : Minimum vertical width of NSEL >= 72 nm";
        internal1( aNSEL, distance < 0.072, extension = NONE, direction = VERTICAL);
}

rNSEL_2 @={ @ "NSEL.2 : Minimum horizontal width of NSEL >= 84 nm";
        internal1( aNSEL, distance < 0.084, extension = NONE, direction = HORIZONTAL);
}

rNSEL_3 @={ @ "NSEL.3 : Minimum enclosure of ACT by NSEL in the vertical direction >= 15 nm";
        enclose( aACT, aNSEL, distance < 0.015, extension = NONE, direction = VERTICAL);
}


rPSEL_1 @={ @ "PSEL.1 : Minimum vertical width of PSEL >= 72 nm";
        internal1( aPSEL, distance < 0.072, extension = NONE, direction = VERTICAL);
}

rPSEL_2 @={ @ "PSEL.2 : Minimum horizontal width of PSEL >= 84 nm";
        internal1( aPSEL, distance < 0.084, extension = NONE, direction = HORIZONTAL);
}

rPSEL_3 @={ @ "PSEL.3 : Minimum enclosure of ACT by PSEL in the vertical direction >= 15 nm";
        enclose( aACT, aPSEL, distance < 0.015, extension = NONE, direction = VERTICAL);
}


rNSEL_PSEL_1 @= { @ "NSEL.PSEL.1 : NSEL and PSEL should not overlap";
        interacting( aNSEL, aPSEL, include_touch = NONE);
}
