// fn main() {
//     println!("Hello, world!");
//     another_function();
// }

// fn another_function(){
//     println!("Another function!");
// }

// Trying to pass arguments to a function

// fn main(){
//     another_function(5);
// }

// fn another_function(x: i32)
// {
//     println!("The value of x is {x}");
// }

// Passing variables with various data types

// fn main() {
//     print_labeled_measurements(5, 'h');
// }

// fn print_labeled_measurements(value: i32, unit: char) {
//     println!("The measurement is {value}{unit}");
// }

// Differentiating statements and expressions

// fn main()
// {
//     let y =(let x=6);
// }
// the above returns an error during compilation due to 
// y expects an expression, but rather finds a statement 

// assigning function  to expressions
// fn main(){
//     let y ={
//         let x =3;
//         // The following is an expression and not a statement as semicolon isnt used
//         // using semicolon would turn in into a statement 
//         x+1
//     };
//     println!("The value of y is {y}");
// }

fn main()
{
    let x =plus_one(sixty_nine());
    println!("The value of x is {x}");
}

fn sixty_nine()->i32{
    60+9
    // RETURNS 69

}
 fn plus_one (x:i32)->i32{
    x+1
    // INCREASES VALUE BY 1
 }