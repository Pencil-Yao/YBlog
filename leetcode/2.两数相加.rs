//----------------------------------------------------------------------------------
type ListNodePtr2 = Box<ListNode2>;

struct ListNode2 {
    val: i32,
    next: Option<ListNodePtr2>,
}

impl ListNode2 {
    fn new(val: i32) -> ListNode2 {
        ListNode2 {
            val,
            next: None,
        }
    }

    fn prepare(mut self, val: i32) -> ListNode2 {
        let mut cur_node = &mut self;
        while cur_node.next.is_some() {
            cur_node  = cur_node.next.as_mut().unwrap().as_mut();
        }
        cur_node.next = Some(Box::new(ListNode2::new(val)));
        self
    }
}

fn two_num_add2(mut l1: Option<ListNodePtr2>, mut l2: Option<ListNodePtr2>) -> Option<ListNodePtr2> {
    let mut step:i32 = 0;
    let mut res = Box::new(ListNode2::new(step));
    let mut cur_node = res.as_mut();

    while l1.is_some() || l2.is_some() {


        let mut l1v = 0;
        let mut l2v = 0;
        if let Some(l1r) = l1 {
            l1v = l1r.val;
            l1 = l1r.next;
        }

        if let Some(l2r) = l2 {
            l2v = l2r.val;
            l2 = l2r.next;
        }

        let mut res_val = l1v + l2v + step;
        step = res_val / 10;
        res_val %= 10;

        cur_node.val = res_val;

        if l1.is_some() || l2.is_some() || step != 0 {
            let next_node = Box::new(ListNode2::new(step));
            cur_node.next = Some(next_node);
            cur_node = cur_node.next.as_mut().unwrap().as_mut();
        }
    };
    Some(res)
}
//----------------------------------------------------------------------------------


fn main() {

    let node_a_list2 = ListNode2::new(1).prepare(2).prepare(3);
    let node_b_list2 = ListNode2::new(4).prepare(5).prepare(9);
    let _res_list2 = two_num_add2(Some(Box::new(node_a_list2)), Some(Box::new(node_b_list2)));
    println!("finish!");
}

