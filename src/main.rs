use std::collections::HashMap;

use anyhow::Result;
use rust_dynamic_binding::DynamicCallable;

use rust_dynamic_binding::{
    make_dynamic_0, make_dynamic_1, make_dynamic_2, make_dynamic_3, make_dynamic_4,
};

fn function_that_takes_int_returns_string(x: &i32) -> String {
    format!("{}", x)
}

fn two_int_to_string(x: &i32, y: &i32) -> String {
    format!("{} {}", x, y)
}

fn add_int(x: &i32, y: &i32) -> i32 {
    x + y
}
fn add_float(x: &f32, y: &f32) -> f32 {
    x + y
}
fn multiply_int(x: &i32, y: &i32) -> i32 {
    x * y
}
fn multiply_float(x: &f32, y: &f32) -> f32 {
    x * y
}
fn int_to_float(x: &i32) -> f32 {
    *x as f32
}
fn float_to_int(x: &f32) -> i32 {
    *x as i32
}

fn all_functions() -> HashMap<String, Box<dyn DynamicCallable>> {
    let mut functions: HashMap<String, Box<dyn DynamicCallable>> = HashMap::new();
    functions.insert("add_int".to_string(), Box::new(make_dynamic_2(add_int)));
    functions.insert("add_float".to_string(), Box::new(make_dynamic_2(add_float)));
    functions.insert(
        "multiply_int".to_string(),
        Box::new(make_dynamic_2(multiply_int)),
    );
    functions.insert(
        "multiply_float".to_string(),
        Box::new(make_dynamic_2(multiply_float)),
    );
    functions.insert(
        "int_to_float".to_string(),
        Box::new(make_dynamic_1(int_to_float)),
    );
    functions.insert(
        "float_to_int".to_string(),
        Box::new(make_dynamic_1(float_to_int)),
    );
    functions
}

fn main() -> Result<()> {
    // Example of using the generic implementation with 1 argument
    let dynamic_f1 = make_dynamic_1(function_that_takes_int_returns_string);
    let arguments: Vec<Box<dyn std::any::Any>> = vec![Box::new(1)];
    let result = dynamic_f1.call(&arguments.as_slice())?;
    let display_result = result.downcast_ref::<String>().unwrap();
    println!("Generic Result (1 arg): {}", display_result);

    // Example with 0 arguments
    let dynamic_f0 = make_dynamic_0(|| "Hello, World!".to_string());
    let arguments: Vec<Box<dyn std::any::Any>> = vec![];
    let result = dynamic_f0.call(&arguments.as_slice())?;
    let display_result = result.downcast_ref::<String>().unwrap();
    println!("Generic Result (0 args): {}", display_result);

    // Example with 2 arguments
    let dynamic_f2 = make_dynamic_2(two_int_to_string);
    let arguments: Vec<Box<dyn std::any::Any>> = vec![Box::new(1), Box::new(2)];
    let result = dynamic_f2.call(&arguments.as_slice())?;
    let display_result = result.downcast_ref::<String>().unwrap();
    println!("Generic Result (2 args): {}", display_result);

    // Example with 3 arguments
    let dynamic_f3 = make_dynamic_3(|x: &i32, y: &i32, z: &i32| format!("{} {} {}", x, y, z));
    let arguments: Vec<Box<dyn std::any::Any>> = vec![Box::new(1), Box::new(2), Box::new(3)];
    let result = dynamic_f3.call(&arguments.as_slice())?;
    let display_result = result.downcast_ref::<String>().unwrap();
    println!("Generic Result (3 args): {}", display_result);

    // Example with 4 arguments
    let dynamic_f4 =
        make_dynamic_4(|x: &i32, y: &i32, z: &i32, w: &i32| format!("{} {} {} {}", x, y, z, w));
    let arguments: Vec<Box<dyn std::any::Any>> =
        vec![Box::new(1), Box::new(2), Box::new(3), Box::new(4)];
    let result = dynamic_f4.call(&arguments.as_slice())?;
    let display_result = result.downcast_ref::<String>().unwrap();
    println!("Generic Result (4 args): {}", display_result);

    test_graph_like_thing()?;

    Ok(())
}

fn test_graph_like_thing() -> Result<()> {
    let functions = all_functions();

    // Simulate something like
    // add_int(3, 2)      -> add_int
    // multiply_int(3, 2) -/

    let inputs: &[Box<dyn std::any::Any>] = &[Box::new(3), Box::new(2)];
    // a = 3 + 2 = 5
    let a = functions["add_int"].call(&inputs)?;
    // b = 3 * 2 = 6
    let b = functions["multiply_int"].call(&inputs)?;

    // add the two
    // c = 5 + 6 = 11
    let c = functions["add_int"].call(&[a, b].as_slice())?;

    let c_value = c.downcast_ref::<i32>().unwrap();
    assert_eq!(*c_value, 11);

    Ok(())
}
