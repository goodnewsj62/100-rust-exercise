# **Important Notes On my Rust Learning**

### Overflows

adding an extra digit greater than the capacity for a numeric type causes and overflow. this can be statically analyze during compile time and will cause a panic as rust does not know how to handle it.
if you modify cargo.toml to allow override then rust would not panic

```toml
[proile.dev]
overflow-checks=false
```

a better solution will be to use wrapping*[operation] or saturating*[operation]
eg:

```rust
let v:u8 = 255.saturating_add(1);
```

here v will remain at u8::Max (255) as u8 maximum is 255

but the reverse is the case for wrapping

```rust
let v:u8 =  255.wrapping_add(1);
```

the result of v here will be 0 as it has been wrapped

### Range Slice

when working with range and you want a range up to a certain number without losing any value (like -1 the max)

you use this

```rust
for v in 0..=5 {
    /* v will start at 0 and end at 5 not 4 as its default behaviour */
    println!("{}", v);
}


```

### References

The reference to every data stored in the heap lives on the stack. Actually the variable holds the reference and metadata which live on the stack

by default systems uses 4bytes (32bit machine) or 8bytes (64bit machines) to store references.

Every other piece of information stored as metadata also occupies space on the stack. Each piece of information get the default bits of the machine. eg: if the system is a 32bit system, each information is stored as 4bytes.

to get the default size of a reference based on the the architecture of the machine you can do this

```rust
use std::mem;

const WEIGHT:usize =  mem::size_of::<&()>();

fn main() {

    println!("{}",WEIGHT );
    /*
        my system is 64bits so the value is 8 which means
        8bytes (64bits/8bits)
    */
}
```

so a String type will have a size on my system as 24bytes

```rust
let v =  String::with_capacity(5);
```

```
v: [reference,  length,  capacity]
   |
_______
|heap |
|_____|
```

the memory here

- reference: 8bytes
- length: 8bytes
- capacity: 8bytes

references to a String are 8bytes on a 64 bit system. The reason is because reference are allocated 8bytes to be stored

### Slices

There is an exception to the above statement when it comes to slices. slices are stored with 2 x the size of a regular reference. why?
A slice contains both the pointer to the part of the data it references and the length

### Dual Traits

From and Into are dual trait. when a type implements the From traits it already implements the Into trait by default.

the trait From is a trait that converts one type into the desired type.

In the case of the String type the From trait has been implemented

: here is how the from trait looks like

```rust
// inside std::convert;
pub trait From<T>:Sized {
    pub fn from(value:T) -> Self
}
```

this is the reason we can call and obtain a String type from &str

```rust
let converted =  String::from("slice");
```

One good thing to know is that From and Into are all sub-traits. They are sub-trait of Sized. That means both the type being converted and the type that implement the From & Into trait must be sized types.

why? remember Size type is an auto and marker trait implemented for all types with known sizes at compile time by default.  
so here "From\<T\>" T is Sized

```rust
impl<T,U> trait Into<U> for T
where U: From<T>
{
    fn into(self) -> U{
        U::from(self);
    }
}
```

so when you call `"ref".into();` this is internally gonna `String::from("ref");`
