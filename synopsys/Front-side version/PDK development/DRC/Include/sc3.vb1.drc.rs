

rVB1_1 @= { @ "VB1.1 : Horizontal width of VB1 >= 21 nm";
       internal1( aVB1, < 0.021, extension = NONE, direction = HORIZONTAL);
}

rVB1_2 @= { @ "VB1.2 : Vertical height of VB1 >= 38 nm";
       internal1( aVB1, < 0.038, extension = NONE, direction = VERTICAL);
}

// not_inside_touching_edge + angle_edge
rVB1_3 @= { @ "VB1.3 : Vertical edges of VB1 should align with M5";
        gVB1_vert_edges = angle_edge( aVB1, angles = 90);
        not_inside_touching_edge( gVB1_vert_edges, aM5);
}

rVB1_4 @= { @ "VB1.4 : Horizontal edges of VB1 should align with M6";
        gVB1_horz_edges = angle_edge( aVB1, angles = 0);
        not_inside_touching_edge( gVB1_horz_edges, aM6);
}

// VB1.5 minimum spacing between VB1

// VB1.6 minimum corner-to-corner spacing
rVB1_6 @= { @ "VB1.6 : VB1 corner-to-corner spacing >= 27 nm";
        external_corner1( aVB1, distance < 0.027, type = CONVEX_TO_CONVEX);
}

rVB1_7 @= { @ "VB1.7 : VB1 must interact with M5";
        not_interacting( aVB1, aM5, include_touch = NONE);
}

rVB1_8 @= { @ "VB1.8 : VB1 must interact with M6";
        not_interacting( aVB1, aM6, include_touch = NONE);
}


rVB1_9 @= { @ "VB1.M5.EN : VB1 enclosure by M5 on opposite sides in the vertical direction >= 6 nm";
        enclose( aVB1, aM5, < 0.006, extension = NONE, direction = VERTICAL);
}

rVB1_10 @= { @ "VB1.M6.EN : VB1 enclosure by M6 on opposite sides in the horizontal direction >= 6 nm";
        enclose( aVB1, aM6, < 0.006, extension = NONE, direction = HORIZONTAL);
}
