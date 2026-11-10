/*
Declare an i32 variable assigned to 1337.
Use the underscore character to add a visual
separator between the numbers.
 
Cast the i32 to an i16 integer and assign the result
to a separate variable.
 
Declare a floating-point value of your choosing.
Print out the number with 3 digits of precision.
 
Declare a 'with_milk' variable set to a Boolean.
Declare a 'with_sugar` variable set to a Boolean.
 
Declare a 'is_my_type_of_coffee` variable. It should
be set to true if the coffee has both milk and sugar.
 
Declare an `is_acceptable_coffee` variable. It should
be set to true if the coffee has either milk or
sugar.
 
Declare an array with four i8 integers of your choosing
Print out the array in its Debug representation.
 
Declare a tuple consisting of the integer, float,
a Boolean, and the array that you previously declared.
Print out the tuple in its Debug representation.
*/

fn main() {
    let some_number: i32 = 1_337;
    let some_number_i16 = some_number as i16;

    let some_float: f32 = 2.48291;
    println!("{:.3}", some_float);

    let with_milk: bool = true;
    let with_sugar: bool = false;

    let is_my_type_of_coffee = with_milk && with_sugar;
    let is_acceptable_coffee = with_milk || with_sugar;
    
    let some_array: [i8; 4] = [1, 8, 40, 32];
    println!("{:?}", some_array); 

    let some_tuple = (some_number_i16, some_float, true, some_array);
    // two ways to print the tuple
    println!("{:#?}", some_tuple);
    dbg!(some_tuple);
}
