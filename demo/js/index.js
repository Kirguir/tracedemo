const rust = import('../../pkg');
rust
    .then(rust => {
        let jason_err = rust.jason_err();
        console.log(jason_err instanceof Error);
        console.log(jason_err instanceof rust.JasonError);

        let detached_err = rust.detached_err();
        console.log(detached_err instanceof Error);
        console.log(detached_err instanceof rust.JasonError);
        console.log(detached_err instanceof rust.DetachedStateError);
    });
