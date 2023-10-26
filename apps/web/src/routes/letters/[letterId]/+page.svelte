<script lang="ts">
	import { Avatar, Button, Card, Heading, P } from 'flowbite-svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<div class="container mx-auto">
	<Card class="max-w-xl grid grid-flow-row gap-y-4 mx-auto">
		<div class="flex row place-items-center gap-x-4">
			<Avatar src={data.fromUser.profile_image} rounded size="lg" />
			<Heading tag="h2" class="text-lg">{data.fromUser.display_name}</Heading>
		</div>
		<Heading>{data.letter.subject}</Heading>
		<time datetime={data.letter.created_at.toISOString() ?? ''}
			>{data.letter.created_at.toLocaleString()}</time
		>
		<P>{data.letter.message}</P>
		<Button class="justify-self-end" href={`/letters/new?toUserId=${data.recipientUserId}`}
			>Reply</Button
		>
	</Card>
</div>
