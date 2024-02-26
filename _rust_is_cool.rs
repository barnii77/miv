use std::marker::PhantomData;


trait Xyz<T: ?Sized> {
    fn fun(&self, x: &T);
}

struct Example<'a, 'b: 'a, T: Default + ?Sized, State, const N: usize> {
    x: &'a mut [&'b dyn Xyz<T>; N],
    _state: PhantomData<State>,
}

impl<'a, 'b: 'a, T: Default + ?Sized, State, const N: usize> std::fmt::Debug for Example<'a, 'b, T, State, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Example")
            .finish()
    }
}

#[derive(Debug)]
struct MyType<T>(T);

impl Default for MyType<i32> {
    fn default() -> Self {
        MyType(0)
    }
}

impl<T: std::fmt::Debug> Xyz<Self> for MyType<T> {
    fn fun(&self, x: &Self) {
        println!("{:?} {:?}", self, x);
    }
}

fn main() {
    // println!("{:?}", (1..3).collect::<Vec<usize>>());
    let a = [MyType(1), MyType(2), MyType(3)];
    let mut b: [&dyn Xyz<MyType<i32>>; 3] = [&a[0], &a[1], &a[2]];
    let _example = Example {
        x: &mut b,
        _state: PhantomData::<()>,
    };
    println!("{:?}", _example);
    let clo = move || {_example.x;};
    clo();
}
