use std::io::{ self, BufRead };

use std::env;

#[derive(Eq,PartialEq,Clone,Hash,Debug)]
struct Sensor
{
	sx: i32,
	sy: i32,
	bx: i32,
	by: i32,
}

#[derive(Eq,PartialEq,Clone,Hash,Debug)]
struct Range
{
	s: i32,
	e: i32,
}


fn sensor_covers( sensor: &Sensor, y: i32 ) -> Option< ( i32, i32 ) >
{
	let range = ( sensor.bx - sensor.sx ).abs() + ( sensor.by - sensor.sy ).abs();

	let dist = ( sensor.sy - y ).abs();

	let dx = range - dist;
	println!( " {:?} {} {} {}", sensor, range, dist, dx );
	if dx < 0
	{
		return Option::None;
	}

	return Option::Some( ( sensor.sx - dx, sensor.sx + dx ) );
}


fn main()
{
    let args: Vec<String> = env::args().collect();
	if args.len() != 2 
	{
		println!( "Invalid args. Need a Y value");
		return;
	}	

	let mut lines = io::stdin().lock().lines();

	let mut sensors: Vec<Sensor> = Vec::new();

	while let Some( line ) = lines.next()
	{
		let cur_line = line.unwrap();

		let trimmed = cur_line.replace( "Sensor at x=", "" );
		let trimmed = trimmed.replace( ", y=", "," );
		let trimmed = trimmed.replace( ": closest beacon is at x=", "," );
		let trimmed = trimmed.replace( ", y=", "," );

		let coord: Vec<&str> = trimmed.split( "," ).collect();
		sensors.push( Sensor
			{
				sx: coord[0].parse::<i32>().unwrap(),
				sy: coord[1].parse::<i32>().unwrap(),
				bx: coord[2].parse::<i32>().unwrap(),
				by: coord[3].parse::<i32>().unwrap(),
			} );
	}

	println!( "Sensors: {:?}", sensors );

	let y = args[1].parse::<i32>().unwrap();

	let mut ranges: Vec< ( i32, i32 ) > = Vec::new();
	for sensor in sensors
	{
		match sensor_covers( &sensor, y )
		{
			Some( r ) => ranges.push( r ),
			None => {}
		}
	}

	ranges.sort();
	println!( "Ranges: {:?}", ranges );

	let mut total_area = 0;
	let mut cur_range = ranges[0].clone();
	for i in 1..ranges.len()
	{
		let r = ranges[i];

		println!( " {:?} <= {:?} (total area {} )", cur_range, r, total_area );
		if ( r.0 - cur_range.1 ) <= 1
		{
			if r.1 <= cur_range.1
			{
				// total overlap, just discard the range we're processing
				// cur_range: +---------+
				//         r:    +---+
			}
			else
			{
				// partial overlap, merge the ranges
				// cur_range: +---------+
				//         r:         +---+
				cur_range.1 = r.1;
			}
		}
		else
		{
			// no overlap, push cur range, and make cur_range=r
			// cur_range: +---------+
			//         r:             +---+
		 	total_area += cur_range.1 - cur_range.0;

			cur_range = r;
		}
	}
	println!( " {:?}", cur_range );
	total_area += cur_range.1 - cur_range.0 ;

	println!( "Total area: {}", total_area );
}

