const rust = import('./pkg');
rust
  .then(rust => {
    let jason = new rust.Jason()

    let room_handle = jason.room_handle();

    jason.dispose();

    room_handle.join().catch(function (err) {
      console.log("name:" + err.name());
      console.log("description:" + err.description());
      console.log("cause:" + err.cause());
      console.log("backtrace:" + err.backtrace());

    })
  });
