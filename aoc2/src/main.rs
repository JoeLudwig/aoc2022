#![allow(non_snake_case)]
use std::io::{ self, BufRead };


#[derive(Eq,PartialEq,Copy,Clone)]
enum EPlay
{
	Rock,
	Paper,
	Scissors,
}

#[derive(Eq,PartialEq,Copy,Clone)]
enum EResult
{
	Win,
	Lose,
	Draw,
}

fn ParsePlay( c: char ) -> EPlay
{
	match c 
	{
		'A' => return EPlay::Rock,
		'B' => return EPlay::Paper,
		'C' => return EPlay::Scissors,
		'X' => return EPlay::Rock,
		'Y' => return EPlay::Paper,
		'Z' => return EPlay::Scissors,
		_ => panic!( "Invalid input {}", c )
	}
}		

fn ParseResult( c: char ) -> EResult
{
	match c 
	{
		'X' => return EResult::Lose,
		'Y' => return EResult::Draw,
		'Z' => return EResult::Win,
		_ => panic!( "Invalid input {}", c )
	}
}		


fn ComputePlayScore( play: EPlay ) -> i32
{
	match play
	{
		EPlay::Rock => return 1,	
		EPlay::Paper => return 2,	
		EPlay::Scissors => return 3,	
	}
}

fn ComputeResultScore( result: EResult ) -> i32
{
	match result
	{
		EResult::Win => return 6,	
		EResult::Lose => return 0,	
		EResult::Draw => return 3,	
	}
}


fn ABeatsB( a: EPlay, b: EPlay ) -> bool
{
	if a == EPlay::Rock && b == EPlay::Scissors
	{
		return true;
	}
	else if a == EPlay::Scissors && b == EPlay::Paper
	{
		return true;
	}
	else if a == EPlay::Paper && b == EPlay::Rock
	{
		return true;
	}
	return false;
}

fn PlayForResult( them: EPlay, result: EResult ) -> EPlay
{
	if result == EResult::Draw
	{
		return them;
	}
	else if result == EResult::Win
	{
		match them
		{
			EPlay::Rock => return EPlay::Paper,
			EPlay::Paper => return EPlay::Scissors,
			EPlay::Scissors => return EPlay::Rock,
		}
	}
	else
	{
		match them
		{
			EPlay::Rock => return EPlay::Scissors,
			EPlay::Paper => return EPlay::Rock,
			EPlay::Scissors => return EPlay::Paper,
		}
	}
}


fn ComputeScore( me: EPlay, them: EPlay ) -> i32
{
	let mut score = ComputePlayScore( me );
	if me == them
	{
		score += 3;
	}
	else if ABeatsB( me, them )
	{
		score += 6;
	}

	return score;
}

fn main()
{
	let mut lines = io::stdin().lock().lines();

	let mut total = 0;
	while let Some( line ) = lines.next()
	{
		let curLine = line.unwrap();
		if curLine.len() < 3
		{
			break;
		}

		let them = ParsePlay( curLine.chars().nth( 0 ).unwrap() );
		let result = ParseResult( curLine.chars().nth( 2 ).unwrap() );
		let me = PlayForResult( them, result );
		let score = ComputeResultScore( result ) + ComputePlayScore( me );
		total += score;
		//println!( "Score: {}", score );
	}
	println!( "Total: {}", total );
}

