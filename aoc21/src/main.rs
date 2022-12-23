#![allow(unused_imports)]
use std::io::{ self, BufRead };
use std::fmt;
use std::cmp;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;


#[derive(Clone,Debug)]
enum Op 
{
	Number( i64 ),
	Add( String, String ),
	Subtract( String, String),
	Multiply( String, String),
	Divide( String, String),
	Human,
}

fn match_pair( op: Op ) -> ( String, String )
{
	
	match op
	{
		Op::Add( a, b ) 
		| Op::Subtract( a, b ) 
		| Op::Multiply( a, b )
		| Op::Divide( a, b ) 
		=> 
		{ 
			return ( a, b );
		}
		_ => panic!( "match_pair can only be called on add/subtract/multiply/divide ops: {:?}", op ),
	}
}

fn run_op( monkey_name: &String, monkeys: &HashMap< String, Op > ) -> Option< i64 >
{
	let op = monkeys.get( monkey_name ).unwrap();

	let mut a_value: Option< i64 >;
	let mut b_value: Option< i64 >;

	match op
	{
		Op::Human => return None,
		Op::Number( n ) => return Some( *n ),
		Op::Add( a, b ) 
		| Op::Subtract( a, b ) 
		| Op::Multiply( a, b )
		| Op::Divide( a, b ) 
		=> 
		{ 
			a_value = run_op( a, monkeys );
			b_value = run_op( b, monkeys );
		}
	}
	
	if a_value == Option::None || b_value == Option::None
	{
		return Option::None;
	}

	let a_value = a_value.unwrap();
	let b_value = b_value.unwrap();

	return match op
	{
		Op::Add( _, _ ) => Some( a_value + b_value ),
		Op::Subtract( _, _ ) => Some( a_value - b_value ),
		Op::Multiply( _, _ ) => Some( a_value * b_value ),
		Op::Divide( _, _ ) => Some( a_value / b_value ),
		_ => panic!("How did we get here? {:?}", op ),
	};
}


fn main()
{
	let mut lines = io::stdin().lock().lines();
	

	let mut monkeys: HashMap< String, Op > = HashMap::new();

	while let Some( line ) = lines.next()
	{
		let cur_line = line.unwrap();
		
		let ( monkey_name, details ) = cur_line.split_once( ": " ).unwrap();

		let args: Vec<&str> = details.split( ' ' ).collect();
		println!( "monkey_name = {}, details={}, args={:?}", monkey_name, details, args );
		let op = if args.len() == 1
		{
			Op::Number( args[0].parse::<i64>().unwrap() )
		}
		else
		{
			assert!( args.len() == 3 );
			let a = args[0].to_string();
			let b = args[2].to_string();
			match args[1]
			{
				"+" => Op::Add( a, b ),
				"-" => Op::Subtract( a, b ),
				"*" => Op::Multiply( a, b ),
				"/" => Op::Divide( a, b ),
				_ => panic!( "Unknown op {}", args[1] ),
			}
		};

		monkeys.insert( monkey_name.to_string(), op );
	}

	println!( "root: {:?}", run_op( &"root".to_string(), &monkeys ) );

	//let root_op = monkeys.get( &"root".to_string(), &monkeys ).unwrap();


}

