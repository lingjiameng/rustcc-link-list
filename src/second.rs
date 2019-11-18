use std::mem;

pub struct List{
    head: Link,
}

type Link = Option<Box<Node>>;

pub struct Node{
    elem: i32,
    next: Link,
}

impl List{
    pub fn new() ->Self{
        List{ head: None }
    }

    pub fn push(&mut self, elem:i32){
        let new_node = Box::new(Node{
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
    pub fn pop(&mut self) -> Option<i32>{
        match self.head.take(){
            None => None,
            Some(box_node) => {
                self.head = box_node.next;
                Some(box_node.elem)
            },
        }
    }
    pub fn top(&self)->Option<&i32>{
        match &self.head{
            None => None,
            Some(ref box_node)=>{
                Some(&box_node.elem)
            }
        }
    }
}

impl Drop for List{
    fn drop(&mut self){
        let mut cur_link = self.head.take();
        while let Some(mut box_node) = cur_link{
            cur_link = box_node.next.take();
        }
    }
}