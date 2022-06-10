codebake (working title) is a toy data-processing framework and lisp language inspired by [Cyberchef](https://gchq.github.io/CyberChef/). it's still got a *long* way to come, so bear with us!

# usage
the principle object in codebake is the *Dish*, which is just a container of data that is manipulated by *operations*. right now, there are only two operations in codebake: `rot13` and `reverse`. codebake is currently usable only from the lisp.

**example:**
```
codebake> (def my-dish (dish "hello world!"))
my-dish
codebake> (def my-recipe (recipe (rot13 13) reverse))
my-recipe
codebake> (bake my-recipe my-dish)
Dish("!qyebj byyru")
codebake> (bake my-recipe my-dish)
Dish("hello world!")
```

**explanation:**
* the `dish` function creates a dish out of raw data. currently, dishes can only be created out of strings.
* the `recipe` function creates a recipe (just a list of functions that operate on dishes) out of its arguments. a recipe is applied **in-order**. that is, the recipe `(recipe (rot13 13) reverse)` will apply the `rot13` operation before applying `reverse`.
* the `bake` function applies a recipe to a `Dish`.

# building
codebake follows the standard `cargo build` procedure!
