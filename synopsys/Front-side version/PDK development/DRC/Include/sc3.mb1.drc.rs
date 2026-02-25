
rMB1_1 @= { @ "MB1.1 : Vertical width of MB1 >= 38 nm";
        internal1(aMB1, < 0.038, extension = NONE, direction = VERTICAL);
}

rMB1_2 @= { @ "MB1.2 : Horizontal length of MB1 >= 55 nm";
        internal1(aMB1, < 0.055, extension = NONE, direction = HORIZONTAL);
}

rMB1_3 @= { @ "MB1.3 : Vertical spacing between MB1 layers >= 38 nm";
        external1(aMB1, < 0.038, extension = NONE, direction = VERTICAL);
}

rMB1_4 @= { @ "MB1.4 : Horizontal (TtT) spacing between MB1 layers on same track >= 55 nm";
        external1(aMB1, < 0.055, extension = NONE, direction = HORIZONTAL);
}

rMB1_5 @= { @ "MB1.5 : MB1 corner-to-corner spacing >= 45 nm";
        external_corner1( aMB1, distance < 0.045, type = CONVEX_TO_CONVEX);
}

rMB1_6 @= { @ "MB1.6 : MB1 must not bend";
        not_rectangles(aMB1);
}
