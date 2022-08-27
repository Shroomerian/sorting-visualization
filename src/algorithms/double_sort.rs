/*!
Double sort works by separating the elements into pairs of two called nodes, this allows
to order the two elements within the node extremely fast since it's a 1:1 memory swap.

Grouping elements by nodes allows a simple loop statement to order the whole vector in n/2 - 1 reads maximum by utilizing BinaryHeaps

# Usage

```toml
[dependencies]
double_sort = "1.0.0"
```
*/

use super::{Algorithm, Array};

use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::cmp::Ordering;
use std::convert::TryInto;

#[derive(PartialEq,PartialOrd,Eq,Debug)]
struct Node(u32,Option<u32>); 

impl Node {
    //Rearranges the node in the correct order
    fn order(&mut self) {
        if let Some(_number) = self.1 {
            switch(&mut self.0,self.1.as_mut().unwrap());
        }
    }

    //Informs if there is None present in the structure
    fn none_present(&self) -> bool {
        if self.1 == None {
            true
        } else {
            false
        }
    }

    fn get_left(&self) -> u32 {
        self.0
    }

    fn get_right(&self) -> u32 {
        self.1.unwrap()
    }

}

//Checks if the left element is the smallest and if not then swaps with the right element so they are ordered from left to right.
fn switch<T: PartialOrd>(left: &mut T,right: &mut T) -> bool {
    if left > right {
        std::mem::swap(left,right);
        return true;
    } else {
    false
    }
}

impl Ord for Node {
    fn cmp(&self,other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

pub fn double_sort(list: Array) {
    

    if list.len() <= 2 {
        let mut node = Node(list.get(0),Some(list.get(1)));
        node.order();

        
        list.set(0,node.get_left());
        list.set_color(0, [0.0, 1.0, 0.0, 0.8]);
        
        if !node.none_present() {
        list.set(1,node.get_right());
        list.set_color(1, [0.0, 1.0, 0.0, 0.8]);
        }
        
        return;
    }

    //BinaryHeap is used due to it's efficient ordering capabilities.
    let mut heap = BinaryHeap::new();

    //Mutable values used to control the while loop
    let _counter = 0; //Amount of times the loop ran for
    let mut nothing = 0; //Amount of times nothing was done on a read

    let mut _counter = 0;

    let mut node: Node;

    while _counter != list.len() {

        let left = list.get(_counter);

        if _counter == list.len() - 1 {
            node = Node(left,None);

            node.order();
            heap.push(Reverse(node));
            break;
        }

        _counter += 1;

        let right = list.get(_counter);

        node = Node(left,Some(right));

        node.order();
        heap.push(Reverse(node));

        _counter += 1;
    }

    _counter = 0;

    let mut total = 0;

    //Final sort of the values by comparing left and right values of neighbouring nodes
    loop {
        let mut left = heap.pop().unwrap().0;

        if heap.is_empty() {
            list.set(_counter, left.get_left());
            list.set_color(_counter, [0.0, 1.0, 0.0, 0.8]);

            _counter += 1;

            if !left.none_present() {
                list.set(_counter,left.get_right());
                list.set_color(_counter, [0.0, 1.0, 0.0, 0.8]);

                _counter += 1;
            }
            
            break;
        }

        let mut right = heap.pop().unwrap().0;

        let switched: bool; //Checks whether anything was changed

        if let Some(_number) = left.1 {
            switched = switch(left.1.as_mut().unwrap(),&mut right.0);
        } else {
            switched = switch(&mut left.0,&mut right.0);
        }

        if !switched {
            list.set(_counter, left.get_left());
            list.set_color(_counter, [0.0, 1.0, 0.0, 0.8]);

            _counter += 1;

            if !left.none_present() {
                list.set(_counter,left.get_right());
                list.set_color(_counter, [0.0, 1.0, 0.0, 0.8]);

                _counter += 1;
            }

            //Increment the times where read did nothing
            nothing += 1;

            if right.none_present() {
                list.set(_counter,right.get_left());
                list.set_color(_counter, [0.0, 1.0, 0.0, 0.8]);

                _counter += 1;

                if heap.len() == 1 {
                    let left = heap.pop().unwrap().0;

                    list.set(_counter, left.get_left());
                    list.set_color(_counter, [0.0, 1.0, 0.0, 0.8]);

                    _counter += 1;
        
                    if !left.none_present() {
                        list.set(_counter,left.get_right());
                        list.set_color(_counter, [0.0, 1.0, 0.0, 0.8]);

                        _counter += 1;
                    }

                    //Info dump
                    #[cfg(debug_assertions)]
                    {
                        println!("Total reads done: {}",_counter);
                        println!("Total number of memory switches: {}", _counter - nothing);
                    }

                    break;
                }
                
            } else {
                heap.push(Reverse(right));
            }

            continue;
        }

        left.order();
        right.order();

        total += 1;

        if heap.is_empty() {
            list.set(_counter, left.get_left());
            list.set_color(_counter, [0.0, 1.0, 0.0, 0.8]);
    
            _counter += 1;
    
            if !left.none_present() {
                list.set(_counter,left.get_right());
                list.set_color(_counter, [0.0, 1.0, 0.0, 0.8]);
    
                _counter += 1;
            }
            
            list.set(_counter, right.get_left());
            list.set_color(_counter, [0.0, 1.0, 0.0, 0.8]);
    
            _counter += 1;
    
            if !right.none_present() {
                list.set(_counter,right.get_right());
                list.set_color(_counter, [0.0, 1.0, 0.0, 0.8]);
    
                _counter += 1;
            }

            break;
        }

        //Everything is pushed back into the heap so nothing is lost.
        list.set(_counter, left.get_left());
        list.set_color(_counter, [0.0, 1.0, 0.0, 0.8]);

        _counter += 1;

        if !left.none_present() {
            list.set(_counter,left.get_right());
            list.set_color(_counter, [0.0, 1.0, 0.0, 0.8]);

            _counter += 1;
        }

        heap.push(Reverse(right));

    }

    //Info dump
    #[cfg(debug_assertions)]
    {
        println!("Total reads done: {}",total);
        println!("Total number of memory switches: {}", total - nothing);
    }

}

pub struct DoubleSort;

impl Algorithm for DoubleSort {
    fn sort(&self, array: Array) {
        double_sort(array);
    }

    fn name(&self) -> String {
        "Double sort".to_string()
    }
}