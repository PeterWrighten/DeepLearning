use neuronika;
use ndarray::array;

///! whole variable is f32 type. on the other word, only f32 type available.
fn main() {
    let shape = [3, 4];
    let rand_variable = neuronika::rand(shape);
    println!("Rand variable:\n{}", rand_variable);
    let ones_variable = neuronika::ones(shape);
    println!("Ones variable: \n{}", ones_variable);
    let constant_variable = neuronika::full(shape, 7.);
    println!("Full variable: \n{}", constant_variable);
    let array = array![1., 2.];
    let x_ndarray = neuronika::from_ndarray(array);
    println!("From ndarray: \n {}", x_ndarray);

}
