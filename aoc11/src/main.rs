use std::io::{ self, BufRead };

use std::cmp;

#[derive(Eq,PartialEq,Clone,Hash,Debug)]
enum Operator
{
	MultParam,
	AddParam,
	Square,
}
	
#[derive(Eq,PartialEq,Clone,Hash,Debug)]
struct Monkey
{
	items: Vec<i64>,

	operator: Operator,
	param: i64,

	divisor: i64,
	throw_false: i32,
	throw_true: i32,
	
	inspections: i64,
}


fn main()
{
	let mut lines = io::stdin().lock().lines();

	let mut monkey: Vec<Monkey> = Vec::new();

	let mut max_value = 1;
	while let Some( line ) = lines.next()
	{
		let cur_line = line.unwrap();

		// skip the first line
		assert!( cur_line.starts_with( "Monkey " ) );

		let cur_line = lines.next().unwrap().expect("items");
		let ( prefix, items ) = cur_line.split_once( ": " ).unwrap();
		assert!( prefix == "  Starting items" );
		// items = "122, 345"
		let items = items.replace( ",", "" );
		let items: Vec< &str > = items.split( ' ' ).collect();
		
		let cur_line = lines.next().unwrap().expect("op");
		let args: Vec< &str > = cur_line.split( ' ' ).collect();
		// [ "", "", "Operator:", "new", "=", "old", "+", "345" ]
		// [ "", "", "Operator:", "new", "=", "old", "*", "old" ]
		assert!( args[2] == "Operation:" && args[4] == "=" );
		let mut op = match args[6] {
			"+" => Operator::AddParam,
			"*" => Operator::MultParam,
			_ => panic!( "Unknown operation" ),
		};
		let mut param = 0;
		if op == Operator::MultParam && args[7] == "old"
		{
			op = Operator::Square;
		}
		else
		{
			param = args[7].parse::<i64>().unwrap();
		}

		let cur_line = lines.next().unwrap().expect("op");
		let ( prefix, arg ) = cur_line.rsplit_once( ' ' ).unwrap();
		// [ "  Test: divisible by", "35" ]
		assert!( prefix == "  Test: divisible by" );
		let divisor = arg.parse::<i64>().unwrap();

		let cur_line = lines.next().unwrap().expect("op");
		let ( prefix, arg ) = cur_line.rsplit_once( ' ' ).unwrap();
		// [ "    If true: throw to monkey", "3" ]
		assert!( prefix == "    If true: throw to monkey" );
		let throw_true = arg.parse::<i32>().unwrap();

		let cur_line = lines.next().unwrap().expect("op");
		let ( prefix, arg ) = cur_line.rsplit_once( ' ' ).unwrap();
		// [ "    If false: throw to monkey", "6" ]
		assert!( prefix == "    If false: throw to monkey" );
		let throw_false = arg.parse::<i32>().unwrap();

		let mut item_vec: Vec<i64> = Vec::new();
		for &item in &items
		{
			item_vec.push( item.parse::<i64>().unwrap() );
		}

		monkey.push( Monkey 
		{
			items: item_vec,
			operator: op,
			param: param,
			divisor: divisor,
			throw_true: throw_true,
			throw_false: throw_false,
			inspections: 0,
		} );

		max_value *= divisor;

		match lines.next()
		{
			Some( _ ) => {},
			None => break,
		}

	}
	
	//let n = 20;
	let n = 10000;
	for round in 0..n
	{
		for m in 0..monkey.len()
		{
			let mut throws: Vec<(usize, i64 ) > = Vec::new();
			let monk = &mut monkey[ m ];
			for item in &monk.items
			{
				let item = match monk.operator
				{
					Operator::AddParam => item + monk.param,
					Operator::MultParam => item * monk.param,
					Operator::Square => item * item,
				};
				//let item = item / 3;
				// don't care about the actual value except as a multiple of the divisors
				let item = item % max_value;

				let target = if ( item % monk.divisor ) == 0 { monk.throw_true } else { monk.throw_false };

				throws.push( ( target as usize, item ) );

				monk.inspections += 1;
			}
			monk.items.clear();

			for ( target, item ) in throws
			{
				monkey[ target ].items.push( item );
			}

		}
		println!( "Round: {}", round );

		let mut mid = 0;
		for m in &monkey
		{
	//		println!("  Monkey {}: {:?}", mid, m.items );
			mid += 1;
		}
	}
	
	let mut inspections: Vec<i64> = Vec::new();
	for m in monkey
	{
		inspections.push( m.inspections );
	}

	inspections.sort();
	inspections.reverse();

	println!( "Monkey Business: {}", inspections[0] * inspections[1] );
}
