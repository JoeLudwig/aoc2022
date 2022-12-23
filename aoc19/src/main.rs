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
	obsidian_cost_ore: u32,
	obsidian_cost_clay: u32,
	geode_cost_ore: u32,
	geode_cost_obsidian: u32,
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
			obsidian_cost_ore: args[ 19 ].parse::<u32>().unwrap(),
			obsidian_cost_clay: args[ 22 ].parse::<u32>().unwrap(),
			geode_cost_ore: args[ 28 ].parse::<u32>().unwrap(),
			geode_cost_obsidian: args[ 31 ].parse::<u32>().unwrap(),
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
			self.obsidian_cost_ore, self.obsidian_cost_clay,
			self.geode_cost_ore, self.geode_cost_obsidian, ) )
	}
}


#[derive(Eq,PartialEq,Clone,Hash,Debug)]
struct State
{
	blueprint: Blueprint,
	minute: u32,
	command_history: Vec<Command>,

	ore: u32,
	clay: u32,
	obsidian: u32,
	geode: u32,

	ore_robots: u32,
	clay_robots: u32,
	obsidian_robots: u32,
	geode_robots: u32,
}

impl State
{
	fn new( blueprint: &Blueprint ) -> State
	{
	
		return State
		{
			blueprint: blueprint.clone(),
			minute: 1,
			command_history: Vec::new(),

			ore: 0,
			clay: 0,
			obsidian: 0,
			geode: 0,

			ore_robots: 1,
			clay_robots: 0,
			obsidian_robots: 0,
			geode_robots: 0,
		};
	}
	
	fn can_afford_ore( &self ) -> bool
	{
		return self.ore >= self.blueprint.ore_cost_ore;
	}

	fn can_afford_clay( &self ) -> bool
	{
		return self.ore >= self.blueprint.clay_cost_ore;
	}

	fn can_afford_obsidian( &self ) -> bool
	{
		return self.ore >= self.blueprint.obsidian_cost_ore
			&& self.clay >= self.blueprint.obsidian_cost_clay;
	}

	fn can_afford_geode( &self ) -> bool
	{
		return self.ore >= self.blueprint.geode_cost_ore
			&& self.obsidian >= self.blueprint.geode_cost_obsidian;
	}

	fn can_afford( &self, command: Command ) -> bool
	{
		return match command
		{
			Command::Wait => true,
			Command::Ore => self.can_afford_ore(),
			Command::Clay => self.can_afford_clay(),
			Command::Obsidian => self.can_afford_obsidian(),
			Command::Geode => self.can_afford_geode(),
		}
	}

	fn buy( &mut self, command: Command )
	{
		match command
		{
			Command::Wait => {},
			Command::Ore => self.buy_ore(),
			Command::Clay => self.buy_clay(),
			Command::Obsidian => self.buy_obsidian(),
			Command::Geode => self.buy_geode(),
		}
	}

	fn buy_ore( &mut self ) 
	{
		assert!(self.can_afford_ore() );
		self.ore_robots += 1;
		self.ore -= self.blueprint.ore_cost_ore;
		self.command_history.push( Command::Ore );
		//println!( "       +ore");
	}

	fn buy_clay( &mut self ) 
	{
		assert!(self.can_afford_clay() );
		self.clay_robots += 1;
		self.ore -= self.blueprint.clay_cost_ore;
		self.command_history.push( Command::Clay );
		//println!( "       +clay");
	}

	fn buy_obsidian( &mut self ) 
	{
		assert!(self.can_afford_obsidian() );
		self.obsidian_robots += 1;
		self.ore -= self.blueprint.obsidian_cost_ore;
		self.clay -= self.blueprint.obsidian_cost_clay;
		self.command_history.push( Command::Obsidian );
		//println!( "       +obsidian");
	}

