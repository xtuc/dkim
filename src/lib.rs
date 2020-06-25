//! # Goals
//! 
//! When one step is completed, the major part of the crate version is increased.
//! 
//! 1. Make it work
//!     - [x] Relaxed canonicalization algorithm
//!     - [x] Verifying
//!     - [x] Simple canonicalization algorithm
//!     - [ ] Signing
//!     - [ ] Documentation
//! 2. Make it robust
//!     - [ ] Fulfill each "MUST" of the RFC
//!     - [ ] Fulfill each "SHOULD" of the RFC
//!     - [ ] Write tests
//!     - [ ] Eradicate unwraps
//! 3. Make it fast
//!     - [ ] Benchmarks
//!     - [ ] Compare to other implementations
//!     - [ ] Optimize

pub mod verifier;
pub mod canonicalization;
pub mod hash;
pub mod email;
pub mod dkim;

pub enum SigningAlgorithm {
    RSASha1,
    RSASha256,
}

pub enum MessageCanonicalization {
    Simple,
    Relaxed
}

#[cfg(test)]
mod tests {
    const MAIL: &str = include_str!("mail.txt");
    #[test]
    fn it_works() {
        use crate::email::Email;
        use std::convert::TryFrom;

        let mail = Email::try_from(MAIL).unwrap();

        mail.verify().unwrap()
    }

