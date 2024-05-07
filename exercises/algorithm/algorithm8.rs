#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value);
    }

    pub fn dequeue(&mut self) -> Result<T, &'static str> {
        if!self.elements.is_empty() {
            Ok(self.elements.remove(0))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &'static str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty")
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct MyStack<T> { // 将 myStack 改为 MyStack
    q1: Queue<T>,
    q2: Queue<T>,
}

impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
        }
    }

    pub fn push(&mut self, elem: T) {
        if self.q1.is_empty() {
            self.q1.enqueue(elem);
        } else {
            let temp = self.q1.dequeue().unwrap(); // 这里改为 expect 来处理错误
            self.q2.enqueue(elem);
            while let Ok(item) = self.q1.dequeue() { // 处理错误
                self.q2.enqueue(item);
            }
            std::mem::swap(&mut self.q1, &mut self.q2);
        }
    }

    pub fn pop(&mut self) -> Result<T, &'static str> {
        if self.q1.is_empty() {
            Err("Stack is empty")
        } else {
            self.q1.dequeue()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut s = MyStack::<i32>::new();
        assert_eq!(s.pop(), Ok(2));

        s.push(1);
        s.push(2);
        s.push(3);

        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));

        s.push(4);
        s.push(5);

        assert_eq!(s.is_empty(), false);

        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));

        assert_eq!(s.pop(), Err("Stack is empty"));

        assert_eq!(s.is_empty(), true);
    }
}