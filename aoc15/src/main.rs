use std::io::{ self, BufRead };

#[derive(Eq,PartialEq,Clone,Hash,Debug)]
struct Sensor
{
	sx: i64,
	sy: i64,
	bx: i64,
	by: i64,
}

type Range = ( i64, i64 );
type RangeVec = Vec< Range >;

fn sensor_covers( sensor: &Sensor, y: i64 ) -> Option< Range >
{
	let range = ( sensor.bx - sensor.sx ).abs() + ( sensor.by - sensor.sy ).abs();

	let dist = ( sensor.sy - y ).abs();

	let dx = range - dist;
	//println!( " {:?} {} {} {}", sensor, range, dist, dx );
	if dx < 0
	{
		return Option::None;
	}

	return Option::Some( ( sensor.sx - dx, sensor.sx + dx ) );
}


fn merge_ranges( ranges: &RangeVec ) -> RangeVec
{
	let mut merged_ranges: RangeVec = Vec::new();
	if ranges.len() < 2
	{
		return merged_ranges;
	}

	let mut cur_range = ranges[0].clone();
	for i in 1..ranges.len()
	{
		let r = ranges[i];

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
			merged_ranges.push( cur_range );
			cur_range = r.clone();
		}
	}

	merged_ranges.push( cur_range.clone() );
	return merged_ranges;
}

fn main()
{
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
				sx: coord[0].parse::<i64>().unwrap(),
				sy: coord[1].parse::<i64>().unwrap(),
				bx: coord[2].parse::<i64>().unwrap(),
				by: coord[3].parse::<i64>().unwrap(),
			} );
	}

	//println!( "Sensors: {:?}", sensors );

	let mut found_it = false;
	for y in 0..4000001
	{
		let mut ranges: RangeVec = Vec::new();
		for sensor in &sensors
		{
			match sensor_covers( &sensor, y )
			{
				Some( r ) => ranges.push( r ),
				None => {}
			}
		}

		ranges.sort();

		let ranges = merge_ranges( &ranges );
		//println!( "{} : {:?}", y, ranges );

		if ranges.len() != 2
		{
			continue;
		}

		if ranges[0].1 + 2 != ranges[1].0
		{
			continue;
		}

		assert!( !found_it );

		println!( "{:?}", ranges );
		let x = ranges[0].1 + 1;
		println!( "frequency: {}", 4000000 * x + y ); 
	}

}

