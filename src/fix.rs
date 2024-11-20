use crate::node::Root;
impl <K, O, V> Root<K, O, V> {
pub fn fix_right_border_of_plentiful(&mut self) {
        let mut cur_node = self.borrow_mut();
        while let Internal(internal) = cur_node.force() {
            let mut last_kv = internal.last_kv().consider_for_balancing();
            assert!(last_kv.left_child.len() >= MIN_LEN * 2);
            let right_child_len = last_kv.right_child_len();
            if right_child_len < MIN_LEN {
                last_kv.bulk_steal_left(MIN_LEN - right_child_len);
            }
            cur_node = last_kv.into_right_child();
        }
    }
}
