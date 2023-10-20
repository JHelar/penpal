<script lang="ts">
	import type { PageData } from './$types';
	import { page } from '$app/stores';
	import { Button, Heading } from 'flowbite-svelte';
	export let data: PageData;

	$: console.log($page.data.session);
</script>

<div class="container mx-auto">
	<Heading>Letters</Heading>
	<Button href="/letters/new">New</Button>
	<ul>
		{#each data.letters as letter}
			<li>
				<span class="text-base font-bold">{letter.id}</span>
				<span class="text-base">{letter.message}</span>
			</li>
		{/each}
	</ul>
</div>
