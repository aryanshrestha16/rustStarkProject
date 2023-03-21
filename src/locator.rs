use std::cmp::Ordering;
use std::collections::HashMap;

pub trait PriorityQueue<T: PartialOrd> {
    fn enqueue(&mut self, ele: T) -> ();
    fn dequeue(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
}

/**
    An optional definition of a Node struct you may find useful
**/
struct Node<T> {
    priority: i32,
    data: T,
}

/** 
    These traits are implemented for Nodes to make them comparable 
**/
impl<T> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Node<T>) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self.priority == other.priority
    }
}


/** 
    You must implement the above trait for the vector type 
**/
impl<T: PartialOrd> PriorityQueue<T> for Vec<T> {
    /**
        This functions pushes a given element onto the queue and
        reorders the queue such that the min heap property holds.
        See the project specifications for more details on how this
        works.
    **/
    fn enqueue(&mut self, ele: T) -> () {
        self.push(ele);
        let mut index = self.len() - 1;
        while index != 0 {
            if self[(index - 1)/2] > self[index]{
                self.swap((index - 1)/2,index);
                index = (index - 1 )/ 2 ;
            }
            else  {
            index = 0;
            }
        }
    }

    /**
        This function removes the root element from the queue and
        reorders the queue such that it maintains the min heap
        property.  See the project specifications for more details.
        You should return the deleted element in the form of an option.
        Return None if the queue was initially empty, Some(T) otherwise.
    **/
    fn dequeue(&mut self) -> Option<T> {
        if self.len() == 0 {
            return None;
        }
        let mut index = 0;
        let root = self.swap_remove(0);
        while index < self.len() {
            if (index * 2 ) + 1 < self.len(){
                if (index * 2 ) + 2 < self.len(){
                    if self [(index * 2) + 1 ] < self [(index * 2) + 2] && self[(index * 2) + 1] < self[index] {
                       self.swap((index * 2) + 1,index);
                       index = index * 2 + 1;
                    }
                    else if self [(index * 2) + 2 ] < self [(index * 2) + 1] && self[(index * 2) + 2] < self[index]{
                        self.swap((index * 2) + 2,index);
                        index = index * 2 + 2;
                    }
                    else{
                        break;
                    }
                }
                else {
                    if self[(index * 2) + 1 ] < self[index]{
                        self.swap((index * 2 ) + 1 , index);
                        index = (index * 2) + 1;
                    }
                    else {
                        break;
                    }
                }
            }
            else {
                break;
            }
        }
        return Some (root);
    }

    /**
        This function returns the element that would be removed
        if dequeue were called on the queue.  There should be no
        mutations to the queue.  Return the element in the form
        of an option.  Return None if the queue is empty, Some(T)
        otherwise.
    **/
    fn peek(&self) -> Option<&T> {
        if self.len() == 0 {
            return None;
        }
        else {
            return self.get(0);
        }
    }
}


/**
    You must implement this function that computes the orthogonal
    distance between two coordinates.  Remember, orthogonal distance
    is not like Euclidean distance.  See the specifications for more
    details.
**/
pub fn distance(p1: (i32,i32), p2: (i32,i32)) -> i32 {
    return (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();
}

/**
    You must implement this function that determines which enemy Stark
    should battle and their coordinates.  You are given two hashmaps for
    allies and enemies.  Each maps a name to their current coordinates.
    You can assume that the allies hashmap will always have a name
    called "Stark" included.  Return the name and coordinates of the enemy
    Stark will battle in the form of a 3-tuple.  See the specifications
    for more details on how to choose which enemy.
**/
pub fn target_locator<'a>(allies: &'a HashMap<&String, (i32,i32)>, enemies: &'a HashMap<&String, (i32,i32)>) -> (&'a str,i32,i32) {
    if enemies.is_empty() {
        return ("No Enemies",0,0);
    }
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();

    for (a,apos) in allies{
        for (e,epos) in enemies{
            let x = Node {priority: distance(*apos,*epos) , data: (a,e,epos)};
            v1.enqueue(x);
        }
    }
    while v1.peek() != None {
        let close = v1.dequeue();
        match close {
            Some(y) => {
                if y.data.0 == &"Stark" {
                    if v2.contains(y.data.1) == false {
                        return (y.data.1,y.data.2.0,y.data.2.1);
                    }
                    if v2.contains(y.data.0) == false {
                        v2.push(y.data.0);
                        v2.push(y.data.1);
                    }
                }

            }
            None => return ("Error",0,0),
        }
    }
    return ("Error",0,0);
}


