#[cfg(mine)]
fn cond_function() {
    println!("mine cond!!!");
}

#[cfg(not(mine))]
fn cond_function() {
    println!("not mine cond!!!");
}

fn main() {
   cond_function();
}

