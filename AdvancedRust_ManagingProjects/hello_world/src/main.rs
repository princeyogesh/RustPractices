fn main() {
    crate::hello::english();    //hello absolute path
    crate::hello::spanish();    //hola absolute path
    crate::hello::casual::english(); // heyy casual absolute path
    /////////////////////////////
    
    hello::spanish();   //hola relative path
}

mod hello {
    pub fn english() {
       println!("hello");
    }

    pub fn spanish() {
        println!("hola");
    }


    pub mod casual {
        pub fn english() {
            println!("heyyy");
           // crate::hello::spanish();        //absolute path
            //super::spanish();                //relative path
        }

    }
}
