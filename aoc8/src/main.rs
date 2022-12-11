
use std::io::{ self, BufRead };
use std::cmp;


fn main()
{
	let mut lines = io::stdin().lock().lines();

	let mut width = 0;
	let mut trees: Vec<i32> = Vec::new();

	while let Some( line ) = lines.next()
	{
		let cur_line = line.unwrap();
		if cur_line.len() == 0
		{
			break;
		}
		
		if width == 0 
		{
			width = cur_line.len();
		}

		for c in cur_line.chars()
		{
			trees.push( c as i32 - '0' as i32 );
		}
	}
	
	let height = trees.len() / width;

	let mut views: Vec<i32> = Vec::new();
	views.resize( trees.len(), 0 );


	// from top
	for x in 0..width
	{
		let mut max: i32 = -1;
		for y in 0..height
		{
			let i = x + y * height;
			let h = trees[ i ];
			if h > max
			{
				max = h;

				let old_views = views[ i ];
				views[ i ] = old_views + 1;
			}
		}
	}

	// from left
	for y in 0..height
	{
		let mut max: i32 = -1;
		for x in 0..width
		{
			let i = x + y * height;
			let h = trees[ i ];
			if h > max
			{
				max = h;

				let old_views = views[ i ];
				views[ i ] = old_views + 1;
			}
		}
	}

	// from bottom
	for x in 0..width
	{
		let mut max: i32 = -1;
		for y in ( 0..height ).rev()
		{
			let i = x + y * height;
			let h = trees[ i ];
			if h > max
			{
				max = h;

				let old_views = views[ i ];
				views[ i ] = old_views + 1;
			}
		}
	}

	// from right
	for y in 0..height
	{
		let mut max: i32 = -1;
		for x in ( 0..width ).rev()
		{
			let i = x + y * height;
			let h = trees[ i ];
			if h > max
			{
				max = h;

				let old_views = views[ i ];
				views[ i ] = old_views + 1;
			}
		}
	}

	println!( "views: {:?}", views );

	let mut visible = 0;
	for view in &views
	{
		if view > &0
		{
			visible += 1;
		}
	}
	println!( "visible: {}", visible );

	// scenic 
	let mut most_scenic = 0;
	for x in 0..width
	{
		for y in 0..height
		{
			let i = x + y * height;
			let h = trees[ i ];

			// left
			let mut left = 0;
			if x > 0
			{
				for tx in (0..x).rev()
				{
					let th = trees[ tx + y * height ];
					left+=1;
					if th >= h 
					{
						break;
					}
				}
			}

			// right
			let mut right = 0;
			for tx in (x+1)..width
			{
				let th = trees[ tx + y * height ];
				right+=1;
				if th >= h 
				{
					break;
				}
			}

			// up
			let mut up = 0;
			if y > 0
			{
				for ty in (0..y ).rev()
				{
					let th = trees[ x + ty * height ];
					up+= 1;
					if th >= h 
					{
						break;
					}
				}
			}

			// down
			let mut down = 0;
			for ty in (y+1)..height
			{
				let th = trees[ x + ty * height ];
				down+= 1;
				if th >= h 
				{
					break;
				}
			}

			let scenic = left * right * up * down;
			println!( "({}, {}) [{}, {}, {}, {}] {}", x, y, left, right, up, down, scenic );

			most_scenic = cmp::max( scenic, most_scenic );
		}
	}

	println!( "most_scenic: {}", most_scenic );
}
