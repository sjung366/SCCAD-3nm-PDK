
rM9_1 @= { @ "M9.1 : Vertical width of M9 >= 38 nm";
        internal1(aM9, < 0.038, extension = NONE, direction = VERTICAL);
}

rM9_2 @= { @ "M9.2 : Horizontal length of M9 >= 55 nm";
        internal1(aM9, < 0.055, extension = NONE, direction = HORIZONTAL);
}

rM9_3 @= { @ "M9.3 : Vertical spacing between M9 layers >= 38 nm";
        external1(aM9, < 0.038, extension = NONE, direction = VERTICAL);
}

rM9_4 @= { @ "M9.4 : Horizontal (TtT) spacing between M9 layers on same track >= 55 nm";
        external1(aM9, < 0.055, extension = NONE, direction = HORIZONTAL);
}

rM9_5 @= { @ "M9.5 : M9 corner-to-corner spacing >= 45 nm";
        external_corner1( aM9, distance < 0.045, type = CONVEX_TO_CONVEX);
}

rM9_6 @= { @ "M9.6 : M9 must not bend";
        not_rectangles(aM9);
}
