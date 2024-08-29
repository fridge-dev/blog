struct MyError;

async fn call_api(c: &mut usize) -> Result<(), MyError> {
    *c += 1;
    Ok(())
}

async fn do_some_cool_stuff(c: &mut usize) {
    let _: Result<_, _> = call_api(c);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn asdf() {
        let mut a = 1;
        do_some_cool_stuff(&mut a).await;
        assert_eq!(a, 2);
    }
}
