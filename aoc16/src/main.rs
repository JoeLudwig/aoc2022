use std::io::{ self, BufRead };
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::cmp;



type TunnelMap = HashMap<i32, i32>;


#[derive(Eq,PartialEq,Clone,Debug)]
struct Valve
{
	id: i32,
	flow_rate: i32,
	exits: TunnelMap,
}

type ValveMap = HashMap<i32, Valve>;

fn name_to_id( name: &str ) -> i32
{
	assert!( name.len() == 2 );
	let mut id: i32 = 0;
	for c in name.chars()
	{
		id *= 100;
		id += ( c as i32 - 'A' as i32 ) + 1;
	}
	return id;
}

const AA_ID: i32 = 101;

fn id_to_name( id: i32 ) -> String
{
	let mut id = id;
	let mut out: String = String::new();
	while id != 0
	{
		let c = ( id % 100 - 1 ) + 'A' as i32;
		out.insert( 0, c as u8 as char);

		id /= 100;
	}
	return out;
}

impl Valve
{
	fn parse( line: &String ) -> Valve
	{
		let line = line.replace( "Valve ", "" );
		let (name, line ) = line.split_once( ' ' ).unwrap();
		let line = line.replace( "has flow rate=", "" );
		let line = line.replace( "; tunnels lead to valves", "," );
		let line = line.replace( "; tunnel leads to valve", "," );
		let args: Vec< &str > = line.split( ", " ).collect();

		let flow_rate = args[0].parse::<i32>().unwrap();
		let mut exits: TunnelMap = TunnelMap::new();
		for exit in &args[ 1..args.len() ]
		{
			exits.insert( name_to_id( exit ), 1 );
		}

		return Valve 
		{
			id: name_to_id( name ),
			flow_rate: flow_rate,
			exits: exits,
		}
	}
}

impl fmt::Display for Valve 
{
	fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result
	{
		let mut s = String::from( "" );
		let mut dests: Vec<String> = Vec::new();
		for k in self.exits.keys()
		{
			dests.push( id_to_name( *k ) );
		}
		dests.sort();

		for d in dests
		{
			let c = self.exits.get( &name_to_id( &d ) ).unwrap();
			if s.len() > 0
			{
				s.push_str( "   " );
			}

			s.push_str( &format!( "{} {:2}", d, c ) );
		}

		f.write_fmt( format_args!( "V {} {:2}   {}", id_to_name( self.id ), self.flow_rate, s ) )
	}
}

fn dump_valves( valves: &ValveMap )
{
	let mut valve_names: Vec<String> = Vec::new();
	for k in valves.keys()
	{
		valve_names.push( id_to_name( *k ) );
	}
	valve_names.sort();
	
	for name in valve_names
	{
		println!( " {}", valves.get( &name_to_id( &name ) ).unwrap() );
	}
	println!("");
}

fn add_secondary_links( valves: &ValveMap ) -> ( bool, ValveMap )
{
	let mut collapsed_any = false;
	let mut out: ValveMap = ValveMap::new();
	for ( id, valve ) in valves.iter() 
	{
		// don't even bother with zero flow rate valves. They won't survive the first pass
		// except our start point. Should probably keep that
		//if valve.flow_rate == 0 && name != "AA"
		//{
		//	collapsed_any = true;
		//	continue;
		//}

		let mut new_exits: TunnelMap = TunnelMap::new();
		for (d, c) in &valve.exits
		{
			// keep the link to the valve itself
			new_exits.insert( *d, c.clone() );
		}

		for (d, c) in &valve.exits
		{
			let dest = valves.get( d ).unwrap();
			for (d2, c2 ) in &dest.exits
			{
				// don't bother looping pack to our start node
				if d2 == id
				{
					continue;
				}
				let cost = c + c2;
				//println!( "Looking at {} -> {} -> {}  cost: {:2}", name, d, d2, cost );

				match new_exits.get_mut( d2 )
				{
					Some( found_cost ) =>
					{
						if cost < *found_cost
						{
						//	println!( "  Updating cost {} -> {}", found_cost, cost );
							collapsed_any = true;
							*found_cost = cost;
						}
					},
					None =>
					{
						collapsed_any = true;
						//println!( "  inserting new tunnel");
						new_exits.insert( *d2, cost );
					},
				}
			}
		}

		let mut updated_valve = valve.clone();
		updated_valve.exits = new_exits;
		out.insert( *id, updated_valve );
	}
	return ( collapsed_any, out );
}

#[derive(Eq,PartialEq,Clone,Debug)]
struct Path
{
	so_far: String,
	curr : i32,
	remaining: Vec<i32>,
	flow_rate: i32,
	total_flow: i32,
	rounds_left: i32,
}