	fn buy_geode( &mut self ) 
	{
		assert!(self.can_afford_geode() );
		self.geode_robots += 1;
		self.ore -= self.blueprint.geode_cost_ore;
		self.obsidian -= self.blueprint.geode_cost_obsidian;
		self.command_history.push( Command::Geode );
		//println!( "       +geode");
	}

	fn wait( &mut self )
	{
		self.command_history.push( Command::Wait );
	}

	fn advance( &mut self )
	{
		self.minute += 1;
		self.ore += self.ore_robots;
		self.clay += self.clay_robots;
		self.obsidian += self.obsidian_robots;
		self.geode += self.geode_robots;
	}
}

impl fmt::Display for State
{
	fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result
	{
		let mut hist = String::new();
		for c in &self.command_history
		{
			hist.push( match c {
				Command::Wait => '.',
				Command::Ore => 'M',
				Command::Clay => 'C',
				Command::Obsidian => 'O',
				Command::Geode => 'G',
			} );
		}

		f.write_fmt( format_args!( "min {:2} BP {}: res  {:2}/{:2}/{:2}/{:2} robots {:2}/{:2}/{:2}/{:2}: {}",
			self.minute,
			self.blueprint.id,
			self.ore,
			self.clay,
			self.obsidian,
			self.geode,
			self.ore_robots,
			self.clay_robots,
			self.obsidian_robots,
			self.geode_robots,
			hist ) )
	}

}


#[derive(Eq,PartialEq,Clone,Copy,Hash,Debug)]
enum Command
{ 
	Wait = 0, // let robots work. Do nothing
	Ore = 1, 
	Clay = 2,
	Obsidian = 3,
	Geode = 4,
}

impl fmt::Display for Command
{
	fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result
	{
		f.write_fmt( format_args!( "{:?}", self ) )
	}

}

#[derive(Eq,PartialEq,Clone,Hash,Debug)]
struct Node
{
	state: State,
	command: Command,
	score: i32,
}

fn compute_score( state : &State, command: Command ) -> i32
{
	let minutes_left = if state.minute < MINUTE_LIMIT { MINUTE_LIMIT - state.minute } else { 0 };
	let total_geodes = state.geode + state.geode_robots * minutes_left;

	let rounds_to_complete = match command
	{
		Command::Ore =>
		{
			if state.ore >= state.blueprint.ore_cost_ore
			{
				0
			}
			else
			{
				( state.blueprint.ore_cost_ore - state.ore ) / state.ore_robots
			}
		},

		Command::Clay =>
		{
			if state.ore >= state.blueprint.clay_cost_ore
			{
				0
			}
			else
			{
				( state.blueprint.clay_cost_ore - state.ore ) / state.ore_robots
			}
		},

		Command::Obsidian =>
		{
			cmp::max(
				if state.ore >= state.blueprint.obsidian_cost_ore
				{
					0
				}
				else
				{
					( state.blueprint.obsidian_cost_ore - state.ore ) / state.ore_robots
				},
				if state.clay >= state.blueprint.obsidian_cost_clay
				{
					0
				}
				else
				{
					( state.blueprint.obsidian_cost_clay - state.clay ) / state.clay_robots
				},
			)
		},

		Command::Geode =>
		{
			cmp::max(
				if state.ore >= state.blueprint.geode_cost_ore
				{
					0
				}
				else
				{
					( state.blueprint.geode_cost_ore - state.ore ) / state.ore_robots
				},
				if state.obsidian >= state.blueprint.geode_cost_obsidian
				{
					0
				}
				else
				{
					( state.blueprint.geode_cost_obsidian - state.obsidian ) / state.obsidian_robots
				},
			)
		},

		Command::Wait => 0,
	};
	let rounds_to_complete = rounds_to_complete as i32;
	let max_rounds: i32 = 3;
	let penalty: i32 = if rounds_to_complete > max_rounds
	{
		rounds_to_complete - max_rounds
	}
	else
	{
		0
	};
	
	let positives = total_geodes * 20 
		+ 2 * ( state.ore_robots + state.clay_robots + state.obsidian_robots )
		+ 1 * ( state.ore + state.clay + state.obsidian )
		+ 3 * ( command as u32 );
	return positives as i32
		- 2 * ( penalty * penalty )
		+ 500000;
}


