#![allow(unused_imports)]
use std::io::{ self, BufRead };
use std::fmt;
use std::cmp;
use std::collections::HashSet;
use std::collections::VecDeque;


#[derive(Clone,Copy,Debug)]
struct Num
{
	id: usize,
	number: i32,
}


fn dump_numbers( numbers: &Vec< Num > )
{
	let mut out = String::new();
	for n in numbers
	{
		if out.len() != 0
		{
			out.push_str( ", " );
		}

		out.push_str( &format!( "{}", n.number ) );
	}
	println!( "{}", out );
}

fn main()
{
	let mut lines = io::stdin().lock().lines();
	
	let mut numbers: Vec< Num > = Vec::new();

	while let Some( line ) = lines.next()
	{
		let cur_line = line.unwrap();

		numbers.push( Num{ id: numbers.len(), number: cur_line.parse::<i32>().unwrap() } );
	}

	dump_numbers( &numbers );
	println!("");

	let len = numbers.len() as i32;
	for i in 0..numbers.len()
	{
		// find the number that matches i, wherever it currently is
		let mut ind = 0;
		for test in 0..numbers.len()
		{
			if numbers[test].id == i
			{
				ind = test;
				break;
			}
		}

		if numbers[ ind ].number != 0
		{
			let to_move = numbers[ind].number;
			let mut insert_after = ind as i32 + to_move;

			println!( "");
			if insert_after < 0
			{
				insert_after -= 1;
				let n = insert_after.abs() / len;
				println!( "insert_after={}, n={}", insert_after, n );
				insert_after += ( n + 1 ) * len;
			}

			insert_after = insert_after % len;
			/*if insert_after > ind as i32
			{
				// I'll remove the thing at ind first, which implicitly increases 
				// insert_after by one. So subtract that one back out
				insert_after -= 1;
			} */
			//insert_after = if insert_after <= ind as i32 { insert_after } else { insert_after - 1 };
			println!( "i={}, ind={}, insert_after={}", i, ind, insert_after );
			let prev = ( (insert_after + len ) % len ) as usize;
			let next = ( ( (insert_after + 1 ) % len ) as usize ) % numbers.len();
			println!( "{} moves between {} and {}", to_move,
		 		numbers[ prev ].number,
		 		numbers[ next ].number,
			);
			/*
			*/

			if insert_after as usize == numbers.len() || insert_after == 0
			{
				let moving = numbers.remove( ind );
				numbers.push( moving );
			}
			else
			{
				if insert_after > ind as i32
				{
					// we're trying to insert after a slot, but removing moves that slot to the left, 
					// so the insert call below (which is insert before) is essentially insert after
					let moving = numbers.remove( ind );
					numbers.insert( insert_after as usize, moving );
				}
				else
				{

					let moving = numbers.remove( ind );
					numbers.insert( insert_after as usize + 1, moving );
				}
			}
			dump_numbers( &numbers );
		}
		else
		{
			println!( "\n0 does not move" );
			dump_numbers( &numbers );
		}
	}

	dump_numbers( &numbers );

	let mut ind = 0;
	for test in 0..numbers.len()
	{
		if numbers[test].number == 0
		{
			ind = test;
			break;
		}
	}

	let a = numbers[ (ind + 1000 ) % numbers.len() ].number;
	let b = numbers[ (ind + 2000 ) % numbers.len() ].number;
	let c = numbers[ (ind + 3000 ) % numbers.len() ].number;
	println!( "coords: {}", a+b+c);
}
