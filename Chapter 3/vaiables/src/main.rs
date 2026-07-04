const PI:f32 = 3.14;

fn main() {

    //after MINUTES_IN_DAY we have to give the type must
    const MINUTES_IN_DAY:i32 = 24 * 60; 

    println!("The value of PI is {PI}");
    println!("The value of minutes is {MINUTES_IN_DAY}");

}


// by defalut the variable is rust is immutable
//but if you want to create a mutable variable that can change later you have to use
//mut after let and before vairable name to make it mutable.
