// Permutation of a mutable slice.
pub struct Permutations<'a, T: Ord> {
    elements: &'a mut [T],
}

impl<'a, T: Ord> Permutations<'a, T> {
    pub fn new(elements: &'a mut [T]) -> Self {
        Self { elements }
    }

    pub fn get(&self) -> &[T] {
        self.elements
    }

    pub fn get_mut(&mut self) -> &mut [T] {
        self.elements
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> bool {
        if self.elements.len() < 2 {
            return false;
        }

        let mut i = self.elements.len() - 1;
        loop {
            i -= 1;
            if self.elements[i] < self.elements[i + 1] {
                let mut k = self.elements.len() - 1;
                while self.elements[i] >= self.elements[k] {
                    k -= 1;
                }
                self.elements.swap(i, k);
                self.elements[i + 1..].reverse();
                return true;
            }
            if i == 0 {
                self.elements.reverse();
                return false;
            }
        }
    }

    pub fn previous(&mut self) -> bool {
        if self.elements.len() < 2 {
            return false;
        }

        let mut i = self.elements.len() - 1;
        loop {
            i -= 1;
            if self.elements[i] > self.elements[i + 1] {
                let mut k = self.elements.len() - 1;
                while self.elements[i] <= self.elements[k] {
                    k -= 1;
                }
                self.elements.swap(i, k);
                self.elements[i + 1..].reverse();
                return true;
            }
            if i == 0 {
                self.elements.reverse();
                return false;
            }
        }
    }
}

// Partition
pub fn partition<T>(slice: &mut [T], pivot: T) -> (usize, usize)
where
    T: std::cmp::PartialOrd,
{
    unsafe {
        let begin = slice.as_mut_ptr();
        let end = begin.offset(slice.len() as isize);

        let mut ptr = begin;
        let mut eq_end = ptr;
        let mut lt_end = ptr;

        while ptr < end {
            if *ptr < pivot {
                std::ptr::swap(ptr, lt_end);
                lt_end = lt_end.offset(1);
            } else if *ptr == pivot {
                std::ptr::swap(ptr, eq_end);
                if lt_end > eq_end {
                    std::ptr::swap(ptr, lt_end);
                }
                eq_end = eq_end.offset(1);
                lt_end = lt_end.offset(1);
            }
            ptr = ptr.offset(1);
        }

        let start_eq_range = (lt_end as usize - eq_end as usize) / std::mem::size_of::<T>();
        let start_gt_range = (lt_end as usize - begin as usize) / std::mem::size_of::<T>();

        while eq_end > begin {
            eq_end = eq_end.offset(-1);
            lt_end = lt_end.offset(-1);
            std::ptr::swap(eq_end, lt_end);
        }

        (start_eq_range, start_gt_range)
    }
}

pub fn split_at_partitions<T>(slice: &mut [T], pivot: T) -> (&mut [T], &mut [T], &mut [T])
where
    T: std::cmp::PartialOrd,
{
    let (start_eq, start_gt) = partition(slice, pivot);
    let (less_than, rest) = slice.split_at_mut(start_eq);
    let (equal, greater_than) = rest.split_at_mut(start_gt - start_eq);
    (less_than, equal, greater_than)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_permutation() {
        let mut nums = vec![1, 2, 3, 3];
        let mut p = Permutations::new(&mut nums);
        assert!(p.next());
        assert_eq!(p.get(), [1, 3, 2, 3]);
        assert!(p.next());
        assert_eq!(p.get(), [1, 3, 3, 2]);
        assert!(p.next());
        assert_eq!(p.get(), [2, 1, 3, 3]);
        assert!(p.next());
        assert_eq!(p.get(), [2, 3, 1, 3]);
        assert!(p.next());
        assert_eq!(p.get(), [2, 3, 3, 1]);
        assert!(p.next());
        assert_eq!(p.get(), [3, 1, 2, 3]);
        assert!(p.next());
        assert_eq!(p.get(), [3, 1, 3, 2]);
        assert!(p.next());
        assert_eq!(p.get(), [3, 2, 1, 3]);
        assert!(p.next());
        assert_eq!(p.get(), [3, 2, 3, 1]);
        assert!(p.next());
        assert_eq!(p.get(), [3, 3, 1, 2]);
        assert!(p.next());
        assert_eq!(p.get(), [3, 3, 2, 1]);
        assert!(!p.next());
        assert_eq!(p.get(), [1, 2, 3, 3]);
    }

    #[test]
    fn test_previous_permutation() {
        let mut nums = vec![3, 3, 2, 1];
        let mut p = Permutations::new(&mut nums);
        assert!(p.previous());
        assert_eq!(p.get(), [3, 3, 1, 2]);
        assert!(p.previous());
        assert_eq!(p.get(), [3, 2, 3, 1]);
        assert!(p.previous());
        assert_eq!(p.get(), [3, 2, 1, 3]);
        assert!(p.previous());
        assert_eq!(p.get(), [3, 1, 3, 2]);
        assert!(p.previous());
        assert_eq!(p.get(), [3, 1, 2, 3]);
        assert!(p.previous());
        assert_eq!(p.get(), [2, 3, 3, 1]);
        assert!(p.next());
        assert_eq!(p.get(), [3, 1, 2, 3]);
        assert!(p.previous());
        assert_eq!(p.get(), [2, 3, 3, 1]);
        assert!(p.previous());
        assert_eq!(p.get(), [2, 3, 1, 3]);
        assert!(p.previous());
        assert_eq!(p.get(), [2, 1, 3, 3]);
        assert!(p.previous());
        assert_eq!(p.get(), [1, 3, 3, 2]);
        assert!(p.previous());
        assert_eq!(p.get(), [1, 3, 2, 3]);
        assert!(p.previous());
        assert_eq!(p.get(), [1, 2, 3, 3]);
        assert!(!p.previous());
        assert_eq!(p.get(), [3, 3, 2, 1]);
    }
}
