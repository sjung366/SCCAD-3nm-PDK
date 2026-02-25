
rV7_1 @= { @ "V7.1 : Horizontal width of V7 >= 21 nm";
       internal1( aV7, < 0.021, extension = NONE, direction = HORIZONTAL);
}

rV7_2 @= { @ "V7.2 : Vertical height of V7 >= 38 nm";
       internal1( aV7, < 0.038, extension = NONE, direction = VERTICAL);
}

// not_inside_touching_edge + angle_edge
rV7_3 @= { @ "V7.3 : Vertical edges of V7 should align with M5";
        gV7_vert_edges = angle_edge( aV7, angles = 90);
        not_inside_touching_edge( gV7_vert_edges, aM5);
}

rV7_4 @= { @ "V7.4 : Horizontal edges of V7 should align with M6";
        gV7_horz_edges = angle_edge( aV7, angles = 0);
        not_inside_touching_edge( gV7_horz_edges, aM6);
}

// V7.5 minimum spacing between V7

// V7.6 minimum corner-to-corner spacing
rV7_6 @= { @ "V7.6 : V7 corner-to-corner spacing >= 27 nm";
        external_corner1( aV7, distance < 0.027, type = CONVEX_TO_CONVEX);
}

rV7_7 @= { @ "V7.7 : V7 must interact with M5";
        not_interacting( aV7, aM5, include_touch = NONE);
}

rV7_8 @= { @ "V7.8 : V7 must interact with M6";
        not_interacting( aV7, aM6, include_touch = NONE);
}


rV7_9 @= { @ "V7.M5.EN : V7 enclosure by M5 on opposite sides in the vertical direction >= 6 nm";
        enclose( aV7, aM5, < 0.006, extension = NONE, direction = VERTICAL);
}

rV7_10 @= { @ "V7.M6.EN : V7 enclosure by M6 on opposite sides in the horizontal direction >= 6 nm";
        enclose( aV7, aM6, < 0.006, extension = NONE, direction = HORIZONTAL);
}
