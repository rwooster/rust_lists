/* Stack Interface */
pub struct Stack<T> {
    top: Link<T>,
}

pub struct IntoIter<T>(Stack<T>);

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}


/* Internal */
struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { top: None }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node { 
            elem: value,
            next: self.top.take(), 
        });

        self.top = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.top.take().map(|boxed_node| {
            let node = *boxed_node;
            self.top = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.top.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.top.as_ref().map(|node| &**node) }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.top.as_mut().map(|node| &mut **node) }
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut cur_link = self.top.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl <'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

impl <'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
	use super::Stack;

    #[test]
    fn basics() {
        let mut stack = Stack::new();

        // Check empty stack behaves right
        assert_eq!(stack.pop(), None);

        // Populate stack
        stack.push(1);
        stack.push(2);
        stack.push(3);

        // Check normal removal
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        stack.push(4);
        stack.push(5);

        // Check normal removal
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(4));

        // Check exhaustion
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn generics() {
        let mut stack = Stack::new();
        stack.push(1);
        assert_eq!(stack.pop(), Some(1));

        let mut stack2 = Stack::new();
        stack2.push("hello");
        assert_eq!(stack2.pop(), Some("hello"));
    }

    #[test]
    fn peek() {
        let mut stack = Stack::new();
		assert_eq!(stack.peek(), None);
		assert_eq!(stack.peek_mut(), None);
		stack.push(1); stack.push(2); stack.push(3);
		
		assert_eq!(stack.peek(), Some(&3));
		assert_eq!(stack.peek_mut(), Some(&mut 3));

        stack.peek_mut().map(|value: &mut i32| { *value = 42 });
		assert_eq!(stack.peek(), Some(&42));
        assert_eq!(stack.pop(), Some(42));
    }

	#[test]
    fn into_iter() {
        let mut stack = Stack::new();
        stack.push(1); stack.push(2); stack.push(3);

        let mut iter = stack.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
    }

	#[test]
    fn iter() {
        let mut stack = Stack::new();
        stack.push(1); stack.push(2); stack.push(3);

        {
            let mut iter = stack.iter();
            assert_eq!(iter.next(), Some(&3));
            assert_eq!(iter.next(), Some(&2));
            assert_eq!(iter.next(), Some(&1));
        }
        stack.push(17);
        let mut iter = stack.iter();
        assert_eq!(iter.next(), Some(&17));
    }

	#[test]
	fn iter_mut() {
		let mut stack = Stack::new();
		stack.push(1); stack.push(2); stack.push(3);

		let mut iter = stack.iter_mut();
		assert_eq!(iter.next(), Some(&mut 3));
		assert_eq!(iter.next(), Some(&mut 2));
		assert_eq!(iter.next(), Some(&mut 1));
	}
}


