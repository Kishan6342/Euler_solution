

fn euler_method(n: usize, t0: f64, t_end:f64, y0:f64  )
{ let h = (t_end- t0) / n as f64;
    let mut t: f64 = t0;   //initialization the value of t
    let mut y: f64 = y0;   //initialization the value of y

    println!("t,y");

    for _ in 0..n {
        let dy = t.cos() - y;
        y += h * dy;
        t += h;
        println!("{:.10},{:.10}", t, y);

    }
    
}


fn main() { euler_method(1000,0.0, 5.0,1.0  ) }
