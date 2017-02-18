fn main() {
        //showcases shadowing in rust but also goes a little further and shows us how strings work in rust
        //more on this later on
    let x:  &'static str = "fun";

        //Establishes is a string
    {
        let (x,y) = (1,2);
        //reads out string content
    println!("String Read outs for x as {} ", x);
    println!("and as for y {}", y);
    }
        // {} backets infer binding and unbinding of particular definitions
    {
        // i32 defines a new rule as to what is to be stored in x
        // the new ruquirement is that is an interger
        // of at least 32 bits, this is known as a type
        let x: i32 = 5;
    println!("interger x is {}", x);
    }
    {
        //'mut' introduces variable mutability where a new
        // value can be introduced into a binding
    let mut x = 5;
        x = 10;
        println!("the original value of x was 5");
        println!("it now is {}", x);
    }

        //showcases the original value as binding is placed back into its first state
    println!("Learning RUST is {}", x);
}
