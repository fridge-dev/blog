---
layout: post
title: Understandability over DRY
---

Let me get my contentious opinion out of the way right away: I think [DRY](https://en.wikipedia.org/wiki/Don%27t_repeat_yourself) is an over-valued software philosophy. \*gasp!\*

After ranting about this to my team, I wanted to write out my thoughts on cases when DRY can be sacrificed. I think the real risk is blindly following DRY without using higher brain power to see if it actually helps code understandability or not. 

I've since stumbled upon a couple of posts that explain my mentality in a much more concise and coherent way. I'll leave my drafted rant here, but you should really read the 2 posts below.

> "you shouldn't be dogmatic about when you start writing abstractions but instead write the abstraction when it feels right and don't be afraid to duplicate code until you get there."
> \- [https://kentcdodds.com/blog/aha-programming]

> "duplication is far cheaper than the wrong abstraction"
> \- [https://sandimetz.com/blog/2016/1/20/the-wrong-abstraction]

### Struct method vs mod function

If you have a data struct, `MyData`, and you have some other module with business logic to evaluate that data, should you write a struct method `impl MyData { fn method(&self, ...) { ... } }` or a mod level function `fn method(my_data: &MyData, ...) { ... }`?

I see so often people choosing the former, which I assume is out of premature optimization to not want to duplicate the code in case the same method is needed in a different mod later.

I prefer the latter. The module with business logic is the reason we're writing a new function. Even if the function may be generally applicable to the data struct, it's the context of our module that dictates the new method's behavior. IMO writing the new behavior as a mod level function is easy to understand when reading both the struct and the mod's code, and you avoid bloating the data struct's public method signature with the union of all possible interpretations of the data.

### Example code 

Example adding method on MyError:

```rust
// rpc_types.rs
pub(crate) enum CreateResourceError {
    InvalidRequest,
    ServiceUnavailable,
    ServiceInternalError,
    Timeout,
    ClientSideCircuitBreaker,
}

impl MyError {
    pub(crate) fn is_retryable(&self) -> bool {
        // ...
    }
    
    pub(crate) fn should_trigger_circuit_breaker(&self) -> bool {
        // ...
    }
}

// api_call.rs
fn create_resource(/* ... */) -> Result<Response, CreateResourceError> {
    loop {
        match client.create_resource(request) {
            Ok(ok) => return Ok(ok),
            Err(err) => {
                if err.should_trigger_circuit_breaker() {
                    client.count_err_towards_circuit_breaker();
                }
                if !err.is_retryable() {
                    return Err(err);
                }
            }
        } 
    }
}
```

Example adding method in mod:

```rust
// rpc_types.rs
pub(crate) enum CreateResourceError {
    InvalidRequest,
    ServiceUnavailable,
    ServiceInternalError,
    Timeout,
    ClientSideCircuitBreaker,
}

// api_call.rs
fn create_resource(/* ... */) -> Result<Response, CreateResourceError> {
    loop {
        match client.create_resource(request) {
            Ok(ok) => return Ok(ok),
            Err(err) => {
                if should_trigger_circuit_breaker(&err) {
                    client.count_err_towards_circuit_breaker();
                }
                if !is_retryable(&err) {
                    return Err(err);
                }
            }
        } 
    }
}

fn is_retryable(err: &CreateResourceError) -> bool {
    // ...
}

fn should_trigger_circuit_breaker(err: &CreateResourceError) -> bool {
    // ...
}
```

The difference is subtle, but I think the latter style is way easier to understand the flow of code and to maintain over time. I have seen similar code where the logic of circuit breaking and retries is nuanced, and evolved iteratively over time. Having overly specific business logic in the data struct (or enum in this example) was confusing and required navigating to a different crate to view, and the concept of retryable was not re-used elsewhere.

### When to DRY?

Philosophically, I don't think DRY be the thing to optimize for. It's okay to potentially have small code snippets scoped duplicated within an entire large project. I think DRY's goal should be to de-duplicate small code snippets within the same module or large code snippets within a project.

Understandability trumps all.
