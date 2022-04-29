# Named Booleans

Booleans are pretty simple, a bool can only ever be one of two values, yet I still find they are a disproportionate source of confusion, misunderstanding, and bugs.

Why?

## Booleans are bad

When literals are used as method args, it requires the reader to either know the method signature or go check it to be certain. When bool is used as a return type, it again requires readers of the call site to either know the method or go check the method doc to understand what true means.

```rust
fn main() {
    // Method takes bool as arg
    let client = my_client::connect(
        "github.com",
        true,
        false,
    );

    // Method returns bool to caller
    let mut my_task = MyTask::new();
    let state_changed = my_task.run();
    if state_changed {
        println!("Running task resulted in state change");
    }
}
```

Some appreciation of this problem comes from recognizing that we read code far more often than we write it. And it is easy to underestimate how often we're reading code *not in an IDE* (e.g. StackOverflow, Github, peer code review). At my job, I spend a considerable amount of time reading code in a web browser. Having to check another file means having to open another page/tab, and sometimes it requires multiple page hops if I need to search to find the method definition that I want to understand more. Heck, even searching for something on the same page requires to interrupt my concious flow and put another item on my cognitive stack that needs to be popped.

Even if you're writing code in an IDE, it requires the best intention of the author to read the method signature and doc and do the right thing with that information.

So how can we make it better?

## Booleans, but better

### Labels on literals

To relieve potential misunderstandings of literals as method arg, we can introduce a team convention where all literals should have an in-line comment label.

```rust
let client = my_client::connect(
    "github.com",
    /* enable TLS */ true,
    /* enable Nagle's Algorithm*/ false,
);
```

Voila! Problem solved. Well... Kind of. It relies on best intentions of human programmers, which will simply never be as reliable as some type of programmatic enforcement. So how can we do better?

### Separate methods

If we're in the seat of designing the method signature (as opposed to only the caller), then one of the obvious approaches to avoid "boolean hell" and a lot of branching in the method implementation is to split your API into two separate methods, where one method is the complete logic of the `true` branch and one method is the complete logic of the `false` branch.

```rust
let client = my_client::connect_tls("github.com");
```

In practice, I have found this can work well, situationally, but there will still be times where either there are too many bool params resulting in a combinatorial amount of methods or it just doesn't make logical sense to split into multiple methods.

### Create enum types

Like this

```rust
pub enum SecurityMode {
    None,
    Tls,
}

pub enum NaglesAlgorithm {
    Enabled,
    Disabled,
}

pub fn connect(domain: impl Into<String>, security_mode: SecurityMode, nagles_algorithm) -> Client {
    // ...
}

//---------------------

let client = my_client::connect(
    "github.com",
    SecurityMode::Tls,
    NaglesAlgorithm::Disabled,
);
```

My oh my, we got a beauty on our hands. Look at that. You will now force every caller to explicitly specify, in unconfusing, unmistakable, human-readable text without have to check and double check the documentation that they got a booleans "true" case correctly.

#### Zero cost abstraction

Sounds like a buzz word. What does it mean? In this case, it means we can achieve this higher level abstraction to improve developer experience **without any additional runtime cost**.

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

#### As a return type

Force caller to explicitly match on returned value.

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

Okay, it's a bit more verbose, but it's unmistakeable for the caller to use.

## Named Options

I don't think this is as important to do all of the time, but I like to apply the same thinking to `Option`. A lot of times, a var name and `Some` or `None` can still require some double takes and extra cognitive overhead.

We can give explicit names to `Some` and `None` by defining our own enum. I find this more helpful when existing as a field on a struct or inside an enum variant, rather than appearing in a method signature.

```rust
struct WorkScheduler {
    // Shutdown task that tracks if we've started the background shutdown task.
    //
    // None: Shutdown task has not yet started.
    // Some: Shutdown task is running.
    shutdown_task: Option<JoinHandle<()>>,

    // ...
}

//---------------------

enum ShutdownTask {
    NotStarted,
    Running(oneshot::Receiver<()>),
}

struct WorkScheduler {
    shutdown_task: ShutdownTask

    // ...
}
```

It may very well be subjective, but I prefer the 2nd way where the code entirely represents the ideas, and nearly 0 documentation is required to be written by author or read by consumer.

## Cost of conventions

Every fancy abstraction that you introduce has a cost. A small conceptual cost that new readers and maintainers will have to learn, on top of the many other bits of knowledge they must acquire to be effective in a code base. There is a certain charm of writing the simplest possible code using standard lib.

The point of this post is to make people aware of this choice and give people the option to consider using in some situations. As always, situational judgement is required.

Good luck!
