struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");
    // After this line both _c and _d will be dropped.
    //
    let e = CustomSmartPointer {
        data: String::from("some data"),
    };
    // Rust doesn't allow calling drop() explicitly, because it will call drop automatically.
    // e.drop();
    // Instead call std::mem::drop function will allow 'early drop'.
    drop(e);
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");
}
