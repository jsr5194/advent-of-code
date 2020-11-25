
mod intcode;

fn main() {
	// initialize cpu
	let ip = 0;
	let interrupt = false;
	let mut cpu = intcode::IntcodeCpu { ip, interrupt };

	// print the cpu
	println!("{}", cpu);
    
	// temporarially define intcode program
	let program = String::from("1,9,10,3,2,3,11,0,99,30,40,50");

	// run the program
	let result = cpu.run(program);

	// display the results
	println!("[*] Result: {}", result);


}
