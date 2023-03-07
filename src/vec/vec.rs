// =================================================================================================
// Copyright (c) 2023 Viet-Hoa Do <doviethoa@doviethoa.com>
//
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// =================================================================================================

use core::{mem, ptr, slice};

// =================================================================================================
// Common vector
// =================================================================================================

pub trait CommonVec<T> {
    /// Returns the number of elements that the vector can hold with the current storage space.
    fn capacity(&self) -> usize;

    /// Reserves a capacity of at least `self.len() + additional` elements.
    ///
    /// The collection might be more aggressive in term of over-allocating
    /// compared to [`reserve_exact`] to avoid frequent reallocation.
    fn reserve(&mut self, additional: usize) {
        self.try_reserve(additional).unwrap();
    }

    /// Reserves a capacity of at least `self.len() + additional` elements.
    ///
    /// Unlike [`reserve`], the collection will not deliberately over-allocate
    /// to avoid frequent reallocation.
    fn reserve_exact(&mut self, additional: usize) {
        self.try_reserve_exact(additional).unwrap();
    }

    /// Tries to reserves a capacity of at least `self.len() + additional` elements.
    ///
    /// The collection might be more aggressive in term of over-allocating
    /// compared to [`reserve_exact`] to avoid frequent reallocation.
    fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError>;

