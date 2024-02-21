#[derive(Debug)]
pub struct Queue<T> {
    pub size: usize,
    pub head: usize,
    pub tail: usize,
    pub data: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new(size:usize) -> Self {
        Queue {
            size,
            head: 0,
            tail: 0,
            data: Vec::with_capacity(size),
        }    
    }

    pub fn push(&mut self, element: T){
        if self.tail == self.data.len() && self.tail < self.size {
            self.data.push(element);
        } else {
            self.data[self.tail] = element;
        }

        if self.tail + 1 < self.size {
            self.tail += 1;
        } else {
            self.tail = 0;
        }
    }

    pub fn pop(&mut self) -> Option<&T>{
        let value = self.data.get(self.head);

        if self.head + 1 < self.size {
            self.head += 1;
        } else {
            self.head = 0;
        }

        value
    }
}



fn main() {
    let mut queue = Queue::new(5);
    queue.push(1);
    queue.push(2);
    queue.push(3);
    queue.push(4);
    queue.push(5);
    queue.pop();
    queue.pop();

    println!("QUEUE is : {:?}", queue);
}
