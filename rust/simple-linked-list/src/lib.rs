pub struct SimpleLinkedList<T> {
  head: Option<Box<Node<T>>>,
}

struct Node<T> {
  element: T,
  next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
      SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
      let mut len = 0;
      let mut pointer = &(self.head);
      while pointer.is_some() {
        len += 1;
        pointer = &(pointer.as_ref().unwrap().next);
      }
      len
    }

    pub fn push(&mut self, _element: T) {
      // let new_node = Node { element: _element, next: self.head };
      // self.head = Option(Box(new_node));
      self.head = Some(Box::new(Node {
        element: _element,
        next: self.head.take()
      }));
    }

    pub fn pop(&mut self) -> Option<T> {
      // let popped = self.head.take();
      // self.head = self.head.next;
      if (self.head.is_none()) {
        return None;
      }
      let mut curr_head = self.head.take().unwrap(); // Node
      self.head = curr_head.next.take(); // Node
      Some(curr_head.element) // T
    }

    pub fn peek(&self) -> Option<&T> {
      match self.head {
        None => None,
        _ => Some(&self.head.as_ref().unwrap().element)
      }
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {  
      let mut list = Self::new();
      let mut pointer = &self.head;
      while pointer.is_some() {
        list.push(pointer.as_ref().unwrap().element.clone());
        pointer = &(pointer.as_ref().unwrap().next);
      }
      list
    }
}

// creates a linked list from a Vector
impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(_item: &[T]) -> Self {
      let mut result = Self::new();
      for i in _item.iter() {
        result.push(i.clone());
      }
      result
    }
}

// creates a Vector from a LinkedList
impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
      let mut result = Vec::new();
      while let Some(i) = self.pop().take() {
        result.push(i);
      }
      result.into_iter().rev().collect()
    }
}
