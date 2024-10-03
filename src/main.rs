use std::slice;

// extern function(used to call external functions from external code)
extern "C"{
    fn abs(input: i32) -> i32;
}    


// in rust global variables are called static variables
static Hello: &str = "Hello raiden"; // must annotate the type of static variables


// accessing mutable static variables is unsafe
// static variables hold a fixed adddress in the memory
static mut Counter: i32 = 0;

fn add_to_cnt(inc: i32){
    unsafe{
        Counter += inc;
    }
}


// ability to impliment unsafe trait
unsafe trait Foo{
    // method
}

unsafe impl Foo for i32{
    // method implimentation
}


fn main() {
   // dereferencing raw pointers 
   let mut num = 5;

   let r1 = &num as *const i32; // immutable
   let r2 = &mut num as *mut i32; // mutable
   // we made a mutable and immutable raw pointer pointing to the same location 

   // in order to dereference a raw pointer we have to use "unsafe" keyword
   unsafe{
     println!("r1 is: {}", *r1);
     println!("r2 is: {}", *r2);
   }        


   // unsafe functions
   unsafe fn danger() {}

   // we must always wrap an unsafe func inside an unsafe block
   unsafe{
     danger();
   }    


   // safe abstraction over unsafe code
   let mut v = vec![8, 9, 2, 3, 5, 4];

   let r = &mut v[..];

   let (a, b) = r.split_at_mut(3);

   assert_eq!(a, &mut[8, 9, 2]);
   assert_eq!(b, &mut[3, 5, 4]);


   // as calling external code is unsafe
   unsafe{
     println!("absolute value of -4 acc to c is: {}", abs(-4));
   }    


   // using the mutable static variable
   add_to_cnt(4);
   unsafe{
     println!("counter: {}", Counter);
   }
}



fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]){
   let len = slice.len();
   let ptr = slice.as_mut_ptr(); // getting a raw mutable poinnter

   assert!(mid <= len);

   unsafe{
     (
        slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(
            ptr.add(mid), len - mid
        ),
     )   
   } 
}   

#[no_mangle]
pub extern "C" fn call_from_c(){
    println!("calling a rust func from c");
} // so that external func exactly knows what is the name of the func is
