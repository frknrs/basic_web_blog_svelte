<script>
  let isDarkMode = localStorage.getItem('darkMode') === 'true'

  function toggleDarkMode() {
    isDarkMode = !isDarkMode
    document.body.classList.toggle('dark-mode', isDarkMode)
    localStorage.setItem('darkMode', isDarkMode.toString())
  }

  // Apply the initial dark mode state
  document.body.classList.toggle('dark-mode', isDarkMode)
</script>

<button class="dark-mode-toggle" on:click={toggleDarkMode}>
  {#if isDarkMode}
    <svg viewBox="0 0 24 24" fill="currentColor">
      <path
        d="M9 21c0 .55.45 1 1 1h4c.55 0 1-.45 1-1v-1H9v1zm3-19C8.14 2 5 5.14 5 9c0 2.38 1.19 4.47 3 5.74V17c0 .55.45 1 1 1h6c.55 0 1-.45 1-1v-2.26c1.81-1.27 3-3.36 3-5.74 0-3.86-3.14-7-7-7zm2.85 11.1l-.85.6V16h-4v-2.3l-.85-.6C7.8 12.16 7 10.63 7 9c0-2.76 2.24-5 5-5s5 2.24 5 5c0 1.63-.8 3.16-2.15 4.1z"
      />
    </svg>
  {:else}
    <svg viewBox="0 0 24 24" fill="currentColor">
      <path
        d="M9 21c0 .55.45 1 1 1h4c.55 0 1-.45 1-1v-1H9v1zm3-19C8.14 2 5 5.14 5 9c0 2.38 1.19 4.47 3 5.74V17c0 .55.45 1 1 1h6c.55 0 1-.45 1-1v-2.26c1.81-1.27 3-3.36 3-5.74 0-3.86-3.14-7-7-7zm2.85 11.1l-.85.6V16h-4v-2.3l-.85-.6C7.8 12.16 7 10.63 7 9c0-2.76 2.24-5 5-5s5 2.24 5 5c0 1.63-.8 3.16-2.15 4.1z"
      />
    </svg>
  {/if}
</button>

<style>
  .dark-mode-toggle {
    position: fixed;
    top: 20px;
    right: 20px;
    background-color: #89dceb;
    color: #1e1e2e;
    border: none;
    border-radius: 50%;
    width: 40px;
    height: 40px;
    display: flex;
    justify-content: center;
    align-items: center;
    cursor: pointer;
    transition: background-color 0.3s ease;
  }

  .dark-mode-toggle:hover {
    background-color: #f28fad;
  }
</style>
