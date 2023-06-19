use std::thread;

fn main() {
    closures();
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
