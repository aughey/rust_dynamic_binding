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

    Ok(())
}