impl Node
{
	fn new( state: &State, command: Command ) -> Node
	{
		return Node
		{
			state: state.clone(),
			command: command,
			score: compute_score( &state, command ),
		}
	}
}

impl fmt::Display for Node
{
	fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result
	{
		f.write_fmt( format_args!( "S {:3}  {:7} {}", self.score, self.command, self.state ) )
	}

}

const MINUTE_LIMIT: u32 = 32;
const BLUEPRINT_LIMIT: usize = 3;
const ORE_ROBOT_LIMIT: u32 = 12;
const CLAY_ROBOT_LIMIT: u32 = 14;

use itertools::Itertools;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

fn best_first( blueprint: &Blueprint ) -> u32
{
	let mut todo: Vec< Node > = Vec::new();
	let initial_state = State::new( blueprint );
	let n1 = Node::new( &initial_state, Command::Ore );
	todo.push( n1 );
	let n2 = Node::new( &initial_state, Command::Clay );
	todo.push( n2 );

	let mut best_geode_count: u32 = 0;
	let mut best_score: Vec<i32> = Vec::new();
	best_score.resize( MINUTE_LIMIT as usize + 2, 0 );

	let mut n: usize = 0;
	while todo.len() > 0
	{
	/*
		let mut best_next = todo.len();
		for n in 0..todo.len()
		{
			if best_next == todo.len()
				|| todo[ n ].score > todo[ best_next].score
			{
				best_next = n;
			}
		}
		*/

		//let mut node = todo.remove( best_next );
		let mut node = todo.pop().unwrap();
		//println!( "Processing from {}", node );
		loop
		{
		    //println!( "   {}", node.state );
			let can_afford = node.state.can_afford( node.command );
			node.state.advance();
			
			if node.state.minute > MINUTE_LIMIT
			{
				if node.state.geode > best_geode_count
				{
					best_score[ node.state.minute as usize ] = compute_score( &node.state, Command::Wait );
					best_geode_count = node.state.geode;
				}
				break;
			}
			else if can_afford 
			{
				node.state.buy( node.command );

				if node.state.ore_robots < ORE_ROBOT_LIMIT 
				{
					let child = Node::new( &node.state, Command::Ore );
					//println!( "      child {}", child );
					if child.score >= best_score[ node.state.minute as usize ]
					{
						todo.push( child );
					}
					else
					{
						println!( "       REJECTED" );
					}
				}
				if node.state.clay_robots < CLAY_ROBOT_LIMIT 
				{
					let child = Node::new( &node.state, Command::Clay );
					//println!( "      child {}", child );
					if child.score >= best_score[ node.state.minute as usize ]
					{
						todo.push( child );
					}
					else
					{
						println!( "       REJECTED" );
					}
				}
				if node.state.clay_robots > 0
				{
					let child = Node::new( &node.state, Command::Obsidian );
					//println!( "      child {}", child );
					if child.score >= best_score[ node.state.minute as usize ]
					{
						todo.push( child );
					}
					else
					{
						println!( "       REJECTED" );
					}
				}
				if node.state.obsidian_robots > 0
				{
					let child = Node::new( &node.state, Command::Geode );
					//println!( "      child {}", child );
					todo.push( child );
				}
				break;
			}
			else
			{
				node.state.wait();
			}
		}

		n += 1;
		if n % 10000000 == 0
		{
			println!( "BP{} TODO {}  best {}", blueprint.id, todo.len(), best_geode_count );
		}
	}

	return best_geode_count;

}

