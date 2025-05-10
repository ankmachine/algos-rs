#[derive(Debug, PartialEq)]
pub struct LinkedList{
    head: Option<Box<Node>>,
}
impl LinkedList{
    pub fn empty() -> LinkedList{
        LinkedList{head: None}
    }
    pub fn push(&mut self, elem: i32) {
        let old_head = self.head.take();
        let new_head = Box::new(Node {
                elem,
                next: old_head,
            });
        self.head = Some(new_head);
    }
    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
    pub fn peek(&self) -> Option<&i32> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

#[derive(Debug, PartialEq)]
struct Node {
    elem: i32,
    next: Option<Box<Node>>,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn works() {
        let mut list:LinkedList = LinkedList::empty();
        list.push(3);
        list.push(2);
        assert_eq!(list, LinkedList { head: Some(Box::new(Node { elem: 2, next: Some(Box::new(Node { elem: 3, next: None })) })) });
        assert_eq!(list.peek(), Some(&2));
        assert_eq!(list.pop(), Some(2));
    }
}
