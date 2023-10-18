<script>
	import { Button } from 'flowbite-svelte';
	import { signIn } from '@auth/sveltekit/client';
</script>

<h1>Login user</h1>
<Button color="green" on:click={() => signIn('google')}>Login</Button>