impl Path
{
	fn children( &self, valves: &ValveMap ) -> Option< Vec< Path > >
	{
		if self.remaining.len() == 0
		{
			return Option::None;
		}

		
		let mut children: Vec< Path > = Vec::new();
		for i in 0..self.remaining.len()
		{
			let next = self.remaining[i];
			let from = valves.get( &self.curr ).unwrap();

			let cost = from.exits.get( &next ).unwrap() + 1;
			
			if cost > self.rounds_left
			{
				// this one isn't an option because it would push us over 30 rounds
				continue;
			}
			let dest = valves.get( &next ).unwrap();

			let mut new_child = Path
			{
				so_far: self.so_far.clone(),
				curr: next,
				remaining: self.remaining.clone(),
				flow_rate: self.flow_rate + dest.flow_rate,
				total_flow: self.total_flow + self.flow_rate * cost,
				rounds_left: self.rounds_left - cost,
			};
			new_child.so_far.push_str( &id_to_name( next ) );
			new_child.remaining.remove( i );

			children.push( new_child );
		}

		if children.len() > 0 
		{
			return Option::Some( children );
		}
		else
		{
			return Option::None;
		}
	}
}


fn run_flow( path: &Vec<i32>, valves: &ValveMap ) -> i32
{
	let mut total_flowed = 0;
	let mut flow_rate: i32 = 0;
	let mut rounds_left: i32 = 30;
	for i in 0..path.len()
	{
		let old_loc = valves.get( if i == 0 { &AA_ID } else { &path[ i-1 ] } ).unwrap();
		let loc = valves.get( &path[ i ] ).unwrap();

		let cost = old_loc.exits.get( &loc.id ).unwrap();
		
		if cost + 1 >= rounds_left
		{
			break;
		}
		
		total_flowed += flow_rate * ( cost + 1 );
		rounds_left -= cost + 1;
		flow_rate += loc.flow_rate;
	}

	total_flowed += flow_rate * rounds_left;
	return total_flowed;
}


fn best_flow_for_valves( good_valves: &Vec<i32>, valves: &ValveMap ) -> i32
{
	let mut visited: HashSet<String> = HashSet::new();
	
	let mut todo: VecDeque< Path > = VecDeque::new();

	todo.push_back( Path
	{ 
		so_far: "AA".to_string(), 
		remaining: good_valves.clone(),
		curr: AA_ID,
		total_flow: 0,
		flow_rate: 0,
		rounds_left: 26,
	} );

	let mut max_flow = 0;
	let mut n = 0;
	let mut calcs = 0;
	while let Some( path ) = todo.pop_back()
	{
		match path.children( &valves )
		{
			None =>
			{
				// should be a real path. Calculate it
				let total_flow = path.total_flow + path.flow_rate * path.rounds_left;
				//println!( "calculating {} {}", total_flow, path.to_string() );

				max_flow = cmp::max( total_flow, max_flow );
				calcs += 1;
			},
			Some( children ) =>
			{
				for child in children
				{
					if !visited.contains( &child.so_far )
					{
						visited.insert( child.so_far.clone() );
						todo.push_back( child );
					}
				}
			},
		}

//		n += 1;
//		if ( n % 100000 ) == 0
//		{
//			println!( "TODO: {:?}  max_flow: {}    calcs: {}", todo.len(), max_flow, calcs );
//		}
	}

	return max_flow;
}

fn main()
{
	let mut lines = io::stdin().lock().lines();

	
	let mut valves = HashMap::new();

	while let Some( line ) = lines.next()
	{
		let cur_line = line.unwrap();

		let valve = Valve::parse( &cur_line );
		valves.insert( valve.id, valve );
	}

	dump_valves( &valves );

	loop
	{
		let ( collapsed_any, new_valves ) = add_secondary_links( &valves );
		if !collapsed_any
		{
			break;
		}
		else
		{
			valves = new_valves;
			//dump_valves( &valves );
		}
	}

	dump_valves( &valves );

	let mut good_valves: Vec<i32> = Vec::new();
	for ( _, valve ) in &valves
	{
		if valve.flow_rate > 0
		{
			good_valves.push( valve.id );
		}
	}


	let mut max_flow = 0;
	let combo_count = (2 as u32 ).pow( good_valves.len() as u32 );
	for mask in 0..combo_count
	{
		let mut me: Vec< i32 > = Vec::new();
		let mut elephant: Vec< i32 > = Vec::new();

		for bit in 0..good_valves.len()
		{
			if ( ( 1 << bit ) & mask ) != 0
			{
				me.push( good_valves[ bit ] );
			}
			else
			{
				elephant.push( good_valves[ bit ] );
			}
		}

		let my_flow = best_flow_for_valves( &me, &valves );
		let elephant_flow = best_flow_for_valves( &elephant, &valves );

		let total_flow = my_flow + elephant_flow;
		max_flow = cmp::max( total_flow, max_flow );


		if ( mask % 100 ) == 0
		{
			println!( " {:5} {}", mask, max_flow );
		}
	}

	println!( "Max flow: {}", max_flow );
}

