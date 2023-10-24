<script>
	import { Button, Heading } from 'flowbite-svelte';
	import { signIn } from '@auth/sveltekit/client';
</script>

<div class="flex flex-col mx-auto max-w-lg">
	<Heading tag="h1" customSize="text-2xl font-extrabold md:text-3xl lg:text-4xl"
		>Sign in to PenPal</Heading
	>
	<Button
		class="mt-4"
		color="alternative"
		pill
		outline
		on:click={() => signIn('google', { callbackUrl: '/me' })}>Sign in with Google</Button
	>
</div>
