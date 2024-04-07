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
    const token = localStorage.getItem('token') // Retrieve the token from localStorage
    const response = await fetch('http://localhost:8000/login', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${token}`, // Include the token in the Authorization header
      },
      body: JSON.stringify(data),
    })
    if (response.ok) {
      console.log('Login successful')
      const data = await response.json()
      localStorage.setItem('token', data.token)
      console.log('Token stored in localStorage:', data.token) // Log the token value
      isLoggedIn.set(true) // Update isLoggedIn state on successful login
      push('/') // Redirect to home page or dashboard
    } else {
      console.error('Login failed')
      // Show an error message
    }
  }
</script>

<main>
  <div class="navbar">
    <ul>
      <li><a href="/#/">Home</a></li>
      {#if $isLoggedIn}
        <li><a href="/#/new-post">New Post</a></li>
        <!-- Ensure this line is within the {#if $isLoggedIn} block -->
        <li><a href="/#/logout">Log Out</a></li>
      {:else}
        <li><a href="/#/login">Login</a></li>
        <li><a href="/#/sign-up">Sign Up</a></li>
      {/if}
    </ul>
  </div>
  <!-- Rest of your Index.svelte content -->
  <div class="post">
    <h2>Login</h2>
    <form on:submit|preventDefault={handleSubmit}>
      <label for="username">Username:</label><br />
      <input type="text" id="username" name="username" /><br />
      <label for="password">Password:</label><br />
      <input type="password" id="password" name="password" /><br />
      <input type="submit" value="Login" />
    </form>
  </div>
</main>

<style>
</style>
