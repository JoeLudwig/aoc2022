use std::io::{ self, BufRead };
use std::fmt;
use std::cmp;


const CHAMBER_WIDTH: usize = 7;
const DEBUG_STUFF: bool = false;

#[derive(Eq,PartialEq,Clone,Copy,Debug)]
enum Dir
{
	Left,
	Right,
}

#[derive(Eq,PartialEq,Clone,Copy,Debug)]
enum RockType
{
	Horiz,
	Plus,
	Seven,
	Vert,
	Box,
}


macro_rules! rock_shape
{
	( $t: expr ) =>
	{
		match $t
		{
			RockType::Horiz => vec![ ( 0, 0 ), ( 1, 0 ), ( 2, 0 ), ( 3, 0 ) ],
			RockType::Plus => vec![ ( 1, 0 ), ( 0, 1 ), ( 1, 1 ), ( 2, 1 ), ( 1, 2 ) ],
			RockType::Seven => vec![ ( 0, 0 ), ( 1, 0 ), ( 2, 0 ), ( 2, 1 ), ( 2, 2 ) ],
			RockType::Vert => vec![ ( 0, 0 ), ( 0, 1 ), ( 0, 2 ), ( 0, 3 ) ],
			RockType::Box => vec![ ( 0, 0 ), ( 1, 0 ), ( 0, 1 ), ( 1, 1 ) ],
		}
	}
}
	
type RockShape = Vec< ( usize, usize ) >;

#[derive(Eq,PartialEq,Clone,Debug)]
struct Rock
{
	rock_type: RockType,
	rock_shape: RockShape,
	x: usize,
	y: usize,
}

impl Rock
{
	fn new( rock_type: RockType, y: usize ) -> Rock
	{
		return Rock
		{
			x: 2,
			y: y,
			rock_type: rock_type,
			rock_shape: rock_shape!( rock_type ),
		};
	}

	fn draw( self: &Self, grid: &mut Grid )
	{
		for p in &self.rock_shape
		{
			let x = self.x + p.0;
			let y = self.y + p.1;

			grid.set( x, y );
		}
	}


	fn will_fit( self: &Self, dx: isize, dy: isize, grid: &Grid ) -> bool
	{
		let x: isize = self.x as isize + dx;
		let y: isize = self.y as isize + dy;
		for p in &self.rock_shape
		{
			let x = x + p.0 as isize;
			let y = y + p.1 as isize;

			if x < 0 || x >= CHAMBER_WIDTH as isize
			{
				return false;
			}

			if y < 0 
			{
				return false;
			}

			if grid.is_set( x as usize, y as usize )
			{
				return false;
			}
		}

		return true;
	}

	fn fall( self: &mut Self, grid: &Grid ) -> bool
	{
		if self.will_fit( 0, -1, grid )
		{
			if DEBUG_STUFF { println!( "Rock falls" ); }
			self.y -= 1;
			return true;
		}
		else
		{
			if DEBUG_STUFF { println!( "Rock tries to fall, but nothing happens" ); }
		}

		return false;
	}


	fn push( self: &mut Self, dir: Dir, grid: &Grid )
	{
		let dx: isize = match dir 
		{
	 		Dir::Left => -1,
			Dir::Right => 1,
		};

		if self.will_fit( dx, 0, grid )
		{
			if DEBUG_STUFF { println!( "Jet pushes {:?}", dir ); }
			self.x = ( ( self.x as isize ) + dx ) as usize;
		}
		else
		{
			if DEBUG_STUFF { println!( "Jet pushes {:?}, but nothing happens", dir ); }
		}
	}

	fn dump( self: &Self, grid: &Grid )
	{
		if !DEBUG_STUFF 
		{
			return;
		}
		let mut vis_grid = grid.clone();
		self.draw( &mut vis_grid );
		vis_grid.dump();
		println!("");
	}

}

#[derive(Eq,PartialEq,Clone,Debug)]
struct Grid
{
	height: usize,
	max_height: usize,
	cells: Vec<u8>,
}

impl Grid
{
	fn new( height: usize ) -> Grid
	{
		let mut cells: Vec<u8> = Vec::new();
		cells.resize( height, 0 );
		return Grid
		{
			height: height,
			max_height: 0,
			cells: cells,
		};
	}

	fn is_set( self: &Self, x: usize, y: usize ) -> bool
	{
		assert!( y < self.height );
		let bit: u8 = 1 << x;
		return 0 != ( self.cells[ y ] & bit );
	}

	fn set( self: &mut Self, x: usize, y: usize )
	{
		assert!( y < self.height );
		assert!( !self.is_set( x, y ) );
		let bit: u8 = 1 << x;
		let old = self.cells[ y ];
		self.cells[ y ] = old | bit;
		self.max_height = cmp::max( self.max_height, y + 1 );
	}

	fn row ( self: &Self, y: usize ) -> u8
	{
		return self.cells[y];
	}

