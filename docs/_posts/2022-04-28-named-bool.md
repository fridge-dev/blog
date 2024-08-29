---
layout: post
title: Named Booleans
---

Booleans are pretty simple. A `bool` can only ever be one of two values, yet I still find they are a disproportionate source of confusion, misunderstanding, and bugs.

Why?

## Booleans aren't self describing

When a caller uses `bool` literals as method args, it requires the reader to either know the method being called or go check the method signature to be certain. When `bool` is used as a return type, it again requires readers of the call site to either know the method or go check the method doc to understand what `true` or `false` means.

```rust
// Method takes bool as arg
let client = my_client::connect(
    "github.com",
    true,
    false,
);
```

```rust
// Method returns bool to caller
let mut my_task = MyTask::new();
let state_changed = my_task.run();
if state_changed {
    println!("Running task resulted in state change");
}
```

Some appreciation of this problem comes from recognizing that we read code far more often than we write it- and it is easy to underestimate how often we're reading code *not in an IDE* (e.g. StackOverflow, Github, peer code review). At my job, I spend a considerable amount of time reading code in a web browser. Having to check another file can mean having to open another page/tab, and sometimes it requires multiple page hops if I need to search to find the method definition that I want to understand more. Even searching for something on the same page requires me to interrupt my concious flow and put another item on my cognitive stack.

If you're writing code in an IDE, it still requires the best intention of the author to read the method signature and doc and do the right thing with that information.

So how can we make it better?

## Booleans, but better

### 1. Labels on literals

We can introduce a team convention where all literals should have an in-line comment label.

```rust
let client = my_client::connect(
    "github.com",
    /* enable TLS */ true,
    /* enable Nagle's Algorithm */ false,
);
```

This relies on best intentions of humans, which will never be as reliable as some type of programmatic enforcement. Let's do better.

### 2. Separate methods

One of the obvious approaches to avoid "boolean hell" and a lot of branching in the method implementation is to split your API into two separate methods, where one method is the complete logic of the `true` branch and one method is the complete logic of the `false` branch.

```rust
let client = my_client::connect_tls("github.com");
```

In practice, I have found this can situationally work well, but there will inevitably be situations where either there are too many bool params resulting in a combinatorial amount of methods or it doesn't make logical sense to split into multiple methods.

### 3. New-type wrapper

We can use rust's [new-type idiom](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) to wrap the bool and give some named methods for accessing the underlying value.

```rust
pub struct UseTls(bool);

impl UseTls {
    pub fn use_tls() -> Self { UseTls(true) }

    pub fn no_tls() -> Self { UseTls(false) }

    pub fn should_use_tls(&self) -> bool {
        *self.0
    }
}

pub fn connect(
    domain: impl Into<String>,
    use_tls: UseTls,
    // ...
) -> Client {
    if use_tls.should_use_tls() {
        // ...
    } else {
        // ...
    }

    // ...
}

//---------------------

let client = my_client::connect(
    "github.com",
    UseTls::use_tls(),
    // ...
);
```

This pattern works well, but it requires some discipline from programmers to not get lazy and fallback to using a raw `bool` when it's not the right layer of abstraction to do so, which in my experience happens a non-zero amount of time.

### 4. Named boolean enum types

My preferred solution is to model a `bool` as an enum with 2 variants. This way, `true` and `false` each get a more specific name. It's a "named boolean".

```rust
pub enum SecurityMode {
    None,
    Tls,
}

pub enum NaglesAlgorithm {
    Disabled,
    Enabled,
}

pub fn connect(
    domain: impl Into<String>,
    security_mode: SecurityMode,
    nagles_algorithm: NaglesAlgorithm,
) -> Client {
    // ...
}

//---------------------

let client = my_client::connect(
    "github.com",
    SecurityMode::Tls,
    NaglesAlgorithm::Disabled,
);
```

Beautiful! You will now force every caller to explicitly specify their method args in unconfusing, unmistakable, human-readable text without having to check and double check the documentation that they chose the correct `true` or `false`.

#### Named boolean return type

Here's the second confusing code snippet from the first section.

```rust
// Method returns bool to caller
let mut my_task = MyTask::new();
let state_changed = my_task.run();
if state_changed {
    println!("Running task resulted in state change");
}
```

We can apply the same pattern to return types and force the caller to explicitly match on the returned value.

```rust
pub struct MyTask { /* ... */ }

pub enum RunOutcome {
    StateChanged,
    NoChange,
}

impl MyTask {
    pub fn new() -> Self { /* ... */ }

    pub fn run(&mut self) -> RunOutcome {
        // ...
    }
}

//---------------------

let mut my_task = MyTask::new();
let outcome = my_task.run();
if matches!(outcome, RunOutcome::StateChanged) {
    println!("Running task resulted in state change");
}
```

Okay, so it's a bit more verbose, but it's unmistakeable for the caller to use.

## Named Options

I also like to apply the Named Boolean pattern to `Option`s. A lot of times, a var name and a `Some` or `None` value can still cause a double-take or some extra cognitive overhead.

We can give explicit names to the `Some` and `None` variants by defining our own enum. I find this more helpful as a field within a struct or inside an enum variant, but it could still be helpful in a method signature.

```rust
// Using Option
struct WorkScheduler {
    // This tracks if we've started the background nanny task.
    //
    // None: Nanny task has not yet started.
    // Some: Nanny task is running.
    nanny_task: Option<JoinHandle<()>>,

    // ...
}
```

```rust
// Using named-Option
struct WorkScheduler {
    nanny_task: NannyTask

    // ...
}

enum NannyTask {
    NotStarted,
    Running(JoinHandle<()>),
}
```

It is subjective and situational, but I generally prefer the 2nd way where the code concisely describes the state-space. It requires nearly 0 documentation to be written by author or read by consumer. And you avoid misunderstandings where `None` could be (mis-)interpretted in multiple ways.

## Cost of Abstractions

### Runtime cost

This improves developer experience at the cost of 0 memory overhead and a few extra CPU instructions when doing a `match` on the enum compared to a plain old `if <bool>`.

* [rust playground](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=b24608a5f6d68de594760da6db36611d) showing memory overhead
* [godbolt compiled machine code](https://godbolt.org/z/Y3b5r5v7n) showing CPU overhead

### Cognitive cost

Every fancy abstraction or new concept that you introduce has a cost. A small cognitive cost that new readers and maintainers will have to learn, on top of the many other bits of knowledge they must acquire to be effective in a code base. There is a certain charm of writing the code using only standard lib and minimal dependencies/abstractions.

## Conclusion

The point of this post is to make people aware of this choice and give people some options to think about. I generally prefer the Named Boolean pattern, but I'm still open to other options including just using the raw `bool` if it's simple enough. As always, situational judgement is required. Discuss these ideas with your team to see what ya'll think.

Good luck!
