//! # Usage 
//! This module exposes an interpreter that 
//! takes in the custom markdown employed by the site 
//! and compiles it to HTML (and maybe a little CSS).



/// # Usage 
/// A very basic set of primitive 
/// types.
enum Type {
    Integer, 
    String, 
    Real,
    Boolean,
}

trait Macro {

}

pub struct Macro {
    name: String, 
    params: HashMap<String, Type>
}