    /// Tries to reserves a capacity of at least `self.len() + additional` elements.
    ///
    /// Unlike [`reserve`], the collection will not deliberately over-allocate
    /// to avoid frequent reallocation.
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        return self.try_reserve(additional);
    }

    /// Shrinks the capacity of the vector as much as possible.
    fn shrink_to_fit(&mut self) {}

    /// Shrinks the capacity of the vector as close to `min_capacity` as possible.
    fn shrink_to(&mut self, _min_capacity: usize) {}

    /// Shortens the vector to the first `len` elements and drops the rest.
    ///
    /// If the current number of elements is less than `len`, does nothing.
    fn truncate(&mut self, len: usize) {
        let cur_len = self.len();

        if cur_len > len {
            let drop_ptr = unsafe { self.as_mut_ptr().add(len) };
            let num_drop = cur_len - len;

            let drop_slice = ptr::slice_from_raw_parts_mut(drop_ptr, num_drop);

            unsafe {
                ptr::drop_in_place(drop_slice);
                self.set_len(len);
            }
        }
    }

    /// Returns a slice that contains the entire vector.
    fn as_slice(&self) -> &[T] {
        return unsafe { slice::from_raw_parts(self.as_ptr(), self.len()) };
    }

    /// Returns a mutable slice that contains the entire vector.
    fn as_mut_slice(&mut self) -> &mut [T] {
        return unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.len()) };
    }

    /// Returns a raw pointer to the vector's buffer.
    ///
    /// If the buffer hasn't been allocated, returns a dangling raw pointer.
    fn as_ptr(&self) -> *const T;

    /// Returns a mutable raw pointer to the vector's buffer.
    ///
    /// If the buffer hasn't been allocated, returns a dangling raw pointer.
    fn as_mut_ptr(&mut self) -> *mut T;

    /// Sets the length of the vector to `new_len`.
    unsafe fn set_len(&mut self, new_len: usize);

    /// Removes the element at position `index` and returns it.
    ///
    /// Moves the last element in the vector to position `index`
    /// so that the array becomes contiguous again.
    ///
    /// This method obviously doesn't preserve order, but it's O(1) (i.e. fast).
    /// If preservation of order is needed, use [`remove`] instead.
    fn swap_remove(&mut self, index: usize) -> T {
        let len = self.len();

        if index >= len {
            panic!("Index is out-of-range.");
        }

        let buf_ptr = self.as_mut_ptr();

        unsafe {
            let removed_item = ptr::read(buf_ptr.add(index));
            ptr::copy(buf_ptr.add(len - 1), buf_ptr.add(index), 1);
            self.set_len(len - 1);

            return removed_item;
        }
    }

    /// Inserts the element at position `index`.
    ///
    /// All the elements at and after position `index` will be shifted one position to the right.
    fn insert(&mut self, index: usize, element: T) {
        let len = self.len();
        let capacity = self.capacity();

        assert!(len <= capacity);

        if len == capacity {
            self.reserve(1);
        }

        if index > len {
            panic!("Index is out-of-range.");
        }

        let buf_ptr = self.as_mut_ptr();

        unsafe {
            let index_ptr = buf_ptr.add(index);

            if index < len {
                ptr::copy(index_ptr, index_ptr.add(1), len - index);
            }

            ptr::write(index_ptr, element);

            self.set_len(len + 1);
        }
    }

    /// Removes the element at position `index` and returns it.
    ///
    /// All the elements after position `index` will be shifted one position to the left.
    ///
    /// If preservation of order is not needed, use [`swap_remove`] instead as it is faster.
    fn remove(&mut self, index: usize) -> T {
        let len = self.len();

        if index >= len {
            panic!("Index is out-of-range.");
        }

        let buf_ptr = self.as_mut_ptr();

        unsafe {
            let index_ptr = buf_ptr.add(index);

            let removed_data = ptr::read(index_ptr);
            ptr::copy(index_ptr.add(1), index_ptr, len - index - 1);

            self.set_len(len - 1);

            return removed_data;
        }
    }

    /// Returns only elements `e` for which `f(&e)` returns `true`.
    fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.retain_mut(|e| f(e));
    }

    /// Returns only elements `e` for which `f(&mut e)` returns `true`.
    fn retain_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        let len = self.len();
        let buf_ptr = self.as_mut_ptr();

        let mut i = 0usize;

        while i < len {
            unsafe {
                let curr_ptr = buf_ptr.add(i);
                let is_retained = f(&mut *curr_ptr);

                if !is_retained {
                    ptr::drop_in_place(curr_ptr);
                    break;
                }
            }

            i += 1;
        }

        if i < len {
            let mut new_len = i;

            for i in i + 1..len {
                unsafe {
                    let curr_ptr = buf_ptr.add(i);
                    let is_retained = f(&mut *curr_ptr);

                    if is_retained {
                        // REVISIT: If we can copy more than one elements at a time, it would be faster.
                        let new_last_ptr = buf_ptr.add(new_len);
                        ptr::copy_nonoverlapping(curr_ptr, new_last_ptr, 1);
                        new_len += 1;
                    } else {
                        ptr::drop_in_place(curr_ptr);
                    }
                }
            }

            unsafe {
                self.set_len(new_len);
            }
        }
    }

    /// Removes all elements `e` in the vector that has the same `key(e)` value
    /// with the previous element.
    fn dedup_by_key<F, K>(&mut self, mut key: F)
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq<K>,
    {
        // REVISIT: An explicit implementation might be faster due to less calls to `key(e)`.
        self.dedup_by(|a, b| key(a) == key(b));
    }

    /// Removes all elements in the vector that is considered the same as the previous element.
    ///
    /// Two consecutive elements `a` and `b` are considered the same if `same_bucket(b, a)` is true.
    fn dedup_by<F>(&mut self, mut same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {
        let len = self.len();
        let buf_ptr = self.as_mut_ptr();
        let mut prev_ptr = buf_ptr;

        let mut i = 1usize;

        while i < len {
            unsafe {
                let curr_ptr = buf_ptr.add(i);
                let is_dup = same_bucket(&mut *curr_ptr, &mut *prev_ptr);

                if is_dup {
                    ptr::drop_in_place(curr_ptr);
                    break;
                }

                prev_ptr = curr_ptr;
            }

            i += 1;
        }

        if i < len {
            let mut new_len = i;

            for i in i + 1..len {
                unsafe {
                    let curr_ptr = buf_ptr.add(i);
                    let is_dup = same_bucket(&mut *curr_ptr, &mut *prev_ptr);

                    if is_dup {
                        ptr::drop_in_place(curr_ptr);
                    } else {
                        // REVISIT: If we can copy more than one elements at a time, it would be faster.
                        let new_last_ptr = buf_ptr.add(new_len);
                        ptr::copy_nonoverlapping(curr_ptr, new_last_ptr, 1);
                        new_len += 1;
                    }
                }
            }

            unsafe {
                self.set_len(new_len);
            }
        }
    }

    /// Pushes a new element to the end of the vector.
    fn push(&mut self, value: T) {
        let len = self.len();
        let capacity = self.capacity();
        assert!(len <= capacity);

        if len == capacity {
            self.reserve(1);
        }

        let buf_ptr = self.as_mut_ptr();

        unsafe {
            ptr::write(buf_ptr.add(len), value);
            self.set_len(len + 1);
        }
    }

    /// Removes and returns the last element from the vector.
    ///
    /// If the vector is empty, return [`None`].
    fn pop(&mut self) -> Option<T> {
        let len = self.len();

        if len > 0 {
            let buf_ptr = self.as_ptr();

            unsafe {
                let new_len = len - 1;
                let value = ptr::read(buf_ptr.add(new_len));
                self.set_len(new_len);

                return Some(value);
            }
        } else {
            return None;
        }
    }

    /// Moves all the elements of `other` into `self`.
    ///
    /// `other` will become empty after this.
    fn append<V>(&mut self, other: &mut V)
    where
        V: CommonVec<T>,
    {
        let len = self.len();
        let capacity = self.capacity();

        let other_len = other.len();
        let total_len = len + other_len;

        if total_len > capacity {
            self.reserve(total_len - capacity);
        }

        let buf_ptr = self.as_mut_ptr();
        let other_buf_ptr = other.as_ptr();

        unsafe {
            ptr::copy(other_buf_ptr, buf_ptr.add(len), other_len);

            self.set_len(total_len);
            other.set_len(0);
        }
    }

    // Not implemented: drain

    /// Moves all elements in the vector.
    fn clear(&mut self) {
        let len = self.len();
        let buf_ptr = self.as_mut_ptr();

        unsafe {
            ptr::drop_in_place(ptr::slice_from_raw_parts_mut(buf_ptr, len));
            self.set_len(0);
        }
    }

    /// Returns the number of elements in the vector.
    fn len(&self) -> usize;

    /// Returns whether the vector contains no elements.
    fn is_empty(&self) -> bool {
        let len = self.len();
        return len == 0;
    }

    /// Resizes the vector to the `new_len`.
    ///
    /// If the vector is expanding, each new element will be created by calling `f`.
    fn resize_with<F>(&mut self, new_len: usize, mut f: F)
    where
        F: FnMut() -> T,
    {
        let len = self.len();

        self.truncate(new_len);

        if len > new_len {
            let buf_ptr = self.as_mut_ptr();

            for i in len..new_len {
                unsafe {
                    ptr::write(buf_ptr.add(i), f());
                }
            }

            unsafe {
                self.set_len(new_len);
            }
        }
    }

    // Not implemented: leak

    /// Returns the unused space of the buffer.
    fn spare_capacity_mut(&mut self) -> &mut [mem::MaybeUninit<T>] {
        let len = self.len();
        let capacity = self.capacity();
        let buf_ptr = self.as_mut_ptr();

        return unsafe {
            slice::from_raw_parts_mut(buf_ptr.add(len) as *mut mem::MaybeUninit<T>, capacity - len)
        };
    }
}

