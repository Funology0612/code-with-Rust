fn main() {
    /*
      String - A dynamic piece of text stored on the heap
      at runtime

      &String ("ref String") - A reference to a heap String

      str - A hardcoded, read-only piece of text encoded in
      the binary

      &str ("ref str") - A reference to th text in the memory
      that has loaded the binary file
    */

    let ice_cream = "Cookies and Cream";
    println!("{}", ice_cream);
}
