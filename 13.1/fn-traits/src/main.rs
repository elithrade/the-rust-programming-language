// FnOnce applies to closures that can be called once, all closures implement at least this trait
// because all closures can be called.
// FnMut applies to closures that don't move captured value out of their body, but might mutate the
// captured values.
// Fn applies to closures that don't move captured value and don't mutate the captured value as well
// as capture nothing from their environment.
// impl<T> Option<T> {
//     pub fn unwrap_or_else<F>(self, f: F) -> T
//     where
//         F: FnOnce() -> T,
//     {
//         match self {
//             Some(x) => x,
//             None => f(),
//         }
//     }
// }
// unwrap_or_else can be called only once to return the value in the case of None option.
fn main() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // sort_by_key is defined to take a FnMut closure, because it calls the closure multiple times:
    // once for each item in the slice.
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    // The closure captures value then moves value out of the closure by transferring ownership
    // of value to the sort_operations vector.
    // This closure can be called once; trying to call it a second time wouldn’t work because
    // value would no longer be in the environment to be pushed into sort_operations again!
    // Therefore, this closure only implements FnOnce

    // let mut sort_operations = vec![];
    // let value = String::from("by key called");
    //
    // list.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });
    // println!("{:#?}", list);

    // In this case it only captures a mutable reference Therefore it can be called multiple times.
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
