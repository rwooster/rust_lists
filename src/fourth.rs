use std::rc::Rc;
use std::cell::{Ref, RefMut, RefCell};

pub struct Deque<T> {
    head: Link<T>,
    tail: Link<T>,
}

pub struct IntoIter<T>(Deque<T>);

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> Deque<T> {
    pub fn new() -> Self {
        Deque { head: None, tail: None }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    // Stop pointing at old head
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    // Empty list if this is only thing
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }

	pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
		self.head.as_ref().map(|node| {
			RefMut::map(node.borrow_mut(), |node| &mut node.elem)
		})
	}

	pub fn pop_back(&mut self) -> Option<T> {
		unimplemented!()
	}

	// the reverse cases will basically be the same. No real point in going through them
	// s/tail/head
	// s/next/prev
	// s/front/back

	pub fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}
}

impl<T> Iterator for IntoIter<T> {
	type Item = T;

	fn next(&mut self) -> Option<T> {
		self.0.pop_front()
	}
}

impl<T> DoubleEndedIterator for IntoIter<T> {
	fn next_back(&mut self) -> Option<T> {
		self.0.pop_back()
	}
}
		

#[cfg(test)]
mod test {
    use super::Deque;

    #[test]
    fn basics() {
        let mut deque = Deque::new();

        // Check empty deque behaves right
        assert_eq!(deque.pop_front(), None);

        // Populate deque
        deque.push_front(1);
        deque.push_front(2);
        deque.push_front(3);

        // Check normal removal
        assert_eq!(deque.pop_front(), Some(3));
        assert_eq!(deque.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        deque.push_front(4);
        deque.push_front(5);

        // Check normal removal
        assert_eq!(deque.pop_front(), Some(5));
        assert_eq!(deque.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), None);
    }


	#[test]
	fn peek() {
		let mut deque = Deque::new();
		assert!(deque.peek_front().is_none());
		deque.push_front(1); deque.push_front(2); deque.push_front(3);

		assert_eq!(&*deque.peek_front().unwrap(), &3);
	}
}
