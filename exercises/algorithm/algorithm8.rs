/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if let Some(value) = self.elements.pop() {
            Some(value)
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }
}

pub struct MyStack<T> {
    q1: Queue<T>,
    q2: Queue<T>,
}

impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::new(),
            q2: Queue::new(),
        }
    }

    pub fn push(&mut self, elem: T) {
        self.q1.enqueue(elem);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.q1.is_empty() {
            if !self.q2.is_empty() {
                // Transfer all elements from q2 to q1
                while let Some(value) = self.q2.dequeue() {
                    self.q1.enqueue(value);
                }
            }
        }

        // Now q1 is non-empty or both queues are empty
        self.q1.dequeue()
    }

    pub fn is_empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_with_queues() {
        let mut s = MyStack::<i32>::new();
        assert!(s.pop().is_none());
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(2));
        s.push(4);
        s.push(5);
        assert!(!s.is_empty());
        assert_eq!(s.pop(), Some(5));
        assert_eq!(s.pop(), Some(4));
        assert_eq!(s.pop(), Some(1));
        assert!(s.pop().is_none());
        assert!(s.is_empty());
    }
}