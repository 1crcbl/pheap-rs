# Pairing Heap

From [Wikipedia](https://en.wikipedia.org/wiki/Pairing_heap):
> A pairing heap is a type of heap data structure with relatively simple implementation and excellent practical amortized performance.
> Pairing heaps are heap-ordered multiway tree structures, and can be considered simplified Fibonacci heaps. They are considered a "robust choice" for implementing such algorithms as Prim's MST algorithm.

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
|Average time<br>(milliseconds)|19.78|51.78|24.72|113.40|102.90|

**Note**: currently, my implementation doesn't have support for changing priority operation. Once this feature is implemented, I will run the experiments again.

### Experiment 2
> Each implementation is tasked to insert 1 million elements and the memory consumption will be measured.

For this experiment, I write a simple ```main``` (in ```examples/stress.rs```) and use ```valgrind``` with ```massif``` for the evaluation purpose.

To compile:
```batch
cargo build --examples --release
```

To run valgrind:
```batch
valgrind --tool=massif ./target/release/examples/stress <implementation> 1000000
```

|  | Pairing heap<br>(this crate)  | Addressable pairing heap | Pairing heap <br>(Apasel422) | Priority queue | Keyed priority queue
--- | --- | --- | --- | --- | ---
|Peak heap<br>memory consumption<br> (MB)|30.5|72.0|segfault|62|76|

The image outputs of ```massif-visualiser``` are stored in the folder ```img```.

**Note**: I'm unsure how the difference in (peak) memory consumption arises. I only know that the ```priority-queue``` includes a ```HashMap``` in its implementation for a quick lookup. 

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.