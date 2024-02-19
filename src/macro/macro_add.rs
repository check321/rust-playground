#[macro_export]
macro_rules! add {
/*    // branch a: two attr accumulation.
    ($a:expr,$b:expr) => {
        {
                    $a + $b
        }
    };

    // branch b: only attr should be returned.
    ($a:expr) => {
        {
             $a
        }
    }*/

    ($($a:expr),*) => {
        {
            // branch: no attr.
            0
            // branch: repeat.
            $( + $a)*
        }
    }
}


fn main() {
    let x = 0;
    let sum = add!(8,2);
    println!("sum1: {}", sum);

    let sum2 = add!(x);
    println!("sum2: {}", sum2);

    let sum3 = add!();
    println!("sum3: {}", sum3);
}