
use std::io::{ self, BufRead };
use std::cmp;
use std::collections::HashSet;

#[derive(Eq,PartialEq,Clone,Hash,Debug)]
struct Point
{
	x: i32,
	y: i32,
}

fn move_tail( h: &Point, t: &mut Point )
{
	let dx = t.x - h.x;
	let dy = t.y - h.y;
	if dx.abs() <= 1 && dy.abs() <= 1
	{
		// if we're already adjacent, nothing to do here.
		return;
	}

	if dx < 0
	{
		t.x += 1;
	}
	else if dx > 0
	{
		t.x -= 1;
	}

	if dy < 0
	{
		t.y += 1;
	}
	else if dy > 0
	{
		t.y -= 1;
	}
}

fn main()
{
	let mut lines = io::stdin().lock().lines();

	let mut visited: HashSet<Point> = HashSet::new();

	let mut rope: Vec<Point> = Vec::new();

	let knot_count = 10;
	
	for _i in 0..knot_count
	{
		rope.push( Point { x: 0, y: 0 } );
	}
	visited.insert( rope[0].clone() );

	while let Some( line ) = lines.next()
	{
		let cur_line = line.unwrap();
		if cur_line.len() == 0
		{
			break;
		}
		
		let mut dx = 0;
		let mut dy = 0;
		let args: Vec< &str > = cur_line.split( " " ).collect();
		let n = args[1].parse::<i32>().unwrap();
		match args[0]
		{
			"U" => dy = 1,
			"D" => dy = -1,
			"L" => dx = -1,
			"R" => dx = 1,
			_ => panic!( "Invalid command {}", args[0] ),
		}	

		for _i in 0..n
		{
			rope[0].x += dx;
			rope[0].y += dy;
			
			for j in 1..rope.len()
			{
				let h = rope[ j - 1 ].clone();
				move_tail( &h, &mut rope[ j ]);
			}
			//println!( "{:?}", rope );

			visited.insert( rope.last().unwrap().clone() );
		}
	}
	

	println!( "visited count={}", visited.len() );
}
