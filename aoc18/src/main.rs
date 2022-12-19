use std::io::{ self, BufRead };
use std::fmt;
use std::cmp;
use std::collections::HashSet;
use std::collections::VecDeque;


const CHAMBER_WIDTH: usize = 7;
const DEBUG_STUFF: bool = false;

#[derive(Eq,PartialEq,Clone,Copy,Hash,Debug)]
struct CubeIterator
{
	x: i32,
	y: i32,
	z: i32,
	n: i32,
}

impl Iterator for CubeIterator
{
	type Item = Cube;

	fn next( &mut self ) -> Option<Cube>
	{
		let mut dx = 0;
		let mut dy = 0;
		let mut dz = 0;
		match self.n
		{
			0 => dx = 1,
			1 => dx = -1,
			2 => dy = 1,
			3 => dy = -1,
			4 => dz = 1,
			5 => dz = -1,
			_ => return None,
		}
		self.n += 1;

		return Some( Cube{ x: self.x + dx, y: self.y + dy, z: self.z + dz } );
	}
}

#[derive(Eq,PartialEq,Clone,Copy,Hash,Debug)]
struct Cube
{
	x: i32,
	y: i32,
	z: i32,
}

impl Cube
{
	fn new( line: &str ) -> Cube
	{
		let args: Vec<&str> = line.split( ',' ).collect();
		return Cube
		{
			x: args[0].parse::<i32>().unwrap(),
			y: args[1].parse::<i32>().unwrap(),
			z: args[2].parse::<i32>().unwrap(),
		};
	}

	fn neighbors( self: &Self ) -> CubeIterator
	{
		return CubeIterator{ x: self.x, y: self.y, z: self.z, n: 0 };
	}
}

impl fmt::Display for Cube
{
	fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result
	{
		f.write_fmt( format_args!( "( {}, {}, {} )", self.x, self.y, self.z ) )
	}
}

#[derive(Eq,PartialEq,Clone,Copy,Hash,Debug)]
struct Bounds
{
	min_x: i32,
	min_y: i32,
	min_z: i32,
	max_x: i32,
	max_y: i32,
	max_z: i32,
}

impl Bounds
{
	fn new( cubes: &HashSet< Cube > ) -> Bounds
	{
		let mut bounds = Bounds
		{
			min_x: i32::MAX,
			min_y: i32::MAX,
			min_z: i32::MAX,
			max_x: i32::MIN,
			max_y: i32::MIN,
			max_z: i32::MIN,
		};

		for cube in cubes
		{
			bounds.min_x = cmp::min( bounds.min_x, cube.x );
			bounds.min_y = cmp::min( bounds.min_y, cube.y );
			bounds.min_z = cmp::min( bounds.min_z, cube.z );
			bounds.max_x = cmp::max( bounds.max_x, cube.x );
			bounds.max_y = cmp::max( bounds.max_y, cube.y );
			bounds.max_z = cmp::max( bounds.max_z, cube.z );
		}

		return bounds;
	}

	fn contains( self: &Self, cube: &Cube ) -> bool
	{
		return self.min_x <= cube.x
			&& self.min_y <= cube.y
			&& self.min_z <= cube.z
			&& self.max_x >= cube.x
			&& self.max_y >= cube.y
			&& self.max_z >= cube.z;
	}
}

fn dump_set( title: &str, cubes: &HashSet<Cube> )
{
	println!( "Dumping set {}", title );
	for node in cubes
	{
		println!( "   {}", node );
	}
}


fn fill_pockets( cubes: &HashSet<Cube> ) -> HashSet<Cube>
{
	let bounds = Bounds::new( cubes );
	let mut pockets: HashSet<Cube> = HashSet::new();	
	let mut not_pockets: HashSet<Cube> = HashSet::new();	
	let mut possible_pockets: HashSet<Cube> = HashSet::new();	

	for cube in cubes
	{
		for n in cube.neighbors()
		{
			// just throw everything into our TODO list so we
			// only have to deal with checking for inside vs outside
			// in the loop below
			possible_pockets.insert( n );
		}
	}

	dump_set( "cubes", &cubes );
	//dump_set( "Possible_pockets", &possible_pockets );
	println!( "Bounds {:?}", bounds );
	
	for pp_start in possible_pockets.drain() 
	{
		if !bounds.contains( &pp_start )
		{
			// definitely outside and we can ignore it
			continue;
		}

		if cubes.contains( &pp_start )
		{
			// it's another cube. Not a pocket
			continue;
		}

		if pockets.contains( &pp_start )
		{
			// already figured out it's inside
			continue;
		}

		if not_pockets.contains( &pp_start )
		{
			// already figured out it's outside
			continue;
		}

		//println!( "Checking {} for pocket", pp_start );

		// we can't be sure about this, one, so traverse every one of its neighbors until we find out their status
		let mut todo: VecDeque<Cube> = VecDeque::new();
		todo.push_back( pp_start );

		let mut cluster: HashSet<Cube> = HashSet::new();	

		// we'll assume it's a pocket. If we hit bounds, we'll know it isn't
		let mut is_pocket = true;
		while let Some( node ) = todo.pop_front()
		{
			//println!( "processing node {}", node );
			if !bounds.contains( &node )
			{
				//println!( "  It is outside!" );
				// this whole group is not a pocket. 
				is_pocket = false;
				
				// we also don't need to remember the node itself because
				// its neighbors aren't interesting to us
				continue;
			}
			
			if cluster.contains( &node )
			{
				//println!( "  skipping because we already did this one");
				continue;
			}

			// we won't populate these lists until the cluster is finished, so
			// nothing we're evaluating here should be in either list
			assert!( !pockets.contains( &node ) );
			assert!( !not_pockets.contains( &node ) );

			for n in node.neighbors()
			{
				if !cubes.contains( &n ) && !cluster.contains( &n )
				{
					//println!( "  adding -> {}", n );
					todo.push_back( n );
				}
			}

			cluster.insert( node );
		}

		if is_pocket
		{
			for node in cluster
			{
				//println!( "pockets += {}", node );
				pockets.insert( node );
			}
		}
		else
		{
			for node in cluster
			{
				//println!( "not_pockets += {}", node );
				not_pockets.insert( node );
			}
		}
	}

	return pockets;
}

fn count_faces( cubes: &HashSet<Cube> ) -> i32
{
	let mut empty_faces: i32 = 0;
	for cube in cubes
	{
		for n in cube.neighbors()
		{
			if !cubes.contains( &n )
			{
				empty_faces += 1;
			}
		}
	}

	return empty_faces;
}


fn main()
{
	let mut lines = io::stdin().lock().lines();
	
	let mut cubes: HashSet<Cube> = HashSet::new();

	while let Some( line ) = lines.next()
	{
		let cur_line = line.unwrap();

		let cube = Cube::new( &cur_line );
		cubes.insert( cube );
	}

	println!( "All {:?}", count_faces( &cubes ) );

	let pockets = fill_pockets( &cubes );
	let mut cubes_plus_pockets = cubes.clone();
	for node in pockets
	{
		cubes_plus_pockets.insert( node );
	}
	println!( "Outside faces {:?}", count_faces( &cubes_plus_pockets ) );
}
