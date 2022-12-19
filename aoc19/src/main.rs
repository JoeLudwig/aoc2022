use std::io::{ self, BufRead };
use std::fmt;
use std::cmp;
use std::collections::HashSet;
use std::collections::VecDeque;


const CHAMBER_WIDTH: usize = 7;
const DEBUG_STUFF: bool = false;

#[derive(Eq,PartialEq,Clone,Copy,Hash,Debug)]
struct Blueprint
{
	id: u32,
	ore_cost_ore: u32,
	clay_cost_ore: u32,
	ob_cost_ore: u32,
	ob_cost_clay: u32,
	geode_cost_ore: u32,
	geode_cost_ob: u32,
}

impl Blueprint
{
	fn new( line: &str ) -> Blueprint
	{
		let args: Vec<&str> = line.split( &[ ' ', ':' ] ).collect();
		println!( "{:?}", args );
	
		return Blueprint
		{
			id: args[ 1 ].parse::<u32>().unwrap(),
			ore_cost_ore: args[ 7 ].parse::<u32>().unwrap(),
			clay_cost_ore: args[ 13 ].parse::<u32>().unwrap(),
			ob_cost_ore: args[ 19 ].parse::<u32>().unwrap(),
			ob_cost_clay: args[ 22 ].parse::<u32>().unwrap(),
			geode_cost_ore: args[ 28 ].parse::<u32>().unwrap(),
			geode_cost_ob: args[ 31 ].parse::<u32>().unwrap(),
		};
	}
}

impl fmt::Display for Blueprint
{
	fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result
	{
		f.write_fmt( format_args!( "BP {}: ore {}  clay {} ore   obsidian {} ore {} clay   geode {} ore, {} obsidian",
			self.id,
			self.ore_cost_ore,
			self.clay_cost_ore,
			self.ob_cost_ore, self.ob_cost_clay,
			self.geode_cost_ore, self.geode_cost_ob, ) )
	}
}



fn main()
{
	let mut lines = io::stdin().lock().lines();
	
	let mut blueprints: Vec< Blueprint > = Vec::new();

	while let Some( line ) = lines.next()
	{
		let cur_line = line.unwrap();

		blueprints.push( Blueprint::new( &cur_line ) );
	}

	println!( "All {:?}", blueprints );

}
