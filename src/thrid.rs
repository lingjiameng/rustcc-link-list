//!shared and persistent link list stack

use std::rc::Rc;
/*
list1 = A -> B -> C -> D
list2 = tail(list1) = B -> C -> D
list3 = push(list2, X) = X -> B -> C -> D

replace Bo to Rc (reference counter ) to get shared ownership

list1 -> A ---+
              |
              v
list2 ------> B -> C -> D
              ^
              |
list3 -> X ---+
*/

pub struct List<T>{
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T>{
    elem: T,
    next: Link<T>,
}

impl<T> List<T>{
    pub fn new() -> Self{
        List{ head: None }
    }

    /// create and return a new list with head node pointed to cur list,head elem is elem.
    /// return new head[ elem, ptr --]-->self
    pub fn append(& self,elem:T) -> List<T>{
        List{head : Some(Rc::new(Node{
                elem: elem,
                next: self.head.clone(),
            }))}
    }

    pub fn tail(&self) -> List<T>{
        List{head: self.head.as_ref().and_then(|n|n.next.clone())}
        //List{head: self.head.as_ref().map(|n|n.next.clone()).flatten()}
    }

    pub fn head(&self) -> Option<&T>{
        self.head.as_ref().map(|node| &node.elem)
    }
    pub fn iter<'a>(&'a self) -> Iter<'a,T>{
        Iter{ next: self.head.as_ref().map(|node| &**node) }
        // bellow methods all are fine
        // Iter{ next: self.head.as_ref().map(|node| node.as_ref()) }
        // Iter{ next: self.head.as_ref().map::<&Node<T>,_>(|node| node) }
    }

}

impl<T> Drop for List<T>{
    fn drop(&mut self){
        let mut cur_link = self.head.take();
        while let Some(node) = cur_link{
            if let Ok(mut node) = Rc::try_unwrap(node){
                cur_link = node.next.take();
            }else{
                break;
            }
        }
    }
}

/// stack iter : &T , not consume any data in stack
pub struct Iter<'a,T>{
    next: Option<&'a Node<T>>,
}

impl<'a,T> Iterator for Iter<'a,T>{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item>{
        self.next.map(|node|{ // move happen here in form of copy
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}


#[cfg(test)]
mod test{
    use super::List;

    #[test]
    fn basics() {
        let list : List<i32> = List::new();
        assert_eq!(list.head(),None);

        let list = list.append(1).append(2).append(3);
        assert_eq!(list.head(),Some(&3));

        let list = list.tail();
        assert_eq!(list.head(),Some(&2));

        let list = list.tail();
        assert_eq!(list.head(),Some(&1));
        
        let list = list.tail();
        assert_eq!(list.head(),None);
       
        let list = list.tail();
        assert_eq!(list.head(),None);
    }

    #[test]
    fn iter() {
        let list = List::new().append(1).append(2).append(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}