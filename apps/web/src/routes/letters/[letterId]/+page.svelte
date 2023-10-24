<script lang="ts">
	import { Button, Card, Heading, P } from 'flowbite-svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<div class="container mx-auto">
	<Card class="max-w-xl grid grid-flow-row gap-y-4 mx-auto">
		<Heading>{data.letter?.subject}</Heading>
		<Heading tag="h2">{data.letter?.to_user_id}</Heading>
		<time datetime={data.letter?.created_at.toISOString() ?? ''}
			>{data.letter?.created_at.toLocaleString()}</time
		>
		<P>{data.letter?.message}</P>
		<Button class="justify-self-end" href={`/letters/new?toUserId=${data.letter?.to_user_id}`}
			>Reply</Button
		>
	</Card>
</div>
