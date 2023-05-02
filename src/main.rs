/*
This is my program to implement Linear Algebra Calculations (and possibly visualizations) in Rust!
Current workflow: Micro editor, chatGPT help, Learning to Program Rust 2nd edition (rust 2021)

TO DO:
Documentation
Vector Struct:
	Add iter() method

*/



use std::io;
//Here we will declare libraries we want to bring into scope.

//Here I will define the most basic component of Linear Algebra, the vector


struct Vector {
	elements: Vec<f64>	
}

impl Vector{
	fn new(elements: Vec<f64>) -> Self{
		Self {elements}
	}

	fn iter(&self) -> std::slice::Iter<'_,f64> {
		self.elements.iter()
	}

	fn enumerate(&self) -> std::iter::Enumerate<std::slice::Iter<'_, f64>> {
		self.elements.iter().enumerate()
	}

	fn scale(&self, scalar: f64) -> Vector {
		let mut new_data = self.elements.clone();
		for value in new_data.iter_mut() {
			*value *= scalar;
		}
		Vector::new(new_data)
	}

	fn add(&self, operand: &Vector) -> Vector {
		let mut new_data = self.elements.clone();
		for (i, &value) in operand.elements.iter().enumerate() {
			new_data[i] += value;
		}
		Vector::new(new_data)
	}
}

fn main() {
	let v_1 = Vector::new(vec![1.0,2.0,3.0]);
	let v_2 = Vector::new(vec![1.0,1.0,1.0]);
	let mut v_3 = v_1.add(&v_2);
	v_3 = v_3.scale(1.17);
	for(i, &value) in v_3.iter().enumerate() {
		println!("{value}",);
	}
}
