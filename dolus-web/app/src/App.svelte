<script>
  import router from 'page'
  import Chart from './Chart.svelte'
  import Histogram from './Histogram.svelte'
  import Header from './Header.svelte'

  let page

  router('/', () => (page = Histogram))
  router('/index', () => (page = Histogram))
  router('/history', () => (page = Chart))

  router.start()
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
  <Header pages={['index', 'history']} />
  <svelte:component this={page} />
</main>
