use std::io::{ self, BufRead };
use std::cmp;


fn process_line( input: String )
{
	println!( "Processing {:10}", &input[0..cmp::min( input.len(), 15 ) ] );

	let marker_len = 14;
	for i in marker_len..input.len()
	{
		let mut dup = false;
		let start = i - marker_len;
		let end = i;
		for j in start..end
		{
			for k in ( j + 1 )..end
			{
				let a = input.chars().nth( j );
				let b = input.chars().nth( k );
				dup = dup || a == b;
			}
		}

		if !dup 
		{
			println!( "  marker at {}", i );
			return;
		}
	}

	println!( "Marker not found" );
}

fn main()
{
	let mut lines = io::stdin().lock().lines();

	// read the initial state
	while let Some( line ) = lines.next()
	{
		let cur_line = line.unwrap();
		if cur_line.len() == 0
		{
			// done with initial state
			break;
		}

		process_line( cur_line );
	}
}