    /*#[test]
    fn go_comp_headers() {
        use  prettydiff::{diff_chars};
        let headers = [109,105,109,101,45,118,101,114,115,105,111,110,58,49,46,48,13,10,114,101,102,101,114,101,110,99,101,115,58,60,67,65,78,99,61,50,85,85,99,48,113,114,117,98,53,109,52,120,80,79,87,69,115,122,86,43,112,103,61,43,82,107,80,80,69,65,101,112,98,113,107,122,66,50,97,79,71,50,102,115,65,64,109,97,105,108,46,103,109,97,105,108,46,99,111,109,62,32,60,67,65,78,99,61,50,85,86,80,75,67,82,117,65,97,88,119,66,101,115,74,45,72,100,45,69,114,61,117,49,56,82,107,78,107,77,72,56,97,117,102,43,103,81,88,103,101,77,49,79,81,64,109,97,105,108,46,103,109,97,105,108,46,99,111,109,62,32,60,67,65,78,99,61,50,85,85,43,114,118,101,100,114,56,48,121,68,71,86,118,117,86,119,88,118,43,109,117,104,110,43,82,88,99,76,80,121,105,52,81,70,54,74,99,51,49,104,102,76,103,64,109,97,105,108,46,103,109,97,105,108,46,99,111,109,62,32,60,67,65,78,99,61,50,85,86,82,121,99,81,105,75,66,67,49,57,74,84,80,105,49,67,53,71,78,53,52,54,86,55,49,67,53,57,100,110,82,111,105,72,80,84,45,75,82,115,49,106,103,64,109,97,105,108,46,103,109,97,105,108,46,99,111,109,62,32,60,67,65,78,99,61,50,85,88,83,90,53,56,83,117,104,86,104,80,50,112,86,74,50,75,86,78,100,77,86,49,55,103,68,102,76,65,84,121,84,45,99,72,72,69,56,117,72,69,78,114,103,64,109,97,105,108,46,103,109,97,105,108,46,99,111,109,62,32,60,67,65,78,99,61,50,85,88,82,82,99,116,109,106,110,50,117,61,66,43,80,104,116,86,83,54,57,103,95,77,119,114,115,109,119,67,121,76,117,107,110,56,54,87,122,98,115,65,52,99,81,64,109,97,105,108,46,103,109,97,105,108,46,99,111,109,62,13,10,105,110,45,114,101,112,108,121,45,116,111,58,60,67,65,78,99,61,50,85,88,82,82,99,116,109,106,110,50,117,61,66,43,80,104,116,86,83,54,57,103,95,77,119,114,115,109,119,67,121,76,117,107,110,56,54,87,122,98,115,65,52,99,81,64,109,97,105,108,46,103,109,97,105,108,46,99,111,109,62,13,10,102,114,111,109,58,77,117,98,101,108,111,116,105,120,32,60,109,117,98,101,108,111,116,105,120,64,103,109,97,105,108,46,99,111,109,62,13,10,100,97,116,101,58,77,111,110,44,32,50,50,32,74,117,110,32,50,48,50,48,32,49,50,58,50,48,58,48,48,32,43,48,50,48,48,13,10,109,101,115,115,97,103,101,45,105,100,58,60,67,65,78,99,61,50,85,85,57,87,119,70,52,69,56,98,114,100,86,77,65,54,67,100,51,51,79,98,84,49,70,89,118,95,84,119,80,113,82,107,45,77,99,101,85,70,95,98,80,50,119,64,109,97,105,108,46,103,109,97,105,108,46,99,111,109,62,13,10,115,117,98,106,101,99,116,58,82,101,58,32,72,101,121,33,13,10,116,111,58,109,117,98,101,108,111,116,105,120,64,49,56,53,46,49,51,49,46,49,57,52,46,49,50,55,13,10];
        let headers = unsafe {String::from_utf8_unchecked((&headers).to_vec())};
        let headers2 = crate::canonicalization::canonicalize_headers_relaxed(MAIL, vec!["mime-version","references","in-reply-to","from","date","message-id","subject","to"]);


        println!("{}", diff_chars(&headers, &headers2));
    }

    #[test]
    fn go_comp_body() {
        use  prettydiff::{diff_chars};
        let headers =  [45,45,48,48,48,48,48,48,48,48,48,48,48,48,55,102,99,97,97,51,48,53,97,56,97,57,57,54,53,101,13,10,67,111,110,116,101,110,116,45,84,121,112,101,58,32,116,101,120,116,47,112,108,97,105,110,59,32,99,104,97,114,115,101,116,61,34,85,84,70,45,56,34,13,10,67,111,110,116,101,110,116,45,84,114,97,110,115,102,101,114,45,69,110,99,111,100,105,110,103,58,32,113,117,111,116,101,100,45,112,114,105,110,116,97,98,108,101,13,10,13,10,69,110,99,111,114,101,32,117,110,32,33,13,10,13,10,76,101,32,100,105,109,46,32,50,49,32,106,117,105,110,32,50,48,50,48,32,61,67,51,61,65,48,32,49,55,58,48,56,44,32,77,117,98,101,108,111,116,105,120,32,60,109,117,98,101,108,111,116,105,120,64,103,109,97,105,108,46,99,111,109,62,32,97,32,61,67,51,61,65,57,61,13,10,99,114,105,116,32,58,13,10,13,10,62,32,66,111,110,106,111,117,114,44,13,10,62,32,67,101,99,105,32,101,115,116,32,117,110,32,116,101,115,116,46,13,10,62,32,46,13,10,62,13,10,62,13,10,62,32,76,101,32,100,105,109,46,32,50,49,32,106,117,105,110,32,50,48,50,48,32,61,67,51,61,65,48,32,49,55,58,48,55,44,32,77,117,98,101,108,111,116,105,120,32,60,109,117,98,101,108,111,116,105,120,64,103,109,97,105,108,46,99,111,109,62,32,97,32,61,67,51,61,13,10,61,65,57,99,114,105,116,32,58,13,10,62,13,10,62,62,32,66,111,110,106,111,117,114,44,13,10,62,62,32,74,101,32,115,117,105,115,32,117,110,101,32,111,108,105,118,101,46,13,10,62,62,13,10,62,62,32,76,101,32,100,105,109,46,32,50,49,32,106,117,105,110,32,50,48,50,48,32,61,67,51,61,65,48,32,49,54,58,53,56,44,32,77,117,98,101,108,111,116,105,120,32,60,109,117,98,101,108,111,116,105,120,64,103,109,97,105,108,46,99,111,109,62,32,97,32,61,67,51,61,13,10,61,65,57,99,114,105,116,32,58,13,10,62,62,13,10,62,62,62,32,113,100,100,122,113,122,100,113,13,10,62,62,62,13,10,62,62,62,32,76,101,32,100,105,109,46,32,50,49,32,106,117,105,110,32,50,48,50,48,32,61,67,51,61,65,48,32,49,54,58,53,56,44,32,77,117,98,101,108,111,116,105,120,32,60,109,117,98,101,108,111,116,105,120,64,103,109,97,105,108,46,99,111,109,62,32,97,32,61,13,10,61,67,51,61,65,57,99,114,105,116,32,58,13,10,62,62,62,13,10,62,62,62,62,32,113,115,100,113,13,10,62,62,62,62,13,10,62,62,62,62,32,76,101,32,100,105,109,46,32,50,49,32,106,117,105,110,32,50,48,50,48,32,61,67,51,61,65,48,32,49,54,58,53,54,44,32,77,117,98,101,108,111,116,105,120,32,60,109,117,98,101,108,111,116,105,120,64,103,109,97,105,108,46,99,111,109,62,32,97,32,61,13,10,61,67,51,61,65,57,99,114,105,116,32,58,13,10,62,62,62,62,13,10,62,62,62,62,62,32,122,100,113,100,13,10,62,62,62,62,62,13,10,62,62,62,62,62,32,76,101,32,100,105,109,46,32,50,49,32,106,117,105,110,32,50,48,50,48,32,61,67,51,61,65,48,32,49,54,58,53,52,44,32,77,117,98,101,108,111,116,105,120,32,60,109,117,98,101,108,111,116,105,120,64,103,109,97,105,108,46,99,111,109,62,32,97,13,10,62,62,62,62,62,32,61,67,51,61,65,57,99,114,105,116,32,58,13,10,62,62,62,62,62,13,10,62,62,62,62,62,62,32,72,101,121,13,10,62,62,62,62,62,62,13,10,62,62,62,62,62,13,10,13,10,45,45,48,48,48,48,48,48,48,48,48,48,48,48,55,102,99,97,97,51,48,53,97,56,97,57,57,54,53,101,13,10,67,111,110,116,101,110,116,45,84,121,112,101,58,32,116,101,120,116,47,104,116,109,108,59,32,99,104,97,114,115,101,116,61,34,85,84,70,45,56,34,13,10,67,111,110,116,101,110,116,45,84,114,97,110,115,102,101,114,45,69,110,99,111,100,105,110,103,58,32,113,117,111,116,101,100,45,112,114,105,110,116,97,98,108,101,13,10,13,10,60,100,105,118,32,100,105,114,61,51,68,34,108,116,114,34,62,69,110,99,111,114,101,32,117,110,32,33,60,47,100,105,118,62,60,98,114,62,60,100,105,118,32,99,108,97,115,115,61,51,68,34,103,109,97,105,108,95,113,117,111,116,101,34,62,60,100,105,118,32,100,105,114,61,13,10,61,51,68,34,108,116,114,34,32,99,108,97,115,115,61,51,68,34,103,109,97,105,108,95,97,116,116,114,34,62,76,101,61,67,50,61,65,48,100,105,109,46,32,50,49,32,106,117,105,110,32,50,48,50,48,32,61,67,51,61,65,48,61,67,50,61,65,48,49,55,58,48,56,44,32,61,13,10,77,117,98,101,108,111,116,105,120,32,38,108,116,59,60,97,32,104,114,101,102,61,51,68,34,109,97,105,108,116,111,58,109,117,98,101,108,111,116,105,120,64,103,109,97,105,108,46,99,111,109,34,62,109,117,98,101,108,111,116,105,120,64,103,109,97,105,108,46,99,111,109,60,47,97,61,13,10,62,38,103,116,59,32,97,32,61,67,51,61,65,57,99,114,105,116,61,67,50,61,65,48,58,60,98,114,62,60,47,100,105,118,62,60,98,108,111,99,107,113,117,111,116,101,32,99,108,97,115,115,61,51,68,34,103,109,97,105,108,95,113,117,111,116,101,34,32,115,116,121,108,101,61,13,10,61,51,68,34,109,97,114,103,105,110,58,48,112,120,32,48,112,120,32,48,112,120,32,48,46,56,101,120,59,98,111,114,100,101,114,45,108,101,102,116,58,49,112,120,32,115,111,108,105,100,32,114,103,98,40,50,48,52,44,50,48,52,44,50,48,52,41,59,112,97,100,100,105,110,103,61,13,10,45,108,101,102,116,58,49,101,120,34,62,60,100,105,118,32,100,105,114,61,51,68,34,108,116,114,34,62,66,111,110,106,111,117,114,44,60,100,105,118,62,67,101,99,105,32,101,115,116,32,117,110,32,116,101,115,116,46,60,47,100,105,118,62,60,100,105,118,62,46,60,98,114,62,60,61,13,10,98,114,62,60,47,100,105,118,62,60,47,100,105,118,62,60,98,114,62,60,100,105,118,32,99,108,97,115,115,61,51,68,34,103,109,97,105,108,95,113,117,111,116,101,34,62,60,100,105,118,32,100,105,114,61,51,68,34,108,116,114,34,32,99,108,97,115,115,61,51,68,34,103,109,97,61,13,10,105,108,95,97,116,116,114,34,62,76,101,61,67,50,61,65,48,100,105,109,46,32,50,49,32,106,117,105,110,32,50,48,50,48,32,61,67,51,61,65,48,61,67,50,61,65,48,49,55,58,48,55,44,32,77,117,98,101,108,111,116,105,120,32,38,108,116,59,60,97,32,104,114,101,102,61,13,10,61,51,68,34,109,97,105,108,116,111,58,109,117,98,101,108,111,116,105,120,64,103,109,97,105,108,46,99,111,109,34,32,116,97,114,103,101,116,61,51,68,34,95,98,108,97,110,107,34,62,109,117,98,101,108,111,116,105,120,64,103,109,97,105,108,46,99,111,109,60,47,97,62,38,103,61,13,10,116,59,32,97,32,61,67,51,61,65,57,99,114,105,116,61,67,50,61,65,48,58,60,98,114,62,60,47,100,105,118,62,60,98,108,111,99,107,113,117,111,116,101,32,99,108,97,115,115,61,51,68,34,103,109,97,105,108,95,113,117,111,116,101,34,32,115,116,121,108,101,61,51,68,34,61,13,10,109,97,114,103,105,110,58,48,112,120,32,48,112,120,32,48,112,120,32,48,46,56,101,120,59,98,111,114,100,101,114,45,108,101,102,116,58,49,112,120,32,115,111,108,105,100,32,114,103,98,40,50,48,52,44,50,48,52,44,50,48,52,41,59,112,97,100,100,105,110,103,45,108,101,102,61,13,10,116,58,49,101,120,34,62,60,100,105,118,32,100,105,114,61,51,68,34,108,116,114,34,62,66,111,110,106,111,117,114,44,60,100,105,118,62,74,101,32,115,117,105,115,32,117,110,101,32,111,108,105,118,101,46,60,47,100,105,118,62,60,47,100,105,118,62,60,98,114,62,60,100,105,118,61,13,10,32,99,108,97,115,115,61,51,68,34,103,109,97,105,108,95,113,117,111,116,101,34,62,60,100,105,118,32,100,105,114,61,51,68,34,108,116,114,34,32,99,108,97,115,115,61,51,68,34,103,109,97,105,108,95,97,116,116,114,34,62,76,101,61,67,50,61,65,48,100,105,109,46,32,50,61,13,10,49,32,106,117,105,110,32,50,48,50,48,32,61,67,51,61,65,48,61,67,50,61,65,48,49,54,58,53,56,44,32,77,117,98,101,108,111,116,105,120,32,38,108,116,59,60,97,32,104,114,101,102,61,51,68,34,109,97,105,108,116,111,58,109,117,98,101,108,111,116,105,120,64,103,109,61,13,10,97,105,108,46,99,111,109,34,32,116,97,114,103,101,116,61,51,68,34,95,98,108,97,110,107,34,62,109,117,98,101,108,111,116,105,120,64,103,109,97,105,108,46,99,111,109,60,47,97,62,38,103,116,59,32,97,32,61,67,51,61,65,57,99,114,105,116,61,67,50,61,65,48,58,60,61,13,10,98,114,62,60,47,100,105,118,62,60,98,108,111,99,107,113,117,111,116,101,32,99,108,97,115,115,61,51,68,34,103,109,97,105,108,95,113,117,111,116,101,34,32,115,116,121,108,101,61,51,68,34,109,97,114,103,105,110,58,48,112,120,32,48,112,120,32,48,112,120,32,48,46,56,101,61,13,10,120,59,98,111,114,100,101,114,45,108,101,102,116,58,49,112,120,32,115,111,108,105,100,32,114,103,98,40,50,48,52,44,50,48,52,44,50,48,52,41,59,112,97,100,100,105,110,103,45,108,101,102,116,58,49,101,120,34,62,60,100,105,118,32,100,105,114,61,51,68,34,108,116,114,34,61,13,10,62,113,100,100,122,113,122,100,113,60,47,100,105,118,62,60,98,114,62,60,100,105,118,32,99,108,97,115,115,61,51,68,34,103,109,97,105,108,95,113,117,111,116,101,34,62,60,100,105,118,32,100,105,114,61,51,68,34,108,116,114,34,32,99,108,97,115,115,61,51,68,34,103,109,97,61,13,10,105,108,95,97,116,116,114,34,62,76,101,61,67,50,61,65,48,100,105,109,46,32,50,49,32,106,117,105,110,32,50,48,50,48,32,61,67,51,61,65,48,61,67,50,61,65,48,49,54,58,53,56,44,32,77,117,98,101,108,111,116,105,120,32,38,108,116,59,60,97,32,104,114,101,102,61,13,10,61,51,68,34,109,97,105,108,116,111,58,109,117,98,101,108,111,116,105,120,64,103,109,97,105,108,46,99,111,109,34,32,116,97,114,103,101,116,61,51,68,34,95,98,108,97,110,107,34,62,109,117,98,101,108,111,116,105,120,64,103,109,97,105,108,46,99,111,109,60,47,97,62,38,103,61,13,10,116,59,32,97,32,61,67,51,61,65,57,99,114,105,116,61,67,50,61,65,48,58,60,98,114,62,60,47,100,105,118,62,60,98,108,111,99,107,113,117,111,116,101,32,99,108,97,115,115,61,51,68,34,103,109,97,105,108,95,113,117,111,116,101,34,32,115,116,121,108,101,61,51,68,34,61,13,10,109,97,114,103,105,110,58,48,112,120,32,48,112,120,32,48,112,120,32,48,46,56,101,120,59,98,111,114,100,101,114,45,108,101,102,116,58,49,112,120,32,115,111,108,105,100,32,114,103,98,40,50,48,52,44,50,48,52,44,50,48,52,41,59,112,97,100,100,105,110,103,45,108,101,102,61,13,10,116,58,49,101,120,34,62,60,100,105,118,32,100,105,114,61,51,68,34,108,116,114,34,62,113,115,100,113,60,47,100,105,118,62,60,98,114,62,60,100,105,118,32,99,108,97,115,115,61,51,68,34,103,109,97,105,108,95,113,117,111,116,101,34,62,60,100,105,118,32,100,105,114,61,13,10,61,51,68,34,108,116,114,34,32,99,108,97,115,115,61,51,68,34,103,109,97,105,108,95,97,116,116,114,34,62,76,101,61,67,50,61,65,48,100,105,109,46,32,50,49,32,106,117,105,110,32,50,48,50,48,32,61,67,51,61,65,48,61,67,50,61,65,48,49,54,58,53,54,44,32,61,13,10,77,117,98,101,108,111,116,105,120,32,38,108,116,59,60,97,32,104,114,101,102,61,51,68,34,109,97,105,108,116,111,58,109,117,98,101,108,111,116,105,120,64,103,109,97,105,108,46,99,111,109,34,32,116,97,114,103,101,116,61,51,68,34,95,98,108,97,110,107,34,62,109,117,98,101,61,13,10,108,111,116,105,120,64,103,109,97,105,108,46,99,111,109,60,47,97,62,38,103,116,59,32,97,32,61,67,51,61,65,57,99,114,105,116,61,67,50,61,65,48,58,60,98,114,62,60,47,100,105,118,62,60,98,108,111,99,107,113,117,111,116,101,32,99,108,97,115,115,61,51,68,34,103,61,13,10,109,97,105,108,95,113,117,111,116,101,34,32,115,116,121,108,101,61,51,68,34,109,97,114,103,105,110,58,48,112,120,32,48,112,120,32,48,112,120,32,48,46,56,101,120,59,98,111,114,100,101,114,45,108,101,102,116,58,49,112,120,32,115,111,108,105,100,32,114,103,98,40,50,48,52,61,13,10,44,50,48,52,44,50,48,52,41,59,112,97,100,100,105,110,103,45,108,101,102,116,58,49,101,120,34,62,60,100,105,118,32,100,105,114,61,51,68,34,108,116,114,34,62,122,100,113,100,60,47,100,105,118,62,60,98,114,62,60,100,105,118,32,99,108,97,115,115,61,51,68,34,103,109,61,13,10,97,105,108,95,113,117,111,116,101,34,62,60,100,105,118,32,100,105,114,61,51,68,34,108,116,114,34,32,99,108,97,115,115,61,51,68,34,103,109,97,105,108,95,97,116,116,114,34,62,76,101,61,67,50,61,65,48,100,105,109,46,32,50,49,32,106,117,105,110,32,50,48,50,48,32,61,13,10,61,67,51,61,65,48,61,67,50,61,65,48,49,54,58,53,52,44,32,77,117,98,101,108,111,116,105,120,32,38,108,116,59,60,97,32,104,114,101,102,61,51,68,34,109,97,105,108,116,111,58,109,117,98,101,108,111,116,105,120,64,103,109,97,105,108,46,99,111,109,34,32,116,97,114,61,13,10,103,101,116,61,51,68,34,95,98,108,97,110,107,34,62,109,117,98,101,108,111,116,105,120,64,103,109,97,105,108,46,99,111,109,60,47,97,62,38,103,116,59,32,97,32,61,67,51,61,65,57,99,114,105,116,61,67,50,61,65,48,58,60,98,114,62,60,47,100,105,118,62,60,98,108,61,13,10,111,99,107,113,117,111,116,101,32,99,108,97,115,115,61,51,68,34,103,109,97,105,108,95,113,117,111,116,101,34,32,115,116,121,108,101,61,51,68,34,109,97,114,103,105,110,58,48,112,120,32,48,112,120,32,48,112,120,32,48,46,56,101,120,59,98,111,114,100,101,114,45,108,101,102,61,13,10,116,58,49,112,120,32,115,111,108,105,100,32,114,103,98,40,50,48,52,44,50,48,52,44,50,48,52,41,59,112,97,100,100,105,110,103,45,108,101,102,116,58,49,101,120,34,62,60,100,105,118,32,100,105,114,61,51,68,34,108,116,114,34,62,72,101,121,60,47,100,105,118,62,13,10,60,47,98,108,111,99,107,113,117,111,116,101,62,60,47,100,105,118,62,13,10,60,47,98,108,111,99,107,113,117,111,116,101,62,60,47,100,105,118,62,13,10,60,47,98,108,111,99,107,113,117,111,116,101,62,60,47,100,105,118,62,13,10,60,47,98,108,111,99,107,113,117,111,116,101,62,60,47,100,105,118,62,13,10,60,47,98,108,111,99,107,113,117,111,116,101,62,60,47,100,105,118,62,13,10,60,47,98,108,111,99,107,113,117,111,116,101,62,60,47,100,105,118,62,13,10,13,10,45,45,48,48,48,48,48,48,48,48,48,48,48,48,55,102,99,97,97,51,48,53,97,56,97,57,57,54,53,101,45,45,13,10];
        let body = unsafe {String::from_utf8_unchecked((&headers).to_vec())};
        let body2 = crate::canonicalization::canonicalize_body_relaxed(MAIL.to_string());


        println!("{}", diff_chars(&body, &body2));
        assert_eq!(body.len(), body2.len());
    }

    #[test]
    fn go_comp_hash() {
        let h = base64::encode(vec![113,143,43,206,203,247,83,49,66,163,89,50,198,225,234,190,103,1,102,172,31,210,244,239,144,141,64,70,99,165,44,130]);
        let sig = base64::encode(vec![172,124,52,109,163,122,74,69,33,226,168,158,219,127,219,243,144,200,6,88,146,71,111,78,141,204,228,64,99,13,22,8,240,106,250,46,85,226,86,247,130,190,33,75,17,152,125,186,224,139,222,100,233,88,62,39,213,7,253,89,52,51,165,153,164,78,14,219,8,229,160,200,100,49,11,83,197,241,125,32,31,223,249,234,182,147,128,195,17,94,105,82,244,48,91,113,104,172,136,220,46,21,110,173,41,81,21,46,230,147,203,65,114,195,254,189,77,238,161,232,204,89,150,180,207,253,174,146,228,49,24,178,166,139,239,25,244,168,233,104,113,102,116,255,66,80,251,102,124,20,97,1,135,78,63,207,91,246,33,34,59,134,219,188,185,185,19,167,156,153,55,98,91,133,73,34,94,2,216,27,105,254,70,209,176,163,54,50,193,37,106,212,165,136,141,123,156,132,31,124,34,206,144,73,233,64,179,152,9,130,59,155,157,82,134,37,7,27,0,143,211,22,52,213,226,19,76,161,177,224,112,72,198,81,158,180,115,246,77,152,18,78,169,84,5,154,233,220,97,161,91,178,104,245,150,121]);
        println!("{:?}", h);
        println!("{:?}", sig);
    }*/
}