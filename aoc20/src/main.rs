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
	number: i64,
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

	let mult = 811589153;
	let mix_count = 10;
	while let Some( line ) = lines.next()
	{
		let cur_line = line.unwrap();

		numbers.push( Num{ id: numbers.len(), number: mult * cur_line.parse::<i64>().unwrap() } );
	}

	dump_numbers( &numbers );
	println!("");

	for _ in 0..mix_count
	{
		let len = numbers.len() as i64;
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
				let moving = numbers.remove( ind );
				let mut insert_before = ind as i64;

				//println!( "");

				// first add how many positions it wants to move
				insert_before += to_move;

				// then make it positive if it isn't
				let len = numbers.len() as i64;
				if insert_before < 0
				{
					let n = insert_before.abs() / len;
					//println!( "insert_before={}, n={}", insert_before, n );
					insert_before += ( n + 1 ) * len;
				}

				// then make it in the right range
				insert_before = insert_before % len;

				/*
				println!( "i={}, ind={}, insert_before={}", i, ind, insert_before );
				let prev = ( (insert_before + len  - 1 ) % len ) as usize;
				let next = insert_before as usize;
				println!( "{} moves between {} and {}", to_move,
					numbers[ prev ].number,
					numbers[ next ].number,
				);
				*/

				if insert_before == 0
				{
					numbers.push( moving );
				}
				else
				{
					numbers.insert( insert_before as usize, moving );
				}
				//dump_numbers( &numbers );
			}
			else
			{
				//println!( "\n0 does not move" );
				//dump_numbers( &numbers );
			}
		}
		dump_numbers( &numbers );
	}


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
