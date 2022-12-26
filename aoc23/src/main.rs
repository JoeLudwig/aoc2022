#![allow(unused_imports)]
use std::io::{ self, BufRead };
use std::fmt;
use std::cmp;
use std::ops::Index;
use std::ops::IndexMut;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;


type Pt = ( i32, i32 );

#[derive(Clone,Copy,Debug)]
struct Elf
{
	pos: Pt,
	proposed_pos: Option< Pt >,
}

impl Elf
{
	fn new( pos: Pt ) -> Elf
	{
		return Elf
		{
			pos,
			proposed_pos: Option::None,
		};
	}
}


#[derive(Clone,Copy,Debug)]
struct Bounds
{
	top: i32,
	bottom: i32,
	left: i32,
	right: i32,
}

type ElfVec = Vec< Elf >;
fn elf_bounds( elves: &ElfVec ) -> Bounds
{
	let mut top: i32 = i32::MAX;
	let mut left: i32 = i32::MAX;
	let mut bottom: i32 = i32::MIN;
	let mut right: i32 = i32::MIN;

	for elf in elves
	{
		top = cmp::min( elf.pos.1, top );
		bottom = cmp::max( elf.pos.1, bottom );
		left = cmp::min( elf.pos.0, left );
		right = cmp::max( elf.pos.0, right );
	}

	return Bounds { top, bottom, left, right };
}

fn elf_occupied( elves: &ElfVec ) -> HashSet< Pt >
{
	let mut occupied: HashSet< Pt > = HashSet::new();
	for elf in elves
	{
	 	occupied.insert( elf.pos );	
	}

	return occupied;
}

fn dump_elves( elves: &ElfVec )
{
	let bounds = elf_bounds( elves );
	let occupied = elf_occupied( elves );

	for y in bounds.top..( bounds.bottom + 1 )
	{
		let y = y as i32;
		let mut line = String::new();

		for x in bounds.left..( bounds.right + 1 )
		{
			let x = x as i32;
			line.push( if occupied.contains( &( x, y ) ) { '#' } else { '.' } );
		}

		println!( "{}", line );
	}
}


fn elf_proposal( elf : &Elf, occupied: &HashSet< Pt >, round: usize ) -> Option< Pt >
{
	let ( x, y ) = elf.pos;

	let nw = occupied.contains( &( x - 1, y - 1 ) ); 
	let n = occupied.contains( &( x, y - 1 ) ); 
	let ne = occupied.contains( &( x + 1, y - 1 ) ); 

	let sw = occupied.contains( &( x - 1, y + 1 ) ); 
	let s = occupied.contains( &( x, y + 1 ) ); 
	let se = occupied.contains( &( x + 1, y + 1 ) ); 

	let e = occupied.contains( &( x + 1, y ) ); 
	let w = occupied.contains( &( x - 1, y ) ); 

	// if nothing is occupied, the elf won't move
	if !nw && !n && !ne && !e && !w && !sw && !s && !se
	{
		return Option::None;
	}

	let order = vec![
		( !nw && !n && !ne, Option::Some( ( x, y - 1 ) ) ),
		( !sw && !s && !se, Option::Some( ( x, y + 1 ) ) ),
		( !nw && !w && !sw, Option::Some( ( x - 1, y ) ) ),
		( !ne && !e && !se, Option::Some( ( x + 1, y ) ) ),
	];


	for n in 0..4
	{
		let n = ( n + round ) % 4;
		let ( test, res ) = order[ n ];
		if test
		{
			return res;
		}
	}

	return Option::None;
}


fn main()
{
	let mut lines = io::stdin().lock().lines();
	

	let mut elves = ElfVec::new();

	let mut y: i32 = 0;

	while let Some( line ) = lines.next()
	{
		let cur_line = line.unwrap();

		for ( x, c ) in cur_line.char_indices()
		{
			if c == '#'
			{
				elves.push( Elf::new( ( x as i32, y as i32 ) ) );
			}
		}

		y += 1;
	}

	dump_elves( &elves );
	println!("");

	let mut round: usize = 0;
	loop 
	{
		let occupied = elf_occupied( &elves );
		let mut proposals = HashMap::<Pt, u32>::new();
		
		// make all the proposals
		for elf in &mut elves
		{
			elf.proposed_pos = elf_proposal( elf, &occupied, round );
			match elf.proposed_pos
			{
				Option::None => {},
				Option::Some( pt ) =>
				{
					match proposals.get_mut( &pt )
					{
						Option::None => { proposals.insert( pt, 1 ); },
						Option::Some( old_count ) => *old_count += 1,
					}
				},
			}
		}

		// execute all the proposals
		let mut elves_moved = 0;
		for elf in &mut elves
		{
			match elf.proposed_pos
			{
				Option::None => {},
				Option::Some( dest ) =>
				{
					match proposals.get( &dest )
					{
						Option::Some( 1 ) =>
						{
							// exactly one elf proposed moving here
							elf.pos = dest;
							elves_moved += 1;
						},
						_ => {}, // otherwise do nothing
					}
				}
			}
		}

		if elves_moved == 0
		{
			println!( "No elves moved in round {}", round + 1 );
			break;
		}

		//dump_elves( &elves );
		//println!( "" );

		round += 1;
	}

	let bounds = elf_bounds( &elves );
	let area = ( 1 + bounds.bottom - bounds.top ) * ( 1 + bounds.right - bounds.left );
	println!(" area: {}, empty area: {}", area, area as usize - elves.len() );
}

