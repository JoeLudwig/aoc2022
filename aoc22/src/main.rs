#![allow(unused_imports)]
use std::io::{ self, BufRead };
use std::fmt;
use std::cmp;
use std::ops::Index;
use std::ops::IndexMut;
use std::collections::HashSet;
use std::collections::VecDeque;


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Dir
{
	Right = 0,
	Down = 1,
	Left = 2,
	Up = 3,
}

fn rotate_dir( dir: Dir, rots: i32 ) -> Dir
{
	return match ( dir as i32 + rots + 4 ) % 4
	{
		0 => Dir::Right,
		1 => Dir::Down,
		2 => Dir::Left,
		3 => Dir::Up,
		_ => panic!( "Invalid number of rots or something {:?}, {}", dir, rots ),
	};
}

#[derive(Clone,Copy,Debug)]
struct GridCell
{
	blocked: bool,
	last_travel: Option< Dir >,
}

impl GridCell
{
	fn clear() -> GridCell
	{
		return GridCell
		{
			blocked: false,
			last_travel: None,
		};
	}

	fn block() -> GridCell
	{
		return GridCell
		{
			blocked: true,
			last_travel: None,
		};
	}

	fn disp( self ) -> char
	{
		return if self.blocked
		{
			'#'
		}
		else
		{
			match self.last_travel
			{
				Option::None => '.',
				Option::Some( dir ) => match dir
				{
					Dir::Up => '^',
					Dir::Right => '>',
					Dir::Left => '<',
					Dir::Down => 'v',
				},
			}
		};
	}
}


#[derive(Clone,Debug)]
struct Grid
{
	data: Vec<GridCell>,
	face: Face,
	width: usize,
	height: usize,
	x_slot: usize,
	y_slot: usize,
	rots: i32,
}


