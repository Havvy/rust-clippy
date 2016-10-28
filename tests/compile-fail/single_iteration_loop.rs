// #![feature(plugin)]
// #![plugin(clippy)]

// #![deny(single_iteration_loop)]

fn main () {
	// loop { //~ ERROR loop executing exactly once, doing nothing
	// 	return;
	// }

	loop { //~ ERROR loop executing exactly once, doing nothing
		break;
	}

	// loop { //~ ERROR loop executing exactly once, doing nothing
	// 	return
	// }
}