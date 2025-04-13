// iterators1.rs
//
// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.
//
// Make me compile by filling in the `???`s
//
// Execute `rustlings hint iterators1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = my_fav_fruits.iter();   // 修改：使用 iter() 而不是 iterator()

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));     // 修改：返回 Some(&"banana")
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));     // 修改：返回 Some(&"custard apple")
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));     // 修改：返回 Some(&"avocado")
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));     // 修改：返回 Some(&"peach")
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));     // 修改：返回 Some(&"raspberry")
    assert_eq!(my_iterable_fav_fruits.next(), None);     // 修改：当没有更多元素时返回 None
}
