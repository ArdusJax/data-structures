use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<QNode<T>>>>;

#[derive(Debug)]
pub struct QNode<T> {
    value: T,
    next: Link<T>,
}

impl<T> QNode<T> {
    pub fn new(item: T) -> Rc<RefCell<QNode<T>>> {
        Rc::new(RefCell::new(QNode {
            value: item,
            next: None,
        }))
    }
}

#[derive(Debug)]
pub struct Queue<T> {
    pub length: i32,
    head: Link<T>,
    tail: Link<T>,
}

impl<T: Copy> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            length: 0,
            head: None,
            tail: None,
        }
    }
    pub fn enqueue(&mut self, item: T) {
        let node = Some(QNode::new(item));
        match self.tail.take() {
            Some(current) => {
                current.borrow_mut().next = node.clone();
                self.tail = node;
                self.length += 1;
            }
            None => {
                self.head = node.clone();
                self.tail = node;
                self.length += 1;
            }
        }
    }
    pub fn dequeue(&mut self) {
        match self.head.take() {
            Some(head) => {
                self.head = Rc::try_unwrap(head)
                    .ok()
                    .expect("unable to dequeue")
                    .into_inner()
                    .next;
                self.length -= 1;
            }
            _ => {}
        }
    }
    pub fn peek(mut self) -> Option<T> {
        if let Some(v) = self.head.take() {
            Some(
                Rc::try_unwrap(v)
                    .ok()
                    .expect("something went wrong")
                    .into_inner()
                    .value,
            )
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_simple_queue() {
        let mut sq: Queue<i32> = Queue::new();
        sq.enqueue(32);
        sq.enqueue(13);
        sq.enqueue(3);
        sq.enqueue(330);
        dbg!(sq);
    }
    #[test]
    fn peek_simple_queue() {
        let mut sq: Queue<i32> = Queue::new();
        sq.enqueue(32);
        sq.enqueue(13);
        sq.enqueue(3);
        sq.enqueue(330);
        let res = sq.peek();
        dbg!(res);
    }
    #[test]
    fn dequeue_simple_queue() {
        let mut sq: Queue<i32> = Queue::new();
        sq.enqueue(32);
        sq.enqueue(13);
        sq.enqueue(3);
        sq.enqueue(330);
        sq.dequeue();
        dbg!(sq);
    }
}
