<script>
  import { drawHistogram, histogramPainter } from './charts.js'

  let painter
  let words = []

  histogramPainter().then((p) => {
    painter = p
    words = painter.words()
  })

  $: draw = (() => {
    for (const word of words) {
      setTimeout(() =>
        drawHistogram({
          chartId: `dolus-histogram-${word}`,
          painter,
          word,
        }),
      )
    }
  })()
</script>

<style>
  .container {
    display: flex;
    align-items: flex-start;
    flex-wrap: wrap;
  }
  .data-explorer {
    flex-grow: 1;
  }
</style>

<div class="container">
  {#each words as w}
    <div class="data-explorer">
      <canvas id={`dolus-histogram-${w}`}  width=1200 height=300/>
    </div>
  {/each}
</div>
