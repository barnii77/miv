use std::ops::RangeBounds;

fn is_empty_range(range: &impl RangeBounds<usize>) -> bool {
    let start = match range.start_bound() {
        std::ops::Bound::Included(&start) => start,
        std::ops::Bound::Excluded(&start) => start + 1,
        std::ops::Bound::Unbounded => 0,
    };
    let end = match range.end_bound() {
        std::ops::Bound::Included(&end) => end + 1,
        std::ops::Bound::Excluded(&end) => end,
        std::ops::Bound::Unbounded => 0,
    };
    start == end
}

fn splice<T>(
    vec: &mut Vec<T>,
    range: impl RangeBounds<usize>,
    splice_vec: impl Iterator<Item = T>,
) {
    // empty range means just insert everything at the index that the range
    if range.end_bound() == std::ops::Bound::Unbounded || is_empty_range(&range) {
        vec.splice(range, splice_vec);
    } else {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&start) => start,
            std::ops::Bound::Excluded(&start) => start + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            std::ops::Bound::Included(&end) => end + 1,
            std::ops::Bound::Excluded(&end) => end,
            std::ops::Bound::Unbounded => vec.len(),
        };
        let size_shortened = end - start;
        let splice_shortened = splice_vec.take(size_shortened);
        vec.splice(range, splice_shortened);
    }
}

#[derive(Debug)]
pub struct GapWindow {
    index: usize,
    size: usize,
}

/// A gap buffer for small sized T types (preferably smaller than a pointer)
#[derive(Debug)]
pub struct GapBuffer<T> {
    buffer: Vec<T>,
    gap_window: GapWindow,
}

