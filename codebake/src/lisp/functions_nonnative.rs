//! This file contains some functions which can actually be defined
//! *within* the lisp. These are all just strings which are evaluated
//! inside the lisp before control is given to the user.
//!

pub static FUNCTIONS_NONNATIVE: &[&'static str] = &[LISP_MAP, LISP_REDUCE];

static LISP_MAP: &'static str = "
(defn map (f lis)
  (if (empty? lis)
    (quote ())
    (cons (f (first lis)) (map f (rest lis)))))
";

static LISP_REDUCE: &'static str = "
(defn reduce (f acc lis)
  (if (empty? lis)
    acc
    (f (first lis) (reduce f acc (rest lis)))))
";
