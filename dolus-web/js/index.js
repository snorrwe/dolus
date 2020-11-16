console.time("main");
import("../pkg/index.js")
  .then((mod) =>
    fetch("https://dolus.herokuapp.com/api/words")
      .then((r) => r.json())
      .then((d) => [mod, d])
  )
  .then(([mod, words]) => {
    const pms = words.sort().map((w) => {
      return mod
        .fetch_data(w, `https://dolus.herokuapp.com/api/counts?word=${w}`)
        .then((painter) => {
          console.log(
            `Painting ${painter.numberOfItems()} items from word ${w}`
          );
          const key = `draw ${w}`;
          console.time(key);
          const el = document.createElement("CANVAS");
          el.classList.add("chart");
          el.setAttribute("width", 1500);
          el.setAttribute("height", 800);
          document.body.appendChild(el);
          const id = `plot-${w}`;
          el.id = id;
          try {
            painter.draw(id);
          } catch (err) {
            console.error("Failed to draw", w, err);
          }
          console.timeEnd(key);
        });
    });
    return Promise.all(pms);
  })
  .then(() => console.timeEnd("main"))
  .catch(console.error);
