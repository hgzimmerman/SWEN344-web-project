const js = import("./wasm_calls.js");

js.then(js => {
    let x = js.login("Hello there");
    x.then(y => console.log(y))
});