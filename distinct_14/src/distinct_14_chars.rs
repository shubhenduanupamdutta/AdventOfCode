//! Problem Statement
//! Your device's communication system is correctly detecting packets, but still isn't working. It
//! looks like it also needs to look for *messages*.
//!
//! A start of message marker is just like a start of packet marker, but it is a sequence of 14 distinct
//! characters instead of 4.
//!
//! Here are first position of start of message markers for all the above examples.
//!
//! - `mjqjpqmgbljsphdztnvjfqwrcgsmlb`: first marker after character `19`
//! - `bvwbjplbgvbhsrlpgdmjqwftvncz`: first marker after character `23`
//! - `nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg`: first marker after character `29`
//!
use std::collections::HashSet;

pub fn simple_solution(i: &[u8]) -> usize {
    return i
        .windows(14)
        .position(|w| w.iter().collect::<HashSet<_>>().len() == 14)
        .map(|i| i + 14)
        .unwrap();
}

/// This is a faster solution that goes to next window as soon as it finds a duplicate character.
pub fn faster_solution(i: &[u8]) -> usize {
    return i
        .windows(14)
        .position(|w| {
            let mut hash_set = HashSet::new();
            for x in w {
                if !hash_set.insert(x) {
                    return false;
                }
            }
            return true;
        })
        .map(|i| i + 14)
        .unwrap();
}

/// This is a faster solution that goes to next window as soon as it finds a duplicate character, but
/// uses a vector instead of a hash set.
pub fn faster_with_vec_solution(i: &[u8]) -> usize {
    return i
        .windows(14)
        .position(|w| {
            let mut vec = Vec::with_capacity(14);
            for x in w {
                if vec.contains(x) {
                    return false;
                }
                vec.push(*x);
            }
            return true;
        })
        .map(|i| i + 14)
        .unwrap();
}

/// This is a faster solution that goes to next window as soon as it finds a duplicate character, but
/// uses an array instead of a hash set and vector.
pub fn faster_with_array_solution(i: &[u8]) -> usize {
    return i
        .windows(14)
        .position(|w| {
            let mut arr = [0_u8; 14];
            let mut index = 0;
            for x in w {
                for i in 0..index {
                    if arr[i] == *x {
                        return false;
                    }
                }
                arr[index] = *x;
                index += 1;
            }
            return true;
        })
        .map(|i| i + 14)
        .unwrap();
}

/// This is a faster solution that uses a u32 number and bit manipulation to check for duplicates.
/// This is first time uploaded by Benny
pub fn benny_solution(input: &[u8]) -> usize {
    let mut filter = 0_u32;
    input
        .iter() // Iterate over the input
        .take(14 - 1) // Take the first 13 characters
        .for_each(|c| filter ^= 1 << (c % 32)); // Set the bit corresponding to the character
                                                // Basically we are setting the bit corresponding to a character (of first 13) to 1, if there is only one or odd number of that character, and 0 if there is even number of that character

    input
        .windows(14)
        .position(|w| {
            let first = w[0]; // First character of the window
            let last = w[w.len() - 1]; // Last character of the window
            filter ^= 1 << (last % 32); // Set the bit corresponding to the last character to 1 if it is not already in the window or 0 if it is already in the window
            let res = filter.count_ones() == 14; // if there are 14 bits set to 1, then there are 14 distinct characters
            filter ^= 1 << (first % 32); // Set the bit corresponding to the first character to opposite of what it was before, for next window (because next window shouldn't contain the first character)
            res
        })
        .map(|i| i + 14)
        .unwrap()
}

pub fn david_a_perez_solution(input: &[u8]) -> usize {
    let mut idx = 0;
    while let Some(slice) = input.get(idx..idx + 14) {
        let mut state = 0_u32;
        if let Some(pos) = slice.iter().rposition(|byte| {
            let bit_idx = byte % 32;
            let res = state & (1 << bit_idx) != 0;
            state |= 1 << bit_idx;
            res
        }) {
            idx += pos + 1;
        } else {
            return idx + 14;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_solution() {
        assert_eq!(simple_solution(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(simple_solution(b"bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(simple_solution(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    }

    #[test]
    fn test_faster_solution() {
        assert_eq!(faster_solution(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(faster_solution(b"bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(faster_solution(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    }

    #[test]
    fn test_faster_with_vec_solution() {
        assert_eq!(
            faster_with_vec_solution(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            19
        );
        assert_eq!(
            faster_with_vec_solution(b"bvwbjplbgvbhsrlpgdmjqwftvncz"),
            23
        );
        assert_eq!(
            faster_with_vec_solution(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            29
        );
    }

    #[test]
    fn test_faster_with_array_solution() {
        assert_eq!(
            faster_with_array_solution(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            19
        );
        assert_eq!(
            faster_with_array_solution(b"bvwbjplbgvbhsrlpgdmjqwftvncz"),
            23
        );
        assert_eq!(
            faster_with_array_solution(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            29
        );
    }

    #[test]
    fn test_benny_solution() {
        assert_eq!(benny_solution(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(benny_solution(b"bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(benny_solution(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    }

    #[test]
    fn test_david_a_perez_solution() {
        assert_eq!(david_a_perez_solution(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(david_a_perez_solution(b"bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(david_a_perez_solution(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    }
}
