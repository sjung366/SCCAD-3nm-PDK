

rM1_1 @= { @ "M1.1 : Horizontal width of M1 >= 14 nm";
        internal1(aM1, < 0.014, extension = NONE, direction = HORIZONTAL);
}

rM1_2 @= { @ "M1.2 : Vertical length of M1 >= 28 nm";
        internal1(aM1, < 0.028, extension = NONE, direction = VERTICAL);
}

rM1_3 @= { @ "M1.3 : Horizontal spacing between M1 layers >= 14 nm";
        external1(aM1, < 0.014, extension = NONE, direction = HORIZONTAL);
}

rM1_4 @= { @ "M1.4 : Vertical (TtT) spacing between M1 layers on same track >= 20 nm";
        external1(aM1, < 0.020, extension = NONE, direction = VERTICAL);
}

// M1.5 Corner to corner spacing between M1 layers
rM1_5 @= { @ "M1.5 : M1 corner-to-corner spacing >= 16 nm";
        external_corner1( aM1, distance < 0.016, type = CONVEX_TO_CONVEX);
}

rM1_6 @= { @ "M1.6 : M1 must not bend";
        not_rectangles(aM1);
}