// TryReserveError ---------------------------------------------------------------------------------

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TryReserveError;

// =================================================================================================
// Statically allocated vector
// =================================================================================================

/// A contiguous array of type `T` statically allocated with the capacity of `C` items.
pub struct StaticVec<T, const C: usize> {
    len: usize,
    buffer: mem::MaybeUninit<[T; C]>,
}

// Constructors and destructor ---------------------------------------------------------------------

impl<T, const C: usize> StaticVec<T, C> {
    /// Constructs a new, empty `StaticVec<T, C>`.
    pub const fn new() -> Self {
        return Self { len: 0, buffer: mem::MaybeUninit::uninit() };
    }

    /// Constructs a new, empty `StaticVec<T, C>`.
    ///
    /// The capacity of the vector is determined by the generic constant `C`, which is known
    /// at compile-time, rather than the argument of this function.
    /// This function is implemented in [`StaticVec`] so that it can be used
    /// as a drop-in replacement for other dynamically allocated vector types.
    pub fn with_capacity(_capacity: usize) -> Self {
        return Self::new();
    }
}

// Common vector methods ---------------------------------------------------------------------------

impl<T, const C: usize> CommonVec<T> for StaticVec<T, C> {
    fn capacity(&self) -> usize {
        return C;
    }

    fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        if self.len + additional <= C {
            return Ok(());
        } else {
            return Err(TryReserveError);
        }
    }

    fn as_ptr(&self) -> *const T {
        return self.buffer.as_ptr() as *const T;
    }

    fn as_mut_ptr(&mut self) -> *mut T {
        return self.buffer.as_mut_ptr() as *mut T;
    }

    unsafe fn set_len(&mut self, new_len: usize) {
        debug_assert!(new_len <= C);
        self.len = new_len;
    }

    fn len(&self) -> usize {
        return self.len;
    }
}