fn best_order( blueprint: &Blueprint ) -> u32
{

	let mut seeds: Vec< Vec< Command > > = Vec::new();

	for max_ore in 1..8
	{
		for max_clay in 1..8
		{
			for max_obsidian in 1..8
			{
				for max_early_geode in 0..4
				{

					let mut seed: Vec<Command> = Vec::new();
					for _ in 0..max_ore
					{
						seed.push( Command::Ore );
					}
					for _ in 0..max_clay
					{
						seed.push( Command::Clay );
					}
					for _ in 0..max_obsidian
					{
						seed.push( Command::Obsidian );
					}
					for _ in 0..max_early_geode
					{
						seed.push( Command::Geode );
					}
					seeds.push( seed );
				}
			}
		}
	}

	let mut best_geode_count: u32 = 0;
	let mut tested: HashSet<u64> = HashSet::new();
	
	for seed in &seeds
	{
		let seed_iter = seed.clone().into_iter().permutations( seed.len() );
		for order in seed_iter
		{
			// see if we've already done this one
			//println!( "{:?}", order );

			let mut hasher = DefaultHasher::new();
			for c in &order
			{
				hasher.write_u8( *c as u8 );
			}
			let res: u64 = hasher.finish();
			if tested.contains( &res )
			{
				continue;
			}

			tested.insert( res );
			let mut order = order.clone();

			let mut state = State::new( blueprint );
			for _minute in 1..24
			{
				let command = if order.len() == 0
				{
					// always try to buy geodes at the end
					Command::Geode
				}
				else
				{
					order[0]	
				};

				let can_afford = state.can_afford( command );

				state.advance();
				if can_afford
				{
					state.buy( command );
					if order.len() > 0
					{
						order.remove( 0 );
					}
				}
			}

			best_geode_count = cmp::max( best_geode_count, state.geode );
		}
	}

	return best_geode_count;
}

fn best_counts( blueprint: &Blueprint ) -> u32
{
	let mut best_geode_count: u32 = 0;

	for max_ore1 in 1..5
	{
		for max_clay1 in 1..5
		{
			for obsidian_trigger in 1..5
			{
				for max_ore2 in max_ore1..max_ore1+3
				{
					for max_clay2 in max_clay1..max_clay1+3
					{
						println!( "Trying BP{} maxes  {:2}+{} {:2}+{} - ({})  -", blueprint.id, 
							max_ore1, max_ore2 - max_ore1, 
							max_clay1, max_clay2 - max_clay1, 
							obsidian_trigger,
							);

						let mut state = State::new( blueprint );
						for _minute in 1..24
						{
							let max_ore = if state.obsidian_robots < obsidian_trigger { max_ore1 } else { max_ore2 };
							let max_clay = if state.obsidian_robots < obsidian_trigger { max_clay1 } else { max_clay2 };
							let max_obsidian = 99;

							let buy_ore = state.can_afford_ore() && state.ore_robots < max_ore;
							let buy_clay = state.can_afford_clay() && state.clay_robots < max_clay;
							let buy_obsidian = state.can_afford_obsidian() && state.obsidian_robots < max_obsidian;
							let buy_geode = state.can_afford_geode();

							println!( "     {}", state );

							state.advance();

							if buy_geode
							{
								state.buy_geode();
							}
							else if buy_obsidian
							{
								state.buy_obsidian();
							}
							else if buy_clay
							{
								state.buy_clay();
							}
							else if buy_ore
							{
								state.buy_ore();
							}
							else 
							{
								state.wait();
							}
						}
						println!( "  -> {}", state );

						best_geode_count = cmp::max( best_geode_count, state.geode );
					}
				}
			}
		}
	}

	return best_geode_count;
}

