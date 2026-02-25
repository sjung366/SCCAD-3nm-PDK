
rDUMMY_1 @= { @ "DUMMY.1 : Exact horizontal width of DUMMY = 15 nm";
        gDUMMY1_check = internal1( aDUMMY, distance == 0.015, extension = NONE, direction = HORIZONTAL);
        aDUMMY not gDUMMY1_check;
}

rDUMMY_2 @= { @ "DUMMY.2 : DUMMY may not exist without GATE";
        not_interacting( aDUMMY, aGATE, include_touch = NONE);
}

rDUMMY_3 @= { @ "DUMMY.3 : DUMMY may not bend";
        not_rectangles( aDUMMY);
}

// exact overlap ensured if inside due to same width
rDUMMY_4 @= { @ "DUMMY.4 : DUMMY should exactly overlap GATE in the horizontal direction";
        not_inside( aDUMMY, aGATE, include_touch = ALL);
}

//rDUMMY_5 @= { @ "DUMMY.5 : DUMMY horizo"}

