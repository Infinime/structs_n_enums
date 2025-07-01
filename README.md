I decided to try out the [Rust By Example Book](https://doc.rust-lang.org/rust-by-example) to Learn Rust. `main.rs` is the current rust project or book chapter I'm working on, and I (read:`new_file.exe`) move them to an appropriately named file once done.

# Thoughts so far

- (2024-04-13, 18:38pm): Seems um different but interesting, and it's making me think about memory allocation much more than Python would.
- (2024-04-13, 19:44pm): I have a lot of trouble with the borrowing etc of variables, I kinda get tripped up by it often, for example the `get` function in what is soon to be `linked_list.rs` that i rewrote threw an error when i used `match` but not when i used `if...else`. I remember having some issues like this learning python (objects and classes), so hopefully this irons out later like those did.
- (2024-04-13, 21:16pm): Just created the [`new_file.exe`](https://github.com/Infinime/structs_n_enums/blob/master/src/new_file.exe) program (based on [`new_file.rs`](https://github.com/Infinime/structs_n_enums/blob/master/src/new_file.rs)) to copy all the content from `main.rs` (where I work on them) to their permanent home in the other files
- (2024-04-14, 09:40PM): Done with the timestamp utility
- (2024-04-23, 09:43AM): Decided to move on from the rust by example book for project inspiration, and instead just find things that seem interesting to me so I can try more stuff out.
- (2025-01-01, 12:30AM): New year new me I guess. First commit of the year. Planning to pivot to crypto maybe.
- (2025-02-25, 02:38AM): Writing a hackerrank task to learn float formatting
- (2025-06-22, 08:43PM): Back here again to attack CSLR's Introduction to Algorithms
- (2025-06-28, 08:54PM): implemented the forward and reverse index sorts
- (2025-06-29, 01:04PM): implemented sub sort and tried out the new modular tests

# Progress

- tuples.rs: An intro to Tuples and Tuple structs
- structs_n_enums.rs: the first project i worked on, my first foray into Rust types
- linked_lists.rs: An implementation of the Linked List data structure using Rust's enum
- new_file.py: A python file that copies all the content from `main.rs` (where I work on rust files) to their permanent home in the other files
- new_file.rs: Same thing as the python one but in Rust
- timestamp.rs: A timestamp utility to get the current time and date  in this format: (2024-04-13, 18:38pm) so I can paste into the Thoughts So Far section
- plus_minus.rs: float formatting go brrrrr
- index_sort.rs, reverse_index.rs: index sort and reverse index sort algorithm implementations
- Substitution_sort.rs: can you guys (imaginary audience/fans, hi!) tell that i'm reusing the sort tests lmao. I should probably refactor the tests to be more "rusty"... Can I reuse tests...
- bit_array.[rs, py]: python and rust versions of the binary adder algorithms with tests in both languages
