use std::ptr;

pub struct Queue<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { head: None, tail: ptr::null_mut() }
    }

    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node {
            elem: elem,
            next: None,
        });

        let raw_tail: *mut _ = &mut *new_tail;

        if !self.tail.is_null() {
            unsafe{ (*self.tail).next = Some(new_tail); }
        } else {
            self.head = Some(new_tail);
        }

        self.tail = raw_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;

            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }

            head.elem
        })
    }

	// Iters identical to Stack
}

#[cfg(test)]
mod test {
    use super::Queue;
    #[test]
    fn basics() {
        let mut queue = Queue::new();

        // Check empty queue behaves right
        assert_eq!(queue.pop(), None);

        // Populate queue
        queue.push(1);
        queue.push(2);
        queue.push(3);

        // Check normal removal
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        queue.push(4);
        queue.push(5);

        // Check normal removal
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), Some(4));

        // Check exhaustion
        assert_eq!(queue.pop(), Some(5));
        assert_eq!(queue.pop(), None);

        // Check the exhaustion case fixed the pointer right
        queue.push(6);
        queue.push(7);

        // Check normal removal
        assert_eq!(queue.pop(), Some(6));
        assert_eq!(queue.pop(), Some(7));
        assert_eq!(queue.pop(), None);
    }
}
        