// TODO implement this more efficiently with less iter.cloned()
// maybe require T to implement Copy
impl<T: Clone + Default> GapBuffer<T> {
    pub fn new(cap: usize) -> Self {
        // pad the buffer (because the splice method requires it and is heavily used here)
        let buffer = std::iter::repeat(T::default())
            .take(cap)
            .collect::<Vec<T>>();
        let gap_window = GapWindow {
            index: 0,
            size: cap,
        };
        Self { buffer, gap_window }
    }
    pub fn new_empty() -> Self {
        Self::new(0)
    }
    pub fn insert(&mut self, mut chars: &[T]) {
        // NOTE: empty range means just insert everything at the index that the range
        // starts and ends at
        if self.gap_window.size < chars.len() {
            // realloc will happen and we cannot simply splice
            // this is because if we just spliced, the gap window
            // would be in the wrong place because the extra capacity
            // would be at the end of the vec
            // to avoid this we need to splice the data into the vec
            // then splice the data after the inserted data to the end
            // so we can create a new gap directly after the inserted data
            // this is important because the code is for a text editor
            // and moving the cursor after insert would be very annoying
            // for the user
            // empty range means insert every element in a sequence starting at index

            // first, we insert the data that still fits at the index
            if self.gap_window.size > 0 {
                let range = self.gap_window.index..self.gap_window.index + self.gap_window.size;
                splice(&mut self.buffer, range, chars.iter().cloned());
                chars = &chars[self.gap_window.size..];
                self.gap_window.index += self.gap_window.size;
                self.gap_window.size = 0;
            }

            // then we insert the data that didn't fit at the index using .splice
            let range = self.gap_window.index..self.gap_window.index;
            let old_buffer_capacity = self.buffer.capacity();
            // size will now change (realloc)
            splice(&mut self.buffer, range, chars.iter().cloned());
            let range = self.gap_window.index + chars.len()..;
            let padding_size = self.buffer.capacity() - old_buffer_capacity - chars.len();
            // materialize the padding because otherwise rust compiler go "F U"
            // reason: mutable and immutable references at the same time because iterator
            // contains immutable ref and splice takes &mut self
            let inserted = std::iter::repeat(T::default())
                .take(padding_size)
                .chain(
                    self.buffer
                        [self.gap_window.index + chars.len()..old_buffer_capacity + chars.len()]
                        .iter()
                        .cloned(),
                )
                .collect::<Vec<T>>();
            splice(&mut self.buffer, range, inserted.into_iter());
            self.gap_window.index += chars.len();
            self.gap_window.size = padding_size;
        } else {
            // No realloc required yet, so we can just splice the data in
            // and adjust the gap window
            let range = self.gap_window.index..self.gap_window.index + chars.len();
            splice(&mut self.buffer, range, chars.iter().cloned());
            self.gap_window.index += chars.len();
            self.gap_window.size -= chars.len();
        }
    }
    pub fn delete(&mut self, count: usize) {
        if self.gap_window.index > count {
            self.gap_window.size += count;
            self.gap_window.index -= count;
        } else {
            self.gap_window.size += self.gap_window.index;
            self.gap_window.index = 0;
        }
    }
    pub fn move_gap(&mut self, index: usize) {
        match index.cmp(&self.gap_window.index) {
            std::cmp::Ordering::Less => {
                // move gap to the left
                // remove the data from the gap
                // and add it to the right of the gap
                let removed = self.buffer[index..self.gap_window.index].to_vec();
                let range =
                    index + self.gap_window.size..self.gap_window.index + self.gap_window.size;
                splice(&mut self.buffer, range, removed.into_iter());
                self.gap_window.index = index;
            }
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => {
                // move gap to the right
                // remove the data from the gap
                // and add it to the left of the gap
                let removed = self.buffer
                    [self.gap_window.index + self.gap_window.size..index + self.gap_window.size]
                    .to_vec();
                let range = self.gap_window.index..index;
                splice(&mut self.buffer, range, removed.into_iter());
                self.gap_window.index = index;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod splice_tests {
        use super::*;

        #[test]
        fn test_splice_range_bigger_than_vec() {
            let mut vec = vec![1, 2, 3, 4, 5];
            let splice_vec = vec![6, 7, 8];
            splice(&mut vec, 1..4, splice_vec.into_iter());
            assert_eq!(vec, vec![1, 6, 7, 8, 5]);
        }
        #[test]
        fn test_splice_range_unbounded() {
            let mut vec = vec![1, 2, 3, 4, 5];
            let splice_vec = vec![6, 7, 8];
            splice(&mut vec, 1.., splice_vec.into_iter());
            assert_eq!(vec, vec![1, 6, 7, 8]);
        }
        #[test]
        fn test_splice_range_smaller_than_vec() {
            let mut vec = vec![1, 2, 3, 4, 5];
            let splice_vec = vec![6, 7, 8, 9, 10];
            splice(&mut vec, 1..=3, splice_vec.into_iter());
            assert_eq!(vec, vec![1, 6, 7, 8, 5]);
        }
        #[test]
        fn test_splice_range_empty() {
            let mut vec = vec![1, 2, 3, 4, 5];
            let splice_vec = vec![6, 7, 8, 9, 10];
            splice(&mut vec, 1..1, splice_vec.into_iter());
            assert_eq!(vec, vec![1, 6, 7, 8, 9, 10, 2, 3, 4, 5]);
        }
    }

    #[cfg(test)]
    mod gap_buffer_tests {
        use super::*;

        fn remove_padding<'a>(buffer: impl Iterator<Item = &'a char>) -> Vec<char> {
            buffer.filter(|c| **c != '\0').copied().collect()
        }

        #[test]
        fn test_gap_buffer() {
            let mut buffer = GapBuffer::new(10);
            let string_vec = "hello world".chars().collect::<Vec<char>>();
            buffer.insert(&string_vec);
            buffer.move_gap(2);
            buffer.delete(3);
            buffer.insert(&string_vec);
            assert_eq!(
                remove_padding(buffer.buffer.iter()),
                "hello worldllo world".chars().collect::<Vec<char>>()
            );
        }
        #[test]
        fn test_gap_buffer_delete_makes_difference() {
            // here, the delete actually makes a difference
            let mut buffer = GapBuffer::new(10);
            let string_vec = "hello world".chars().collect::<Vec<char>>();
            let string_vec2 = "hello".chars().collect::<Vec<char>>();
            buffer.insert(&string_vec);
            buffer.move_gap(8);
            buffer.delete(3);
            buffer.insert(&string_vec2);
            assert_eq!(
                remove_padding(buffer.buffer.iter()),
                "hellohellodrld".chars().collect::<Vec<char>>()
            );
        }
        #[test]
        fn test_gap_buffer_empty() {
            let mut buffer = GapBuffer::new_empty();
            let string_vec = "hello world".chars().collect::<Vec<char>>();
            buffer.insert(&string_vec);
            buffer.move_gap(2);
            buffer.delete(3);
            buffer.insert(&string_vec);
            assert_eq!(
                remove_padding(buffer.buffer.iter()),
                "hello worldllo world".chars().collect::<Vec<char>>()
            );
        }
        #[test]
        fn test_gap_buffer_insert() {
            let mut buffer = GapBuffer::new(10);
            let string_vec = "hello world".chars().collect::<Vec<char>>();
            buffer.insert(&string_vec);
            assert_eq!(
                remove_padding(buffer.buffer.iter()),
                "hello world".chars().collect::<Vec<char>>()
            );
        }
        #[test]
        fn test_gap_buffer_insert_realloc() {
            let mut buffer = GapBuffer::new(10);
            let string_vec = "hello world".chars().collect::<Vec<char>>();
            buffer.insert(&string_vec);
            buffer.insert(&string_vec);
            assert_eq!(
                remove_padding(buffer.buffer.iter()),
                "hello worldhello world".chars().collect::<Vec<char>>()
            );
        }
        #[test]
        fn test_gap_buffer_insert_realloc_2() {
            let mut buffer = GapBuffer::new(10);
            let string_vec = "hello world".chars().collect::<Vec<char>>();
            buffer.insert(&string_vec);
            buffer.insert(&string_vec);
            buffer.insert(&string_vec);
            assert_eq!(
                remove_padding(buffer.buffer.iter()),
                "hello worldhello worldhello world"
                    .chars()
                    .collect::<Vec<char>>()
            );
        }
    }
    #[test]
    fn test_gap_buffer_with_padding_checked() {
        // println!("{:?}", (1..3).collect::<Vec<usize>>());
        let mut buffer = GapBuffer::new_empty();
        let string_vec = "hello world".chars().collect::<Vec<char>>();
        buffer.insert(&string_vec);
        buffer.move_gap(2);
        buffer.delete(3);
        buffer.insert(&string_vec);
        assert_eq!(
            buffer.buffer.to_vec(),
            "hello world\0\0llo world".chars().collect::<Vec<char>>()
        );
    }
}
