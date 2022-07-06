//! This file contains some functions which can actually be defined
//! *within* the lisp. These are all just strings which are evaluated
//! inside the lisp before control is given to the user.
//! 

pub static FUNCTIONS_NONNATIVE: &[&'static str] = &[LISP_MAP];

static LISP_MAP: &'static str = "
(defn map (f lis)
  (if (empty lis)
    (quote ())
    (cons (f (head lis)) (map f (rest lis)))))
";
