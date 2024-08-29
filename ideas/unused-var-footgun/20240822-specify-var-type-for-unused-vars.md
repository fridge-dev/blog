Let's look at a sneaky footgun in rust.

## Some innocent code

Suppose you have a method from another mod or crate that looks like this.

```rust
async fn call_api(/* ... */) -> Result<(), MyError> {
     /* ... */
}
```

...and then we have the code that we're writing...

```rust
async fn do_some_cool_stuff(/* ... */) {
    // ... my code ...

    // Calling this method is optional and is a best-effort.
    // We don't care if it errors, so discard it.
    let _ = call_api();

    // ... my code ...
}
```

This code looks innocent enough. It even has a nice comment explaining why we're ignoring the returned result! Do you see the bug?

Take a few seconds to try to appreciate what's going on.

## Problem

The problem is we don't `await` the future returned by `call_api()`. We're not ignoring the `Result`, we're ignoring the `Future`. Remember, `Future` is lazily evaluated and doesn't do work unless polled[1], so the code within `call_api()` will never actually run.

[1] TODO link to Futures documentation.

## Solution

You can fix the bug and generally prevent these mistakes by always specifying the type of unused variables.

Let's update our code.

```rust
// Attempt 1 - bug
let _ = call_api();

// Attempt 2 - specify the type and you get compilation error 'expected `Result<_, _>`, found future'
let _: Result<_, _> = call_api();

// Attempt 3 - fixed
let _: Result<_, _> = call_api().await;
```

The resulting code looks a little ugly, but it saves you from a painful, hard to find bug. This is probably too contentious to ever be included as a standard clippy lint, but consider enforcing this in your code base's conventions.

Alternatively, during code reviews, you should be on high alert whenever you see `_` or `_result` that the method being called is not `async`.

### `Result<_, _>` short hand TODO consider omitting

If this is the first time you've seen `_` used as a rust generic, you can think of `Result<_, _>` in our example as equivalent to `Result<(), MyError>`. Rust allows you to partially specify generic types. For this practice, we don't really care what the inner generic types are, and the compiler can infer it, so we don't have to write it. You'll commonly see this in places like calling `.collect()` and collecting into a `Vec<_>` or a `VecDeque<_>`.

You can read more about it in TODO link rust doc
