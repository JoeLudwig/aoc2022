use std::io::{ self, BufRead };
use std::cell::RefCell;
use std::rc::{ Rc, Weak };

#[derive(Debug)]
struct FSEntry
{
	name: String,
	parent: Option< Weak< RefCell< FSEntry > >>,
	contents: Option< Vec< Rc< RefCell< FSEntry > > > >,
	size: usize,
}

fn make_file( name: String, size: usize, parent: Weak< RefCell< FSEntry > > ) -> Rc< RefCell< FSEntry > >
{
	Rc::new( RefCell::new( FSEntry 
	{
		name: name,
		parent: Some( parent ),
		contents: None,
		size: size,
	} ) )
}
		
fn make_dir( name: String, parent: Weak< RefCell< FSEntry > > ) -> Rc< RefCell< FSEntry > >
{
	Rc::new( RefCell::new( FSEntry 
	{
		name: name,
		parent: Some( parent ),
		contents: Some( Vec::new() ),
		size: 0,
	} ) )
}
		
		
fn dump( node: &Rc< RefCell< FSEntry > >, indent: usize )
{
	let n = node.borrow();
	if n.size == 0 
	{
		println!( "{:>indent$} - {}/", "", n.name );

		for child in n.contents.as_ref().unwrap()
		{
			dump( child, indent + 3 );
		}
	}
	else
	{
		println!( "{:>indent$}  - {} ({})", "", n.name, n.size );
	}
}

fn dir_size( node: &Rc< RefCell< FSEntry > > ) -> usize
{
	let n = node.borrow();
	if n.size == 0 
	{
		let mut total: usize = 0;
		for child in n.contents.as_ref().unwrap()
		{
			total = total + dir_size( child );
		}
		return total;
	}
	else
	{
		return n.size;
	}
}


fn dirs_under_size( node: &Rc< RefCell< FSEntry > >, max_size: usize ) -> usize
{
	let n = node.borrow();
	if n.size == 0 
	{
		let mut total: usize = 0;
		
		for child in n.contents.as_ref().unwrap()
		{
			total = total + dirs_under_size( child, max_size );
		}

		let my_size = dir_size( node );
		if my_size <= max_size
		{
			total = total + my_size;
		}
		return total;
	}
	else
	{
		return 0;
	}
}


fn smallest_dir_at_least( node: &Rc< RefCell< FSEntry > >, min_size: usize ) -> usize
{
	let n = node.borrow();
	if n.size == 0 
	{
		let mut best: usize = 0;
		
		for child in n.contents.as_ref().unwrap()
		{
			let child_dir_size = smallest_dir_at_least( child, min_size );
			if child_dir_size >= min_size 
				&& ( best == 0 || child_dir_size < best )
			{
				best = child_dir_size;
			}
		}

		if best == 0
		{
			// none of our children qualified
			let my_size = dir_size( node );
			if my_size >= min_size
			{
				// if we qualify, nominate us
				best = my_size;
			}
		}

		return best;
	}
	else
	{
		return 0;
	}
}


fn main()
{
	let mut lines = io::stdin().lock().lines();

	let root = Rc::new( RefCell:: new( FSEntry 
		{ name: "/".to_string(), size: 0,
			contents: Some( Vec::new() ), parent: None } ) );
	let mut cd = Rc::clone( &root );
	let mut processing_dir = false;

	// read the initial state
	while let Some( line ) = lines.next()
	{
		let cur_line = line.unwrap();
		if cur_line.len() == 0
		{
			// done with initial state
			break;
		}

		if processing_dir
		{
			let args: Vec< &str> = cur_line.split( " " ).collect();
			if args[0] == "dir"
			{
				cd.borrow_mut().contents.as_mut().unwrap().push( make_dir( args[1].to_string(), Rc::downgrade( &cd ) ) );
			}
			else if args[0] == "$"
			{
				processing_dir = false;
			}
			else
			{
				let size = args[0].parse::<usize>().unwrap();
				cd.borrow_mut().contents.as_mut().unwrap().push( make_file( args[1].to_string(), size, Rc::downgrade( &cd ) ) );
			}
		}

		if !processing_dir
		{
			let args: Vec< &str> = cur_line.split( " " ).collect();
			assert!( args[0] == "$" );
			if args[1] == "cd"
			{
				let old_cd = Rc::clone( &cd );
				if args[2] == "/"
				{
					cd = Rc::clone( &root );
				}
				else if args[2] == ".."
				{
					let a = old_cd.borrow();
					match &a.parent
					{
						Some( parent ) => cd = parent.upgrade().unwrap(),
						None => panic!( "cd .. with no parent" ),
					}
					//cd = old_cd.borrow_mut().parent.unwrap().upgrade().unwrap();
				}
				else
				{
					let a = old_cd.borrow();
					match &a.contents
					{
						Some( l ) => 
						{
							for subdir in l
							{
								if subdir.borrow().name == args[2] 
								{
									cd = Rc::clone( subdir );
								}
									
							}
						},
						None => {}
					}
				}
			}
			else if args[1] == "ls"
			{
				processing_dir = true;
			}
		}	
		
	}

	dump( &root, 0 );
	let total_size = dir_size( &root );
	let drive_size = 70000000;
	let free_space = drive_size - total_size;
	let update_size = 30000000;
	let space_needed = update_size - free_space;
	println!( "Used {}   Free: {}    Needed: {}", total_size, free_space, space_needed );
	println!( "total_under_size: {}", dirs_under_size( &root, 100000 ) );
	println!( "smallest big directory: {}", smallest_dir_at_least( &root, space_needed ) );

}
