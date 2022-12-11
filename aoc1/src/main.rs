use std::io::{ self, BufRead };

#[allow(non_snake_case)]
fn main() 
{
	let mut lines = io::stdin().lock().lines();

	let mut curElfTotal = 0;
	let mut vec: Vec<i32> = Vec::new();
	while let Some( line ) = lines.next()
	{
		let curLine = line.unwrap();

		if curLine.len() == 0 
		{
			vec.push( curElfTotal );
			curElfTotal = 0;
		}
		else
		{
			let cal = curLine.parse::<i32>().unwrap();
			//println!( "Cal: {}", cal );
			curElfTotal += cal;
		}
	}

	vec.sort();
	vec.reverse();

	if vec.len() >= 3
	{
		let total = vec[0] + vec[1] + vec[2];
		println!( "Top three elves: {}", total );
 	}
}
