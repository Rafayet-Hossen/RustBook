//const PI:f32 = 3.14;

fn main() {
    let tup:(i32,f32,bool) = (100,2.4,true);

    let (x,y,z) = tup;

    println!("vlaue of x is {x}");
    println!("vlaue of x is {y}");
    println!("vlaue of x is {z}");

}


// by defalut the variable is rust is immutable
//but if you want to create a mutable variable that can change later you have to use
//mut after let and before vairable name to make it mutable.
