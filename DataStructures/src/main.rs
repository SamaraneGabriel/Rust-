//@author: Gabriel Samarane 
//THIS FILE SERVES AS PRACTICE FOR THE RUST LANGUAGE AND IT DOES SO BY BEING USED TO IMPLEMENT AND TEST STATIC DATA STRUCTURES


use std::io;

const TAMMAX: usize = 1000;

fn main() {

	//1 FOR STACK, 2 FOR QUEUE, 3 FOR LIST

	println!("STACK: 1 ,  QUEUE: 2  , LIST(1): 3 , LIST(2): 4 , CIRCULAR_QUEUE: 5");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("ERROR");

    let num: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("ERRO");
            return;
        }
    };

	match num {
	    1 => test_stack(),
	    2 => test_queue(),
	    3 => test_list_1(),
	    4 => test_list_2(),
	    5 => test_circular(),
	    _ => {
	        println!("Invalid input");
	    }
	}	
	

    

}

//================================================================================================================================================================//
//TEST MODULES


fn test_stack() {
	let mut stack = Stack::new();
	println!("STACK");
	stack.push(1);
	stack.push(2);
	stack.push(3);
	stack.print();
    let removed: Option<i32> = stack.pop();
    println!("REMOVED: {}", removed.expect("EMPTY STACK!"));     
	stack.print();
}

fn test_queue() {
	let mut queue = Queue::new();
	println!("QUEUE");
	queue.enqueue(1);
	queue.enqueue(2);
	queue.enqueue(3);
	queue.print();
    let removed: Option<i32> = queue.dequeue();
    println!("REMOVED: {}", removed.expect("EMPTY QUEUE!"));     
	queue.print();
}


fn test_list_1() {

	let mut list = List::new();
	println!("LIST");
	println!("START INSERTIONS: ");
	list.insert_start(1);
	list.insert_start(2);
	list.insert_start(3);
	list.print();
	println!("END INSERTIONS: ");
	list.insert_end(4);
	list.insert_end(5);
	list.insert_end(6);
	list.print();
	println!("INSERT -1 ON POS 3");
	list.insert(-1, 3);
	list.print();
	println!("INSERT -1 ON POS 1");
	list.insert(-1, 1);
	list.print();
	println!("REMOVE START");
	let mut removed = list.remove_start();
	println!("REMOVED ELEMENT: {}", removed.expect("EMPTY LIST"));
	list.print();
	println!("REMOVE END");
	removed = list.remove_end();
	println!("REMOVED ELEMENT: {}", removed.expect("EMPTY LIST")); 
	list.print();
	println!("REMOVE POSITION 2");
	removed = list.remove(2);
	println!("REMOVED ELEMENT: {}", removed.expect("EMPTY LIST")); 
	list.print();
	
}

fn test_list_2() {

	let mut list = List::new();
	println!("LIST");
	list.insert_end(4);
	list.insert_end(9);
	list.insert_end(2);
	list.print();
	println!("READ-ONLY INVERSION");
	list.show_invert();
	println!("ACTUAL INVERSION OF THE ELEMENTS");
	list.invert();
	list.print();
	
}

fn test_circular() {

	let mut circular = CircularQueue::new();
	println!("CIRCULAR QUEUE");
	circular.enqueue(1);
	circular.enqueue(2);
	circular.enqueue(3);
	circular.print();
	println!("ADDING 4: ");
	circular.enqueue(4);
	circular.print();
	println!("REMOVING ELEMENT");
	let removed = circular.dequeue();
	println!("REMOVED: {}", removed.expect("EMPTY QUEUE"));
	circular.print();
}


//================================================================================================================================================================//
//STACK

struct Stack {
	counter: usize,
	data: [i32; TAMMAX],
}

impl Stack {

	fn new() -> Stack {
		Stack {
			counter: 0,
			data: [0; TAMMAX],
		} 
	}

	fn print(&self) {
		if self.counter == 0 {
			println!("EMPTY STACK");
		}
		else {
			for i in 0..self.counter {
				println!("{}", self.data[i]);
			}
		}
	}

	fn push(&mut self, x: i32) {
		if self.counter >= TAMMAX {
			println!("STACK OVERFLOW!!");
		}
		else {
			self.data[self.counter] = x;
			self.counter += 1;
		}
	}

	fn pop(&mut self) -> Option<i32> {
		if self.counter == 0 {
			None
		}
		else {
			let removed = self.data[self.counter - 1];
			self.counter -=1;
			Some(removed)
		}
	}
}
//================================================================================================================================================================//
//QUEUE

struct Queue {
	counter: usize,
	data: [i32; TAMMAX],
}

impl Queue {

	fn new() -> Queue {
		Queue {
			counter: 0,
			data: [0; TAMMAX],
		}
	}

	fn print(&self) {
		if self.counter == 0 {
			println!("EMPTY QUEUE!");
		}
		else {
			for i in 0..self.counter {
				print!("[{}]	", self.data[i]);
			}
		}
	}

