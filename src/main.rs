use std::fmt;

#[derive(Debug)]
// Define a single node of the linked list
struct LinkedListNode<T> {
    value: T, // The data/value stored in the node
    next: Option<Box<LinkedListNode<T>>>, // Pointer to the next node (if any)
}

impl<T> LinkedListNode<T> {
    // Create a new node with a given value and next pointer
    fn new(value: T, next: Option<Box<LinkedListNode<T>>>) -> LinkedListNode<T> {
        LinkedListNode { value, next }
    }
}

#[derive(Debug)]
// Define the LinkedList structure, which keeps track of the head and tail
struct LinkedList<T> {
    head: Option<Box<LinkedListNode<T>>>, // Points to the first node
    tail: Option<*mut LinkedListNode<T>>, // Raw pointer to the last node (for fast appending)
}

impl<T: PartialEq> LinkedList<T> {
    // Create an empty linked list
    fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    // Insert a value at the front (head) of the list
    fn prepend(&mut self, value: T) {
        // Create a new node, pointing to the current head
        let mut new_node = Box::new(LinkedListNode::new(value, self.head.take()));
        let raw_node: *mut _ = &mut *new_node; // Get raw pointer before moving ownership

        // If the list was empty, the new node is also the tail
        if self.tail.is_none() {
            self.tail = Some(raw_node);
        }

        self.head = Some(new_node); // Set new node as the new head
    }

    // Insert a value at the end (tail) of the list
    fn append(&mut self, value: T) {
        let mut new_node = Box::new(LinkedListNode::new(value, None));
        let raw_node: *mut _ = &mut *new_node;

        if let Some(tail) = self.tail {
            // SAFETY: we trust the raw pointer is valid because we control updates
            unsafe {
                (*tail).next = Some(new_node);
            }
        } else {
            // If the list was empty, new node becomes the head
            self.head = Some(new_node);
        }

        self.tail = Some(raw_node); // Always update the tail to the new node
    }

    // Insert a value at a specific index in the list
    fn insert(&mut self, value: T, index: usize) {
        if index == 0 {
            // Special case: insert at head
            self.prepend(value);
            return;
        }

        let mut current_index = 1;
        let mut current_node = self.head.as_mut();

        // Traverse to the node before the insertion point
        while let Some(node) = current_node {
            if current_index == index {
                let new_node = Box::new(LinkedListNode::new(value, node.next.take()));
                node.next = Some(new_node);
                return;
            }
            current_node = node.next.as_mut();
            current_index += 1;
        }

        // If index is beyond list length, append at end
        self.append(value);
    }
}

impl<T: PartialEq> LinkedList<T> {
    // Find a node by value (returns a reference to the node)
    fn find(&self, value: &T) -> Option<&LinkedListNode<T>> {
        let mut current = self.head.as_deref(); // Convert Option<Box<T>> to Option<&T>

        while let Some(node) = current {
            if &node.value == value {
                return Some(node); // Found a match
            }
            current = node.next.as_deref();
        }
        None // Not found
    }
}

impl<T: PartialEq> LinkedList<T> {
    // Delete the first node that matches the given value
    fn delete(&mut self, value: T) {
        // If the list is empty, do nothing
        if self.head.is_none() {
            return;
        }

        // Special case: delete one or more nodes from the head
        while let Some(ref mut head_node) = self.head {
            if head_node.value == value {
                self.head = head_node.next.take(); // Move head to next node
                if self.head.is_none() {
                    self.tail = None; // If list is now empty, clear tail
                }
            } else {
                break;
            }
        }

        // Traverse the rest of the list to delete matching node
        let mut current = self.head.as_deref_mut();

        while let Some(node) = current {
            if let Some(ref mut next_node) = node.next {
                if next_node.value == value {
                    // Skip over the next node by taking its next
                    let next_next = next_node.next.take();
                    node.next = next_next;

                    // If we deleted the tail, update tail pointer
                    if node.next.is_none() {
                        self.tail = Some(node as *mut _);
                    }
                    return;
                }
            }
            current = node.next.as_deref_mut();
        }
    }
}

// Implement Display so we can print the list cleanly
impl<T: fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = &self.head;

        // Traverse the list and print each value
        while let Some(node) = current {
            write!(f, "{} -> ", node.value)?;
            current = &node.next;
        }

        write!(f, "None")
    }
}

// Demo in main
fn main() {
    let mut list = LinkedList::new();

    // Insert some values
    list.prepend(10);
    list.prepend(20);
    list.prepend(30);
    list.append(1); // Insert at tail
    list.insert(3000, 2); // Insert at index 2

    println!("LISTING LINKED LIST");
    println!("{}", list); // Print entire list

    // Delete value
    list.delete(20);
    println!("LISTING AFTER DELETING");
    println!("{}", list);

    // Try finding a value
    if let Some(node) = list.find(&20) {
        println!("found {}", node.value);
    } else {
        println!("not found");
    }
}

