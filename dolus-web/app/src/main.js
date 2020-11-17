import App from './App.svelte';

import wasm from '../../painter/Cargo.toml';
// WebAssembly files must be loaded async.
const init = async () => {
    const dolus = await wasm();

    const app = new App({
        target: document.body,
        props: {
          // https://svelte.dev/docs#Creating_a_component
        }
    });
};

init();

export default app;
