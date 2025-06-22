

fn plus_minus(arr: &[i32]) -> (f32, f32, f32) {
    let mut plus = 0;
    let mut minus = 0;
    let mut zero = 0;

    for x in arr{
        match x {
            x if (*x<0) => {
                minus += 1;
                println!("Negative number");
            }
            x if (*x>0) => {
                plus += 1;
                println!("Positive number");
            }
            x if (*x==0) => {
                zero += 1;
                println!("Zero");
            }
            _ => {}
        }
    }
    let total = plus+minus+zero;
    let plus_ratio: f32 = plus as f32/total as f32;
    let minus_ratio: f32 = minus as f32/total as f32;
    let zero_ratio: f32 = zero as f32/total as f32;

    return (plus_ratio, minus_ratio, zero_ratio);
}
fn main() {
    let arr: Vec<i32> = [1,2,3,-9,-8,0].to_vec();
    let res = plus_minus(&arr);
    print!("Plus ratio: {:.6}\n", res.0);
    print!("Minus ratio: {:.6}\n", res.1);
    print!("Zero ratio: {:.6}", res.2);
}