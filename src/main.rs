use std::time::Instant;
fn main() {
	for i in (1..10) {
		//println!("Iteration new mine {i}");
		maxop4(i, 100000000);
	}
	
	
	
	
	
	
	//let now = Instant::now();
	//let mut primes3: Vec<u32> = Vec::new();
	//let mut tes = false;
	//for i in (1..10000000) {
	//	tes = is_prime(i);
	//	if tes == true {
	//		primes3.push(i);
	//	}
	//	}
	//let elapsed = now.elapsed();
	//println!("Elapsed theres: {:.2?}", elapsed);
	//println!("Len of vector is: {}", primes3.len());








}
//vector length as a u vs an i
fn maxop4(i: u8, val :u32) {
	let now = Instant::now();
	let mut primes: Vec<u32> = Vec::new();
	let mut continues = 0;
	let mut calcs = 0;
	let mut divsix = 0;
	primes.push(2);
	primes.push(3);
	for d in (5..=val).step_by(2) {
		//println!("{d}");
		let mut counter = 2;
		let mut primech = true;
		//let test = 9.0_f32;
		let d3 : u32 = (d as f64).sqrt() as u32;
		//println!("{test2}");
		//let d_sqrt = d.sqrt();
		//if d % 10000 == 0 {
		//	println!("amount compleated {d}");
		//}
		if d % 6 == 5 {
			//println!("is this a prime? {d}");
			divsix += 1;
			for u in &primes {
			
				if *u <= d3 {
				//println!("{i}");
					calcs += 1;
					if d % u == 0 {
						primech = false;
						continues += 1;
						break;
						}
				}
			else {break;}}
		
		
		} 
		else if d % 6 == 1 {
			//println!("is this a prime? {d}");
			divsix += 1;
			for u in &primes {
			
				if *u <= d3 {
				//println!("{i}");
					calcs += 1;
					if d % u == 0 {
						primech = false;
						continues += 1;
						break;
						}
				}
			else {break;}}
		
		
		} 
				
		
		
		
		
		
		
		
		
		
		else {primech = false;}


		
		
		
		
		
		
		if primech == true {
			primes.push(d);
		}

		
		
		
		
		//d +=2;
		//d2 +=2.0;
	}
	
    let elapsed = now.elapsed();
	println!("Len of vector is: {}", primes.len());
	println!("Len of new is: {}", divsix);
	println!("Iteration {i} : Elapsed: {:.2?}", elapsed);
	//for i in primes { println!("is prime working: {i}");}
	//println!("Calculations done: {calcs}");
	//println!("Continues done: {}", &continues);
	
	
	
}



























fn maxop3(i: u8, val :u32) {
	let now = Instant::now();
	let mut primes: Vec<u32> = Vec::new();
	let mut continues = 0;
	let mut calcs = 0;
	let mut divsix = 0;
	primes.push(2);
	primes.push(3);
	for d in (5..=val).step_by(2) {
		//println!("{d}");
		let mut counter = 2;
		let mut primech = true;
		//let test = 9.0_f32;
		let d3 : u32 = (d as f64).sqrt() as u32;
		//println!("{test2}");
		//let d_sqrt = d.sqrt();
		//if d % 10000 == 0 {
		//	println!("amount compleated {d}");
		//}
		if d % 6 == 5 || d % 6 == 1 {
			//println!("is this a prime? {d}");
			divsix += 1;
			for u in &primes {
			
				if *u <= d3 {
				//println!("{i}");
					calcs += 1;
					if d % u == 0 {
						primech = false;
						continues += 1;
						break;
						}
				}
			else {break;}}
		
		
		} 
		
		
		else {primech = false;}


		
		
		
		
		
		
		if primech == true {
			primes.push(d);
		}

		
		
		
		
		//d +=2;
		//d2 +=2.0;
	}
	
    let elapsed = now.elapsed();
	println!("Len of vector is: {}", primes.len());
	println!("Len of new is: {}", divsix);
	println!("Iteration {i} : Elapsed: {:.2?}", elapsed);
	//for i in primes { println!("is prime working: {i}");}
	//println!("Calculations done: {calcs}");
	//println!("Continues done: {}", &continues);
	
	
	
}



























