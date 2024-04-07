<script>
  import Router from 'svelte-spa-router'
  import DarkModeToggle from './DarkModeToggle.svelte'
  import routes from './routes'
</script>

<main>
  <DarkModeToggle />
  <Router {routes} />
</main>
