init();

async function init() {
    // OLD: else branch imported "../pkg/wasm_demo.js" which no longer exists
    // if (typeof process == "object") {
    //     const [{Chart}, {main, setup}] = await Promise.all([
    //         import("wasm-demo"),
    //         import("./index.js"),
    //     ]);
    //     setup(Chart);
    //     main();
    //  } else {
    //      const [{Chart}, {main, setup}] = await Promise.all([
    //          import("../pkg/wasm_demo.js"),
    //          import("./index.js"),
    //      ]);
    //      setup(Chart);
    //      main();
    //  }

    // We run in the npm/webpack environment.
    const [{Chart}, {main, setup}] = await Promise.all([
        import("wasm-demo"),
        import("./index.js"),
    ]);
    setup(Chart);
    main();
}
