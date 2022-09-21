# WASM Function Call Issue in Vite 3

The purpose of this project is to illustrate an issue I came up against when building a project that calls a wasm function that passes a complex object (an array of arrays of numeric values) from the JS world into the wasm.

This is a very basic example. The wasm function simply takes such an object as input (from the JS code) and returns it as a JSON string, whereupon it is displayed in on the web page.

This project uses the standard wasm helper, built into Vite version 3.

Feel free to let me know if this is my error, or a known bug with one of the packages I'm using.

## Prerequisites

- [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [yarn](https://yarnpkg.com/getting-started/install) (optional - can also use npm with the same results)

## Building and running locally

Install JS dependencies:

```bash
$ yarn install
```

Build the wasm module:

```sh
$ wasm-pack build ./src/hello_wasm --target bundler
```

Start up Vite's local web server:

```sh
$ yarn dev
```

In your terminal window, you should now see:

```
  VITE v3.1.2  ready in 198 ms

  ➜  Local:   http://localhost:5173/
  ➜  Network: use --host to expose
```

Click on the link for the 'Local' URL, and you should see the app open in your web browser.

Open the developer console in your browser, and note the error message shown:

![Screenshot](screenshot.png?raw=true "Screenshot")

## Simpler example works as expected

The JS code attempts to call two functions in the wasm:
- ``lookup``, which takes an array of arrays of numbers and returns a string
- ``get_number``, which simply takes a single number and returns it unchanged

By commenting out the following lines of code and re-building the wasm, it can be seen that the simple, ``get_number`` function works as expected:

- src/hello_wasm/src/lib.rs: 17 - 24
- src/components/HelloWorld.vue: 17

So there would seem to be an issue in binding to wasm functions that take complex objects as their inputs.