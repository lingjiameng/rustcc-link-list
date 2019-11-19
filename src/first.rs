//! an bad i32 stack based on link list
//!list is empty or an elem fallow by a list
//! ## list struct
//! ptr -> | val | ptr | -> | val | ptr(null) |,
//! **list <= ptr**, **node <= [val ,ptr]**,
//! 非常反常识的，链表(ptr)包含在节点(val,ptr)中
use std::mem;

pub struct List{
    head : Link,
}

enum Link{
    Empty,
    More(Box<Node>),
}

struct Node{
    elem: i32,
    next: Link,
}

impl List{
    pub fn new() -> Self{
        List{ head: Link::Empty }
    }

    /// push elem into stack
    pub fn push(&mut self,elem : i32){
        let new_node = Box::new(Node{
            elem: elem, // elem copy to heap here
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    /// move top elem of stack out
    pub fn pop(&mut self) -> Option<i32>{
        match mem::replace(&mut self.head, Link::Empty){
            Link::Empty => None,
            Link::More(node) =>{
                self.head = node.next;
                Some(node.elem)
            },
        }
    }
    /// return ref of top elem in stack
    pub fn top(&self) -> Option<&i32>{
        match &self.head{
            Link::Empty => None,
            Link::More(node) => Some(&node.elem),
        }
    }
}

impl Drop for List{
    fn drop(&mut self){
        let mut cur_link = mem::replace(&mut self.head,Link::Empty);
        
        //so will cur_link
        while let Link::More(box_node) = cur_link{
            cur_link = box_node.next; // partial move of box_node here
            //box_node goes out of scope and gets dropped here
        }
        // 下面这种方法也可以，应该比上面更差一点。
        // 两者共同优点是没有 move elem
        // while let Link::More(mut boxed_node) = cur_link {
        //     cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        // }
    }
}

#[cfg(test)]
mod test{
    use super::List;
    
    #[test]
    fn basics(){
        // empty list;
        let mut stack : List = List::new();
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