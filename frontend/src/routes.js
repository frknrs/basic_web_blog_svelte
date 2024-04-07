   import Home from './Index.svelte';
   import NewPost from './NewPost.svelte';
   import Login from './Login.svelte';
   import SignUp from './SignUp.svelte';

   const routes = {
       '/': Home,
       '/new-post': NewPost,
       '/login': Login,
       '/sign-up': SignUp
   };

   export default routes;
