use std::cmp::{min, max};

#[inline]
pub fn get_guaranteed_extra_blocks() -> i32 {
    1
}

#[inline]
pub fn filter_func(completed_blocks : i32, patt_blocks : i32, blind_blocks : i32) -> i32{
    if blind_blocks == 0 {
        completed_blocks + 1
    } else {
        completed_blocks
    }
}

#[inline]
pub fn get_block_lengths(patt_len : i32, err_rate : f32, thresh : i32) -> Vec<i32>{
    let mut ps : Vec<i32> = Vec::new();
    assert!(thresh <= patt_len);
    for l in thresh..patt_len+1{
        let one_p = (
            (l as f32)
                /
                ((err_rate * (l as f32)).ceil() + 1.0)
        ).ceil() as i32;
        ps.push(one_p);
    }
    let p = *ps.iter().min().unwrap();
    let mut remain = patt_len;
    let mut block_lengths : Vec<i32> = Vec::new();
    while remain > 0{
        let next = min(remain, p);
        block_lengths.push(next);
        remain -= next;
    }
    block_lengths
}

#[inline]
pub fn candidate_condition(
    generous_match_len : i32,
    completed_blocks : i32,
    thresh : i32,
    errors : i32
) -> bool{
    let c1 = generous_match_len >= thresh;
    let c2 = completed_blocks > 0;
    c1 && c2
}