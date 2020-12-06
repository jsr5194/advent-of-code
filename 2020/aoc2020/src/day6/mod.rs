use std::fs;

fn read_input() -> Vec<CustomsGroup> {
	// get raw input
	let filename = "./src/day6/input.txt";
	let contents_str = fs::read_to_string(filename).expect("[!] ERROR: could not read file");
	
	// each line is one person
	// each newline indicates a new family
	let mut contents: Vec<CustomsGroup> = vec![CustomsGroup::default()];

	for line in contents_str.lines() {
		if line == "" {
			contents.push(CustomsGroup::default());
		} else {
			let mut cur_person = CustomsForm::default();
			cur_person.parse(line);
			contents.last_mut().expect("[!] ERROR: Could not get last vector element").add_person(cur_person);
		}
	}

	contents
}

pub fn run_part1() {
	let answers: Vec<CustomsGroup> = read_input();
	let mut result = u32::default();
	for group in answers {
		result += group.get_group_collective_yes_count();
	}
	println!("Day 6 Part 1 Result: {}", result);
}

pub fn run_part2() {
	let answers: Vec<CustomsGroup> = read_input();
	let mut result = u32::default();
	for group in answers {
		result += group.get_group_unanimous_yes_count();
	}
	println!("Day 6 Part 2 Result: {}", result);
}

#[derive(Debug, Default, Clone)]
struct CustomsForm {
	a: bool,
	b: bool,
	c: bool,
	d: bool,
	e: bool,
	f: bool,
	g: bool,
	h: bool,
	i: bool,
	j: bool,
	k: bool,
	l: bool,
	m: bool,
	n: bool,
	o: bool,
	p: bool,
	q: bool,
	r: bool,
	s: bool,
	t: bool,
	u: bool,
	v: bool,
	w: bool,
	x: bool,
	y: bool,
	z: bool,
}

impl CustomsForm {
	fn parse(&mut self, answers: &str) {
		for answer in answers.chars() {
			match answer {
				'a' => self.a = true,
				'b' => self.b = true,
				'c' => self.c = true,
				'd' => self.d = true,
				'e' => self.e = true,
				'f' => self.f = true,
				'g' => self.g = true,
				'h' => self.h = true,
				'i' => self.i = true,
				'j' => self.j = true,
				'k' => self.k = true,
				'l' => self.l = true,
				'm' => self.m = true,
				'n' => self.n = true,
				'o' => self.o = true,
				'p' => self.p = true,
				'q' => self.q = true,
				'r' => self.r = true,
				's' => self.s = true,
				't' => self.t = true,
				'u' => self.u = true,
				'v' => self.v = true,
				'w' => self.w = true,
				'x' => self.x = true,
				'y' => self.y = true,
				'z' => self.z = true,
				_ => panic!("[!] Invalid answer detected"),
			}
		}
	}

	fn get_answers(&self) -> String {
		let mut answer = String::default();
		if self.a { answer += "a" };
		if self.b { answer += "b" };
		if self.c { answer += "c" };
		if self.d { answer += "d" };
		if self.e { answer += "e" };
		if self.f { answer += "f" };
		if self.g { answer += "g" };
		if self.h { answer += "h" };
		if self.i { answer += "i" };
		if self.j { answer += "j" };
		if self.k { answer += "k" };
		if self.l { answer += "l" };
		if self.m { answer += "m" };
		if self.n { answer += "n" };
		if self.o { answer += "o" };
		if self.p { answer += "p" };
		if self.q { answer += "q" };
		if self.r { answer += "r" };
		if self.s { answer += "s" };
		if self.t { answer += "t" };
		if self.u { answer += "u" };
		if self.v { answer += "v" };
		if self.w { answer += "w" };
		if self.x { answer += "x" };
		if self.y { answer += "y" };
		if self.z { answer += "z" };
		answer
	}
}

#[derive(Debug, Default)]
struct CustomsGroup {
	group_collective_form: CustomsForm,
	customs_forms: Vec<CustomsForm>,
}

impl CustomsGroup {
	fn add_person(&mut self, form: CustomsForm) {
		// add the new form to the running list
		self.customs_forms.push(form.clone());

		// build a combined group form
		self.group_collective_form.parse(form.get_answers().as_str());
	}

	fn get_group_collective_yes_count(&self) -> u32 {
		self.group_collective_form.get_answers().len() as u32
	}

	fn get_group_unanimous_yes_count(&self) -> u32 {
		let mut group_unanimous_form = CustomsForm {
			a: true,
			b: true,
			c: true,
			d: true,
			e: true,
			f: true,
			g: true,
			h: true,
			i: true,
			j: true,
			k: true,
			l: true,
			m: true,
			n: true,
			o: true,
			p: true,
			q: true,
			r: true,
			s: true,
			t: true,
			u: true,
			v: true,
			w: true,
			x: true,
			y: true,
			z: true,
		};

		for form in &self.customs_forms {
			if !form.a { group_unanimous_form.a = false };
			if !form.b { group_unanimous_form.b = false };
			if !form.c { group_unanimous_form.c = false };
			if !form.d { group_unanimous_form.d = false };
			if !form.e { group_unanimous_form.e = false };
			if !form.f { group_unanimous_form.f = false };
			if !form.g { group_unanimous_form.g = false };
			if !form.h { group_unanimous_form.h = false };
			if !form.i { group_unanimous_form.i = false };
			if !form.j { group_unanimous_form.j = false };
			if !form.k { group_unanimous_form.k = false };
			if !form.l { group_unanimous_form.l = false };
			if !form.m { group_unanimous_form.m = false };
			if !form.n { group_unanimous_form.n = false };
			if !form.o { group_unanimous_form.o = false };
			if !form.p { group_unanimous_form.p = false };
			if !form.q { group_unanimous_form.q = false };
			if !form.r { group_unanimous_form.r = false };
			if !form.s { group_unanimous_form.s = false };
			if !form.t { group_unanimous_form.t = false };
			if !form.u { group_unanimous_form.u = false };
			if !form.v { group_unanimous_form.v = false };
			if !form.w { group_unanimous_form.w = false };
			if !form.x { group_unanimous_form.x = false };
			if !form.y { group_unanimous_form.y = false };
			if !form.z { group_unanimous_form.z = false };
		}

		group_unanimous_form.get_answers().len() as u32
	}
}