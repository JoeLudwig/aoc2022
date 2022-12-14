use std::io::{ self, BufRead };

use std::cmp::Ordering;

#[derive(Eq,PartialEq,Clone,Hash,Debug)]
pub enum Element
{
	List( Box< Vec< Element > > ),
	Int( i32 ),
}
	

fn parse( s: String ) -> Element
{
	if s.starts_with( '[' )
	{
		let mut depth = 0;
		let mut tokens: Vec<String> = Vec::new();
		let mut token = String::new();
		
		
		for c in s.chars()
		{
			let mut add_to_token = false;
			let mut finish_token = false;
			match c
			{
				'[' => 
				{
					depth += 1;
					add_to_token = depth > 1;
				}
				']' => 
				{
					finish_token = depth == 1;
					add_to_token = depth > 1;
					depth -= 1
				},
				',' => 
				{
					finish_token = depth == 1;
					add_to_token = depth > 1;
				}
				_ => add_to_token = true,
			}

			if add_to_token
			{
				token.push( c );
			}
			if finish_token && token.len() > 0
			{
				tokens.push( token );
				token = String::new();
			}
		}
			
		//println!( "Parsing list: {:?}", tokens );
		let mut parsed: Box< Vec< Element > > = Box::new( Vec::new() );

		for child_str in tokens
		{
			parsed.push( parse( child_str ) );
		}
		
		return Element::List( parsed );
	}

	//println!( "Parsing int: {}", s );
	return Element::Int( str::parse::<i32>( &s ).unwrap() );
}

fn in_order( a: &Element, b: &Element ) -> Ordering
{
	//println!( "     {:?} <= {:?}", a, b );
	match a
	{
		Element::Int( na ) =>
		{
			match b
			{
				Element::Int( nb ) => 
				{
					if na == nb
					{
						return Ordering::Equal;
					}
					else if na < nb
					{
						return Ordering::Less;
					}
					else
					{
						return Ordering::Greater;
					}
				},
				Element::List( lb ) =>
				{
					let la = Element::List( Box::new( vec![ Element::Int( na.clone() ) ] ) );
					return in_order( &la, b );
				}
			}
		},

		Element::List( la ) =>
		{
			match b
			{
				Element::Int( nb ) => 
				{
					let lb = Element::List( Box::new( vec![ Element::Int( nb.clone() ) ] ) );
					return in_order( a, &lb );
				},
				Element::List( lb ) =>
				{
					let mut i = 0;
					while i < la.len() && i < lb.len()
					{
						let res = in_order( &la[i], &lb[i] );
						if res != Ordering::Equal
						{
							return res;
						}

						i += 1;
					}

					if la.len() == lb.len()
					{
						return Ordering::Equal;
					}
					else if la.len() < lb.len()
					{
						return Ordering::Less;
					}
					else
					{
						return Ordering::Greater;
					}
				}
			}
		}
	}
}

fn main()
{
	let mut lines = io::stdin().lock().lines();

	let mut i = 1;
	let mut index_sum = 0;

	let first = Element::List( Box::new( vec![ Element::Int( 2 ) ] ) );
	let second = Element::List( Box::new( vec![ Element::Int( 6 ) ] ) );
	let mut packets = vec![
		first.clone(),
		second.clone(),
	];

	while let Some( line ) = lines.next()
	{
		let cur_line = line.unwrap();
		if cur_line.len() == 0
		{
			continue;
		}

		let a = parse( cur_line );

		let cur_line = lines.next().unwrap().unwrap();
		let b = parse( cur_line );

		let res = in_order( &a, &b );
		println!( "Comparing {:?} to {:?} => {:?}", a, b, res );

		if res != Ordering::Greater
		{
			index_sum += i;
		}

		packets.push( a );
		packets.push( b );
		i += 1;
	}
	
	packets.sort_by( in_order );

	let mut product = 1;
	for i in 0..packets.len()
	{
		if in_order( &first, &packets[i] ) == Ordering::Equal
		{
			product *= i + 1;
		}
		else if in_order( &second, &packets[i] ) == Ordering::Equal
		{
			product *= i + 1;
		}
	}

	println!( "index sum: {}", index_sum );
	println!( "decoder key: {}", product );
}
