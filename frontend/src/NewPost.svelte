<script>
  import { isLoggedIn } from './stores'
  import { push } from 'svelte-spa-router'

  async function handleSubmit(event) {
    event.preventDefault()

    const formData = new FormData(event.target)
    const data = {
      content: formData.get('content'),
      title: formData.get('title'),
    }

    const token = localStorage.getItem('token') // Retrieve the token from localStorage
    const response = await fetch('http://localhost:8000/newpost', {
      method: 'POST',
      // prettier-ignore
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${token}`, // Include the token in the Authorization header
      },
      body: JSON.stringify(data),
    })
    if (response.ok) {
      const data = await response.json();
      console.log('Post saved succesful:', data)
      push('/')
    } else {
      console.error('Saving post failed.')
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
  <div class="post">
    <h2>New Post</h2>
    <form on:submit|preventDefault={handleSubmit}>
      <label for="title">Title:</label><br />
      <input type="text" id="title" name="title" /><br />
      <label for="content">Content:</label><br />
      <textarea id="content" name="content" rows="10" cols="50"></textarea><br
      />
      <input type="submit" value="Submit" />
    </form>
  </div>
</main>

<style>
  /* Scoped styles similar to Login.svelte and SignUp.svelte */
</style>
