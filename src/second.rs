/// A stack impl by link list
pub struct List<T>{
    head: Link<T>,
}
type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T>{
    elem: T,
    next: Link<T>,
}

/// stack into iter : T , consume all data in stack
pub struct IntoIter<T>(List<T>);

impl<T> List<T>{
    pub fn into_iter(self) -> IntoIter<T>{
        IntoIter(self)
    }
}
impl<T> Iterator for IntoIter<T>{
    type Item = T;
    fn next(&mut self)->Option<Self::Item>{
        self.0.pop()
    }
}

/// stack iter : &T , not consume any data in stack
pub struct Iter<'a,T>{
    next: Option<&'a Node<T>>,
}

impl<T> List<T>{
    pub fn iter<'a>(&'a self) -> Iter<'a,T>{
        Iter{ next: self.head.as_ref().map(|node| &**node) }
        // bellow methods all are fine
        // Iter{ next: self.head.as_ref().map(|node| node.as_ref()) }
        // Iter{ next: self.head.as_ref().map::<&Node<T>,_>(|node| node) }
    }
}

impl<'a,T> Iterator for Iter<'a,T>{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item>{
        self.next.map(|node|{
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
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
       self.head.as_ref().map(|node|{
           &node.elem
       })
    }
    pub fn top_mut(&mut self)->Option<&mut T>{
        self.head.as_mut().map(|node|{
            &mut node.elem
        })
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
        for i in 0..=5 {
            stack.push(i);
        }
        for i in (0..=5).rev(){
            assert_eq!(Some(i),stack.pop());
        }
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn top(){
        let mut stack:List<i32> = List::new();
        assert_eq!(stack.top(), None);
        assert_eq!(stack.top_mut(), None);
        stack.push(1); stack.push(2); stack.push(3);
        assert_eq!(stack.top(), Some(&3));
        assert_eq!(stack.top_mut(), Some(&mut 3));
        
        stack.top_mut().map(|v|{
            *v = 32;
        });
        assert_eq!(stack.top(), Some(&32));
        assert_eq!(stack.top_mut(), Some(&mut 32));

    }

    #[test]
    fn into_iter(){
        let mut stack :List<i32> = List::new();
        for i in 0..3{
            stack.push(i);   
        }
        let mut iter = stack.into_iter();
        for i in (0..3).rev(){
            assert_eq!(iter.next(), Some(i));
        }
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn iter() {
        let mut stack :List<i32> = List::new();
        for i in 0..3{
            stack.push(i);   
        }
        let mut iter = stack.iter();
        for i in (0..3).rev(){
            assert_eq!(iter.next(), Some(&i));
        }
        assert_eq!(iter.next(), None);
        for i in (0..3).rev(){
            assert_eq!(stack.pop(), Some(i));        
        }
    }

}
