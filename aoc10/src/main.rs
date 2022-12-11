use std::io::{ self, BufRead };


fn get_ss_sample( cycle: i32, x: i32 ) -> i32
{
	if cycle < 20 || cycle > 220
	{
		return 0;
	}

	if ( ( cycle - 20 ) % 40 ) == 0
	{
		return cycle * x;
	}

	return 0;
}

fn is_lit( cycle: i32, x: i32 ) -> bool
{
	let pos = ( (cycle - 1 ) % 40 ) + 1;
	return pos >= x && pos <= x + 2;
}
	
fn draw( cycle: i32, x: i32, out: &mut String )
{
	out.push( if is_lit( cycle, x ) { '#' } else { '.' } );
	if out.len() == 40
	{
		println!( "{}", out );
		out.clear();
	}
}

fn main()
{
	let mut lines = io::stdin().lock().lines();

	let mut cycle = 0;
	let mut x = 1;
	let mut ss = 0;
	let mut out = String::new();

	while let Some( line ) = lines.next()
	{
		let cur_line = line.unwrap();
		if cur_line.len() == 0
		{
			break;
		}
		
		if cur_line == "noop" 
		{
			cycle+=1;
			draw( cycle, x, &mut out );
			ss += get_ss_sample( cycle, x );
		}
		else
		{
			let args: Vec< &str > = cur_line.split( " " ).collect();
			assert!( args[0] == "addx" );
			let n = args[1].parse::<i32>().unwrap();

			cycle+=1;
			draw( cycle, x, &mut out );
			ss += get_ss_sample( cycle, x );
			cycle+=1;
			draw( cycle, x, &mut out );
			ss += get_ss_sample( cycle, x );
			
			x += n;
		}

	}
	

	println!( "ss total={}", ss );
}