fn best_path( blueprint: &Blueprint ) -> u32
{
	let mut best_geode_count: u32 = 0;

	let mut todo: VecDeque< ( State, Command ) > = VecDeque::new();
	todo.push_back( ( State::new( blueprint ), Command::Wait ) ); 

	while let Some( ( state, command ) ) = todo.pop_back()
	{
		//println!( "Processing {:10?} for {}", command, state );
		
		let mut state = state.clone();
		state.ore += state.ore_robots;
		state.clay += state.clay_robots;
		state.obsidian += state.obsidian_robots;
		state.geode += state.geode_robots;
		state.minute += 1;

		if state.minute == 24
		{
			// We're done. No more commands after this.
			best_geode_count = cmp::max( best_geode_count, state.geode );
			continue;
		}

		match command
		{
			Command::Ore => {
				//println!( "  buying ore robot" );
				state.ore -= state.blueprint.ore_cost_ore;
				state.ore_robots += 1;
			},
			Command::Clay => {
				//println!( "  buying clay robot" );
				state.ore -= state.blueprint.clay_cost_ore;
				state.clay_robots += 1;
			},
			Command::Obsidian => {
				//println!( "  buying Obsidian robot" );
				state.ore -= state.blueprint.obsidian_cost_ore;
				state.clay -= state.blueprint.obsidian_cost_clay;
				state.obsidian_robots += 1;
			},
			Command::Geode => {
				//println!( "  buying Geode robot" );
				state.ore -= state.blueprint.geode_cost_ore;
				state.obsidian -= state.blueprint.geode_cost_obsidian;
				state.geode_robots += 1;
			},
			Command::Wait => {
				//println!( "  waiting" );
			},
		}

		state.command_history.push( command );

		//println!( "   new state: {}", state );
		let mut buy_count = 0;	
		if state.ore >= state.blueprint.ore_cost_ore && state.ore_robots < 5
		{
			//println!( "  +ore" );
			// we can afford another ore robot
			todo.push_back( ( state.clone(), Command::Ore ) );
			buy_count += 1;
		}
		
		if state.ore >= state.blueprint.clay_cost_ore && state.clay_robots < 6
		{
			//println!( "  +clay" );
			// we can afford another Clay robot
			todo.push_back( ( state.clone(), Command::Clay ) );
			buy_count += 1;
		}
		
		if state.ore >= state.blueprint.obsidian_cost_ore
			&& state.clay >= state.blueprint.obsidian_cost_clay
		{
			//println!( "  +obsidian" );
			// we can afford another Obsidian robot
			todo.push_back( ( state.clone(), Command::Obsidian ) );
			buy_count += 1;
		}
		
		if state.ore >= state.blueprint.geode_cost_ore
			&& state.obsidian >= state.blueprint.geode_cost_obsidian
		{
			//println!( "  +geode" );
			// we can afford another Geode robot
			todo.push_back( ( state.clone(), Command::Geode ) );
			buy_count += 1;
		}
		
		// waiting is always an option, but don't do it if we could buy all four types
		if buy_count < 4
		{
			//println!( "  +wait" );
			todo.push_back( ( state, Command::Wait ) );
		}
	}

	return best_geode_count;
}

use std::thread;

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

	let mut threads: Vec< thread::JoinHandle< ( u32, u32 ) > > = Vec::new();
	for i in 0..cmp::min( blueprints.len(), BLUEPRINT_LIMIT )
	{
		let bp = blueprints[i].clone();

		let handle = thread::spawn( move || -> ( u32, u32 )
		{
			let best_geode_count = best_first( &bp );
			let quality = bp.id * best_geode_count;
			println!( "BP {} -  geode_count {}   Quality {}", bp.id, best_geode_count, quality );
			return ( bp.id, best_geode_count );
		} );
		threads.push( handle );
	}

	let mut total_quality = 0;
	let mut geode_count_product = 1;

	for handle in threads
	{
		let ( bpid, geode_count ) = handle.join().unwrap();

		total_quality += bpid * geode_count;
		geode_count_product *= geode_count;
	}

	println!( "Total quality {}", total_quality );
	println!( "Geode count product {}", geode_count_product );
}

