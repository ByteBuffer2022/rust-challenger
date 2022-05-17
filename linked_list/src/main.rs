// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: u32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: u32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    fn append(&mut self, elem: u32) {
        match self.next {
            Some(ref mut a) => a.append(elem),
            None => {
                let node = ListNode::new(elem);
                self.next = Some(Box::new(node))
            }
        }
    }

    fn list(&self) {
        println!("{}", self.val);
        match self.next {
            Some(ref next_address) => next_address.list(),
            None => {}
        }
    }
}

fn is_continue(l1: &Option<Box<ListNode>>, l2: &Option<Box<ListNode>>, carry: &u32) -> bool {
    let result1 = match l1 {
        Some(_) => true,
        None => false,
    };
    let result2 = match l2 {
        Some(_) => true,
        None => false,
    };

    result1 || result2 || (carry > &0)
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut carry:u32 = 0;
    let mut l3: Option<Box<ListNode>> = None;

    let mut l1_clone = l1;
    let mut l2_clone = l2;

    while is_continue(&l1_clone, &l2_clone, &carry) {
        let mut x: u32 = 0;
        let mut y: u32 = 0;

        if let Some(node1) = &l1_clone {
            x = node1.val;
            l1_clone = node1.clone().next;
        }
        if let Some(node2) = &l2_clone {
            y = node2.val;
            l2_clone = node2.clone().next;
        }

        let sum = x + y + carry;
        carry = sum / 10;
        let new_val = sum % 10;

        if let Some(list) = &mut l3 {
            list.append(new_val);
        } else {
            l3 = Some(Box::new(ListNode::new(new_val)));
        }
    }

    l3
}

fn main() {
    // l1 = [2,4,3]
    let mut l1 = ListNode::new(2);
    l1.append(4);
    l1.append(3);

    // l2 = [5,6,4]
    let mut l2 = ListNode::new(5);
    l2.append(6);
    l2.append(4);

    let l3 = add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));

    match l3 {
        Some(list) => list.list(),
        None => {}
    }
}