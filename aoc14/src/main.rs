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

fn draw_grid( grid: &Vec< char >, width: usize, left: usize, right: usize, bottom: usize )
{
	let height = grid.len() / width;
	for y in 0..( bottom + 1 )
	{
		let mut line = String::new();
		for x in left..( right + 1 )
		{
			line.push( grid[ x + y * width ] );
		}
		println!( "{}", line );
	}

	println!( "" );
}

fn main()
{
	let mut lines = io::stdin().lock().lines();

	let mut grid: Vec< char > = Vec::new();
	let width = 1000;
	let height = 500;
	
	grid.resize( width * height, '.' );

	let mut left = 500;
	let mut right = 500;
	let mut bottom = 1;

	let ind = | x: usize, y: usize | -> usize { x + y * width };
	let blocked = | grid: &Vec< char >, x: usize, y: usize | -> bool { grid[ ind( x, y ) ] != '.' };
	let block = | grid: &mut Vec< char >, x: usize, y: usize, c: char |   
	{ 
		grid[ ind( x, y ) ] = c; 
	};


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
				let top_end = cmp::min( cur_y, prev_y );
				let bottom_end = cmp::max( cur_y, prev_y );

				for y in top_end..( bottom_end + 1 )
				{
					block( &mut grid, cur_x, y, '|' );
					left = cmp::min( left, cur_x );
					right = cmp::max( right, cur_x );
					bottom = cmp::max( bottom, y );
				}
			}
			else
			{
				// horizontal line
				assert!( cur_y == prev_y );

				let left_end = cmp::min( cur_x, prev_x );
				let right_end = cmp::max( cur_x, prev_x );

				for x in left_end..( right_end + 1 )
				{
					block( &mut grid, x, cur_y, '-' );
					left = cmp::min( left, x );
					right = cmp::max( right, x );
					bottom = cmp::max( bottom, cur_y );
				}
			}
					
			prev_x = cur_x;
			prev_y = cur_y;
		}		
	}

	{
		left -= 5;
		right += 5;
		bottom += 2;
	}

	for x in 0..width
	{
		block( &mut grid, x, bottom, '=' );
	}
	draw_grid( &grid, width, left, right, bottom );

	let mut sand_drops = 0;
	let mut filled = false;
	while !filled
	{
		let mut x = 500;
		let mut y = 0;
		
		while y <= bottom
		{
			let down_clear: bool = { !blocked( &grid, x, y + 1 ) };
			let left_down_clear: bool = { !blocked( &grid, x - 1, y + 1 ) };
			let right_down_clear: bool = { !blocked( &grid, x + 1, y + 1 ) };
			if down_clear
			{
				y += 1;
			}
			else if left_down_clear
			{
				x -= 1;
				y += 1;
			}
			else if right_down_clear
			{
				x += 1;
				y += 1;
			}
			else 
			{
				block( &mut grid, x, y, 'o' );
				sand_drops += 1;
				break;
			}
		}

		if sand_drops % 100  == 0
		{
			draw_grid( &grid, width, left, right, bottom );
		}
		filled = x == 500 && y == 0;
	}

	draw_grid( &grid, width, left, right, bottom );

	println!( "Sand drops: {}", sand_drops );
}
