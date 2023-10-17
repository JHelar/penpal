<script lang="ts">
	import type { PageData } from './$types';
	export let data: PageData;
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