impl Grid
{
	fn new( lines: &Vec< String >, face: Face, x_slot: usize, y_slot: usize ) -> Grid
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
			face,
			width, height,
			x_slot, y_slot,
			rots: 0,
		};

		grid.data.resize( width * height, GridCell::clear() );

		for y in 0..lines.len()
		{
			let line = &lines[ y ];
			//println!( "{} -> {}", y, line );
			for ( x, c ) in line.char_indices()
			{
				grid.data[ x + y * width ] = match c
				{
					'.' => GridCell::clear(),
					'#' => GridCell::block(),
					_ => panic!( "Unknown grid cell {}", c ),
				}
			}
		}

		return grid;
	}


	fn to_string_vec ( self: &Self ) -> Vec<String>
	{
		if self.rots != 0
		{
			let mut copy_to_rot = self.clone();

			while copy_to_rot.rots > 0
			{
				copy_to_rot.rotate_left();
			}
			while copy_to_rot.rots < 0
			{
				copy_to_rot.rotate_right();
			}

			assert!( copy_to_rot.rots == 0 );
			return copy_to_rot.to_string_vec();
		}

		let mut out: Vec<String> = Vec::new();
		for y in 0..self.height
		{
			let mut line = String::new();
			for x in 0..self.width
			{
				line.push( self.data[ x + y * self.width ].disp() );
				/*line.push( match self.face
				{
					Face::Top => 'T',
					Face::Front => 'F',
					Face::Bottom => 'b',
					Face::Back => 'B',
					Face::Right => 'R',
					Face::Left => 'L',
				} ); */
			}
			out.push( line );
		}

		return out;
	}

	fn start_point( self: &Self ) -> Agent
	{
		for x in 0..self.width
		{
			let cell = &self.data[ x ];
			if !cell.blocked
			{
				return Agent::new( Dir::Right, self.face, x as isize, 0 );
			}
		}
		panic!( "Is 0 a completely blocked row?" );
	}

	fn get( self: &Self, x: isize, y: isize) -> GridCell
	{
		return self.data[ x as usize + y as usize * self.width ];
	}

	fn visit( self: &mut Self, agent: Agent ) 
	{
		let i = agent.x as usize + agent.y as usize * self.width;

		// pre-rotate the arrow so it'll display correctly
		self.data[ i ].last_travel = Some( rotate_dir( agent.dir, -self.rots ) );
	}

	fn transpose( self: &mut Self  )
	{
		// 1 2 3    1 4 7
		// 4 5 6 => 2 5 8
		// 7 8 9    3 6 9
		let mut new_data: Vec<GridCell> = Vec::new();
		new_data.resize( self.data.len(), GridCell::clear() );

		assert!( self.width == self.height );
		for x in 0..self.width
		{
			for y in 0..self.height
			{
				new_data[ x + y * self.width ] = self.data[ y + x * self.width ];
			}
		}

		self.data = new_data;
	}

	
	fn flip_vertical( self: &mut Self  )
	{
		// 1 2 3    7 8 9
		// 4 5 6 => 4 5 6
		// 7 8 9    1 2 3
		let mut new_data: Vec<GridCell> = Vec::new();
		new_data.resize( self.data.len(), GridCell::clear() );

		assert!( self.width == self.height );
		for x in 0..self.width
		{
			for y in 0..self.height
			{
				new_data[ x + y * self.width ] = self.data[ x + ( self.height - 1 - y ) * self.width ];
			}
		}

		self.data = new_data;
	}

	
	fn flip_horizontal( self: &mut Self  )
	{
		// 1 2 3    3 2 1
		// 4 5 6 => 6 5 4
		// 7 8 9    9 8 7
		let mut new_data: Vec<GridCell> = Vec::new();
		new_data.resize( self.data.len(), GridCell::clear() );

		assert!( self.width == self.height );
		for x in 0..self.width
		{
			for y in 0..self.height
			{
				new_data[ x + y * self.width ] = self.data[ ( self.width - 1 - x ) + y * self.width ];
			}
		}

		self.data = new_data;
	}

	
	fn rotate_left( self: &mut Self)
	{
		// 1 2 3    3 6 9
		// 4 5 6 => 2 5 8
		// 7 8 9    1 4 7

		// This is a transpose followed by a vertical flip
		self.transpose();
		self.flip_vertical();
		self.rots -= 1;
	}

	fn rotate_right( self: &mut Self)
	{
		// 1 2 3    7 4 1
		// 4 5 6 => 8 5 2
		// 7 8 9    9 6 3

		// this is a transpose, followed by a horizontal flip
		self.transpose();
		self.flip_horizontal();
		self.rots += 1;
	}

}

