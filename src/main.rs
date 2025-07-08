
#[derive(Debug)]
struct LinkedListNode<T> {
    value: T, //generic T so node can store any type of value
    next: <Option<Box<LinkedListNode<T>>> // Box is used because we dont know the size of next in
                                          // compile time
}
    
//impl for this stuct which create new node using the given value 
impl<T> LinkedListNode<T> { 
    fn new(value: T, next: Option<Box<LinkedListNode<T>>>) -> LinkedListNode<T> { 
        LinkedListNode { value, next }
    }
}

//defining LinedList struct that contain the head and tail
#[derive(Debug)]
struct LinkedList<T> { 
    head: Option<Box<LinkedListNode<T>>>,
    tail: Option<*mut LinkedListNode<T>>, // used raw_pointer to avoid borrowing issue  when we
    // need to modify the tail
}


fn main() {
    println!("Hello, world!");
}
