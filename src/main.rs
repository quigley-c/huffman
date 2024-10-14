use std::io;
use regex::Regex;
mod binary_heap;

fn main() {
    // read stdin
    let lines: Vec<String> = io::stdin()
        .lines()
        .collect::<Result<_,_>>()
        .unwrap();
    let (mut heap, _) = binary_heap::init_heap(lines.clone());

    let mut letters: Vec<i32> = vec![];
    for i in 1..lines.len() {
        let rx = Regex::new("[0-9]+").unwrap();
        if !rx.is_match(&lines[i]) {continue;}
        letters.push(str::parse(&lines[i]).expect(""));
    }

    // heap is initialized with infinity weights so we assign vals to verts here
    for i in 0..heap.data.len() {
        let v = heap.data[i].name;
        let val = letters[i];
        binary_heap::decrease_key(
            &mut heap,
            v,
            val
        );
    }

    let res = huffman(&mut heap);
    println!("{}", res);
}

fn huffman(mut heap: &mut binary_heap::Heap) -> i32 {
    let n = heap.data.len();
    let mut tree = vec![];
    let mut dj = vec![];
    for _ in 0..n-1 {
        // remove the smallest 2 entries
        let v1: binary_heap::Vert = binary_heap::heap_extract(&mut heap).expect("");
        let v2: binary_heap::Vert = binary_heap::heap_extract(&mut heap).expect("");
        // add them together
        let cv = v1.label + v2.label;
        // push the result back on the heap
        let v: binary_heap::Vert = binary_heap::Vert {
            name: heap.data.len(),
            edges: vec![],
            label: cv as i32
        };

        for n in [v1,v2] {
            tree.push(n.label);
            dj.push((tree.len()+1)/2 * 2)
        }

        binary_heap::heap_insert(&mut heap, v);
        // done when we have only 1 entry left on the heap
    }

    let mut bits = 0;
    for i in 0..tree.len() {
        bits = bits + tree[i]
    }

    return bits;
}
