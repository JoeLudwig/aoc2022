use std::io::{ self, BufRead };
use std::cmp;

fn parse_point( s: &str ) -> ( usize, usize )
{
	let ( sx, sy ) = s.split_once( ',' ).unwrap();
	return ( 
		str::parse::<usize>( sx ).unwrap(),
		str::parse::<usize>( sy ).unwrap(),
	);
}

fn main()
{
	let mut lines = io::stdin().lock().lines();

	let mut grid: Vec< bool > = Vec::new();
	let width = 600;
	let height = 500;
	
	grid.resize( width * height, false );

	let ind = | x: usize, y: usize | -> usize { x + y * width };
	let full = | x: usize, y: usize | -> bool { grid[ ind( x, y ) ] };
	let mut block = | x: usize, y: usize |   { grid[ ind( x, y ) ] = true; };

	while let Some( line ) = lines.next()
	{
		let cur_line = line.unwrap();

		let points: Vec< &str > = cur_line.split( " -> " ).collect();
		let ( mut prev_x, mut prev_y ) = parse_point( points[0] );
		for i in 1..points.len()
		{
			let ( cur_x, cur_y ) = parse_point( points[ i ] );
	
			if cur_x == prev_x
			{
				// vertical line
				let top = cmp::min( cur_y, prev_y );
				let bottom = cmp::max( cur_y, prev_y );

				for y in top..bottom
				{
					block( cur_x, y );
				}
			}
			else
			{
				// horizontal line
				assert!( cur_y == prev_y );

				let left = cmp::min( cur_x, prev_x );
				let right = cmp::max( cur_x, prev_x );

				for x in left..right
				{
					block( x, cur_y );
				}
			}
					
			prev_x = cur_x;
			prev_y = cur_y;
		}		
		

	}

}
