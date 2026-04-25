#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn to_list(vals: Vec<i32>) -> Option<Box<ListNode>> {
    vals.into_iter()
        .rev()
        .fold(None, |next, val| Some(Box::new(ListNode { val, next })))
}
