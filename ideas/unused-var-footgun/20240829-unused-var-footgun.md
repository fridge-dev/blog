---
layout: post
title: Specify types of unused vars
---

Let's look at a sneaky footgun in rust.

## Some innocent code

Suppose you have a method from another mod or crate that looks like this.

```rust
async fn try_call_api(/* ... */) -> Result<(), MyError> {
    // ...
}
```

...and then we have the code that we're writing...

```rust
async fn do_some_cool_stuff(/* ... */) {
    // ...

    // Calling this method is a best-effort. We don't care
    // if it returns an error.
    let _ = try_call_api(/* ... */);

    // ...

    other_async_stuff(/* ... */).await;

    // ...
}
```

This code looks innocent enough. It even has a nice comment explaining why we're ignoring a Result. ... But do you see the bug?

Take a few seconds to try to appreciate what's going on. Here's a [rust playground](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=2338dddcd6595b6957648c81f0d2757c) with the buggy code.

## Problem

The problem is we don't `await` the future returned by `try_call_api()`. We're not ignoring the `Result`, we're ignoring the `Future`. Remember, `Future` is lazily evaluated and doesn't do work unless polled [see rustdoc](https://doc.rust-lang.org/std/future/trait.Future.html#runtime-characteristics), so the code within `try_call_api()` will never actually run.

*Aside: If you're unfamiliar with `async`/`await` in rust, I'd strongly recommend reading the first 2-3 chapters of https://rust-lang.github.io/async-book . It is well worth your time.*

### Risk of changing `fn -> Result` to `async fn -> Result`

An alteration of the example buggy code is if the following code exists and works as intended...

```rust
fn call_api() -> Result<(), MyError> { /* ... */ }

fn my_code(/* ... */) {
    // ...
    // ignore Err
    let _ = call_api(/* ... */);
    // ...
}
```

...and we change `fn call_api` to be `async fn call_api`.

We'll see below how to prevent both of these problems.

### Real world problem

This problem might seem easy to catch with reasonable tests, and I completely agree. But we've all been there where we write hundreds, potentially thousand(s) of lines of code (gulp) and only attempt running the tests at the end. Bug hunting in a large diff can easily take up multiple hours!

I'd argue that- assuming you have reasonable tests- our problem is how to speed up potential hours of bug hunting into a few seconds.

## Solution

You can fix the bug and generally prevent these mistakes by always specifying the type of unused variables.

Let's update our code.

```rust
// Attempt 1 - bug
let _ = try_call_api();

// Attempt 2 - specify the type and you get compilation error 'expected `Result<_, _>`, found future'
let _: Result<_, _> = try_call_api();

// Attempt 3 - fixed
let _: Result<_, _> = try_call_api().await;
```

The resulting code looks a little ugly, but it saves you from a painful, hard to find bug. This is probably too contentious to ever be included as a standard clippy lint, but consider enforcing this in your team's conventions.

Alternatively, during code reviews, you should be on high alert of every `_` or `_result` to double check that the method being called is not `async`, AND that any time you change `fn ... -> Result<>` to `async fn ... -> Result<>` to double check each and every caller.
