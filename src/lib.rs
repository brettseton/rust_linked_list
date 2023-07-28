// Define a Node struct to represent individual nodes in the linked list of linked lists
pub struct Node<T> {
    data: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

// Define the LinkedList struct
pub struct LinkedList<T> {
    head: Link<T>
}

impl<T> LinkedList<T> {
    // Create a new empty linked list
    pub fn new() -> Self {
        LinkedList {
            head: None
        }
    }

    // Push a value to the front of the linked list
    pub fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data: data,
            next: self.head.take(),
        });

        self.head = Some(new_node);

    }

    // Pop a value from the front of the linked list
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;

            return node.data;
        })
    }
    
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

// Standard iterator
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.data
        })
    }
}

// Standard into iterator
pub struct IntoIter<T>(LinkedList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// Standard mutable iterator
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.data
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pop_on_empty_lists_is_none() {
        let mut list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_push_and_pop_lists() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(10);
        assert_eq!(list.pop(), Some(10));
        assert_eq!(list.pop(), None);

        list.push(20);
        assert_eq!(list.pop(), Some(20));
        assert_eq!(list.pop(), None);
    }

     #[test]
    fn test_push_and_pop_list_of_lists() {
        let mut list_of_lists: LinkedList<LinkedList<i32>> = LinkedList::new();

        let mut list1: LinkedList<i32> = LinkedList::new();
        list1.push(10);
        list1.push(20);

        let mut list2: LinkedList<i32> = LinkedList::new();
        list2.push(30);
        list2.push(40);
        list2.push(50);

        list_of_lists.push(list1);
        list_of_lists.push(list2);

        if let Some(list) = list_of_lists.pop() {
            let popped_values: Vec<_> = list.into_iter().collect();
            assert_eq!(popped_values, vec![50, 40, 30]);
        } else {
            panic!("Expected a linked list");
        }

        if let Some(mut list) = list_of_lists.pop() {
            list.push(60); // Modify the second list after initialization

            let popped_values: Vec<_> = list.into_iter().collect();
            assert_eq!(popped_values, vec![60, 20, 10]);
        } else {
            panic!("Expected a linked list");
        }
    }

    
    #[test]
    fn test_iterate_empty_list() {
        let list: LinkedList<i32> = LinkedList::new();
        
        let popped_values: Vec<_> = list.into_iter().collect();
        assert_eq!(popped_values, vec![]);
    }
    
    #[test]
    fn test_standard_iter_list() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(10);
        list.push(20);
        list.push(30);
        
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&30));
        assert_eq!(iter.next(), Some(&20));
        assert_eq!(iter.next(), Some(&10));
    }

    #[test]
    fn test_mut_iterator_list() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(10);
        list.push(20);


        for data in list.iter_mut() {
            *data = (*data) * 20;
        } 

        let popped_values: Vec<_> = list.into_iter().collect();
        assert_eq!(popped_values, vec![400, 200]);
    }
}

