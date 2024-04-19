// Define a Node struct to represent each element in the linked list
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

// Define a LinkedList struct to represent the linked list itself
pub struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    // Function to create a new empty linked list
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    // Function to insert a new node at the tail or a specific index
    pub fn insert(&mut self, data: i32, index: Option<usize>) {
        match index {
            Some(idx) => self.insert_at_index(data, idx),
            None => self.insert_at_tail(data),
        }
    }

    // Function to insert a new node at a specific index
    fn insert_at_index(&mut self, data: i32, index: usize) {
        let list_length = self.length();

        if index > list_length {
            panic!("Index out of bounds");
        }

        let mut current = &mut self.head;
        let mut current_index = 0;

        while current_index < index {
            if let Some(node) = current {
                current = &mut node.next;
                current_index += 1;
            } else {
                break;
            }
        }

        let new_node = Some(Box::new(Node {
            data,
            next: current.take(),
        }));
        *current = new_node;
    }

    // Function to insert a new node at the tail of the list
    fn insert_at_tail(&mut self, data: i32) {
        let mut current = &mut self.head;

        while let Some(node) = current {
            if node.next.is_none() {
                node.next = Some(Box::new(Node {
                    data,
                    next: None,
                }));
                return;
            }
            current = &mut node.next;
        }

        // If the list is empty
        *current = Some(Box::new(Node {
            data,
            next: None,
        }));
    }

    // Function to delete a node at a specific index
pub fn delete(&mut self, index: usize) {
    let list_length = self.length();

    if index >= list_length {
        panic!("Index out of bounds");
    }

    if index == 0 {
        self.head = self.head.take().unwrap().next;
        return;
    }

    let mut current = &mut self.head;
    let mut current_index = 0;

    while let Some(node_ref) = current {
        if current_index + 1 == index {
            let next_node = node_ref.next.take().unwrap().next;
            node_ref.next = next_node;
            return;
        }
        current = &mut node_ref.next;
        current_index += 1;
    }
}



    // Function to retrieve the value of a node at a specific index
    pub fn get(&self, index: usize) -> Option<i32> {
        let list_length = self.length();

        if index >= list_length {
            return None;
        }

        let mut current = &self.head;
        let mut current_index = 0;

        while let Some(node) = current {
            if current_index == index {
                return Some(node.data);
            }
            current = &node.next;
            current_index += 1;
        }

        None
    }

    // Function to calculate the length of the linked list
    fn length(&self) -> usize {
        let mut current = &self.head;
        let mut len = 0;

        // Traverse the list and count the number of nodes
        while let Some(node) = current {
            len += 1;
            current = &node.next;
        }

        len
    }
}

fn main() {
    // Create a new linked list
    let mut list = LinkedList::new();

    // Insert elements into the linked list
    list.insert(10, None); // Insert at tail
    list.insert(20, Some(1)); // Insert at index 1
    list.insert(30, Some(2)); // Insert at index 2
    list.insert(40, None); // Insert at tail

    // Update an element in the linked list
    list.insert(35, Some(2)); // Update the element at index 2 to value 35

    // Print the contents of the linked list
    println!("Linked List:");
    for i in 0..list.length() {
        if let Some(data) = list.get(i) {
            println!("Index {}: {}", i, data);
        }
    }

    // Delete an element from the linked list
    list.delete(1); // Delete the element at index 1

    // Print the updated contents of the linked list
    println!("Linked List after deletion:");
    for i in 0..list.length() {
        if let Some(data) = list.get(i) {
            println!("Index {}: {}", i, data);
        }
    }

    // Prompt the user to press Enter before exiting
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
}
