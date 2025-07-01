fn Euler_solution(n: usize){
  let h = 5.0/n as f64;
  let mut t:f64 = 0.0;
  let mut y:f64 = 1.0;
  for _ in 0 .. n{
    let dy = t.cos()-y;
    y += h*dy;
    t += h
    println!("{:.10}, {:.10}",t,y);
}
}
fn main(){
  Euler_solution(2);
}
