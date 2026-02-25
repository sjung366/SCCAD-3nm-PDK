

rM8_1 @= { @ "M8.1 : Vertical width of M8 >= 38 nm";
        internal1(aM8, < 0.038, extension = NONE, direction = VERTICAL);
}

rM8_2 @= { @ "M8.2 : Horizontal length of M8 >= 55 nm";
        internal1(aM8, < 0.055, extension = NONE, direction = HORIZONTAL);
}

rM8_3 @= { @ "M8.3 : Vertical spacing between M8 layers >= 38 nm";
        external1(aM8, < 0.038, extension = NONE, direction = VERTICAL);
}

rM8_4 @= { @ "M8.4 : Horizontal (TtT) spacing between M8 layers on same track >= 55 nm";
        external1(aM8, < 0.055, extension = NONE, direction = HORIZONTAL);
}

rM8_5 @= { @ "M8.5 : M8 corner-to-corner spacing >= 45 nm";
        external_corner1( aM8, distance < 0.045, type = CONVEX_TO_CONVEX);
}

rM8_6 @= { @ "M8.6 : M8 must not bend";
        not_rectangles(aM8);
}
