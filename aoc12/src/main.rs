use std::io::{ self, BufRead };
use std::collections::VecDeque;

use std::cmp;

#[derive(Eq,PartialEq,Clone,Hash,Debug)]
struct Step
{
	x: usize,
	y: usize,
	dir: char,
	cost: u32,
}	


fn main()
{
	let mut lines = io::stdin().lock().lines();

	let mut grid: Vec< u32 > = Vec::new();
	let mut width = 0;
	let mut start_x = 0;
	let mut start_y = 0;
	let mut end_x = 0;
	let mut end_y = 0;
	let mut y = 0;

	while let Some( line ) = lines.next()
	{
		let cur_line = line.unwrap();

		if width == 0 
		{
			width = cur_line.len();
		}

		for ( x, c ) in cur_line.char_indices()
		{
			let actual_c = match c
			{
				'S' => 
				{
					start_x = x;
					start_y = y;
					'a'
				},
				'E' => 
				{
					end_x = x;
					end_y = y;
					'z'
				},
				_ => c,
			};
				
			grid.push( actual_c as u32 );
		}

		y += 1;
	}
	let height = y;
	
	let ind = | x: usize, y: usize | -> usize { x + y * width };
	let h = | x: usize, y: usize | -> u32 { grid[ ind( x, y ) ] };
	let hp = | x: usize, y: usize | -> char { char::from_u32( h( x, y ) ).unwrap() };

	println!( "start: ( {}, {} ) ({}) end: ( {}, {} ) ", start_x, start_y, hp( start_x, start_y ), 
		end_x, end_y );

	let mut visited: Vec< Step > = Vec::new();
	visited.resize( grid.len(), Step { x: 0, y: 0, dir: ' ', cost: 0 } );
	let end_step = Step { x: end_x, y: end_y, dir: 'E', cost: 0 } ;

	let mut todo: VecDeque< Step > = VecDeque::from( [ end_step.clone() ] );

	while let Some( n ) = todo.pop_front()
	{
		// skip ones we've already been to. In theory, because we're searching from the 
		// end point, we won't find a lower cost path
		let v = &visited[ ind( n.x, n.y ) ];
		if v.dir != ' ' && v.cost <= n.cost
		{
			continue;
		}

		if n.x > 0 && h( n.x, n.y ) <= ( h( n.x-1, n.y ) + 1 )
		{
			todo.push_back( Step { x: n.x - 1, y: n.y, dir: '>', cost: n.cost + 1 } );
		}
		if n.y > 0 && h( n.x, n.y ) <= ( h( n.x, n.y-1 ) + 1 )
		{
			todo.push_back( Step { x: n.x, y: n.y - 1, dir: 'v', cost: n.cost + 1 } );
		}
		if n.x < ( width - 1 ) && h( n.x, n.y ) <= ( h( n.x+1, n.y ) + 1 )
		{
			todo.push_back( Step { x: n.x + 1, y: n.y, dir: '<', cost: n.cost + 1 } );
		}
		if n.y < ( height - 1 ) && h( n.x, n.y ) <= ( h( n.x, n.y+1 ) + 1 )
		{
			todo.push_back( Step { x: n.x, y: n.y + 1, dir: '^', cost: n.cost + 1 } );
		}

		let i = ind( n.x, n.y );
		visited[ i ] = n;
	}

	let compute_path = | initial_x: usize, initial_y: usize | -> String
	{
		let mut res = String::from( "" );
		let mut x = initial_x;
		let mut y = initial_y;	
		while x != end_x || y != end_y
		{
			let n = &visited[ ind( x, y ) ];
			res.push( n.dir );

			match n.dir
			{
				'>' => x += 1,
				'<' => x -= 1,
				'^' => y -= 1,
				'v' => y += 1,
				_ => panic!( "Invalid dir on {:?}", n ),
			}
		}

		return res;
	};

	let start_to_end = compute_path( start_x, start_y );
	println!( "From Start: {} ({} steps)", start_to_end, start_to_end.len() );

	// find the a with the lowest cost
	let mut low_cost = u32::MAX;
	let mut low_x = 0;
	let mut low_y = 0;

	for n in &visited
	{
		// skip everything that isn't a
		if hp( n.x, n.y ) != 'a'
		{
			continue;
		}

		// skip nodes we couldn't reach E from
		if n.dir == ' '
		{
			continue;
		}

		// remember the lowest cost node
		if n.cost < low_cost
		{
			println!("remembering {:?}", n );
			low_cost = n.cost;
			low_x = n.x;
			low_y = n.y;
		}
	}

	
	let hiking = compute_path( low_x, low_y );
	println!( "Hiking: {} ({} steps)", hiking, hiking.len() );
}
