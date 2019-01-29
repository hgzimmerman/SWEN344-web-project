const js = import("./wasm_calls.js");

// import {
//     Login,
//     login2
// } from "./wasm_calls"

js.then(js => {
    let x = new js.Login("Dooot Dooot Dooooot");

    x.fetch()
        .then(y => {
            console.log(y);
            js.User.fetch(y)
                 .then(z => console.log(JSON.stringify(z)));
        })
        .catch(z => console.log("failed?"))
        .finally(a => console.log("finally"))
});