thank you so much for your interest in contributing to codebake! there are loads of things you can do to contribute to the project:


# creating issues
filing issues is the easiest way to contribute. this means reporting bugs, suggesting new operations, finding ways to improve current operations, and reporting fixes for documentation. to view all issues, navigate to the [issues page](https://github.com/s-viour/codebake/issues). to create an issue, click [here](https://github.com/s-viour/codebake/issues/new) to go straight to the new issue page. be sure to tag your issue accordingly!


# contributing code
if you intend to contribute code, [fork](https://github.com/s-viour/codebake/fork) the repository and follow [github flow](https://docs.github.com/en/get-started/quickstart/github-flow). additionally, if you're working on an issue, please comment on or create the issue that you intend to fix it so we can assign you to it. if you're contributing to operations, please read the [working with operations](#working-with-operations) section below.


# working with operations
codebake is built of operations, so adding new ones and improving current ones is incredibly helpful (and welcome!). operations live in the [ops](/codebake/src/ops) directory, and the `mod.rs` file in this folder explains much of the coding process for adding new operations. operations are categorized into different files based on what they do. currently there's only two categories: `data format` and `textual`. these categories are reflected in the [data_format.rs](/codebake/src/ops/data_format.rs) and [textual.rs](/codebake/src/ops/textual.rs) files.
