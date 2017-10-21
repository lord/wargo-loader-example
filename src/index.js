const wasm = require("./main.rs")
wasm.initialize({noExitRuntime: true}).then(module => {
  // you can call module.cwrap here to get function wrappers for Rust functions
  const add_plus_five = module.cwrap('add_plus_five', 'number', ['number', 'number'])
  console.log(add_plus_five(1, 2))
})