fn next_step( face_width: usize, curr: Agent ) -> Agent
{
	let mut dx: isize = 0;
	let mut dy: isize = 0;
	match curr.dir
	{
		Dir::Up => dy = -1,
		Dir::Down => dy = 1,
		Dir::Left=> dx = -1,
		Dir::Right => dx = 1,
	}

	let x = curr.x + dx;
	let y = curr.y + dy;
	let last = face_width as isize - 1;

	let overflow = x < 0 || y < 0 
		|| x > last
		|| y > last;

	println!( "{:6?} {:?} ( {}, {} ) => ( {}, {} )  {}",
		curr.face, curr.dir,
		curr.x, curr.y, x, y, overflow );
	let mut next = curr.clone();
	if !overflow
	{
		next.x = x;
		next.y = y;
		return next;
	}

	// remove the increments. We'll handle that explicitly below
	let x = curr.x;
	let y = curr.y;

	let flip = | a: isize | -> isize { return last - a };

	// now it gets complicated
	match curr.face
	{
		Face::Top =>
		{
			match curr.dir
			{
				Dir::Down =>
				{
					next.face = Face::Front;

					// same orientation
					next.y = 0; 
				},

				Dir::Right =>
				{
					next.face = Face::Right;

					// rotate left
					next.x = flip( y );
					next.y = 0;
					next.dir = Dir::Down;
				},
					
				Dir::Left =>
				{
					next.face = Face::Left;

					// rotate right
					next.x = curr.y;
					next.y = 0;
					next.dir = Dir::Down;
				},

				Dir::Up =>
				{
					next.face = Face::Back;

					// flip horiz
					next.x = flip( x );
					next.y = 0;
					next.dir = Dir::Down;
				}
					
			}
		},

		Face::Front =>
		{
			match curr.dir
			{
				Dir::Down =>
				{
					next.face = Face::Bottom;

					// same orientation
					next.y = 0; 
				},

				Dir::Right =>
				{
					next.face = Face::Right;

					// same orientation
					next.x = 0;
					next.y = y;
				},
					
				Dir::Left =>
				{
					next.face = Face::Left;

					// same orientation
					next.x = last;
					next.y = y;
				},

				Dir::Up =>
				{
					next.face = Face::Top;

					// same orientation
					next.x = x;
					next.y = last;
				}
					
			}
		},

		Face::Bottom =>
		{
			match curr.dir
			{
				Dir::Down =>
				{
					next.face = Face::Back;

					// flip horiz
					next.dir = Dir::Up;
					next.x = flip( x );
					next.y = last; 
				},

				Dir::Right =>
				{
					next.face = Face::Right;

					// rotate right
					next.dir = Dir::Up;
					next.x = y;
					next.y = last; 
				},
					
				Dir::Left =>
				{
					next.face = Face::Left;

					// rotate left
					next.dir = Dir::Up;
					next.x = flip( y );
					next.y = last; 
				},

				Dir::Up =>
				{
					next.face = Face::Front;

					// same orientation
					next.dir = Dir::Up;
					next.x = x;
					next.y = last; 
				}
					
			}
		},

		Face::Back =>
		{
			match curr.dir
			{
				Dir::Down =>
				{
					next.face = Face::Bottom;

					// flip horiz
					next.dir = Dir::Up;
					next.x = flip( x );
					next.y = last; 
				},

				Dir::Right =>
				{
					next.face = Face::Left;

					// same orientation
					next.x = 0;
					next.y = y; 
				},
					
				Dir::Left =>
				{
					next.face = Face::Right;

					// same orientation
					next.x = last;
					next.y = y; 
				},

				Dir::Up =>
				{
					next.face = Face::Top;

					// flip horiz
					next.dir = Dir::Down;
					next.x = flip( x );
					next.y = 0; 
				}
			}
		},

		Face::Right =>
		{
			match curr.dir
			{
				Dir::Down =>
				{
					next.face = Face::Bottom;

					// flip horiz
					next.dir = Dir::Left;
					next.x = last;
					next.y = x; 
				},

				Dir::Right =>
				{
					next.face = Face::Back;

					// same orientation
					next.x = 0;
					next.y = y; 
				},
					
				Dir::Left =>
				{
					next.face = Face::Front;

					// same orientation
					next.x = last;
					next.y = y; 
				},

				Dir::Up =>
				{
					next.face = Face::Top;

					// rotate left
					next.dir = Dir::Left;
					next.x = last;
					next.y = flip( x ); 
				}
			}
		},

		Face::Left =>
		{
			match curr.dir
			{
				Dir::Down =>
				{
					next.face = Face::Bottom;

					// rotate right
					next.dir = Dir::Right;
					next.x = 0;
					next.y = flip( x ); 
				},

				Dir::Right =>
				{
					next.face = Face::Front;

					// same orientation
					next.x = 0;
					next.y = y; 
				},
					
				Dir::Left =>
				{
					next.face = Face::Back;

					// same orientation
					next.x = last;
					next.y = y; 
				},

				Dir::Up =>
				{
					next.face = Face::Top;

					// flip horiz
					next.dir = Dir::Right;
					next.x = 0;
					next.y = x;
				}
			}
		},
	}

	return next;
}


