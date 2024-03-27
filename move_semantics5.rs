// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.



#[test]
fn main() 
{
    let mut x = 100; // x = 100
    
    let y = &mut x; // y = 100
    
    *y += 100; // y = 100 + 100 -> y = 200;
   
    let z = &mut x; // z = 100
    // z = 200 + 1000
    *z += 1000;

    assert_eq!(x, 1200);
}