	fn dump( self: &Self )
	{
		for y in ( 0.. self.max_height ).rev()
		{
			let mut out = "|".to_string();
			for x in 0..CHAMBER_WIDTH
			{
				out.push( if self.is_set( x, y ) { '#' } else { '.' } );
			}
			out.push( '|' );
			println!( "{}", out );
		}

		let mut out = "+".to_string();
		for x in 0..CHAMBER_WIDTH
		{
			out.push( '-' );
		}
		out.push( '+' );
		println!( "{}", out );
	}
}


fn main()
{
	let mut lines = io::stdin().lock().lines();
	
	let mut pattern: Vec< Dir > = Vec::new();

	while let Some( line ) = lines.next()
	{
		let cur_line = line.unwrap();

		assert!( pattern.len() == 0 );
		for c in cur_line.chars()
		{
			match c
			{
				'<' => pattern.push( Dir::Left ),
				'>' => pattern.push( Dir::Right ),
				_ => panic!( "Unknown direction {}", c ),
			}
		}
	}

	let rock_pattern = vec![ RockType::Horiz, RockType::Plus, RockType::Seven, RockType::Vert, RockType::Box ];

	let rock_period = pattern.len() * rock_pattern.len();
	let loop_attempts = 20;
	let rock_count = rock_period * loop_attempts;
	let range_to_check = 100;

	println!( "Running rock count {} ", rock_count );
	let mut height_after_rocks: Vec<usize> = Vec::new();
	height_after_rocks.reserve( rock_count + 1);
	height_after_rocks.push( 0 ); // rock IDs start at 1, so this shouldn't get used
	let mut height_deltas: Vec<usize> = Vec::new();
	height_deltas.reserve( rock_count + 1);
	height_deltas.push( 0 );

	let mut rocks: usize = 0;
	let mut rounds: usize = 0;
	let mut grid: Grid = Grid::new( rock_count * 4 + 8 );
	let mut rock_loop_end: usize = 0;
	let mut rock_loop_period: usize = 0;
	while rocks < rock_count && rock_loop_end == 0
	{
		let mut rock = Rock::new( rock_pattern[ rocks % rock_pattern.len() ], grid.max_height + 3 );
		if DEBUG_STUFF { println!( "NEW ROCK!" ); }
		rocks += 1;
	
		loop
		{
			rock.dump( &grid );
			rock.push( pattern[ rounds % pattern.len() ], &grid );
			rock.dump( &grid );
			rounds += 1;
			if !rock.fall( &grid )
			{
				rock.draw( &mut grid );
				if DEBUG_STUFF { grid.dump(); }
				break;
			}
		}

		height_after_rocks.push( grid.max_height );
		height_deltas.push( height_after_rocks[ rocks ] - height_after_rocks[ rocks - 1 ] );
		if rocks > rock_period * 2 && false
		{

			for i in 1..5 
			{
				
				let y_start = grid.max_height - 1;
				let y_end = y_start - range_to_check;

				//println!( "y_start {}   y_end {}", y_start, y_end );
				let y_compare_start = height_after_rocks[ rocks - i * rock_period ];
				let y_offset = y_start - y_compare_start;

				let mut matches = true;
				for y in y_end..y_start
				{
					//println!( "now {}  w/ offset  {}", y, y_offset );
					//println!( "nowh {}  w/ offset h  {}", grid.row( y ), grid.row( y_offset ) );
					if grid.row( y ) != grid.row( y - y_offset )
					{
						matches = false;
						break;
					}
				}

				if matches
				{
					println!( "Found a match at {} (period {} rocks)", grid.max_height, i * rock_period );
					rock_loop_end = rocks;
					rock_loop_period = i * rock_period;
					break;
				}
			}
		}
		
		//println!( "{}", grid.max_height );
	}

	for start in 32..height_deltas.len()
	{
		let mut matched = false;
		for y in 0..30
		{
			if height_deltas[ start ] != height_deltas[ start - y ]
			{
				matched = false;
				break;
			}
		}

		if matched
		{
			println!( "Found one at {}", start );
		}
	}
			

	if rock_loop_end != 0
	{
		let left_to_do = 1000000000000 - rock_loop_end;
		let loops = left_to_do / rock_loop_period;
		let remainder = left_to_do % rock_loop_period;

		let rock_loop_start = rock_loop_end - rock_loop_period;
		let height_per_loop = height_after_rocks[ rock_loop_end ] - height_after_rocks[ rock_loop_start ];

		let remainder_height = height_after_rocks[ rock_loop_start + remainder ] - height_after_rocks[ rock_loop_start ];
		let mega_height = height_after_rocks[ rock_loop_end ] + loops * height_per_loop + remainder_height;

		println!( "left_to_do {}   loops {}   remainder {}   rock_loop_start {}   height_per_loop {}   remainder_height {}",
			left_to_do,
			loops,
			remainder,
			rock_loop_start,
			height_per_loop,
			remainder_height,
		);
		println!( "height after 1000000000000: {}", mega_height );
	}

	println!( "{}", grid.max_height );
	//grid.dump();
}
