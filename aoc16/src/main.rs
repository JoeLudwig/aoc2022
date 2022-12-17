use std::io::{ self, BufRead };
use std::collections::HashMap;
//use std::collections::HashSet;
use std::fmt;



type TunnelMap = HashMap<String, i32>;

#[derive(Eq,PartialEq,Clone,Debug)]
struct Valve
{
	name: String,
	flow_rate: i32,
	exits: TunnelMap,
}

type ValveMap = HashMap<String, Valve>;

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
			exits.insert( exit.to_string(), 1 );
		}

		return Valve 
		{
			name: name.to_string(),
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
			dests.push( k.to_string() );
		}
		dests.sort();

		for d in dests
		{
			let c = self.exits.get( &d ).unwrap();
			if s.len() > 0
			{
				s.push_str( "   " );
			}

			s.push_str( &format!( "{} {:2}", d, c ) );
		}

		f.write_fmt( format_args!( "V {} {:2}   {}", self.name, self.flow_rate, s ) )
	}
}

fn dump_valves( valves: &ValveMap )
{
	let mut valve_names: Vec<String> = Vec::new();
	for k in valves.keys()
	{
		valve_names.push( k.to_string() );
	}
	valve_names.sort();
	
	for name in valve_names
	{
		println!( " {}", valves.get( &name ).unwrap() );
	}
	println!("");
}

fn add_secondary_links( valves: &ValveMap ) -> ( bool, ValveMap )
{
	let mut collapsed_any = false;
	let mut out: ValveMap = ValveMap::new();
	for ( name, valve ) in valves.iter() 
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
			new_exits.insert( d.to_string(), c.clone() );
		}

		for (d, c) in &valve.exits
		{
			let dest = valves.get( d ).unwrap();
			for (d2, c2 ) in &dest.exits
			{
				// don't bother looping pack to our start node
				if d2 == name
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
						new_exits.insert( d2.clone(), cost );
					},
				}
			}
		}

		let mut updated_valve = valve.clone();
		updated_valve.exits = new_exits;
		out.insert( name.clone(), updated_valve );
	}
	return ( collapsed_any, out );
}

fn run_flow( path: &Vec<String>, valves: &ValveMap ) -> i32
{
	let mut total_flowed = 0;
	let mut flow_rate: i32 = 0;
	let mut rounds_left: i32 = 30;
	for i in 1..path.len()
	{
		let old_loc = valves.get( &path[ i-1 ] ).unwrap();
		let loc = valves.get( &path[ i ] ).unwrap();

		let cost = old_loc.exits.get( &loc.name ).unwrap();
		
		assert!( cost + 1 < rounds_left );
		
		total_flowed += flow_rate * ( cost + 1 );
		rounds_left -= cost + 1;
		flow_rate += loc.flow_rate;
	}

	total_flowed += flow_rate * rounds_left;
	return total_flowed;
}

fn main()
{
	let mut lines = io::stdin().lock().lines();

	
	let mut valves = HashMap::new();

	while let Some( line ) = lines.next()
	{
		let cur_line = line.unwrap();

		let valve = Valve::parse( &cur_line );
		valves.insert( valve.name.clone(), valve );
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
			dump_valves( &valves );
		}
	}

	let mut good_valves: Vec<String> = Vec::new();
	for ( name, valve ) in &valves
	{
		if valve.flow_rate > 0
		{
			good_valves.push( name.clone() );
		}
	}

	//let path: Vec<String> = [ "AA", "FJ", "QN", "PY", "AW", "FY", "UV" ].map(String::from).to_vec();
	let path: Vec<String> = [ "AA", "DD", "BB", "JJ", "HH", "EE", "CC" ].map(String::from).to_vec();

/*
	let mut path: Vec<String> = vec![ "AA".to_string() ];
	for i in 0..good_valves.len()
	{
		path.push( good_valves[ i ] );

		for j in 0.good_valves.len()
		{
			if i != j 
			{
				path.push( good_valves[j] );
			}
		}
				
		
	}
	*/

	let total_flow = run_flow( &path, &valves );
	println!( "total_flow: {}", total_flow );

}

