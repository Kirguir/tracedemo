const rust = import('../../pkg');
rust
  .then(rust => {
    let foo = new rust.Foo();

    foo.run()
      .catch(function (err) {
      if (err instanceof Error) {
        console.log("Standart error");
      } else if (err instanceof MyError) {
        console.log("name:" + err.name());
        console.log("description:" + err.message());
      } else {
        console.log("Unknown error");
      }
    });
    console.log("End");
    foo.dispose();
  });
