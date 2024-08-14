# Named Booleans

Booleans are pretty simple. A `bool` can only ever be one of two values, yet I still find they are a disproportionate source of confusion, misunderstanding, and bugs.

Why?

## Booleans are bad

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

Some appreciation of this problem comes from recognizing that we read code far more often than we write it- and it is easy to underestimate how often we're reading code *not in an IDE* (e.g. StackOverflow, Github, peer code review). At my job, I spend a considerable amount of time reading code in a web browser. Having to check another file means having to open another page/tab, and sometimes it requires multiple page hops if I need to search to find the method definition that I want to understand more. Even searching for something on the same page requires me to interrupt my concious flow and put another item on my cognitive stack that needs to be popped.

If you're writing code in an IDE, it still requires the best intention of the author to read the method signature and doc and do the right thing with that information.

So how can we make it better?

## Booleans, but better

### Labels on literals

To relieve potential misunderstandings of literals as method arg, we can introduce a team convention where all literals should have an in-line comment label.

```rust
let client = my_client::connect(
    "github.com",
    /* enable TLS */ true,
    /* enable Nagle's Algorithm */ false,
);
```

Voila! Problem solved. Well... Kind of. It relies on best intentions of human programmers, which will simply never be as reliable as some type of programmatic enforcement. So how can we do better?

### Separate methods

One of the obvious approaches to avoid "boolean hell" and a lot of branching in the method implementation is to split your API into two separate methods, where one method is the complete logic of the `true` branch and one method is the complete logic of the `false` branch.

```rust
let client = my_client::connect_tls("github.com");
```

In practice, I have found this can situationally work well, but there will still be times where either there are too many bool params resulting in a combinatorial amount of methods or it simply doesn't make logical sense to split into multiple methods.

### Create enum types

My preferred solution is to model a `bool` as an enum with 2 variants. This way, each variant gets a name.

```rust
pub enum SecurityMode {
    None,
    Tls,
}

pub enum NaglesAlgorithm {
    Enabled,
    Disabled,
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

My oh my, we got a beauty on our hands. Look at that. You will now force every caller to explicitly specify their method args in unconfusing, unmistakable, human-readable text without have to check and double check the documentation that they chose the correct `true` or `false`.

#### Zero cost abstraction

Sounds like a buzz word. What does it mean? In this case, it means we can achieve this higher level abstraction to improve developer experience **without any additional runtime cost**.

TODO: use godbolt to compare compiled code

```rust
pub enum SecurityMode {
    None,
    Tls,
}

fn main() {
    // prints "1 byte(s)"
    println!("{} byte(s)", std::mem::size_of::<SecurityMode>());

    // prints "1 byte(s)"
    println!("{} byte(s)", std::mem::size_of::<bool>());
}
```
([playground](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=b24608a5f6d68de594760da6db36611d))

#### Enum types as a return type

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

I don't think this is as important to do all of the time, but I like to apply the same thinking to `Option`. A lot of times, a var name and `Some` or `None` can still cause a double-take or some extra cognitive overhead.

We can give explicit names to the `Some` and `None` variants by defining our own enum. I find this more helpful when existing as a field within a struct or inside an enum variant, but it could still be helpful in a method signature.

```rust
// Using Option
struct WorkScheduler {
    // Shutdown task that tracks if we've started the background shutdown task.
    //
    // None: Shutdown task has not yet started.
    // Some: Shutdown task is running.
    shutdown_task: Option<JoinHandle<()>>,

    // ...
}
```

```rust
// Using named-Option
struct WorkScheduler {
    shutdown_task: ShutdownTask

    // ...
}

enum ShutdownTask {
    NotStarted,
    Running(JoinHandle<()>),
}
```

It is subjective and situational, but I generally prefer the 2nd way where the code concisely describes the state-space, and nearly 0 documentation is required to be written by author or read by consumer.

## Cost of conventions

Every fancy abstraction that you introduce has a cost. A small conceptual cost that new readers and maintainers will have to learn, on top of the many other bits of knowledge they must acquire to be effective in a code base. There is a certain charm of writing the simplest possible code using standard lib.

The point of this post is to make people aware of this choice and give people the option to consider using in some situations. As always, situational judgement is required.

Good luck!
