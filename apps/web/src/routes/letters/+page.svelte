<script lang="ts">
	import type { PageData } from './$types';
	export let data: PageData;
</script>

<h1>LETTERS</h1>
<ul>
	{#each data.letters as letter}
		<li>{letter.id}</li>
	{/each}
</ul>
