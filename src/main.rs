/* ==========================================================================
 *
 * MIT License
 *
 * Copyright (c) 2021 by Chris Provenzano, proven-tools
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 *
 * Filename: fibonacci/src/main.rs
 *
 * ==========================================================================
 */
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut init_val_n1 = vec![0];
    let mut init_val_n2 = vec![1];

    if args.len() < 2 {
	panic!("Please type a number!");
    }
    
    let mut index:u64 = args[1].parse()
        .expect("Please type a number!");

    while index > 0 {
        let mut sum:u64;
        let mut carry:u64 = 0;
        let len = init_val_n1.len();
	for i in 0..len { // Don't iterate over grown section
            sum = init_val_n1[i] + init_val_n2[i] + carry;
            init_val_n2[i] = init_val_n1[i];
            init_val_n1[i] = sum % 10;
            if sum >= 10 {
		if i == len - 1 {
		    // Grow vectors
		    init_val_n1.push(1);
		    init_val_n2.push(0);
		} 
		carry = 1;
	    } else {
		carry = 0;
	    }
	}
        index = index - 1;
    }
    for digit in init_val_n1.iter().rev() {
        print!("{}", digit);
    }
    println!("");
}
