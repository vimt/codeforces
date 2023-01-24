use std::collections::BinaryHeap;

pub fn partition<T: Ord + Copy>(elements: &mut Vec<T>, start: usize, end: usize) -> usize {
    let mid = elements[end];
    let mut left = start;
    for i in start..end {
        if elements[i] < mid {
            elements.swap(left, i);
            left += 1;
        }
    }
    elements.swap(left, end);
    return left;
}

pub fn three_way_partition<T: Ord>(elements: &mut Vec<T>, target: T) {
    let mut next_big_pos = elements.len() - 1;
    let mut next_small_pos = 0;
    let mut next_scan_pos = 0;

    while next_scan_pos <= next_big_pos {
        if elements[next_scan_pos] > target {
            elements.swap(next_scan_pos, next_big_pos);
            next_big_pos -= 1;
        } else if elements[next_scan_pos] < target {
            elements.swap(next_scan_pos, next_small_pos);
            next_small_pos += 1;
            next_scan_pos += 1;
        } else {
            next_scan_pos += 1;
        }
    }
}

pub fn partition_reverse<T: Ord + Copy>(elements: &mut Vec<T>, start: usize, end: usize) -> usize {
    let mid = elements[end];
    let mut left = start;
    for i in start..end {
        if elements[i] > mid {
            elements.swap(i, left);
            left += 1;
        }
    }
    elements.swap(left, end);
    return left;
}


pub fn binary_search<T: Ord + Copy>(elements: &mut Vec<T>, start: usize, end: usize, key: T) -> Option<usize> {
    if start > end { return None; }
    let mid = start + (end - start) / 2;
    if elements[mid] > key {
        binary_search(elements, start, mid - 1, key)
    } else if elements[mid] < key {
        binary_search(elements, mid + 1, end, key)
    } else {
        Some(mid)
    }
}


pub fn quick_sort<T: Ord + Copy>(elements: &mut Vec<T>, start: usize, end: usize) {
    if start < end {
        let i = partition(elements, start, end);
        //这蛋疼的usize
        if i != 0 {
            quick_sort(elements, start, i - 1);
        }
        quick_sort(elements, i + 1, end);
    }
}


pub fn min_heapify<T: Ord>(nums: &mut Vec<T>, mut start: usize, end: usize) {
    if nums.len() < 1 || start > end { return; }
    let mut son = start * 2 + 1;
    while son <= end {
        if son + 1 <= end && nums[son + 1] < nums[son] { son += 1; }
        if nums[start] >= nums[son] {
            nums.swap(start, son);
            start = son;
            son = 2 * start + 1;
        } else {
            return;
        }
    }
}

pub fn max_heapify<T: Ord>(nums: &mut Vec<T>, mut start: usize, end: usize) {
    if nums.len() < 1 || start > end { return; }
    let mut son = start * 2 + 1;
    while son <= end {
        if son + 1 <= end && nums[son + 1] > nums[son] { son += 1; }
        if nums[start] <= nums[son] {
            nums.swap(start, son);
            start = son;
            son = 2 * start + 1;
        } else {
            return;
        }
    }
}

pub fn heap_sort<T: Ord>(nums: &mut Vec<T>) {
    // build
    let len = nums.len();
    for i in (0..len / 2).rev() {
        max_heapify(nums, i, len - 1);
    }
    // sort
    for i in (1..len).rev() {
        nums.swap(0, i);
        max_heapify(nums, 0, i - 1);
    }
}

pub fn nth_element<T: Ord + Copy>(nums: &mut Vec<T>, k: usize) {
    let mut start = 0;
    let mut end = nums.len() - 1;
    while start < end {
        let m = partition(nums, start, end);
        if m == k {
            return;
        } else if m > k - 1 {
            end = m - 1;
        } else {
            start = m + 1;
        };
    }
}

pub fn partial_sort<T: Ord + Copy>(nums: &mut Vec<T>, k: usize) -> Vec<T> {
    if k <= 0 {
        return nums.clone();
    }
    let mut heap = BinaryHeap::with_capacity(k + 1);
    let mut ans = vec![nums[0]; nums.len()];

    for i in 0..k {
        heap.push(nums[i])
    }
    for i in k..nums.len() {
        if nums[i] < *heap.peek().unwrap() {
            heap.push(nums[i]);
            ans[i] = heap.pop().unwrap();
        }
    }
    for i in (0..k).rev() {
        ans[i] = heap.pop().unwrap();
    }
    ans
}