fn maxop2(i: u8, val: u32) {
	let now = Instant::now();
	let mut primes: Vec<u32> = Vec::new();
	let mut continues = 0;
	let mut calcs = 0;
	let mut prime_amm = 1;
	let mut addcheck = true;
	let max_prime : u32 = (val as f64).sqrt() as u32;
	primes.push(2);
	for d in (3..=val).step_by(2) {
		//println!("{d}");
		let mut counter = 2;
		let mut primech = true;
		//let test = 9.0_f32;
		let d3 : u32 = (d as f64).sqrt() as u32;
		//println!("{test2}");
		//let d_sqrt = d.sqrt();
		//if d % 10000 == 0 {
		//	println!("amount compleated {d}");
		//}

		for i in &primes {
			if *i <= d3 {
				//println!("{i}");
				calcs += 1;
				if d % i == 0 {
					primech = false;
					continues += 1;
					break;
					
				}
				

		}
			else {break;}}
		
		
		if primech == true {
			prime_amm += 1;
			if addcheck == true {
			primes.push(d);
				if d >= max_prime {
					addcheck = false;
				}
			
			}
		}
		
		
		
		
		//d +=2;
		//d2 +=2.0;
	}
	
    let elapsed = now.elapsed();
	println!("Len of vector is: {}", primes.len());
	println!("Amount of primes is: {prime_amm}");
	println!("Iteration {i} : Elapsed: {:.2?}", elapsed);
	//println!("Calculations done: {calcs}");
	//println!("Continues done: {}", &continues);
	
	
	
}































fn prime_s2_sqrt(i: u8) {
	let mut primes: Vec<u32> = Vec::new();
	let mut continues = 0;
	let mut calcs = 0;
	primes.push(2);
    let now = Instant::now();
	for d in (3..=10000000).step_by(2) {
		//println!("{d}");
		let mut counter = 2;
		let mut primech = true;
		//let test = 9.0_f32;
		let d3 : f64 = (d as f64).sqrt();
		//println!("{test2}");
		//let d_sqrt = d.sqrt();
		//if d % 100000 == 1 {
		//	println!("amount compleated {d}");
		//}

		while (counter as f64) <= d3 {
			calcs += 1;
			if d % counter == 0 {
				//println!("d composeite {d}");
				primech = false;
				
			}
			
			
			counter += 1;
			if primech == false {
				continues += 1;
				break;
			}
		}
		if primech == true {
			primes.push(d);
		}
		
		
		
		
		
		
		//d +=2;
		//d2 +=2.0;
	}
	
    let elapsed = now.elapsed();
	println!("Len of vector is: {}", primes.len());
	println!("Iteration {i} : Elapsed: {:.2?}", elapsed);
	//println!("Calculations done: {calcs}");
	//println!("Continues done: {}", &continues);
	
	
	
}

fn maxop(i: u8, val :u32) {
	let now = Instant::now();
	let mut primes: Vec<u32> = Vec::new();
	let mut continues = 0;
	let mut calcs = 0;
	primes.push(2);
	for d in (3..=val).step_by(2) {
		//println!("{d}");
		let mut counter = 2;
		let mut primech = true;
		//let test = 9.0_f32;
		let d3 : u32 = (d as f64).sqrt() as u32;
		//println!("{test2}");
		//let d_sqrt = d.sqrt();
		//if d % 10000 == 0 {
		//	println!("amount compleated {d}");
		//}

		for i in &primes {
			if *i <= d3 {
				//println!("{i}");
				calcs += 1;
				if d % i == 0 {
					primech = false;
					continues += 1;
					break;
					
				}
				

		}
			else {break;}}
		
		
		if primech == true {
			primes.push(d);
		}

		
		
		
		
		//d +=2;
		//d2 +=2.0;
	}
	
    let elapsed = now.elapsed();
	println!("Len of vector is: {}", primes.len());
	println!("Iteration {i} : Elapsed: {:.2?}", elapsed);
	//println!("Calculations done: {calcs}");
	//println!("Continues done: {}", &continues);
	
	
	
}


fn is_prime(n: u32) -> bool {
    let limit = (n as f64).sqrt() as u32;

    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn prime_s2_sqrt_int(i: u8) {
	let mut primes: Vec<u32> = Vec::new();
	let mut continues = 0;
	let mut calcs = 0;
	primes.push(2);
    let now = Instant::now();
	for d in (3..=10000000).step_by(2) {
		//println!("{d}");
		let mut counter = 2;
		let mut primech = true;
		//let test = 9.0_f32;
		let d3 : u32 = (d as f64).sqrt() as u32;
		//println!("{test2}");
		//let d_sqrt = d.sqrt();
		//if d % 100000 == 1 {
		//	println!("amount compleated {d}");
		//}

		while counter <= d3 {
			calcs += 1;
			if d % counter == 0 {
				//println!("d composeite {d}");
				primech = false;
				continues += 1;
				break;
				
			}
			
			
			counter += 1;

		}
		if primech == true {
			primes.push(d);
		}
		
		
		
		
		
		
		//d +=2;
		//d2 +=2.0;
	}
	
    let elapsed = now.elapsed();
	//println!("Len of vector is: {}", primes.len());
	println!("Iteration {i} : Elapsed: {:.2?}", elapsed);
	//println!("Calculations done: {calcs}");
	//println!("Continues done: {}", &continues);
	
	
	
}