codebake (working title) is a toy data-processing framework and lisp language inspired by [Cyberchef](https://gchq.github.io/CyberChef/). it's still got a *long* way to come, so bear with us!


# usage
the principle object in codebake is the *Dish*, which is just a **mutable** container of data that is manipulated by *operations*. currently, the following basic operations are implemented in codebake:
* `rot13`
* `reverse`
* `to-base64` and `from-base64`
* `to-decimal` and `from-decimal`
* `to-octal` and `from-octal`
* `to-hex` and `from-hex`


## the lisp
---
**note:** currently, the lisp reads line-by-line and will not look for terminating parenthesis on more lines. that means that code like this won't work:
```clj
(defn my-function (x)
    ((rot13 x) (dish "hello world)))
```
sorry! it'll be fixed soon.


the embedded lisp is currently the primary (and only) way of using codebake. there are plans to build a webapp similar to [Cyberchef](https://gchq.github.io/CyberChef/) soon, but for now, the lisp is how you use codebake. here's an example:
```clj
codebake> (def my-dish (dish "hello world!"))
my-dish
codebake> (def my-recipe (recipe (rot13 13) reverse))
my-recipe
codebake> (bake my-recipe my-dish)
Dish("!qyebj byyru")
codebake> (bake my-recipe my-dish)
Dish("hello world!")
```

note that:
* the `dish` function creates a dish out of raw data. currently, dishes can only be created out of strings.
* the `recipe` function creates a recipe (just a list of functions that operate on dishes) out of its arguments. a recipe is applied **in-order**. that is, the recipe `(recipe (rot13 13) reverse)` will apply the `rot13` operation before applying `reverse`.
* the `bake` function applies a recipe to a `Dish`.

the `bake` and `recipe` functions are implemented for convenience, but applying operations directly to dishes is perfectly viable too. additionally, the lisp supports `lambda` and `defn` for defining functions. here's an example demonstrating all this:
```clj
codebake> (defn rot-reverse (n text) (reverse ((rot13 n) (dish text))))
rot-reverse
codebake> (rot-reverse 13 "hello world!")
Dish("!qyebj byyru")
```

## the web interpreter
---
the core codebake project can be compiled to [WASM](https://webassembly.org/) to run in the web. this lets us embed the lisp in a browser. we'd like to incorporate this into a [Cyberchef](https://gchq.github.io/CyberChef/)-like scripting environment that runs in the browser alongside the webapp, but we're still very far off from that. a demo of the web interpreter is available here though: https://saviour.dev/0013


# repository structure
codebake is a [cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) containing all of the main codebake projects. currently, there is: 
* [codebake](/codebake) - the main codebake project. this contains the codebake API, the operations themselves, and the lisp interpreter.
* [web-interpreter](/web-interpreter) - a [Yew](https://yew.rs/) project containing a (very) simple app that embeds the lisp interpreter in the browser.

we intend to add a 3rd project, the webapp, to this workspace soon.

# contributing
contributions are welcome! check out the [CONTRIBUTING](/CONTRIBUTING.md) file to learn more.


# building
codebake follows the standard `cargo build` procedure!
