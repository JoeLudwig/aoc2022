#![allow(unused_imports)]
use std::io::{ self, BufRead };
use std::fmt;
use std::cmp;
use std::collections::HashSet;
use std::collections::VecDeque;


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Dir
{
	Right = 0,
	Down = 1,
	Left = 2,
	Up = 3,
	None = 4,
}

#[derive(Clone,Copy,Debug)]
struct GridCell
{
	exists: bool,
	blocked: bool,
	last_travel: Dir,
}

impl GridCell
{
	fn void() -> GridCell
	{
		return GridCell
		{
			exists: false,
			blocked: false,
			last_travel: Dir::None,
		};
	}

	fn clear() -> GridCell
	{
		return GridCell
		{
			exists: true,
			blocked: false,
			last_travel: Dir::None,
		};
	}

	fn block() -> GridCell
	{
		return GridCell
		{
			exists: true,
			blocked: true,
			last_travel: Dir::None,
		};
	}

	fn disp( self ) -> char
	{
		return if !self.exists
		{
			' '
		}
		else if self.blocked
		{
			'#'
		}
		else
		{
			match self.last_travel
			{
				Dir::None => '.',
				Dir::Up => '^',
				Dir::Right => '>',
				Dir::Left => '<',
				Dir::Down => 'v',
			}
		};
	}
}


#[derive(Clone,Debug)]
struct Grid
{
	data: Vec<GridCell>,
	width: usize,
	height: usize,
}


impl Grid
{
	fn new( lines: &Vec< String > ) -> Grid
	{
		let mut width = 0;
		for line in lines
		{
			width = cmp::max( width, line.len() );
		}
		let height = lines.len();

		let mut grid = Grid
		{
			data: Vec::new(),
			width, height,
		};

		println!( "{}x{}", width, height );
		grid.data.resize( width * height, GridCell::void() );

		for y in 0..lines.len()
		{
			let line = &lines[ y ];
			//println!( "{} -> {}", y, line );
			for ( x, c ) in line.char_indices()
			{
				grid.data[ x + y * width ] = match c
				{
					' ' => GridCell::void(),
					'.' => GridCell::clear(),
					'#' => GridCell::block(),
					_ => panic!( "Unknown grid cell {}", c ),
				}
			}
		}

		return grid;
	}


	fn dump ( self: &Self )
	{
		for y in 0..self.height
		{
			let mut line = String::new();
			for x in 0..self.width
			{
				line.push( self.data[ x + y * self.width ].disp() );
			}

			println!( "{}", line );
		}
	}

	fn start_point( self: &Self ) -> Pt
	{
		for x in 0..self.width
		{
			let cell = &self.data[ x ];
			if cell.exists && !cell.blocked
			{
				return Pt::new( x as isize, 0 );
			}
		}
		panic!( "Is 0 a completely blocked row?" );
	}

	fn first_in_row( self: &Self, y: isize ) -> isize
	{
		for x in 0..self.width
		{
			let cell = &self.data[ x + y as usize * self.width ];
			if cell.exists
			{
				return x as isize;
			}
		}
		panic!( "Is {} a completely blocked row?", y );
	}

	fn last_in_row( self: &Self, y: isize ) -> isize
	{
		for x in ( 0..self.width ).rev()
		{
			let cell = &self.data[ x + y as usize * self.width ];
			if cell.exists
			{
				return x as isize;
			}
		}
		panic!( "Is {} a completely blocked row?", y );
	}

	fn first_in_column( self: &Self, x: isize ) -> isize
	{
		for y in 0..self.height
		{
			let cell = &self.data[ x as usize + y * self.width ];
			if cell.exists
			{
				return y as isize;
			}
		}
		panic!( "Is {} a completely blocked column?", x );
	}

	fn last_in_column( self: &Self, x: isize ) -> isize
	{
		for y in (0..self.height).rev()
		{
			let cell = &self.data[ x as usize + y * self.width ];
			if cell.exists
			{
				return y as isize;
			}
		}
		panic!( "Is {} a completely blocked column?", x );
	}

	fn get( self: &Self, x: isize, y: isize) -> GridCell
	{
		return self.data[ x as usize + y as usize * self.width ];
	}

