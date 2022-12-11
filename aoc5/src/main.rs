
use std::io::{ self, BufRead };


fn main()
{
	let mut lines = io::stdin().lock().lines();

	let mut stack: Vec< Vec< char > > = Vec::new();

	// read the initial state
	while let Some( line ) = lines.next()
	{
		let cur_line = line.unwrap();
		if cur_line.len() == 0
		{
			// done with initial state
			break;
		}

		if stack.len() == 0
		{
			// build the empty stack vectors
			let stack_count = ( cur_line.len() + 1 ) / 4;
			for _i in 0..stack_count
			{
				stack.push( Vec::new() );
			}
		}
			
		// skip the line with the numbers
		if cur_line.chars().nth( 1 ) == Some( '1' ) 
		{
			continue;
		}

		assert!( stack.len() * 4 == cur_line.len() + 1 );

		for i in 0..stack.len()
		{
			let c = cur_line.chars().nth( i * 4 + 1 );
			if c != Some( ' ' )
			{
				stack[ i ].push( c.unwrap() );
			}
		}
	}

	// stacks come in top to bottom, so we want to reverse them
	for i in 0..stack.len()
	{
		stack[ i ].reverse();
	}

	println!( "Before any moves:");
	for s in &stack
	{
		println!( "{:?}", s );
	}

	// now read the instructions
	while let Some( line ) = lines.next()
	{
		let cur_line = line.unwrap();
		if cur_line.len() == 0
		{
			// done with initial state
			break;
		}

		// move N from A to B
		let args : Vec<&str> = cur_line.split( ' ' ).collect();
		let n = args[1].parse::<i32>().unwrap();
		let a = args[3].parse::<usize>().unwrap() - 1;
		let b = args[5].parse::<usize>().unwrap() - 1;

		assert!( n <= stack[ a ].len() as i32 );
		let start = stack[ a ].len() as i32 - n ;
		for i in 0..n
		{
			let c = stack[a][( start + i ) as usize ];
			stack[b].push( c );
		}
		for _ in 0..n
		{
			stack[a].pop();
		}

		println!( "After move {} from {} to {}", n, a, b );
		for s in &stack
		{
			println!( "{:?}", s );
		}

	}

	for s in &stack
	{
		println!( "{:?}", s );
	}

	let mut res: String = "".to_owned();
	for i in 0..stack.len()
	{
		res.push_str( &stack[ i ].last().unwrap().to_string() );	
	}


	println!( "Result: {}", res );
}
