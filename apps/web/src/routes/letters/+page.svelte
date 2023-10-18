<script lang="ts">
	import type { PageData } from './$types';
	import { page } from '$app/stores';
	export let data: PageData;

	$: console.log($page.data.session);
</script>

<h1>LETTERS</h1>
<ul>
	{#each data.letters as letter}
		<li>
			<span class="text-base font-bold">{letter.id}</span>
			<span class="text-base">{letter.message}</span>
		</li>
	{/each}
</ul>
