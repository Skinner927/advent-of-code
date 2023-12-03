use std::fs::File;
use std::{io};
use std::io::BufRead;
use std::iter::Iterator;
use std::{cell::RefCell, rc::Rc};
use std::ops::Deref;

// https://rtoch.com/posts/rust-doubly-linked-list/
pub struct ListNode<T> {
    item: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> ListNode<T> {
    fn new(item: T) -> Self {
        Self {
            item,
            next: None,
            prev: None,
        }
    }
}

type RefListNode<T> = Rc<RefCell<ListNode<T>>>;
type Link<T> = Option<RefListNode<T>>;

#[derive(Default)]
pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while let Some(node) = self.head.take() {
            let _ = node.borrow_mut().prev.take();
            self.head = node.borrow_mut().next.take();
        }
        self.tail.take();
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    /// Push to back
    pub fn push(&mut self, item: T) {
        let node = Rc::new(RefCell::new(ListNode::new(item)));
        self.push_node(node);
    }
    pub fn push_node(&mut self, node: RefListNode<T>) {
        if let Some(prev_tail) = self.tail.take() {
            prev_tail.borrow_mut().next = Some(Rc::clone(&node));
            node.borrow_mut().prev = Some(prev_tail);
        } else {
            self.head = Some(Rc::clone(&node));
        }
        self.tail = Some(node);
        self.size += 1;
    }

    /// Pop from back
    pub fn pop(&mut self) -> Option<T> {
        if let Some(tail) = self.tail.take() {
            self.size -= 1;
            if let Some(new_tail) = tail.borrow_mut().prev.take() {
                new_tail.borrow_mut().next = None;
                self.tail = Some(new_tail);
            } else {
                // No prev on our tail, meaning our tail is also the head
                // so pop it off
                self.head.take();
            }
            // Unwrap the reference counter (RC), then assert no errors (ok),
            // then panic if None (unwrap), then get the actual inner value
            // from the RefCel, and finally ListNode.item.
            return Some(Rc::try_unwrap(tail).ok().unwrap().into_inner().item);
        }
        None
    }
}


fn main() {
    let filename = "sample.txt";

    let mut all_nodes: Vec<RefListNode<i64>> = Vec::new();
    let mut dll: DoublyLinkedList<i64> = DoublyLinkedList::new();

    for line in get_lines(filename) {
        if !line.is_empty() {
            let num = line.parse::<i64>().unwrap();
            let node = Rc::new(RefCell::new(ListNode::new(num)));
            dll.push_node(Rc::clone(&node));
            all_nodes.push(node);
        }
    }

    {
        let mut next = &dll.head;
        let mut store_next;
        for i in 0..dll.size {
            let node = match next {
                None => break,
                Some(n) => Rc::clone(n),
            };
            let other: &RefListNode<i64> = &all_nodes[i];

            println!("{i} Rc::ptr_eq {}", Rc::ptr_eq(&node, other));
            println!("{i} std::ptr::eq {}", std::ptr::eq(node.borrow().deref(), other.borrow().deref()));

            store_next = node.borrow().next.as_ref().map(Rc::clone);
            next = &store_next;
        }

        // Equivalent to above
        (0..dll.size).fold(Rc::clone(dll.head.as_ref().unwrap()), |node, i| {
            let other: &RefListNode<i64> = &all_nodes[i];

            println!("{i} Rc::ptr_eq {}", Rc::ptr_eq(&node, other));
            println!("{i} std::ptr::eq {}", std::ptr::eq(node.borrow().deref(), other.borrow().deref()));

            match node.borrow().next.as_ref() {
                Some(n) => Rc::clone(n),
                // This should only happen on last loop
                None => Rc::clone(&node),
            }
        });
    }

}


fn get_lines(filename: &str) -> impl Iterator<Item=String> {
    let file = File::open(filename).unwrap_or_else(|e| panic!("Cannot open {filename}: {e}"));
    let reader = io::BufReader::new(file);
    reader.lines().enumerate().map(|(i, line)| {
        line.unwrap_or_else(|e| panic!("failed to read line {i}: {e}"))
    })
}
