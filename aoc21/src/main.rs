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

#[derive(Clone,Copy,Debug)]
enum SimpleOp 
{
	Add,
	Subtract,
	SubtractReverse,
	Multiply,
	Divide,
	DivideReverse,
}

fn match_pair( op: &Op ) -> ( String, String )
{
	
	match op
	{
		Op::Add( a, b ) 
		| Op::Subtract( a, b ) 
		| Op::Multiply( a, b )
		| Op::Divide( a, b ) 
		=> 
		{ 
			return ( a.clone(), b.clone() );
		}
		_ => panic!( "match_pair can only be called on add/subtract/multiply/divide ops: {:?}", op ),
	}
}

fn run_op( monkey_name: &String, monkeys: &HashMap< String, Op > ) -> Option< i64 >
{
	let op = monkeys.get( monkey_name ).unwrap();

	let a_value: Option< i64 >;
	let b_value: Option< i64 >;

	match op
	{
		Op::Human => return None,
		Op::Number( n ) => return Some( *n ),
		_ => {},
	}
	
	let ( a, b ) = match_pair( op );
	a_value = run_op( &a, monkeys );
	b_value = run_op( &b, monkeys );
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

fn compute_unknown_value( result: i64, known: i64, op: SimpleOp ) -> i64
{
	return match op
	{
		SimpleOp::Add => result - known, 			// result = known + human
		SimpleOp::Subtract => known - result, 		// result = known - human
		SimpleOp::SubtractReverse => result + known,// result = human - known
		SimpleOp::Multiply => result / known, 		// result = known * human
		SimpleOp::Divide => known / result,			// result = known / human
		SimpleOp::DivideReverse => known * result, 	// result = human / known
	};
}


fn make_human_equal( result: i64, known: i64, op: SimpleOp, human_branch: &String, monkeys: &HashMap< String, Op > ) -> i64
{
	let unknown_value = compute_unknown_value( result, known, op );
	let child_op = monkeys.get( human_branch ).unwrap();
	
	println!( "Determining {} = {} {:?} {} ( {} ) for {:?}",
		result, known, op, human_branch, unknown_value, child_op );
	let simple_op: SimpleOp = match child_op
	{
		Op::Human =>
		{
			return unknown_value;
		},
		Op::Number(_) => panic!( "A simple number monkey shouldn't be on the unresolved side" ),
		Op::Add(_,_) => SimpleOp::Add,
		Op::Subtract(_,_) => SimpleOp::Subtract,
		Op::Multiply(_,_) => SimpleOp::Multiply,
		Op::Divide(_,_) => SimpleOp::Divide,
	};

	let ( a, b ) = match_pair( child_op );

	let a_value = run_op( &a, &monkeys );
	let b_value = run_op( &b, &monkeys );

	return if a_value == Option::None
	{
		let simple_op = match simple_op
		{
			SimpleOp::Subtract => SimpleOp::SubtractReverse,
			SimpleOp::Divide => SimpleOp::DivideReverse,
			_ => simple_op,
		};

		make_human_equal( unknown_value, b_value.unwrap(), simple_op, &a, monkeys )
	}
	else
	{
		make_human_equal( unknown_value, a_value.unwrap(), simple_op, &b, monkeys )
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
		//println!( "monkey_name = {}, details={}, args={:?}", monkey_name, details, args );
		let op = if args.len() == 1
		{
			if monkey_name == "humn"
			{
				Op::Human
			}
			else
			{
				Op::Number( args[0].parse::<i64>().unwrap() )
			}
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

//	println!( "root: {:?}", run_op( &"root".to_string(), &monkeys ) );

	let root_op = monkeys.get( &"root".to_string() ).unwrap();
	let ( a, b ) = match_pair( root_op );

	let a_value = run_op( &a, &monkeys );
	let b_value = run_op( &b, &monkeys );

	let human_value = if a_value == Option::None
	{
		make_human_equal( b_value.unwrap(), 0, SimpleOp::Add, &a, &monkeys )
	}
	else
	{
		make_human_equal( a_value.unwrap(), 0, SimpleOp::Add, &b, &monkeys )
	};

	println!( "Human value: {}", human_value );
}

