fn main() {
    solve_theorem();
}

fn solve_theorem() {
    let mut aa : i32 = 0; 
    let mut bb : i32 = 0;
    let mut cc : i32 = 0;

    for n in 1..101 {
        for a in 1..101 {
            let _a_coef : i32 = a;
            aa = _a_coef.pow(n);             

            for b in 1..101 {
                let _b_coef : i32 = b;
                bb = _b_coef.pow(n);                 

                for c in 1..101 {
                    let _c_coef : i32 = c;
                    cc = _c_coef.pow(n);
                    let aabb : i32 = aa + bb;

                    if aabb == cc
                    {                        
                        println!("");
                        
                        println!("{}^{} + {}^{} = {}^{}", aa, n, bb, n, cc, n);
                        println!("These numbers are equal!");
                    }
                    else {
                        println!("{}^{} + {}^{} = {}^{}", aa, n, bb, n, cc, n);
                        println!("These numbers are NOT equal!");                      
                    }
                }
            }
        }
    }
}

