use std::sync::{Arc, Mutex};
use std::{ thread, vec};


fn multiply_matrices(matrix_a: &[Vec<i32>], matrix_b: &[Vec<i32>]) -> Vec<Vec<i32>>{
    let rows_a = matrix_a.len();
    let cols_a = matrix_a[0].len();
    let cols_b = matrix_b[0].len();

    let result = vec![vec![0; cols_b]; rows_a];

    let result_arc = Arc::new(Mutex::new(result));

    let mut handles = vec![];

    for i in 0..rows_a {
        let matrix_a = matrix_a.to_owned();
        let matrix_b = matrix_b.to_owned();
        let result_arc = Arc::clone(&result_arc);

        let handle = thread::spawn(move || {
            for j in 0..cols_b{
                let mut sum = 0;
                for k in 0..cols_a{
                    sum += matrix_a[i][k] * matrix_b[k][j];
                }

                let mut result = result_arc.lock().unwrap();
                result[i][j] = sum;
            }
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    let result = Arc::try_unwrap(result_arc).unwrap().into_inner().unwrap();
    result
}

fn main() {
   let matrix_a = vec![
        vec![1,2,3],
        vec![4,5,6],
        vec![7,8,9],
   ];
   let matrix_b = vec![
        vec![9,8,7],
        vec![6,5,4],
        vec![3,2,1],
   ];

   let result = multiply_matrices(&matrix_a,&matrix_b);

   println!("Result matrix:");
   for row in result {
    println!("{:?}", row);
   }
}
