<script>
  import { Router, Link, Route } from 'svelte-routing'
  import Chart from './Chart.svelte'
  import Histogram from './Histogram.svelte'
</script>

<style>
  main {
    text-align: left;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>

<main>
  <Router>
    <nav>
      <Link to="/">Current count</Link>
      <Link to="history">History</Link>
    </nav>
    <div>

      <Route path="history" component={Chart} />
      <Route path="/">
        <Histogram />
      </Route>
    </div>

  </Router>
</main>
