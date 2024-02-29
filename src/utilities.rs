use std::ops::{Bound, Index, RangeBounds};

pub struct DynamicRange {
    pub start : usize,
    pub end : usize
}
impl RangeBounds<usize> for DynamicRange {
    fn start_bound(&self) -> Bound<&usize> {
        Bound::Included(&self.start)
    }

    fn end_bound(&self) -> Bound<&usize> {
        if self.end.checked_sub(self.start).is_some_and(|result| result > 0) {
            Bound::Excluded(&self.end)
        } else {
            Bound::Included(&self.end)
        }
    }
}
impl Index<DynamicRange> for String {
    type Output = str;
    fn index(&self, index: DynamicRange) -> &Self::Output {
        match index.end_bound() {
            Bound::Excluded(_) => &self[index.start..index.end],
            Bound::Included(_) => &self[index.start..=index.end],
            _ => panic!()
        }
    }
} 

/// this will construct an inclusive range if the start and end are the same or exclusive if they are not.
pub fn dyn_range(start : usize, end : usize) -> DynamicRange {
    DynamicRange { start: start, end: end }
}