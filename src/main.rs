use rust_linked_list::*;

fn main() {
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

    while let Some(mut list) = list_of_lists.pop() {
        while let Some(value) = list.pop() {
            println!("Popped value: {:?}", value);
        }
    }
}