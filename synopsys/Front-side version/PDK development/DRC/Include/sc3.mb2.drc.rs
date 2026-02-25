
rMB2_1 @= { @ "MB2.1 : Vertical width of MB2 >= 38 nm";
        internal1(aMB2, < 0.038, extension = NONE, direction = VERTICAL);
}

rMB2_2 @= { @ "MB2.2 : Horizontal length of MB2 >= 55 nm";
        internal1(aMB2, < 0.055, extension = NONE, direction = HORIZONTAL);
}

rMB2_3 @= { @ "MB2.3 : Vertical spacing between MB2 layers >= 38 nm";
        external1(aMB2, < 0.038, extension = NONE, direction = VERTICAL);
}

rMB2_4 @= { @ "MB2.4 : Horizontal (TtT) spacing between MB2 layers on same track >= 55 nm";
        external1(aMB2, < 0.055, extension = NONE, direction = HORIZONTAL);
}

rMB2_5 @= { @ "MB2.5 : MB2 corner-to-corner spacing >= 45 nm";
        external_corner1( aMB2, distance < 0.045, type = CONVEX_TO_CONVEX);
}

rMB2_6 @= { @ "MB2.6 : MB2 must not bend";
        not_rectangles(aMB2);
}
