
rM7_1 @= { @ "M7.1 : Vertical width of M7 >= 38 nm";
        internal1(aM7, < 0.038, extension = NONE, direction = VERTICAL);
}

rM7_2 @= { @ "M7.2 : Horizontal length of M7 >= 55 nm";
        internal1(aM7, < 0.055, extension = NONE, direction = HORIZONTAL);
}

rM7_3 @= { @ "M7.3 : Vertical spacing between M7 layers >= 38 nm";
        external1(aM7, < 0.038, extension = NONE, direction = VERTICAL);
}

rM7_4 @= { @ "M7.4 : Horizontal (TtT) spacing between M7 layers on same track >= 55 nm";
        external1(aM7, < 0.055, extension = NONE, direction = HORIZONTAL);
}

rM7_5 @= { @ "M7.5 : M7 corner-to-corner spacing >= 45 nm";
        external_corner1( aM7, distance < 0.045, type = CONVEX_TO_CONVEX);
}

rM7_6 @= { @ "M7.6 : M7 must not bend";
        not_rectangles(aM7);
}
