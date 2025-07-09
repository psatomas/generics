fn make_tuple<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}
fn main() {
    make_tuple(5, "hello");
    make_tuple(5, 13);
    make_tuple(true, 3.14);
    make_tuple(true, false);
}