	fn visit( self: &mut Self, pt: Pt, dir: Dir ) 
	{
		let i = pt.x as usize + pt.y as usize * self.width;
		self.data[ i ].last_travel = dir;
	}

	fn walk( self: &mut Self, start: Pt, dir: Dir, dist: usize ) -> Pt
	{
		assert!( dir != Dir::None );
		let mut dx: isize = 0;
		let mut dy: isize = 0;

		match dir
		{
			Dir::Up => dy = -1,
			Dir::Down => dy = 1,
			Dir::Left=> dx = -1,
			Dir::Right => dx = 1,
			Dir::None => {},
		}

		let mut cur = start.clone();
		for _ in 0..dist
		{
			let mut next = Pt::new( cur.x + dx, cur.y + dy );
			next.x = ( next.x + self.width as isize ) % self.width as isize;
			next.y = ( next.y + self.height  as isize ) % self.height as isize;
			let mut cell = self.get( next.x, next.y );

			// handle the weird wrapping stuff
			if !cell.exists
			{
				match dir
				{
					Dir::Up => 		next.y = self.last_in_column( next.x ),
					Dir::Down => 	next.y = self.first_in_column( next.x ),
					Dir::Left => 	next.x = self.last_in_row( next.y ),
					Dir::Right => 	next.x = self.first_in_row( next.y ),
					Dir::None => {},
				}
				cell = self.get( next.x, next.y );
			}

			if cell.blocked
			{
				// stop and don't move to next
				break;
			}

			cur = next;
			self.visit( cur, dir );
		}			

		return cur;
	}
			
}

#[derive(Clone,Copy,Debug)]
enum Command
{
	Left,
	Right,
	Move( usize ),
}
	

#[derive(Clone,Copy,Debug)]
struct Pt
{
	x: isize,
	y: isize,
}

impl Pt
{
	fn new( x: isize, y: isize ) -> Pt
	{
		return Pt{ x, y };
	}
}


fn main()
{
	let mut lines = io::stdin().lock().lines();
	
	let mut grid_lines: Vec<String> = Vec::new();

	while let Some( line ) = lines.next()
	{
		let cur_line = line.unwrap();
		if cur_line.len() == 0
		{
			break;
		}

		grid_lines.push( cur_line );
	}

	let instructions = lines.next().unwrap().unwrap();

	let mut grid = Grid::new( &grid_lines );
	grid.dump();

	let mut commands: Vec< Command > = Vec::new();
	let mut dist = String::new();
	for c in instructions.chars()
	{
		match c
		{
			'0' | '1' | '2' | '3' | '4'| '5' | '6' | '7' | '8' | '9' => dist.push( c ),
			'R' | 'L' =>
			{
				if dist.len() > 0
				{
					commands.push( Command::Move( dist.parse::<usize>().unwrap() ) );
					dist = String::new();
				}

				commands.push( if c == 'L' { Command::Left } else { Command::Right } );
			},
			_ => panic!( "Unknown instruction character {}", c ),
		}
	}
	if dist.len() > 0
	{
		commands.push( Command::Move( dist.parse::<usize>().unwrap() ) );
	}

	println!( "\n{:?}", commands );

	let mut agent_pos = grid.start_point();
	let mut agent_dir = Dir::Right;
	grid.visit( agent_pos, agent_dir );
	for cmd in commands
	{
		match cmd
		{
			Command::Move( dist ) => agent_pos = grid.walk( agent_pos, agent_dir, dist ),
			Command::Left =>
			{
				agent_dir = match agent_dir
				{
					Dir::Up => Dir::Left,
					Dir::Left => Dir::Down,
					Dir::Down => Dir::Right,
					Dir::Right => Dir::Up,
					Dir::None => Dir::None,
				};
				grid.visit( agent_pos, agent_dir );
			},
			Command::Right =>
			{
				agent_dir = match agent_dir
				{
					Dir::Down => Dir::Left,
					Dir::Right => Dir::Down,
					Dir::Up => Dir::Right,
					Dir::Left => Dir::Up,
					Dir::None => Dir::None,
				};
				grid.visit( agent_pos, agent_dir );
			},
		}
	}

	grid.dump();

	let final_password = ( agent_pos.y + 1 ) * 1000 + ( agent_pos.x + 1 ) * 4 + agent_dir as isize;
	println!( "final password: {}", final_password );
}