/// 树状数组
#[derive(Debug)]
pub struct BinIndexedTree {
    vec: Vec<i32>,
}

impl BinIndexedTree {
    pub fn with_capacity(capacity: usize) -> BinIndexedTree {
        BinIndexedTree { vec: vec![0; capacity] }
    }

    /// lowbit 返回1[末尾0的个数]
    #[inline]
    fn lowbit(v: usize) -> usize {
        // (-(v as i32) & v as i32) as usize
        (v ^ (v - 1)) & v
    }

    /// 将a[i]的值加上 delta，要将包含a[i]的值都加上delta
    pub fn add(&mut self, mut i: usize, delta: i32) {
        while i < self.vec.len() {
            self.vec[i] += delta;
            i += Self::lowbit(i);
        }
    }

    /// 求前缀和
    pub fn sum(&self, mut k: usize) -> i32 {
        let mut ans = 0;
        while k > 0 {
            ans += self.vec[k];
            k -= Self::lowbit(k);
        }
        ans
    }
}

impl From<Vec<i32>> for BinIndexedTree {
    fn from(v: Vec<i32>) -> Self {
        let mut ret = BinIndexedTree::with_capacity(v.len() + 1);
        for i in 1..=v.len() {
            ret.vec[i] = v[i - 1];
            for j in (i - Self::lowbit(i)..i - 1).rev() {
                ret.vec[i] += v[j];
            }
        }
        ret
    }
}

pub fn kmp(s: String, pattern: String) -> i32 {
    let query = s.as_bytes();
    let pattern = pattern.as_bytes();
    let n = query.len();
    let m = pattern.len();
    if m == 0 { return 0; }
    let mut next = vec![0; m];
    let mut j = 0;
    for i in 1..m {
        // why while: aabaaa, last a need while
        while j > 0 && pattern[i] != pattern[j] {
            j = next[j - 1];
        }
        if pattern[i] == pattern[j] { j += 1; }
        next[i] = j;
    }
    j = 0;
    for i in 0..n {
        while j > 0 && query[i] != pattern[j] {
            j = next[j - 1];
        }
        if query[i] == pattern[j] { j += 1; }
        if j == m {
            return (i + 1 - m) as i32;
        }
    }
    -1
}

pub fn asserting_cmp<T: PartialOrd>(a: &T, b: &T) -> std::cmp::Ordering {
    a.partial_cmp(b).expect("Comparing incomparable elements")
}

pub fn binary_search_lower<T: PartialOrd>(slice: &[T], key: &T) -> usize {
    slice.binary_search_by(|x| asserting_cmp(x, key).then(std::cmp::Ordering::Greater)).unwrap_err()
}

pub fn binary_search_upper<T: PartialOrd>(slice: &[T], key: &T) -> usize {
    slice.binary_search_by(|x| asserting_cmp(x, key).then(std::cmp::Ordering::Less)).unwrap_err()
}


pub fn quick_pow(mut base: i64, mut pow: i64, mod0: i64) -> i64 {
    base = base % mod0;
    let mut ans = 1;
    while pow != 0 {
        if pow & 1 == 1 {
            ans = ans * base % mod0;
        }
        base = base * base % mod0;
        pow >>= 1;
    }
    ans
}


#[test]
fn test_quick_sort() {
    let mut elements = vec![2, 1, 5, 4, 7, 9, 5, 2, 1, 0];
    let ele = &mut elements;
    quick_sort(ele, 0, ele.len() - 1);
    println!("{:?}", ele);

    let mut elements = vec![1, 1, 1, 1, 1, 1];
    quick_sort(&mut elements, 0, 5);
    println!("{:?}", elements);
}


#[test]
fn test_heap_sort() {
    let mut elements = vec![2, 1, 5, 4, 7, 9, 5, 2, 1, 0];
    let ele = &mut elements;
    heap_sort(ele);
    println!("{:?}", ele);
}

#[test]
fn test_nth_element() {
    let mut elements = vec![2, 1, 5, 4, 7, 9, 5, 2, 1, 0];
    nth_element(&mut elements, 5);
    println!("{:?}", elements)
}

#[test]
fn test_partial_sort() {
    let mut elements = vec![2, 1, 5, 4, 7, 9, 5, 3, 1, 0];
    println!("{:?}", partial_sort(&mut elements, 5))
}

#[test]
fn test_bin_indexed_tree() {
    let elements = vec![4, 5, 6, 7];
    let tree = BinIndexedTree::from(elements);
    println!("{}", tree.sum(3));
}