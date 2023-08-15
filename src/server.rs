cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {

    // Internal server code we do not want in the ui code.
    pub fn hello_world() {
        println!("Hello World!");
    }

}}
