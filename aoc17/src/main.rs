use std::io::{ self, BufRead };
use std::fmt;
use std::cmp;


const CHAMBER_WIDTH: usize = 7;
const GRID_HEIGHT: usize = 2022 * 4 + 15;
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
	max_height: usize,
	cells: Vec<bool>,
}

impl Grid
{
	fn new() -> Grid
	{
		let mut cells: Vec<bool> = Vec::new();
		cells.resize( CHAMBER_WIDTH * GRID_HEIGHT, false );
		return Grid
		{
			max_height: 0,
			cells: cells,
		};
	}

	fn is_set( self: &Self, x: usize, y: usize ) -> bool
	{
		return self.cells[ x + y * CHAMBER_WIDTH ];
	}

	fn set( self: &mut Self, x: usize, y: usize )
	{
		assert!( !self.is_set( x, y ) );
		self.cells[ x + y * CHAMBER_WIDTH ] = true;
		self.max_height = cmp::max( self.max_height, y + 1 );
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

	let mut rocks: usize = 0;
	let mut rounds: usize = 0;
	let mut grid: Grid = Grid::new();

	while rocks < 2022
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

		//println!( "{}", grid.max_height );
	}

	//println!( "pattern len: {}", pattern.len() );
	println!( "{}", grid.max_height );
	grid.dump();
}
