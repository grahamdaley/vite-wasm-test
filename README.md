# WASM Function Call Issue in Vite 3

This project provides a very basic example of how to integrate a wasm (web assembly) module into a Vite 3 / Vue.js 3 Typescript project.

This is a very basic example. The wasm function simply takes such an object as input (from the JS code) and returns it as a JSON string, whereupon it is displayed in on the web page.

This project uses the standard wasm helper, built into Vite version 3.

Feel free to contact me with any comments or questions.

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
$ wasm-pack build ./src/hello_wasm --target web
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

Click on the link for the 'Local' URL, and you should see the app open in your web browser, as per the screenshot below.

![Screenshot](screenshot.png?raw=true "Screenshot")
