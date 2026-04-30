# Wasm Accelerated Toy Interferometer

This is an exploration of using Rust in a WASM application.

## Running locally

The following script will install needed software and run the server via `npm`.
```
./start-server.sh
```

## Developing with NPM
Then you can run it locally using `npm`:

```bash
wasm-pack build
cd www
npm install
npm start
```

This will start a dev server which will automatically reload your page
whenever you change anything in `www` directory. To update `rust` code
call `wasm-pack build` manually.
