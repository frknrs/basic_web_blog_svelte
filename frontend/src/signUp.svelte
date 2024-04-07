<script>
  import { isLoggedIn } from './stores'
  import { push } from 'svelte-spa-router'

  async function handleSubmit(event) {
    event.preventDefault()
    const formData = new FormData(event.target)
    const data = {
      username: formData.get('username'),
      password: formData.get('password'),
    }
    const response = await fetch('http://localhost:8000/signup', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(data),
    })
    if (response.ok) {
      const data = await response.json()
      localStorage.setItem('token', data.token)
      console.log('Token stored in localStorage:', data.token) // Log the token value
      console.log('Sign-up successful')
      isLoggedIn.set(true) // Update isLoggedIn state on successful sign-up
      push('/')
    } else {
      console.error('Sign-up failed')
    }
  }
</script>

<main>
  <div class="navbar">
    <ul>
      <li><a href="/#/">Home</a></li>
      {#if $isLoggedIn}
        <li><a href="/#/new-post">New Post</a></li>
        <li><a href="/#/logout">Log Out</a></li>
      {:else}
        <li><a href="/#/login">Login</a></li>
        <li><a href="/#/sign-up">Sign Up</a></li>
      {/if}
    </ul>
  </div>
  <div class="post">
    <h2>Sign Up</h2>
    <form on:submit|preventDefault={handleSubmit}>
      <label for="username">Username:</label><br />
      <input type="text" id="username" name="username" /><br />
      <label for="password">Password:</label><br />
      <input type="password" id="password" name="password" /><br />
      <label for="confirm_password">Confirm Password:</label><br />
      <input type="password" id="confirm_password" name="confirm_password" /><br
      />
      <input type="submit" value="Sign Up" />
    </form>
  </div>
</main>

<style>
  /* Scoped styles similar to Login.svelte */
</style>
