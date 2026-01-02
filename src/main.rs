// fn main() {
//     // println!("Hello, world!");
//     let mut x=5;
//     println!("The value of x is {x}");
//     x=6;
//     println!("The value of x is {x}");
// }

// fn main() {
//     let x = 5;
//     let x = x + 1;
//     // first overshadowing of x
//     println!("the value of x afte first overshadowing is {x}");
//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is {x}");
//     }
//     println!("The value of x is {x} (Outer scope)");

//     // using same variable for mutliple data types:
//     let spaces="  ";
//     {
//         let spaces=spaces.len();
//         println!("{spaces} is the length of the string i.e. how many spaces has it got" );
//     }
//     println!("The string represented by spaces is '{spaces}'")
//     //  We'll get a compile time error if we use mut while declaring and then trying to change the data type
// }


// Performing operations 
// fn main() {
//     // performing addition
//     let sum = 29 + 12;
//     println!("The sum of 29 and 12 is {sum}");

//     // performing subtraction
//     let difference = 9848.75 - 9951.3;
//     println!("The difference of 9848.75-9951.3 is: {difference}");

//     // performing multiplication
//     let pdt = 29 * 16 * 11 * 12;
//     println!("The product of 29*16*11*12 is {pdt}");

//     // performing division
//     let quotient = 56.7 / 32.2;
//     println!("The quotient is of 56.7/32.2 is {quotient}");

//     let truncated = -5 / 3;
//     println!("The truncated value of -5 divided by 3 is {truncated}");

//     // finding remainder
//     println!("The remainder of 15%2 is {}", 15 % 2);
// }


// Compound data types
// TUPLES

// fn main ()
// {
//     let tup:(u32, f64, i8)=(2912, 98.4875, -7);
//     let (x,y,z)=tup;
//     println!("The u32 stored value is {x}");
//     println!("The f64 stored value is {y}");
//     println!("The i8 stored value is {}which plus one is {}",z,z+1);

   
    
// }

// Second method of accessing is using period (.) - zero indexed
// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let five_hundred = x.0;

//     let six_point_four = x.1;

//     let one = x.2;

//     println!("i32 placed value is {five_hundred}");
//     println!("f64 placed value is {six_point_four}");
//     println!("8 bit stored value is {one}");

// }

// ARRAYS

// fn main()
// {
//     let arr: [u32;5]=[29,12,16,11,69];
//     // using debug formatting to print the arr
//     println!("The array declared is {:?}",arr);

//     let array=[98.48;10];
//     println!("The array with repetitive elements is {:?}", array);

//     // Elements can be accessed using zero indexing
//     println!("The third element in arr array is {}",arr[2]);
// }

// Invalid array element access 
 use std::io;
 fn main()
 {
    let a =[21 , 11, 21, 7, 6];
    println!("Please enter any array index");

    let mut index =String::new();

    io::stdin().read_line(&mut index).expect("failed to read line");

    let index:usize=index.trim().parse().expect("Index entereed was not a number");

    let el=a[index];
    println!("Element at index {index} is {el}");
 }