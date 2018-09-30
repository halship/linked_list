extern crate linked_list;
use linked_list::LinkedList;

#[derive(Debug)]
struct Object {
    n: u32,
}

impl Object {
    fn new(n: u32) -> Object {
        let obj = Object { n };
        println!("Created {:?}", obj);

        obj
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        println!("Dropped {:?}", *self);
    }
}

fn main() {
    test1();
    println!("-------------------------------------------");
    test2();
    println!("-------------------------------------------");
    test3();
}

fn test1() {
    let mut list = LinkedList::new();
    
    list.push_back(Object::new(0));
    list.push_back(Object::new(1));
    list.push_back(Object::new(2));

    list.pop_back();
    list.pop_front();

    list.push_front(Object::new(3));
    list.push_back(Object::new(4));
    list.push_front(Object::new(5));
}

fn test2() {
    let mut list = LinkedList::new();
    
    list.push_back(Object::new(0));
    list.push_back(Object::new(1));
    list.push_back(Object::new(2));

    list.clear();

    list.push_back(Object::new(3));
}

fn test3() {
    let mut list = LinkedList::new();

    list.push_back(Object::new(0));
    list.push_back(Object::new(1));
    list.push_back(Object::new(2));
    list.push_back(Object::new(3));
    list.push_back(Object::new(4));
    list.push_back(Object::new(5));

    list.remove_if(|obj| obj.n % 2 == 0);
}
