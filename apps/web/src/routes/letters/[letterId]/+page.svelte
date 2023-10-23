<script lang="ts">
	import { Card, Heading, P } from 'flowbite-svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<div class="container mx-auto">
	<Card class="max-w-xl grid grid-flow-row gap-y-4 mx-auto">
		<Heading>{data.letter?.subject}</Heading>
		<time datetime={data.letter?.created_at.toISOString() ?? ''}
			>{data.letter?.created_at.toLocaleString()}</time
		>
		<P>{data.letter?.message}</P>
	</Card>
</div>
