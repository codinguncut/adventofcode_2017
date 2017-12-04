extern crate itertools;
use itertools::unfold;

fn euler1() {
    let fizz : i32 = (0..1000).filter(|n| n % 3 == 0 || n % 5 == 0).sum();
    println!("Euler 1: {}", fizz);
}

fn euler2() {
	let (mut x1, mut x2) = (1u32, 1u32);
	let mut fibonacci = unfold((), move |_| {
		// Attempt to get the next Fibonacci number
		let next = x1.saturating_add(x2);

		// Shift left: ret <- x1 <- x2 <- next
		let ret = x1;
		x1 = x2;
		x2 = next;

		// If addition has saturated at the maximum, we are finished
		if ret == x1 && ret > 1 {
			return None;
		}

		Some(ret)
	});

	let fibsum : u32 = fibonacci.take_while(|&x| x <= 4_000_000).filter(|x| x % 2 == 0).sum();
    println!("Euler 2 {}", fibsum);
}

fn euler3() {
    fn primes(upto: u64) -> Vec<u64> {
        let mut nums : Vec<u64> = (0..upto+1).collect::<Vec<_>>();
        let root =  (upto as f64).sqrt() as u64;
        for div in 2..root {
            let mut idx = div*2;
            while idx <= upto {
                nums[idx as usize] = 0;
                idx += div;
            }
        }

        return nums.into_iter().filter(|&n| n > 1).collect::<Vec<_>>();
    }
    println!("primes {:?}", primes(100));

    fn prime_factors(target: u64) -> Vec<u64> {
        return primes(target).into_iter().filter(|&n| target % n == 0).collect::<Vec<_>>();
    }
    println!("factors {:?}", prime_factors(600851475143));
}

