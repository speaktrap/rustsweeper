use rand::distributions::{Distribution, Uniform};
use std::io;


fn letter_to_index(letter: char) -> usize {
    let uppercase_letter = letter.to_ascii_uppercase();
    (uppercase_letter as u8 - b'A' + 1) as usize
}

fn read_input(max_x: usize, max_y: usize) -> (usize, usize) {
	let mut x: usize;
	let mut y: usize;
	loop {
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Failed to read line");
		let input = input.trim();
		
		if (&input[0..1]).chars().next().expect("Expect letter").is_alphabetic() {
			x = letter_to_index((&input[0..1]).chars().next().unwrap());
		} else {
			x = max_x + 1;
			}
		if (&input[1..]).chars().next().expect("Expect number").is_numeric() {
			y = (&input[1..]).parse().unwrap();
		} else {
			y = max_y + 1;
			}
		if x>0 && x<=max_x && y>0 && y<=max_y {
			break;
			} 
		}
	(x-1,y-1)
}

fn next_char(c: &mut char) {
	*c = std::char::from_u32(*c as u32 + 1).unwrap_or(*c)
	}

fn write_abc(l: usize) -> String {
	let mut c: char = 'a';
	let mut ret = String::new();
	ret.push_str("    ");
	for _ in 0..l {
		ret.push(c);
		ret.push(' ');
		next_char(&mut c);
		}
	return ret;
	}
	
fn print_map<const height: usize, const width: usize>(map: [[i32; width]; height], show_mines: bool) -> i32 {		
	clearscreen::clear().unwrap();
	println!();
	println!("{}", write_abc(width));
	println!();
	let mut left_to_uncover = 0;
	for j in 0..height {
		if j<9 {print!(" ");}
		print!("{}  ", j+1);
		for i in 0..width {
			let value = map[i][j];
			if value < 9 {
				//if show_mines == true {
				//	print!("{} ", value);
				//} else {
					left_to_uncover += 1;
					print!("◼ ");
					//}
				}
			if value == 9 {
				if show_mines == true {
					print!("* ");
				} else {
					print!("◼ ");
					}
				}
			if value == 10 {
				print!(". ")
				}
			
			if value > 10 {
				print!("{} ", value-10)
				}
			
			}
		println!();
		}
	println!();
	left_to_uncover
	}

fn main() {
	const width: usize = 12;
	const height: usize = 12;
	const luck: i32 = 8;

	let mut rng = rand::thread_rng();
	let chance = Uniform::from(0..luck);
	
	let mut map = [[0;width];height];
	
	
	//PLACE MINES
	for y in 0..height {
		for x in 0..width {
			if chance.sample(&mut rng) == 0 {
				map[x][y] = 9;	
			} else {
				map[x][y] = 0;
				}
			}
		}
	
	//GIVE HINTS
	let mut count_mines:i32;
	for y in 0..height {
		for x in 0..width {
			if map[x][y] < 9 {
				count_mines = 0;
				for i in (y.saturating_sub(1))..=(y + 1).min(height - 1) {
   					for j in (x.saturating_sub(1))..=(x + 1).min(width - 1) {
        					if map[j][i] == 9 {
            						count_mines += 1;
        						}
    						}
					}
				map[x][y] = count_mines;
				}
			}
		}

	let mut left_to_uncover: i32;

	loop {
		left_to_uncover = print_map(map, false);
		if left_to_uncover == 0 {
			print_map(map, true);
			println!("Good job mate :)");
			break}
			else {println!("There are {} tiles left", left_to_uncover);}
		
		let (x, y) = read_input(width, height);
		
		let value = map[x][y];
		if value  == 9 {
			print_map(map, true);
			println!("You died :(");
			break
			}
		
		let mut stack: [(usize, usize); 256] = [(0, 0); 256];
		let mut stack_len = 1;
		stack[0] = (x, y);

		while stack_len > 0 {
			stack_len -= 1;
			let (x, y) = stack[stack_len];
			if map[x][y] != 0 {
				if map[x][y] < 9 {
					map[x][y] += 10;
					}
				continue;
			}
			map[x][y] = 10;	

			if x > 0 {
				stack[stack_len] = (x - 1, y);
				stack_len += 1;
				}
			if y > 0 {
				stack[stack_len] = (x, y - 1);
				stack_len += 1;
				}
			if x < map.len() - 1 {
				stack[stack_len] = (x + 1, y);
				stack_len += 1;
				}
			if y < map[0].len() - 1 {
				stack[stack_len] = (x, y + 1);
				stack_len += 1;
				}
			}
		}
	}
