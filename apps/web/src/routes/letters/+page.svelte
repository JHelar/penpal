<script lang="ts">
	import type { PageData } from './$types';
	import { Button, Card, Heading } from 'flowbite-svelte';

	export let data: PageData;
</script>

<div class="container mx-auto">
	<div class="max-w-xl grid grid-flow-row gap-y-4 mx-auto">
		<Heading>Letters</Heading>
		<Button class="justify-self-start" href="/letters/new">New letter</Button>
		<ul class="flex flex-col gap-y-4 w-full">
			{#each data.letters as letter}
				<li>
					<Card class="w-full" href={`/letters/${letter.id}`}>
						<h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
							{letter.created_at.toLocaleString()}
						</h5>
						<p class="font-normal text-gray-700 dark:text-gray-400 leading-tight">
							{letter.subject}
						</p>
					</Card>
				</li>
			{/each}
		</ul>
	</div>
</div>
