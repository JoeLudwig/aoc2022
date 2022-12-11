use std::io::{ self, BufRead };

fn range_in_range( s1: i32, e1: i32, s2: i32, e2: i32 ) -> bool
{
	return s1 >= s2 && e1 <= e2;
}

fn strict_subrange( s1: i32, e1: i32, s2: i32, e2: i32 ) -> bool
{
	return range_in_range( s1, e1, s2, e2 ) || range_in_range( s2, e2, s1, e1 );
}

fn overlap( s1: i32, e1: i32, s2: i32, e2: i32 ) -> bool
{
	return !( s1 > e2 || s2 > e1 );
}

fn main()
{
	let mut lines = io::stdin().lock().lines();
	let mut count = 0;
	while let Some( line ) = lines.next()
	{
		let cur_line = line.unwrap();
		if cur_line.len() == 0
		{
			break;
		}

		let v: Vec<&str> = cur_line.split( |c| c == ',' || c == '-' ).collect();
		
		let s1 = v[0].parse::<i32>().unwrap();
		let e1 = v[1].parse::<i32>().unwrap();
		let s2 = v[2].parse::<i32>().unwrap();
		let e2 = v[3].parse::<i32>().unwrap();

		assert!( s1 <= e1 );
		assert!( s2 <= e2 );

		//if strict_subrange( s1, e1, s2, e2 )
		if overlap( s1, e1, s2, e2 )
		{
			count += 1;
		}

	}
	println!( "Count: {}", count );
}
