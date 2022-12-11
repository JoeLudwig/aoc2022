use std::io::{ self, BufRead };


fn main_old()
{
	let mut lines = io::stdin().lock().lines();
	let mut total = 0;
	while let Some( line ) = lines.next()
	{
		let cur_line = line.unwrap();
		if cur_line.len() == 0
		{
			break;
		}

		let mut present : u64 = 0;
		let half = cur_line.len() / 2;
		for t in cur_line.char_indices()
		{
			let ( i, c ) = t;
			let bit_index = if c >= 'a' && c <= 'z' 
			{ 
				( c as i32 ) - ( 'a' as i32 )
			} 
			else
			{
				26 + ( c as i32 ) - ( 'A' as i32 )
			};
			let bit = 1 << bit_index;
			let priority = bit_index + 1;
			if i < half
			{
				// record what we found
				present |= bit;
			}
			else
			{
				//check for dups
				if ( present & bit ) != 0
				{
					println!( "Found dup: {} pri {}", c, priority );
					total += priority;
					break;
				}
			}	
		}

	}
	println!( "Total: {}", total );
}

fn main()
{
	let mut lines = io::stdin().lock().lines();
	let mut total = 0;
	let mut member_count = 0;
	let mut badge_accumulator: u64 = 0;
	while let Some( line ) = lines.next()
	{
		let cur_line = line.unwrap();
		if cur_line.len() == 0
		{
			break;
		}

		let mut present : u64 = 0;
		for t in cur_line.char_indices()
		{
			let ( i, c ) = t;
			let bit_index = if c >= 'a' && c <= 'z' 
			{ 
				( c as i32 ) - ( 'a' as i32 )
			} 
			else
			{
				26 + ( c as i32 ) - ( 'A' as i32 )
			};
			let bit = 1 << bit_index;
			present |= bit;
		}

		if badge_accumulator == 0
		{
			badge_accumulator = present;
		}
		else
		{
			badge_accumulator = badge_accumulator & present;
		}
		member_count += 1;
	
		if member_count == 3 
		{
			for bit_index in 0..52
			{
				if ( badge_accumulator & ( 1 << bit_index ) ) != 0
				{
					total += bit_index + 1;
					println!( "badge is {}", bit_index );
					break;
				}
			}
			
			badge_accumulator = 0;
			member_count = 0;
		}
	}

	println!( "Total: {}", total );
}

