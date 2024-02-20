#[derive(Debug)]
struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T> LinkedList<T>{
    pub fn new() -> Self {
        LinkedList(None)
    }

    pub fn push_front(&mut self, data:T){
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }

    pub fn push_back(&mut self , data:T){
        match self.0 {
            Some((_, ref mut child)) => child.push_back(data),
            None => self.push_front(data),
        }
    }
}

fn main() {
    let mut linked_list = LinkedList::new();
    linked_list.push_front(3);
    linked_list.push_back(12);
    linked_list.push_front(1);
    println!("Linked List is : {:?}", linked_list);
}
