const rust = import("./pkg/wasm_calls");


rust.then(r => {
    let x = new r.Login("Dooot Dooot Dooooot");

    x.fetch()
        .then(y => {
            console.log(y);
            r.User.fetch(y)
                 .then(z => console.log(JSON.stringify(z)));
        })
        .catch(z => console.log("failed?"))
        .finally(a => console.log("finally"))
});