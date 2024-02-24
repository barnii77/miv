mod gap_buffer;

fn main() {
    // println!("{:?}", (1..3).collect::<Vec<usize>>());
    let mut buffer = gap_buffer::GapBuffer::<char>::new_empty();
    let string_vec = "hello world".chars().collect::<Vec<char>>();
    buffer.insert(&string_vec);
    buffer.move_gap(2);
    buffer.delete(3);
    buffer.insert(&string_vec);
    println!("{:?}", buffer);
}
