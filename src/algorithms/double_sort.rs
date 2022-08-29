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

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 0.8];

use super::{Algorithm, Array};
use double_sort::double_heap_sort;

#[derive(PartialEq,PartialOrd,Eq,Debug,Clone, Copy)]
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

    fn contains(&self,element: u32) -> bool {
        if element == self.0 {
            true
        } else {
            false
        }
    }

    fn change_array(&self,index: usize, array: &Array) {
        array.set(index,self.0);

        if let Some(_number) = self.1 {
            array.set(index+1,self.1.unwrap());
        }
    }

    fn sort_array(&self,index: usize, array: &Array) {
        array.set(index,self.0);
        array.set_color(index, GREEN);

        if let Some(_number) = self.1 {
            array.set(index+1,self.1.unwrap());
            array.set_color(index+1,GREEN);
        }
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


pub fn double_graphic_sort(array: Array) {

    //Mutable values used to control the while loop
    let mut counter = 0; //Amount of times the loop ran for
    let mut index = 0;
    let mut nothing = 0; //Amount of times nothing was done on a read

    if array.len() <= 2 {

        if array.len() == 1 {
            return;
        }

        let mut node = Node(array.get(0),Some(array.get(1)));

        node.order();

        return;
    }


    let mut vector = Vec::new();

    let mut node: Node;

    while counter < array.len() - 1 {

        if counter % 2 == 0 {
            node = Node(array.get(counter),Some(array.get(counter+1)));
            node.order();
            vector.push(node);
        }

        counter += 1;
    }
    
    let mut reference_vec = Vec::new();
    let mut temp_vec = Vec::new();

    for node in &vector {
        temp_vec.push(node.0);
    }

    double_heap_sort(&mut temp_vec);

    for reference in temp_vec {
        let left_node = *vector.iter().find(|x| x.contains(reference) == true).unwrap();

        left_node.change_array(index, &array);

        reference_vec.push(left_node);

        index += 2;
    }

    vector = reference_vec;

    //Reset counter
    counter = 0;
    index = 0;

    //Final sort of the values by comparing left and right values of neighbouring nodes
    loop {

        let mut left = vector[counter];

        if counter == vector.len() - 1 {
            left.sort_array(index,&array);
            break;
        }

        let mut right = vector[counter+1];

        let switched: bool; //Checks whether anything was changed

        if let Some(_number) = left.1 {
            switched = switch(left.1.as_mut().unwrap(),&mut right.0);
        } else {
            switched = switch(&mut left.0,&mut right.0);
        }

        if !switched {
            vector[counter] = left;
            left.sort_array(index,&array);

            index += 2;

            //Increment the times where read did nothing
            nothing += 1;
            counter += 1;

            if right.none_present() {
                vector[counter] = right;

                right.sort_array(index,&array);

                index += 2;

                if counter == vector.len() - 1 {
                    break;
                }

                if counter == vector.len() - 2 {
                    let left = vector[counter+1];

                    left.sort_array(index,&array);

                    //Info dump
                    #[cfg(debug_assertions)]
                    {
                        println!("Total reads done: {}",counter);
                        println!("Total number of memory switches: {}", counter - nothing);
                    }

                    break;
                }
                
            }
            continue;
        }

        left.order();
        right.order();

        vector[counter] = left;
        vector[counter+1] = right;
        //Increment counter
        counter += 1;

        left.sort_array(index,&array);

        index += 2;

        //Everything is pushed back into the heap so nothing is lost.

        let mut temp_vec = Vec::new();
        let mut reference_vec = Vec::new();

        for node in &vector {
            temp_vec.push(node.0);
        }
    
        double_heap_sort(&mut temp_vec);

        let mut temp_index = 0;
    
        for reference in temp_vec {
            let left_node = *vector.iter().find(|x| x.contains(reference) == true).unwrap();

            left_node.change_array(temp_index, &array);
    
            reference_vec.push(left_node);

            temp_index += 2;
        }
    
        vector = reference_vec;

    }

    counter = 0;

    //Info dump
    #[cfg(debug_assertions)]
    {
        println!("Total reads done: {}",counter);
        println!("Total number of memory switches: {}", counter - nothing);
    }

}



pub struct DoubleSort;

impl Algorithm for DoubleSort {
    fn sort(&self, array: Array) {
        double_graphic_sort(array);
    }

    fn name(&self) -> String {
        "Double sort".to_string()
    }
}