# setup

`cargo new --lib neural` for library.

`cargo new neural` for executable.

The `Cargo.toml` file describes the project, while the `Cargo.lock` is responsible for consistent builds (like the build directory in cmake).

Rust allows `mod.rs` files to group multiple related files into a single module.

# library vs executable

`main.rs` is used for building an executable, while `lib.rs` is used to build a library (a static library by default).

# module system

While C++ uses *namespace*, Rust uses *modules*. The *pub* keyword makes the module public so other files can use it. By default, Rust modules are private, meaning they cannot be accessed from other files unless explicitly marked pub.

Each Rust file inside `src/` represents a module if it is referenced with `mod <name>;`.

```
src/
├── lib.rs       (or main.rs)
├── math/
│   ├── activation.rs
│   ├── mod.rs
```

- If you create math/activation.rs, it defines a module named activation inside math.
- If you create math/mod.rs, it defines a module named math.

`lib.rs` (or `main.rs` if it's a binary crate)
```
pub mod math; // Declares `math` as a module (Rust looks for math/mod.rs)
```

`math/mod.rs`
```
pub mod activation;  // Declares `activation` as a submodule
```

`math/activation.rs`
```
pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}
```

## Accessing in `lib.rs`

```
pub use math::activation::sigmoid;
```

Now, external code can use:

```
use neural::sigmoid;
```

`super` refers to the parent module of the current module. It is used when you want to access items from the module above in the hierarchy.

# function

This below is a public function that takes a double (64 bit float) and returns a double. Note the missing **;** at the last expression of the function, this means that the value of this expression is *returned* from the function. There is a *return* keyword as well, but that should be used for early returns.

```
pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}
```

Note: `exp()` in Rust is directly available for *f64* (std::f64::exp). Similarly, `std::max(0.0, x)` in C++ would convert to `x.max(0.0)` in Rust.

# traits

E.g. use the Add, Sub, Mul and Div traits for operator overloading (+, -, * and /). Implementing these traits allows custom types (like Matrix<T>) to support arithmetic operations.

```
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T> {
    data: Vec<Vec<T>>,
}
```

-> automatically implements debug printing (`println!("{:?}", matrix);`), copying (`.clone()`) and equality operator (`==`). Also note that it does not implement the `Default` trait (equivalent to a deleted default constructor in C++).

```
impl<T: Default + Clone + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + PartialEq> Matrix<T> {
    ...
}
```

This means that the functions inside this impl block are only available for types T that satisfy all these trait bounds. For T to be used in Matrix<T>, it must implement:
- Default -> Must have a default value (T::default()). (Has a default constructor.)
- Clone -> Must support .clone().
- Copy -> Must be cheaply copyable (e.g., i32, f64, but not Vec<T>). (Has default copy constructor, meaning it is trivially copyable.)
- Add<Output = T> -> Must support + (e.g., a + b).
- Sub<Output = T> -> Must support - (e.g., a - b).
- Mul<Output = T> -> Must support * (e.g., a * b).
- Div<Output = T> -> Must support / (e.g., a / b).
- PartialEq -> Must support == and != for equality checks.

This is a constructor function for Matrix<T>, and it initializes a matrix of size rows × cols with default values for type T.

```
pub fn new(rows: usize, cols: usize) -> Self {
    if rows == 0 || cols == 0 {
        panic!("Matrix dimensions cannot be zero");
    }
    Self {
        data: vec![vec![T::default(); cols]; rows],
    }
}
```

`pub fn new(rows: usize, cols: usize) -> Self` -> public function that creates a new matrix object. `panic` stop execution.

`Self {...}`  is equivalent to `Matrix {...}`, creating an instance of `Matrix<T>`.

`data: vec![vec![T::default(); cols]; rows]` -> creates a vector of vectors (`Vec<Vec<T>>`) of size `rows x cols`. Each element is initialized using `T::default()`, meaning it requires `T: Default`.

# OOP

Rust moves away from classical OOP in several ways. In Rust, methods need `self` explicitly as their first parameter. This is basically how you would go around implementing object-oriented-like behavior using C.

```
// Rust
impl Matrix<T> {
    pub fn rows(&self) -> usize {
        self.data.len()
    }
}
// C++
class Matrix {
public:
    size_t rows() const { return data.size(); }
};
```

# (no) exceptions

Rust doesn't have exceptions, so `panic!()` is a crash, not a recoverable error. Instead, Rust uses `Result<T, E>` for error handling. However, the constructor can also return a value.

```
// panic
pub fn new(rows: usize, cols: usize) -> Self {
    if rows == 0 || cols == 0 {
        panic!("Matrix dimensions cannot be zero");
    }
    Self {
        data: vec![vec![T::default(); cols]; rows],
    }
}
// recoverable
pub fn new(rows: usize, cols: usize) -> Result<Self, String> {
    if rows == 0 || cols == 0 {
        return Err("Matrix dimensions cannot be zero".to_string());
    }
    Ok(Self {
        data: vec![vec![T::default(); cols]; rows],
    })
}

// usage
match Matrix::<i32>::new(3, 3) {
    Ok(matrix) => println!("{:?}", matrix),
    Err(e) => println!("Error: {}", e),
}
```

# subscription operator

In Rust, you can overload the [] subscription operator, but it's not as flexible as in C++. Rust uses the `Index` and `IndexMut` traits to overload [].

```
use std::ops::{Index, IndexMut};

impl<T> Index<usize> for Matrix<T> {
    type Output = [T]; // This returns a row as a slice

    fn index(&self, row: usize) -> &Self::Output {
        &self.data[row]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.data[row]
    }
}

fn main() {
    let mut matrix = Matrix::<i32>::new(3, 3).unwrap();
    matrix[0][0] = 42;  // Works like a normal subscription operator!
    println!("{}", matrix[0][0]); // Prints 42
}
```

There are two key limitations in Rust’s operator overloading:
- `Index` Can Only Return a Reference (`&T` or `&[T]`). Rust does not allow `Index` to return a value (`T`) -> it must return a reference. In C++, `operator[]` can return a copy, but in Rust it cannot. This forces us to return a reference (`&T`), which means we can't directly return a computed value. A `get()` function gives more flexibility, allowing us to return `Option<T>` for safe access.
- No Custom Error Handling (`Index` Panics on Out-of-Bounds). If we implement `Index`, Rust will panic on out-of-bounds access. With a `get()` function, we can return `Option<T>` and handle errors gracefully.

```
impl<T> Matrix<T> {
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get(row)?.get(col)
    }
}
```

- `matrix[10][10]` will panic if the matrix is smaller.
- `matrix.get(10, 10)` will return None safely.

`Option<T>` is similar to C++ `std::optional`.

The question mark (`?`) is a Rust shorthand for error propagation when working with `Option<T>` or `Result<T, E>`.

```
// without ?
fn get_element(matrix: &Matrix<i32>, row: usize, col: usize) -> Option<&i32> {
    match matrix.data.get(row) {
        Some(inner) => match inner.get(col) {
            Some(value) => Some(value),
            None => None,
        },
        None => None,
    }
}
// with ?
fn get_element(matrix: &Matrix<i32>, row: usize, col: usize) -> Option<&i32> {
    matrix.data.get(row)?.get(col)
}
```

- If the value is `Some(x)`, it is returned normally.
- If the value is `None`, it propagates `None` up the call chain (instead of continuing execution).
- The `?` operator only works inside functions that return `Option<T>` (or `Result<T, E>`).

# template parameter constraints

These are identical

```
impl<T: Add<Output = T> + Copy> Add for Matrix<T> {...}
impl<T> Add for Matrix<T> where T: Add<Output=T> + Copy {...}
impl<T> Add for Matrix<T>
where
    T: Add<Output=T>,
    T: Copy
{...}
```

# closure / lambda

In Rust, `|x| x * x` is a lambda expression, also known as a closure. It is equivalent to a lambda function in C++ `[](auto x) { return x * x; }`. While in C++ the capture is explicit `[=]`, `[&]`, `[this]`, in Rust it is inferred.

# phantom data

```
pub struct RandomGenerator<T> {
    rng: StdRng,
    _marker: PhantomData<T>
}
```

`PhantomData<T>` is used in the `RandomGenerator<T>` struct to indicate that the struct is associated with a type `T`, even though it doesn't actually store any value of type `T`. Rust's type system enforces strict rules about ownership and lifetimes. The `PhantomData<T>` is a zero-sized type that tells the Rust compiler that RandomGenerator has some kind of association with the type `T`, even though `T` is not explicitly stored as a field in the struct. Even though `RandomGenerator` does not hold any actual value of type `T`, it is still important to ensure that the struct is aware of the type `T`. Without `PhantomData<T>`, the compiler would think that `RandomGenerator` has no association with `T`, and might not correctly handle type-related issues (such as lifetime or variance).

# move vs copy

- Move: If a type does not implement `Copy`, like a `String`, then passing it by value will move the ownership to the function. After that, the original value cannot be used.
- Copy: Types that implement the `Copy` trait, like `f32`, are copied when passed by value. The original value remains available for use after the function call.

Copy Semantics for `f32`: `f32` is a `Copy` type in Rust. This means that when you pass f32 by value to the function, a copy of the value is made, rather than moving the value. So, after calling the function, the original value can still be used.

```
pub fn get_factor (self) -> f32 {
    return self.factor;
}
```

When you define a method like this and take `self` 'by value', it means that ownership of the entire object (the instance of the struct) is moved into the method. After this method call, the instance of the struct that called this method can no longer be used because ownership has been transferred (moved) into the method. After the method completes, the self instance is moved, and you cannot use the struct anymore unless you explicitly borrow it or clone it beforehand. This means if you call `get_factor`, the struct instance calling it will be effectively "consumed" and cannot be used after the call.

You should take self by reference when you don’t need ownership of the struct and want to be able to use the struct after the method is called. This is the most common case, especially for getter methods, because they typically just need to read from the struct without taking ownership. In this case, self is borrowed immutably, meaning the struct can still be used after the method call.

```
pub fn get_factor(&self) -> f32 {
    self.factor
}
```

You should take self by mutable reference if you need to modify the struct, but still don’t want to take ownership of it. This allows you to change the internal state of the struct while still maintaining ownership elsewhere.

```
pub fn set_factor(&mut self, factor: f32) {
    self.factor = factor;
}
```