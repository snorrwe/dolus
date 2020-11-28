<script>
  import page from 'page'

  export let pages = []
</script>

<style>
  .link {
    margin: 5px;
  }
</style>

<div>
  {#each pages as p}
    <!-- svelte-ignore a11y-missing-attribute -->
    <a
      class="link"
      on:click={(e) => {
        page(p)
        e.preventDefault()
      }}>
      {p}
    </a>
  {/each}
</div>
