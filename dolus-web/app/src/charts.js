import wasm from "../../painter/Cargo.toml";

var dolusImport = null;
var painters = {};

// WebAssembly files must be loaded async.
dolusImport = (async () => {
    return await wasm();
})();

export async function draw({ chartId, word, mouseX, forceFetch = false }) {
    const dolus = await dolusImport;
    if (!word) return;

    let painter;
    if (!painters[word] || forceFetch) {
        painter = await dolus.fetchData(
            word,
            `https://dolus.herokuapp.com/api/counts?word=${word}`
        );
        painters[word] = painter;
    } else {
        painter = painters[word];
    }

    painter.draw(chartId, mouseX);

    return painter;
}

export async function loadWords() {
    const resp = await fetch("https://dolus.herokuapp.com/api/words");
    const payload = await resp.json();
    return payload;
}

export function closestValues({ word, x }) {
    if (x < 0) {
        x = 0;
    }
    if (x > 1.0) {
        throw "X must be normalized";
    }

    const painter = painters[word];
    if (!painter) {
        throw `Painter ${word} not found`;
    }

    return painter.getClosest(x);
}
