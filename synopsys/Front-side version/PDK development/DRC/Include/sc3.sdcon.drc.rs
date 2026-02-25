
rSDCON_1 @= { @ "SDCON.1 : Horizontal width of SDCON = 15 nm";
        gSDCON1_check = internal1( aSDCON, == 0.015, extension = NONE, direction = HORIZONTAL);
        aSDCON not gSDCON1_check;
}

// only for SDCON on different net, exceoptions for BPR connection to VDD/VSS
rSDCON_2 @= { @ "SDCON.2 : Vertical (TtT) spacing between SDCON >= 20 nm";
        external1( aSDCON, < 0.020, extension = NONE, direction = VERTICAL, connectivity = DIFFERENT_NET, connect_sequence = CONNECT_DB );
}

rSDCON_3 @= { @ "SDCON.3 : SDCON may not bend";
        not_rectangles( aSDCON);
}

rSDCON_4 @= { @ "SDCON.ACT.1 : Vertical extension of SDCON past ACT >= 5 nm";
        enclose( aACT, aSDCON, < 0.005, extension = NONE, direction = VERTICAL);
}

// checking only for < 6nm
rSDCON_5 @= { @ "SDCON.GATE.1 : Horizonal spacing between SDCON and GATE = 6 nm";
        external2( aSDCON, aGATE, < 0.006, extension = NONE, direction = HORIZONTAL);
        //gSDCON5_check = external2( aSDCON, aGATE, == 0.006, extension = NONE, direction = HORIZONTAL);
        //aSDCON not gSDCON5_check;
}
