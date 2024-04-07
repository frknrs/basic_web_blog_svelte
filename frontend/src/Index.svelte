<script>
  let posts = []
  import { onMount } from 'svelte'
  import { isLoggedIn } from './stores'

  onMount(async () => {
    const response = await fetch('http://localhost:8000/get_posts')
    if (response.ok) {
      posts = await response.json()
    } else {
      console.error('Error fetching posts:', await response.text())
    }
  })
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
</main>

{#each posts as post}
  <div class="post">
    <p class="title">Title: {post.title}</p>
    <p class="author">Author: {post.user}</p>
    <p class="date">Date: {post.date}</p>
    <br />
    <p class="context">{post.context}</p>
  </div>
{/each}

<style>
  .post {
    border: 1px solid #cdd6f4;
    margin-bottom: 20px;
    padding: 20px;
  }

  .post p {
    margin: 0;
  }

  .post .title {
    font-weight: bold;
  }

  .post .author {
    font-style: italic;
  }

  .post .date {
    color: #888;
  }
</style>
