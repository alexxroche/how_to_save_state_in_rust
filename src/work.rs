use crate::our_data::ST;
use std::convert::TryInto;

pub fn solve(st: &mut ST) -> () {
    let max_depth: u16 = st.len().try_into().unwrap();
    // if max_depth is even then polarity is 0 else 1
    let polarity = match &max_depth {
        md if md % 2 == 0 => 0,
        _ => 1,
    };
    let new_node = vec![ST {
        v: Some((max_depth, polarity)),
        c: None,
    }];
    st.add_child(new_node);
    println!(": {:?}", max_depth);
}
