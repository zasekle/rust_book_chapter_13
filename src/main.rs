use std::thread;

fn main() {
    closures();
    processing_a_series_of_items();
    loops_vs_iterators();
}

fn closures() {

    //A closure is the name of what C++ calls a lambda (as far as what I can tell anyway). It is
    // an anonymous function that you can save in a variable or pass as arguments to other
    // functions.

    let mut hello = 5;

    let capture_environment = || {
        println!("hello: {hello}");
    };

    //Closures capture the environment variables.
    capture_environment();

    //Closures can have parameters and they don't (usually) require type annotations like
    // functions and methods do.
    let parameter = |a| {
        println!("a: {a}");
    };

    parameter(5);

    //Closures can have type annotations if desired and sometimes the compiler will require them.
    // The cases where they are required seem to be complex cases where the compiler can't
    // determine the type.
    let type_annotation = |a: i32| {
        println!("a: {a}");
    };

    type_annotation(4);

    //Closures can have a return value, for example below the return value type is `isize` and the
    // return value is 5.
    let return_value = || -> isize { 5 };

    println!("return_value: {}", return_value());

    //There are three ways parameters can be passed into a function, borrowing immutably, borrowing
    // mutably, and taking ownership. All three of these are possible with closures as well
    let one = 1;
    let mut two = 2;
    let three = 3;

    let borrow_immutably = || {
        println!("one: {one}")
    };
    let mut borrow_mutably = || {
        two += 1;
        println!("two: {two}");
    };
    //This is mostly used with multi threading.
    let take_ownership = move || {
        println!("three: {three}");
    };

    borrow_immutably();
    borrow_mutably();
    take_ownership();

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    //Passing closures to functions seems to require generics or trait objects. Generics are
    // preferred.

    fn hello_world<F>(hi: F)
        where
            F: FnOnce() -> isize
    {
        println!("hello_world() called {}", hi());
    }

    hello_world(|| {42});

    //There are three traits that closers will automatically implement.
    // 1) FnOnce applies to closures that can be called once.
    // 2) FnMut applies to closures that don't move captures values out of their both, but that
    //  might mutate the captured values.
    // 3) Fn applies to closures that don't move captures values out of their body and that don't
    //  mutate captures values, as well as closures that capture nothing from their environments.

    let moving_string = String::from("my_string");

    fn consume_string(str: String) {
        println!("consume_string() : {str}")
    };

    //This is an example of a closure that implements FnOnce. This is because after the first call
    // the string `moving_string` is consumed. Therefore, the closure cannot be called a second
    // time.
    let my_closure = || {
        consume_string(moving_string);
    };

    my_closure();
    // my_closure(); //Will not compile with a second call.
}

fn processing_a_series_of_items() {
    //In Rust, iterators are lazy, this means they have no effect until the methods that consume the
    // iterator are called.
    let v = vec!["a", "b", "c"];

    let v_iterator = v.iter();

    for i in v_iterator {
        println!("i: {}", i);
    }

    //Overloading an iterator in this language is actually fairly easy. First of all I just need
    // to override basic `Item` type and the `next()` function. Then I can simply call next().
    struct IteratorTest {
        first: String,
        second: String,
        at_first: usize,
    }

    impl Iterator for IteratorTest {
        type Item = String;

        fn next(&mut self) -> Option<Self::Item> {
            self.at_first += 1;
            if self.at_first == 1 {
                Some(self.first.clone())
            } else if self.at_first == 2 {
                Some(self.second.clone())
            } else {
                None
            }
        }
    }

    let mut iterator_test = IteratorTest{
        first: String::from("first_string"),
        second: String::from("second_string"),
        at_first: 0,
    };

    //Next will `consume` or use up the iterator.
    println!("first_test: {:?}", iterator_test.next());
    println!("second_test: {:?}", iterator_test.next());
    println!("none_test: {:?}", iterator_test.next());

    //into_iter() will take control of the calling object.
    let v_iter = v.into_iter();
    let mut v = vec!["a", "b", "c", "d"];
    let v_iter_mut = v.iter_mut();

    println!("v_iter: {:?} v_iter_mut {:?}", v_iter, v_iter_mut);

    //There are several inbuilt functions that consume the entire iterator. These functions are
    // called `consuming adapters`. sum() is an example of a consuming adapter.
    let v = vec![1,2,3];

    println!("sum: {}", v.iter().sum::<i32>());

    //There are also `iterator adaptors` which are methods that don't consume the iterator. Instead
    // they produce a new (and probably different) iterator.
    let iterator_adaptor = v.iter().map(|a| a + 1);

    println!("iterator_adaptor: {:?}", iterator_adaptor);

    //Things like filter and map are called directly on the iterator in this language.

    //Side note. Doing something like `for i in v.iter()` is fine in rust. It will use a single
    // iterator, not a new iterator for each access.
}

fn loops_vs_iterators() {
    //A quote from Bjarne Stroustrup is.
    // "In general, C++ implementations obey the zero-overhead principle: What you don’t use, you
    // don’t pay for. And further: What you do use, you couldn’t hand code any better."
    // This is also how Rust implements zero-overhead in its features. Iterators are an example of
    // this, they are compiled down to roughly the same code as a for loop. But they provide a
    // simple abstraction for the reader and writer.
}