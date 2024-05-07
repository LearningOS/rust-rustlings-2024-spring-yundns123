#[derive(Debug)]
struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn len(&self) -> usize {
        self.size
    }

    fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.data.pop()
    }

    fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        self.data.last()
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            return None;
        }
        self.data.last_mut()
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&self) -> Iter<T> {
        let mut iterator = Iter {
            stack: self.data.iter().collect(),
        };
        iterator
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut {
            stack: self.data.iter_mut().collect(),
        };
        iterator
    }
}

struct IntoIter<T>(Stack<T>);

impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(val) = self.0.pop() {
            Some(val)
        } else {
            None
        }
    }
}

struct Iter<'a, T: 'a> {
    stack: Vec<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

struct IterMut<'a, T: 'a> {
    stack: Vec<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

fn bracket_match(bracket: &str) -> bool {
    let mut stack = Stack::new();

    for c in bracket.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => {
                if let Some(top) = stack.peek() {
                    if *top!= '(' {
                        return false;
                    } else {
                        stack.pop();
                    }
                } else {
                    return false;
                }
            }
            ']' => {
                if let Some(top) = stack.peek() {
                    if *top!= '[' {
                        return false;
                    } else {
                        stack.pop();
                    }
                } else {
                    return false;
                }
            }
            '}' => {
                if let Some(top) = stack.peek() {
                    if *top!= '{' {
                        return false;
                    } else {
                        stack.pop();
                    }
                } else {
                    return false;
                }
            }
            _ => {}
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bracket_matching_1() {
        let s = "(2+3){func}[abc]";
        assert_eq!(bracket_match(s), false);
    }

    #[test]
    fn bracket_matching_2() {
        let s = "(2+3)*(3-1";
        assert_eq!(bracket_match(s), false);
    }

    #[test]
    fn bracket_matching_3() {
        let s = "{{([])}}";
        assert_eq!(bracket_match(s), false);
    }

    #[test]
    fn bracket_matching_4() {
        let s = "{{(}[)]}";
        assert_eq!(bracket_match(s), false);
    }

    #[test]
    fn bracket_matching_5() {
        let s = "[[[]]]]]]]]]";
        assert_eq!(bracket_match(s), false);
    }

    #[test]
    fn bracket_matching_6() {
        let s = "";
        assert_eq!(bracket_match(s), true);
    }
}