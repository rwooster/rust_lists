use std::mem;

/* Stack Interface */
pub struct Stack {
    top: Link,
}


/* Internal */
struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    Rest(Box<Node>),
}

impl Stack {
    pub fn new() -> Self {
        Stack { top: Link::Empty }
    }

    pub fn push(&mut self, value: i32) {
        let new_node = Box::new(Node { 
            elem: value,
            next: mem::replace(&mut self.top, Link::Empty), 
        });

        self.top = Link::Rest(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.top, Link::Empty) {
            Link::Empty => None,
            Link::Rest(boxed_node) => {
                let node = *boxed_node;
                self.top = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for Stack {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.top, Link::Empty);

        while let Link::Rest(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
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
}


