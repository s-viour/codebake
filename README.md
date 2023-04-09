codebake (working title) is a toy data-processing framework and lisp language inspired by [Cyberchef](https://gchq.github.io/CyberChef/). it's still got a *long* way to come, so bear with us! additionally, we have a [wiki](https://github.com/s-viour/codebake/wiki) containing a tutorial and reference documentation for working with codebake.


# usage
the principle object in codebake is the *Dish*, which is just a **mutable** container of data that is manipulated by *operations*. for a list of all operations implemented in codebake, check the [Operation Reference](https://github.com/s-viour/codebake/wiki/Operation-Reference)


## the lisp
the embedded lisp is currently the primary (and only) way of using codebake. there are plans to build a webapp similar to [Cyberchef](https://gchq.github.io/CyberChef/) soon, but for now, the lisp is how you use codebake. here's an example:
```lisp
codebake> (def my-dish d"hello world!")
my-dish
codebake> (def my-recipe (recipe (rot13 13) reverse to-base64))
my-recipe
codebake> (def undo-my-recipe (recipe from-base64 reverse (rot13 13)))
undo-my-recipe
codebake> (bake my-recipe my-dish)
Dish("IXF5ZWJqIGJ5eXJ1")
codebake> (bake undo-my-recipe :ans)
Dish("hello world!")
```

note that:
* the literal `d"hello world!"` isn't a regular string, it's a *dish literal* which creates a `Dish`.
* the `recipe` function creates a recipe (just a list of functions that operate on dishes) out of its arguments. a recipe is applied **in-order**. that is, the recipe `(recipe (rot13 13) reverse)` will apply the `rot13` operation before applying `reverse`.
* the `bake` function applies a recipe to a `Dish`.
* the `:ans` symbol is always defined and is the last *successful* result from the interpreter.

the `bake` and `recipe` functions are implemented for convenience, but applying operations directly to dishes is perfectly viable too. additionally, the lisp supports `lambda` and `defn` for defining functions. here's an example demonstrating all this:
```lisp
codebake> (defn rot-reverse (n text) (reverse ((rot13 n) (dish text))))
rot-reverse
codebake> (rot-reverse 13 "hello world!")
Dish("!qyebj byyru")
```

for more information on the Lisp, see the [Lisp Reference](https://github.com/s-viour/codebake/wiki/Lisp-Reference).

## the web interpreter
the core codebake project can be compiled to [WASM](https://webassembly.org/) to run in the web. this lets us embed the lisp in a browser. we'd like to incorporate this into a [Cyberchef](https://gchq.github.io/CyberChef/)-like scripting environment that runs in the browser alongside the webapp, but we're still very far off from that. a demo of the web interpreter is available here though: https://saviour.dev/0013


# repository structure
codebake is a [cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) containing all of the main codebake projects. currently, there is: 
* [codebake](/codebake) - the main codebake project. this contains the codebake API, the operations themselves, and the lisp interpreter.
* [web-interpreter](/web-interpreter) - a [Yew](https://yew.rs/) project containing a (very) simple app that embeds the lisp interpreter in the browser.
* [wiki-updater](/wiki-updater/) - a custom github action that updates the [wiki](https://github.com/s-viour/codebake/wiki) automatically on push. 

we intend to add a 3rd project, the webapp, to this workspace soon.

# contributing
contributions are welcome! check out the [CONTRIBUTING](/CONTRIBUTING.md) file to learn more.


# building
codebake follows the standard `cargo build` procedure!
