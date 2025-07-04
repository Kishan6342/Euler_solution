<<<<<<< HEAD


fn euler_method(n: usize, t0: f64, t_end:f64, y0:f64  )
=======
fn euler_method(n: usize, t0: f64, t_end:f64, y0:f64  ) 
>>>>>>> 97cc43e3d60585b7097cf2f8ebe128dfb553566a
{ let h = (t_end- t0) / n as f64;
    let mut t: f64 = t0;   //initialization the value of t
    let mut y: f64 = y0;   //initialization the value of y

<<<<<<< HEAD
    println!("t,y");
=======
    // println!("t,y");
>>>>>>> 97cc43e3d60585b7097cf2f8ebe128dfb553566a

    for _ in 0..n {
        let dy = t.cos() - y;
        y += h * dy;
        t += h;
<<<<<<< HEAD
        println!("{:.10},{:.10}", t, y);

    }
    
}


fn main() { euler_method(1000,0.0, 5.0,1.0  ) }
=======
        
    }
    println!("the value of y at {:.10} is {:.10}", t, y);
}

fn main() { euler_method(20,0.0, 5.0,1.0  ) }
>>>>>>> 97cc43e3d60585b7097cf2f8ebe128dfb553566a
