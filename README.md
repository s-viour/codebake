codebake (working title) is a toy data-processing framework and lisp language inspired by [Cyberchef](https://gchq.github.io/CyberChef/). it's still got a *long* way to come, so bear with us!

# usage
the principle object in codebake is the *Dish*, which is just a container of data that is manipulated by *operations*. right now, there are only two operations in codebake: `rot13` and `reverse`. codebake is currently usable only from the lisp. example:
```
; (def mine (dish "hello world"))
mine
; ((rot13 13) ((reverse) mine))
Dish("qyebj byyru")
; ((rot13 13) ((reverse) mine))
Dish("hello world")
```

explanation: `dish` is a function that constructs a `Dish` from a string. the `rot13` function takes a number and generates a new function that rotates a `Dish` by that number. `reverse` is a function that generates a new function that reverses the contents of a `Dish`. 

# building
codebake follows the standard `cargo build` procedure; however, we use **rust nightly** since codebake requires [RFC 2132](https://github.com/rust-lang/rust/issues/44490) for closure-copying functionality.
