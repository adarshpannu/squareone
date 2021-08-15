// heap (priority queue)

use std::cmp::Ordering;

#[derive(Debug)]
struct Heap<T, CMPFN>
where
    CMPFN: Fn(&T, &T) -> bool, {
    arr: Vec<Option<T>>,
    cmpfn: CMPFN, // Return true if arg1 has higher priority than arg2
}

impl<T, CMPFN> Heap<T, CMPFN>
where
    CMPFN: Fn(&T, &T) -> bool,
{
    fn new(cmpfn: CMPFN) -> Self {
        let mut arr = Vec::new();
        arr.push(None);
        Heap { arr, cmpfn }
    }

    fn push(&mut self, elem: T) {
        self.arr.push(Some(elem));
        self.heapify_up();
    }

    fn pop(&mut self) -> Option<T> {
        let lastidx = self.arr.len() - 1;
        if lastidx >= 1 {
            self.arr.swap(1, lastidx);
            let elem = self.arr.pop().unwrap();
            self.heapify_down();
            elem
        } else {
            None
        }
    }

    fn get(&self, idx: usize) -> &T {
        self.arr[idx].as_ref().unwrap()
    }

    fn heapify_up(&mut self) {
        let mut idx = self.arr.len() - 1;
        while idx > 1 {
            let parent = self.get(idx / 2);
            let child = self.get(idx);
            if (self.cmpfn)(child, parent) {
                self.arr.swap(idx, idx / 2);
            }
            idx = idx / 2;
        }
    }

    fn get_min_child(&self, idx: usize) -> usize {
        let mut minidx = idx;
        for childidx in vec![2 * idx, 2 * idx + 1] {
            if (childidx < self.arr.len())
                && (self.cmpfn)(self.get(childidx), self.get(minidx))
            {
                minidx = childidx;
            }
        }
        minidx
    }

    fn heapify_down(&mut self) {
        let mut idx = 1;
        while idx < self.arr.len() - 1 {
            let minidx = self.get_min_child(idx);
            if idx == minidx {
                break;
            }
            self.arr.swap(idx, minidx);
            idx = minidx;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::heap::*;

    fn run_heap_test(input: Vec<String>) {
        let mut input_sorted = input.clone();
        input_sorted.sort();

        let mut results = vec![];

        let mut heap: Heap<String, _> =
            Heap::new(|e1: &String, e2: &String| e1.cmp(e2) == Ordering::Less);
        for elem in input {
            heap.push(elem);
        }

        while let Some(elem) = heap.pop() {
            results.push(elem);
        }
        //dbg!(&results);
        //dbg!(&input_sorted);
        assert_eq!(&results, &input_sorted);
    }

    #[test]
    fn test1() {
        let input: Vec<_> = vec![
            "hello", "world", "here", "comes", "rust", "to", "rule", "you",
            "all",
        ]
        .iter()
        .map(|e| e.to_string())
        .collect();
        run_heap_test(input);
    }

    #[test]
    fn test2() {
        let input: Vec<_> =
            vec!["hello"].iter().map(|e| e.to_string()).collect();
        run_heap_test(input);
    }

    #[test]
    fn test3() {
        let input: Vec<String> =
            vec![].iter().map(|e: &String| e.to_string()).collect();
        run_heap_test(input);
    }
}