fn walk( cube: &mut Vec<Grid>, start: Agent, dist: usize ) -> Agent
{
	assert!( cube.len() == 6 );

	let face_width = cube[ Face::Top ].width;

	let mut curr = start.clone();
	for _ in 0..dist
	{
		let next = next_step( face_width, curr );
		let cell = cube[ next.face ].get( next.x, next.y );

		if cell.blocked
		{
			println!( "   BLOCKED" );
			// stop and don't move to next
			break;
		}

		curr = next;
		cube[ curr.face ].visit( curr );
	}			

	return curr;
}


fn dump_cube( cube: &Vec<Grid> )
{
	assert!( cube.len() == 6 );

	let x_start: usize = 0;
	let y_start: usize = 0;
	let mut x_end: usize = 0;
	let mut y_end: usize = 0;
	let mut face_width = 0;

	for face in cube
	{
		face_width = face.width;
		y_end = cmp::max( y_end, face.y_slot );
		x_end = cmp::max( x_end, face.x_slot );
	}

	let mut lines: Vec<String> = Vec::new();
	lines.resize( face_width * ( 1 + y_end - y_start ), String::new() );

	let mut blank: String = String::new();
	for _ in 0..face_width
	{
		blank.push( ' ' );
	}

	// un-mut some things
	let blank = blank;
	let face_width = face_width;
	let x_end = x_end;
	let y_end = y_end;

	for y in y_start..( y_end + 1 )
	{
		for x in x_start..( x_end + 1 )
		{
			let mut found_face = false;
			for face in cube
			{
				if face.x_slot == x && face.y_slot == y 
				{
					found_face = true;
					
					let face_lines = face.to_string_vec();

					for i in 0..face_width
					{
						lines[i + y * face_width].push_str( &face_lines[i] );
					}
				}

			}
			if !found_face
			{
				for i in 0..face_width
				{
					lines[i + y * face_width].push_str( &blank );
				}
			}
		}
	}	

	for line in lines
	{
		println!( "{}", line );
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
struct Agent
{
	dir: Dir,
	face: Face,
	x: isize,
	y: isize,
}

impl Agent
{
	fn new( dir: Dir, face: Face, x: isize, y: isize ) -> Agent
	{
		return Agent{ dir, face, x, y };
	}
}

#[derive(Clone,Copy,Debug)]
enum Face
{
	Top = 0,
	Front = 1,
	Bottom = 2,
	Back = 3,
	Right = 4,
	Left = 5,
}

impl Index<Face> for Vec<Grid> {
    type Output = Grid;

    fn index(&self, face: Face) -> &Self::Output {
		assert!( self.len() == 6 );
		return &self[ face as usize ];
    }
}

impl IndexMut<Face> for Vec<Grid> {
    fn index_mut(self: &mut Self, face: Face) -> &mut Self::Output {
		assert!( self.len() == 6 );
		return &mut self[ face as usize ];
    }
}

fn main()
{
	let mut lines = io::stdin().lock().lines();
	

	let mut grid_lines: Vec<Vec<String>> = Vec::new();
	for _ in 0..( 6 * 6 )
	{
		grid_lines.push( Vec::new() );
	}

	let mut face_width: usize = 0;
	let mut y: usize = 0;
	let mut y_face: usize = 0;

	let mut top_x_face: usize = 0;
	let mut top_y_face: usize = 0;

	while let Some( line ) = lines.next()
	{
		let cur_line = line.unwrap();
		if cur_line.len() == 0
		{
			break;
		}

		if face_width == 0
		{
			face_width = if cur_line.len() / 4 < 6 { 4 } else { 50 };
			println!( "Detected that face_width is {}", face_width );
		}

		let mut x_face: usize = 0;
		let mut curr = cur_line.clone();
		while curr.len() > 0
		{
			let ( this, next ) = curr.split_at( face_width );

			if !this.starts_with( " " )
			{
				grid_lines[ x_face + 6 * y_face ].push( this.to_string() );

				// the first face we actually find content in is the top
				if top_x_face == 0 && top_y_face == 0
				{
					// it's possible that the top is actually 0,0
					// in which case we'll set that coord over and over
					// but since we are setting it back to 0,0,
					// nobody cares
					top_x_face = x_face;
					top_y_face = y_face;
				}
			}
			
			curr = next.to_string();
			x_face += 1;
		}

		y += 1;
		if y >= face_width
		{
			y = 0;
			y_face += 1;
		}
	}

	let face_width = face_width; // drop mut
	/*
	for y in 0..6
	{
		for x in 0..6 
		{
			let lines = &grid_lines[ x + y * 6 ];
			if lines.len() == 0
			{
				println!("\n{}, {} is a void", x, y );
				continue;
			}

			if x == top_x_face && y == top_y_face
			{
				println!( "\n{}, {} =  TOP    ===========", x, y );
			}
			else
			{
				println!( "\n{}, {} =====================", x, y );
			}
			for line in lines
			{
				assert!( face_width == line.len() );
				println!("   {}", line );
			}
		}
	}
	*/

	let mut cube: Vec< Option< Grid > > = Vec::new();
	cube.resize( 6, Option::None );
	
	// top is easy
	cube[ Face::Top as usize ] = Some( Grid::new( &grid_lines[ top_x_face + top_y_face * 6 ], Face::Top,
		top_x_face, top_y_face ) );

	// front is easy in the data we have
	assert!( grid_lines[ top_x_face + ( top_y_face + 1 ) * 6 ].len() > 0 );
	cube[ Face::Front as usize ] = Some( Grid::new( &grid_lines[ top_x_face + ( 1 + top_y_face ) * 6 ], Face::Front,
		top_x_face, top_y_face + 1 ) );

	// Bottom is easy in the data we have
	assert!( grid_lines[ top_x_face + ( top_y_face + 2 ) * 6 ].len() > 0 );
	cube[ Face::Bottom as usize ] = Some( Grid::new( &grid_lines[ top_x_face + ( 2 + top_y_face ) * 6 ], Face::Bottom,
		top_x_face, top_y_face + 2 ) );

	// Find the right side
	if grid_lines[ top_x_face + 1 + top_y_face * 6 ].len() > 0
	{
		// Check for right off of top
		// need to rotate the right side 
		let mut right = Grid::new( &grid_lines[ top_x_face + 1 + top_y_face * 6 ], Face::Right,
			top_x_face + 1, top_y_face );
		right.rotate_right();
		cube[ Face::Right as usize ] = Some( right );
	}
	else if grid_lines[ top_x_face + 1 + ( top_y_face + 1 ) * 6 ].len() > 0
	{
		// right off of front
		// this one is facing the right way
		let right = Grid::new( &grid_lines[ top_x_face + 1 + ( top_y_face + 1 ) * 6 ], Face::Right,
			top_x_face + 1, top_y_face + 1 );
		cube[ Face::Right as usize ] = Some( right );
	}
	else if grid_lines[ top_x_face + 1 + ( top_y_face + 2 ) * 6 ].len() > 0
	{
		// right off of bottom
		// need to rotate this one to the right
		let mut right = Grid::new( &grid_lines[ top_x_face + 1 + ( top_y_face + 2 ) * 6 ], Face::Right,
			top_x_face + 1, top_y_face + 2 );
		right.rotate_left();
		cube[ Face::Right as usize ] = Some( right );
	}
	else
	{
		panic!( "Couldn't find right" );
	}
		
	// Find the left side
	if top_x_face > 0 && grid_lines[ top_x_face - 1 + top_y_face * 6 ].len() > 0
	{
		// Check for Left off of top
		// need to rotate the left side 
		let mut left = Grid::new( &grid_lines[ top_x_face - 1 + top_y_face * 6 ], Face::Left,
			top_x_face - 1, top_y_face );
		left.rotate_right();
		cube[ Face::Left as usize ] = Some( left );
	}
	else if top_x_face > 0 && grid_lines[ top_x_face - 1 + ( top_y_face + 1 ) * 6 ].len() > 0
	{
		// left off of front
		// this one is facing the right way
		let left = Grid::new( &grid_lines[ top_x_face - 1 + ( top_y_face + 1 ) * 6 ], Face::Left,
			top_x_face - 1, top_y_face + 1 );
		cube[ Face::Left as usize ] = Some( left );
	}
	else if top_x_face > 0 && grid_lines[ top_x_face - 1 + ( top_y_face + 2 ) * 6 ].len() > 0
	{
		// left off of bottom
		// need to rotate this one to the right
		let mut left = Grid::new( &grid_lines[ top_x_face - 1 + ( top_y_face + 2 ) * 6 ], Face::Left,
			top_x_face - 1, top_y_face + 2 );
		left.rotate_right();
		cube[ Face::Left as usize ] = Some( left );
	}
	else
	{
		panic!( "Couldn't find left" );
	}
		

	// Find the back
	if top_x_face >= 2 && grid_lines[ top_x_face - 2 + ( top_y_face + 1 ) * 6 ].len() > 0
	{
		// Check for two Left off of front
		// this one is facing the right way
		let back = Grid::new( &grid_lines[ top_x_face - 2 + ( top_y_face + 1 ) * 6 ], Face::Back,
			top_x_face - 2, top_y_face + 1 );
		cube[ Face::Back as usize ] = Some( back );
	}
	else if top_x_face > 0 && grid_lines[ top_x_face - 1 + ( top_y_face + 3 ) * 6 ].len() > 0
	{
		// down off the left side
		let mut back = Grid::new( &grid_lines[ top_x_face - 1 + ( top_y_face + 3 ) * 6 ], Face::Back,
			top_x_face - 1, top_y_face + 3 );
		back.rotate_right();
		cube[ Face::Back as usize ] = Some( back );
	}
	else
	{
		panic!( "Couldn't find back" );
	}
		

	let mut cube_found: Vec< Grid > = Vec::new();
	for face in cube
	{
		cube_found.push( face.unwrap() );
	}
	let mut cube = cube_found;


	let instructions = lines.next().unwrap().unwrap();

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

	let mut agent = cube[ Face::Top ].start_point();
	cube[ Face::Top ].visit( agent );
	for cmd in commands
	{
		println!( "Executing {:?}", cmd );
		match cmd
		{
			Command::Move( dist ) => agent = walk( &mut cube, agent, dist ),
			Command::Left =>
			{
				agent.dir = rotate_dir( agent.dir, -1 );
				cube[ agent.face ].visit( agent );
			},
			Command::Right =>
			{
				agent.dir = rotate_dir( agent.dir, 1 );
				cube[ agent.face ].visit( agent );
			},
		}
	}

	dump_cube( &cube );


	let last: isize = face_width as isize;
	let face = &cube[ agent.face ];
	let ( rot_x, rot_y ) = match face.rots
	{
		0 => ( agent.x, agent.y ),
		1 => ( agent.y, last - agent.x ),
		-1 => ( last - agent.y, agent.x ),
		_ => panic!( "Some BS"),
	};
	let rot_dir = rotate_dir( agent.dir, face.rots );

	println!( "{} * 1000 + {} * 4 + {}", agent.y + 1, agent.x + 1, agent.dir as isize );
	println!( "{} * 1000 + {} * 4 + {}", rot_y + 1, rot_x + 1, rot_dir as isize );

	let final_x = rot_x + face_width as isize * face.x_slot as isize;
	let final_y = rot_y + face_width as isize * face.y_slot as isize;
	println!( "{} * 1000 + {} * 4 + {}", final_y + 1, final_x + 1, rot_dir as isize );
	let final_password = ( final_y + 1 ) * 1000 + ( final_x + 1 ) * 4 + rot_dir as isize;
	println!( "final password: {}", final_password );
}
