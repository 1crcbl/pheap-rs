# Pairing Heap

[![Crates.io](https://img.shields.io/crates/v/pheap)](https://crates.io/crates/pheap) [![Documentation](https://docs.rs/pheap/badge.svg)](https://docs.rs/pheap) 

From [Wikipedia](https://en.wikipedia.org/wiki/Pairing_heap):
> A pairing heap is a type of heap data structure with relatively simple implementation and excellent practical amortized performance.
> Pairing heaps are heap-ordered multiway tree structures, and can be considered simplified Fibonacci heaps. They are considered a "robust choice" for implementing such algorithms as Prim's MST algorithm.

A min-pairing heap supports the following operations:
- ```find_min```: finds the minimum element of the heap, which is the root.
- ```merge```: combines two heaps together.
- ```insert```: adds a new element into the heap.
- ```delete_min```: remove the root and reorder its children nodes.
- ```decrease_key```: decrease the priority of an element. Standard implementation of a heap data structure does not support searching for a key efficiently (which is the case in this crate). Thus, this operation can take very long time, with an upper bound of ```O(2^(sqrt(log log n)))```.

## Benchmarks
To measure the performance of this implementation, I choose the following libraries that are available on [crates.io](https://crates.io/) to experiment:
- [Addressable pairing heap](https://crates.io/crates/addressable-pairing-heap)
- [Apasel422's implementation of pairing heap](https://crates.io/crates/pairing-heap)
- [Priority queue](https://crates.io/crates/priority-queue)
- [Keyed priority queue](https://crates.io/crates/keyed_priority_queue)

*If I miss any libraries, please let me know.*

The experiments are conducted on my PC with the following spec:
> OS: Fedora 34 64-<bit
> CPU: AMD® Ryzen 7 3800x 8-core processor 
> RAM: 32 GB

### Experiment 1
> Each implementation is tasked to execute 1000 insertions / 0 deletes, then 999 insertions / 1 deletes (remove the top element), until the number of deletes is 1000. This means each implementation has to execute 500_500 insertions and 500_500 deletions.

For this experiment, I use the crate [```criterion```](https://crates.io/crates/criterion) to measure the performance of each implementation. 

|  | Pairing heap<br>(this crate)  | Addressable pairing heap | Pairing heap <br>(Apasel422) | Priority queue | Keyed priority queue
--- | --- | --- | --- | --- | ---
|Average time<br>(milliseconds)|20.37|56.6|24.18|116.84|111.30|

### Experiment 2
> Each implementation is tasked to execute 1000 insertions / 1000 priority update / 0 deletes, then 999 insertions / 999 priority updates | 1 deletes (remove the top element), until the number of deletes is 1000.

|  | Pairing heap<br>(this crate)  | Addressable pairing heap | Pairing heap <br>(Apasel422) | Priority queue | Keyed priority queue
--- | --- | --- | --- | --- | ---
|Average time<br>(seconds)|1.399|No implementation|No implementation|0.171|0.142|

For this experiment, the pairing heap fairs worse than other two libraries. This is due to the fact that pairing heap data structures must search for keys, which in worse cases takes ```O(n)``` time, while other implementations leverage the fast lookup power from hash map.

### Experiment 3
> Each implementation is tasked to insert 1 million elements and the memory consumption will be measured.

For this experiment, I write a simple ```main``` (in ```examples/stress.rs```) and use ```valgrind``` with ```massif``` for the evaluation purpose.

To compile:
```batch
cargo build --examples --release
```

To run valgrind:
```batch
valgrind --tool=massif ./target/release/examples/stress <implementation> <number of nodes to be inserted>
```

The commandline argument ```<implementation>``` accepts the following options:
- ```pairing_heap```
- ```priority_queue```
- ```keyed_priority_queue```
- ```addressable_pairing_heap```
- ```ap422_pairing_heap```

|  | Pairing heap<br>(this crate)  | Addressable pairing heap | Pairing heap <br>(Apasel422) | Priority queue | Keyed priority queue
--- | --- | --- | --- | --- | ---
|Peak heap<br>memory consumption<br> (MB)|30.5|72.0|segfault|62|76|

The image outputs of ```massif-visualiser``` are stored in the folder ```img```.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.