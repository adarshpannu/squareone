use rand::Error;

#[derive(Debug)]
struct Heap<T: PartialOrd> {
    arr: Vec<Option<T>>,
}

impl<T: PartialOrd> Heap<T> {
    fn new(capacity: usize) -> Heap<T> {
        let mut arr = Vec::with_capacity(capacity + 1);
        arr.push(None);
        Heap { arr }
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

    fn is_left_child(idx: usize) -> bool {
        idx % 2 == 0
    }

    fn get(&self, idx: usize) -> &T {
        self.arr[idx].as_ref().unwrap()
    }

    fn heapify_up(&mut self) {
        let mut idx = self.arr.len() - 1;
        while idx > 1 {
            let parent = self.get(idx / 2);
            let child = self.get(idx);
            if child < parent {
                self.arr.swap(idx, idx / 2);
            }
            idx = idx / 2;
        }
    }

    fn get_min_child(&self, idx: usize) -> usize {
        let mut minidx = idx;
        for childidx in vec![2 * idx, 2 * idx + 1] {
            if (childidx < self.arr.len())
                && (self.get(childidx) < self.get(minidx))
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

#[test]
fn test() {
    println!("Heap");

    let mut heap: Heap<i32> = Heap::new(100);
    for elem in vec![17, 12, 0, 3, 200] {
        heap.push(elem);
    }
    dbg!(&heap);

    while let Some(elem) = heap.pop() {
        println!("min = {}", elem);
        //dbg!(&heap);
    }
}