	fn enqueue(&mut self, x: i32) {
		if self.counter >= TAMMAX {
			println!("FULL QUEUE");
		}
		else {
			self.data[self.counter] = x;
			self.counter += 1;
		}
	}

	fn dequeue(&mut self) -> Option<i32> {
		if self.counter == 0 {
			None
		}
		//SPECIAL CASE FOR 1 ELEMENT REMOVAL
		else if self.counter == 1{
			let removed = self.data[0];					
			self.counter -= 1;
			Some(removed)
		}
		else {
			let removed = self.data[0];

			for i in 1..self.counter {
				self.data[i - 1] = self.data[i];
			}

			self.counter -= 1;
			Some(removed)
		}
	}
}


//================================================================================================================================================================//
//LIST

struct List {
	counter: usize,
	data: [i32; TAMMAX],
}

impl List {

	fn new() -> List {
		List {
			counter: 0,
			data: [0; TAMMAX],
		}
	}

	fn print(&self) {
		if self.counter == 0 {
			println!("EMPTY LIST");
		}
		else {
			for i in 0..self.counter {
				print!("[{}]	", self.data[i]);
			}
		}
		println!();
	}

	fn insert_start(&mut self, x: i32) {
		if self.counter >= TAMMAX {
			println!("THE LIST IS FULL!"); 
		}
		else {
			for i in (1..=self.counter).rev() {
				self.data[i] = self.data[i - 1];
			}
			self.data[0] = x;
			self.counter += 1;
		}
	}

	fn insert_end(&mut self, x: i32) {
		if self.counter >= TAMMAX {
			println!("THE LIST IS FULL!"); 
		}
		else {
			self.data[self.counter] = x;
			self.counter += 1;
		}
	}

	fn insert(&mut self, x: i32, pos: usize) {
		if self.counter >= TAMMAX {
			println!("THE LIST IS FULL!"); 
		}
		else if pos == 0 {
			self.insert_start(x);
		}
		else if pos == self.counter {
			self.insert_end(x);
		}
		else {
			for i in ((pos + 1)..=self.counter).rev() {
				self.data[i] = self.data[i - 1];
			}
			self.data[pos] = x;
			self.counter += 1;
		}
	}

	fn remove_start(&mut self) -> Option<i32> {
		if self.counter == 0 {
			None 
		}
		else {
			let removed = self.data[0];
			for i in 0..self.counter {
				self.data[i] = self.data[i + 1];
			}
			self.counter -= 1;
			Some(removed)
		}
	}

	fn remove_end(&mut self) -> Option<i32> {
		if self.counter == 0 {
			None
		}
		else {
			let removed = self.data[self.counter - 1];
			self.counter -= 1;
			Some(removed)
		}
	}

	fn remove(&mut self, pos: usize) -> Option<i32> {
		if self.counter == 0 {
			None
		}
		else if pos == self.counter - 1 {
			self.remove_end()
		}
		else if pos == 0 {
			self.remove_start()
		}
		else {
			let removed = self.data[pos];
			for i in pos..self.counter {
				self.data[i] = self.data[i + 1];
			}
			self.counter -= 1;
			Some(removed)
		}
	}

	fn show_invert(&self) {
		if self.counter == 0 {
			println!("EMPTY LIST");
		}
		else{
			for i in (0..self.counter).rev() {
				print!("[{}]	", self.data[i]);
			}
			println!();
		}
	}

	fn swap(&mut self, pos1: usize, pos2: usize) {
		let temp = self.data[pos1];
		self.data[pos1] = self.data[pos2];
		self.data[pos2] = temp;
	}

	fn invert(&mut self) {
		if self.counter == 0 {
			println!("EMPTY LIST");
		}
		else {
			for cont in 0..self.counter/2 {
				self.swap(cont, self.counter - (cont + 1));
			} 
		}		
	}
	
}

//================================================================================================================================================================//
//CIRCULAR QUEUE

 struct CircularQueue {
 	 data: [i32; TAMMAX],
 	 first: usize,
 	 last: usize,
 }

 impl CircularQueue {

	fn new() -> CircularQueue {
		CircularQueue {
			data: [0; TAMMAX],
			first: 0,
			last: 0,
		}
	}

	fn print(&self) {
		if self.last == self.first {
			println!("EMPTY QUEUE!");
		}
		else {
			let mut i = self.first;
			while i != self.last {
				print!("[{}]	", self.data[i]);
				i = (i + 1) % TAMMAX;
			}
			println!();
		}
	}

	fn enqueue(&mut self, x: i32) {
		if (self.last + 1) % TAMMAX == self.first {
			println!("QUEUE IS FULL!");
		}
		else {
			self.data[self.last] = x;
			self.last = (self.last + 1) % TAMMAX;
		}
	}

	fn dequeue(&mut self) -> Option<i32> {
		if self.last == self.first {
			None
		}
		else {
			let removed = self.data[self.first];
			self.first = (self.first + 1) % TAMMAX;
			Some(removed)
		}
	}
 	
 }


	



