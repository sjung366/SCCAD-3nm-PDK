
rM0_1 @= { @ "M0.1 : Exact Vertical width of M0 = 12 nm";
	gM01_check = internal1( aM0, == 0.012, extension = NONE, direction = VERTICAL);	
        aM0 not gM01_check;
}

rM0_2 @= { @ "M0.2 : Minimum Horizontal width of M0 >= 24 nm";
	internal1( aM0, < 0.024, extension = NONE, direction = HORIZONTAL);
}

rM0_3 @= { @ "M0.3 : Minimum Vertical spacing of M0 >= 12 nm";
	external1(aM0, < 0.012, extension = RADIAL, direction = VERTICAL); 
}

rM0_4 @= { @ "M0.4 : Minimum Horizontal (TtT) spacing between two M0 layers on the same track >=17 nm";
        external1(aM0, < 0.017, extension = NONE, intersecting = { }, direction = HORIZONTAL, orientation = {PARALLEL}, look_thru = NONE);
}

rM0_5 @= { @ "M0.5 : M0 corner-to-corner spacing >= 15 nm";
        external_corner1( aM0, distance < 0.015, type = CONVEX_TO_CONVEX);
}

// M0.7 track pattern

rM0_6 @= { @ "M0.6 : M0 must not bend";
    not_rectangles(aM0);
}


