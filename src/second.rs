pub struct List<T>{
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T>{
    elem: T,
    next: Link<T>,
}

impl<T> List<T>{
    pub fn new() ->Self{
        List{ head: None }
    }

    pub fn push(&mut self, elem: T){
        let new_node = Box::new(Node{
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
    pub fn pop(&mut self) -> Option<T>{
        self.head.take().map(|node|{
            self.head = node.next;
            node.elem
        })
    }
    pub fn top(&self)->Option<&T>{
        match self.head {
            None => None,
            Some(ref box_node) =>{
                Some(&box_node.elem)
            },
        }
    }
}

impl<T> Drop for List<T>{
    fn drop(&mut self){
        let mut cur_link = self.head.take();
        while let Some(mut box_node) = cur_link{
            cur_link = box_node.next.take();
        }
    }
}

#[cfg(test)]
mod test{
    use super::List;

    #[test]
    fn basics(){
        // empty list;
        let mut stack : List<i32> = List::new();
        assert_eq!(stack.top(), None);
        for i in 0..=5 {
            stack.push(i);
        }
        for i in (0..=5).rev(){
            assert_eq!(Some(&i),stack.top());
            assert_eq!(Some(i),stack.pop());
        }
        assert_eq!(stack.pop(), None);
        
        for i in 0..10{
            stack.push(i);
        }
    }
}