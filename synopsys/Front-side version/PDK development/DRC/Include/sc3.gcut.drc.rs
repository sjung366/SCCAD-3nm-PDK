

rGCUT_1 @= { @ "GCUT.1 : GCUT vertical width >= 10 nm";
        internal1( aGCUT, distance < 0.010, extension = NONE, direction = VERTICAL);
}

rGCUT_2 @= { @ "GCUT.2 : GCUT horizontal length >= 42 nm";
        internal1( aGCUT, distance < 0.042, extension = NONE, direction = HORIZONTAL);
}

// GCUT.3 GCUT horizontal spacing

// GCUT.4 GCUT vertical spacing

rGCUT_5 @= { @ "GCUT.5 : GCUT may not bend";
        not_rectangles( aGCUT);
}

rGCUT_6 @= { @ "GCUT.6 : GCUT may not exist without GATE";
        not_interacting( aGCUT, aGATE, include_touch = NONE);
}

rGCUT_7 @= { @ "GCUT.7 : GCUT vertical edge may not lie inside or coincide with GATE vertical edge";
        gGCUT_edge = angle_edge( aGCUT, angles = {90.0});
        gGATE1 = aGATE not aDUMMY;
        interacting_edge( gGCUT_edge, gGATE1);
}

rGCUT_8 @= { @ "GCUT.8 : GCUT may not interact with ACT";
        interacting( aGCUT, aACT, include_touch = ALL);
}

rGCUT_9 @= { @ "GCUT.ACT.1 : Vertical spacing between GCUT and ACT >= 10 nm";
        external2( aGCUT, aACT, distance < 0.010, extension = NONE, direction = VERTICAL);
}

