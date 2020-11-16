console.time("load");
import("../pkg/index.js")
  .then((mod) =>
    mod
      .fetch_data("https://dolus.herokuapp.com/api/counts")
      .then((d) => [mod, d])
  )
  .then(([mod, data]) => {
    console.timeEnd("load");
    console.time("draw");
    const keys = data.getWords();
    const pms = keys.sort().map((key, index) => {
      (async () => {
        const el = document.createElement("CANVAS");
        el.classList.add("chart");
        el.setAttribute("width", 1500);
        el.setAttribute("height", 800);
        document.body.appendChild(el);
        const id = `plot-${key}`;
        el.id = id;
        try {
          data.draw(key, id);
        } catch (err) {
          console.error("Failed to draw", key, err);
        }
      })();
    });
    return Promise.all(pms);
  })
  .then(() => console.timeEnd("draw"))
  .catch(console.error);
