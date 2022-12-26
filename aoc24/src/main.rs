#![allow(unused_imports)]
use std::io::{ self, BufRead };
use std::fmt;
use std::cmp;
use std::ops::Index;
use std::ops::IndexMut;
use std::collections::HashSet;
use std::collections::VecDeque;


#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
enum Dir
{
	Right = 0,
	Down = 1,
	Left = 2,
	Up = 3,
}


type Pt=(usize, usize );

#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
struct Move
{
	pos: Pt,
	round: usize, // the round when the party arrived here
}

impl Move
{
	fn new( pos: &Pt, round: usize ) -> Move
	{
		return Move { pos: *pos, round };
	}

	fn next( self, dir: Dir ) -> Move
	{
		let ( x, y ) = self.pos;
		let pos = match dir
		{
			Dir::Up =>
			{
				assert!( y > 0 );
				( x, y - 1 )
			},
			Dir::Down =>
			{
				( x, y + 1 )
			},
			Dir::Left =>
			{
				assert!( x > 0 );
				( x - 1, y )
			},
			Dir::Right =>
			{
				( x + 1, y )
			},
		};

		return Move{ pos, round: self.round + 1 };
	}

}

type BlizzardVec= Vec<usize>;

#[derive(Clone,Debug)]
struct Weather
{
	down: Vec<BlizzardVec>,
	up: Vec<BlizzardVec>,
	right: Vec<BlizzardVec>,
	left: Vec<BlizzardVec>,
	width: usize,
	height: usize,
}


impl Weather
{
	fn new( width: usize, height: usize ) -> Weather
	{
		let mut weather = Weather
		{
			width, height,
			down: Vec::new(),
			up: Vec::new(),
			right: Vec::new(),
			left: Vec::new(),
		};

		weather.down.resize( width, Vec::new() );
		weather.up.resize( width, Vec::new() );
		weather.right.resize( height, Vec::new() );
		weather.left.resize( height, Vec::new() );

		return weather;
	}

	fn blocked( self: &Self, pt: &Pt, round: usize ) -> bool
	{
		return self.blizzard_dir( pt, round ).len() > 0;
	}

	fn blizzard_dir( self: &Self, pt: &Pt, round: usize ) -> Vec< Dir >
	{
		let ( x, y ) = *pt;

		let mut res = Vec::<Dir>::new();
		for bliz in &self.down[ x ]
		{
			// see if this blizzard is at Y at this time
			let bliz_y = ( bliz + round ) % self.height;
			if bliz_y == y
			{
				res.push( Dir::Down );
			}
		}

		for bliz in &self.up[ x ]
		{
			// see if this blizzard is at Y at this time
			let space = ( round / self.height ) + 1;
			let bliz_y = ( space * self.height + bliz - round ) % self.height;
			if bliz_y == y
			{
				res.push( Dir::Up );
			}
		}

		for bliz in &self.right[ y ]
		{
			// see if this blizzard is at X at this time
			let bliz_x = ( bliz + round ) % self.width;
			if bliz_x == x
			{
				res.push( Dir::Right );
			}
		}

		for bliz in &self.left[ y ]
		{
			// see if this blizzard is at X at this time
			let space = ( round / self.height ) + 1;
			let bliz_x = ( space * self.width + bliz - round ) % self.width;
			if bliz_x == x
			{
				res.push( Dir::Left );
			}
		}

		return res;
	}

	fn dump( self: &Self, round: usize )
	{
		let mut top = "# ".to_string();
		top.push_str( &"#".repeat( self.width ) );
		println!( "{}", top );


		for y in 0..self.height
		{
			let mut line = String::new();
			for x in 0..self.width
			{
				let pt = ( x, y );	
				let blizzards = self.blizzard_dir( &pt, round );
				let c = match blizzards.len()
				{
					0 => '.',
					1 => match blizzards[0]
					{
						Dir::Up => '^',
						Dir::Down => 'v',
						Dir::Left => '<',
						Dir::Right => '>',
					},
					2 => '2',
					3 => '3',
					4 => '4',
					5 => '5',
					6 => '6',
					7 => '7',
					8 => '8',
					9 => '9',
					_ => '*',
				};

				line.push( c );
			}

			println!( "#{}#", line );
		}

		println!( "{} #\n", "#".repeat( self.width ) );
	}

	fn add_blizzard( self: &mut Self, dir: Dir, pt: &Pt )
	{
		let ( x, y ) = *pt;
		match dir
		{
			Dir::Up => self.up[x].push( y ),
			Dir::Down => self.down[x].push( y ),
			Dir::Left => self.left[y].push( x ),
			Dir::Right => self.right[y].push( x ),
		}
	}
}


fn main()
{
	let mut lines = io::stdin().lock().lines();
	

	let mut grid_lines: Vec<String> = Vec::new();

	while let Some( line ) = lines.next()
	{
		let cur_line = line.unwrap();
		grid_lines.push( cur_line );
	}

	let width = grid_lines[0].len() - 2;
	let height = grid_lines.len() - 2;

	let mut weather = Weather::new( width, height );
	for y in 0..height
	{
		let line = &grid_lines[ y + 1 ];
		for ( i, c ) in line.char_indices()
		{
			if i == 0 || i == width+1
			{
				continue;
			}

			let x = i - 1;
			let pt = ( x, y );
			match c
			{
				'.' => {}, // empty space
				'>' => weather.add_blizzard( Dir::Right, &pt ),
				'<' => weather.add_blizzard( Dir::Left, &pt ),
				'^' => weather.add_blizzard( Dir::Up, &pt ),
				'v' => weather.add_blizzard( Dir::Down, &pt ),
				_ => panic!( "Unknown char {}", c ),
			}
		}
	}	

	let round: usize = 0;

	let mut todo: VecDeque<Move> = VecDeque::new();

	// add all the blank spots in the first <width> rounds to the TODO list
	let start = ( 0, 0 );
	for r in 0..width
	{
		if !weather.blocked( &start, r )
		{
			todo.push_back( Move::new( &start, r ) );
		}
	}

	let mut best_time = usize::MAX;

	println!( "Basin is {} x {}", width, height );
	let mut visited: HashSet< Move > = HashSet::new();

	while let Some( curr ) = todo.pop_front()
	{
		if visited.contains( &curr )
		{
			continue;
		}

		// see if the party immediately got murdered
		if weather.blocked( &curr.pos, curr.round )
		{
			//oops. Dead elves
			continue;
		}

		// abandon any branch that's longer than the best path we've found so far
		if curr.round > best_time
		{
			break;
		}

		let ( x, y ) = curr.pos;

		// see if we're there!
		if x == width - 1 && y == height - 1
		{
			best_time = cmp::min( best_time, curr.round + 1 );
			println!( "found a new best {}", best_time );
			break;
		}

		//println!("processing {:?}", curr );
		visited.insert( curr );

		if x > 0
		{
			todo.push_back( curr.next( Dir::Left ) );
		}
		if y > 0
		{
			todo.push_back( curr.next( Dir::Up ) );
		}
		if x < width - 1
		{
			todo.push_back( curr.next( Dir::Right ) );
		}
		if y < height - 1
		{
			todo.push_back( curr.next( Dir::Down ) );
		}

		// we can always wait in place 
		todo.push_back( Move{ pos: curr.pos, round: curr.round + 1 } );
	}

	println!(" best time: {}", best_time );
			
}
