

rV6_1 @= { @ "V6.1 : Horizontal width of V6 >= 21 nm";
       internal1( aV6, < 0.021, extension = NONE, direction = HORIZONTAL);
}

rV6_2 @= { @ "V6.2 : Vertical height of V6 >= 38 nm";
       internal1( aV6, < 0.038, extension = NONE, direction = VERTICAL);
}

// not_inside_touching_edge + angle_edge
rV6_3 @= { @ "V6.3 : Vertical edges of V6 should align with M5";
        gV6_vert_edges = angle_edge( aV6, angles = 90);
        not_inside_touching_edge( gV6_vert_edges, aM5);
}

rV6_4 @= { @ "V6.4 : Horizontal edges of V6 should align with M6";
        gV6_horz_edges = angle_edge( aV6, angles = 0);
        not_inside_touching_edge( gV6_horz_edges, aM6);
}

// V6.5 minimum spacing between V6

// V6.6 minimum corner-to-corner spacing
rV6_6 @= { @ "V6.6 : V6 corner-to-corner spacing >= 27 nm";
        external_corner1( aV6, distance < 0.027, type = CONVEX_TO_CONVEX);
}

rV6_7 @= { @ "V6.7 : V6 must interact with M5";
        not_interacting( aV6, aM5, include_touch = NONE);
}

rV6_8 @= { @ "V6.8 : V6 must interact with M6";
        not_interacting( aV6, aM6, include_touch = NONE);
}


rV6_9 @= { @ "V6.M5.EN : V6 enclosure by M5 on opposite sides in the vertical direction >= 6 nm";
        enclose( aV6, aM5, < 0.006, extension = NONE, direction = VERTICAL);
}

rV6_10 @= { @ "V6.M6.EN : V6 enclosure by M6 on opposite sides in the horizontal direction >= 6 nm";
        enclose( aV6, aM6, < 0.006, extension = NONE, direction = HORIZONTAL);
}
