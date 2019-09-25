import {MyError} from "../../src/error";

const rust = import('../../pkg');
rust
  .then(rust => {
    let foo = new rust.Foo();

    try {
      foo.run()
    } catch (err) {
      if (err instanceof MyError) {
        console.log("name:" + err.name());
        console.log("description:" + err.message());
      } else if (err instanceof Error) {
        console.log("Standart error");
      } else {
        console.log("Unknown error");
      }
    }
    console.log("End");
    foo.dispose();
  });
