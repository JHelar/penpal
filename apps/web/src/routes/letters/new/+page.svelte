<script lang="ts">
	import { Button, Heading, Helper, Input, Label, Select, Textarea } from 'flowbite-svelte';
	import type { TextareaProps } from 'flowbite-svelte/dist/forms/Textarea.svelte';
	import type { ActionData, PageData } from './$types';

	export let data: PageData;
	export let form: ActionData;

	let recipient = form?.toUserId ?? data.toUserId;
	let lockRecipient = Boolean(data.toUserId);

	const textareaprops: TextareaProps = {
		id: 'message',
		name: 'message',
		label: 'Your message',
		rows: 4,
		placeholder: 'Write a message...',
		minlength: 250,
		required: true,
		value: form?.message ?? ''
	};
</script>

<div class="container mx-auto">
	<form method="post" class="max-w-xl grid grid-flow-row gap-y-4 mx-auto">
		<Heading>New letter</Heading>
		{#if form?.reason.general}
			<Helper color="red">{form.reason.general}</Helper>
		{/if}
		<Label class="block space-y-2">
			<span>Recipient</span>
			<Select required disabled={lockRecipient} bind:value={recipient} items={data.recipients} />
			<input type="hidden" name="to_user_id" value={recipient} />
			{#if form?.reason && 'to_user_id' in form.reason}
				<Helper color="red">{form.reason.to_user_id}</Helper>
			{/if}
		</Label>
		<Label class="block space-y-2">
			<span>Subject</span>
			<Input
				label="Subject"
				id="subject"
				name="subject"
				value={form?.subject ?? ''}
				required
				placeholder="Hello there..."
			/>
			{#if form?.reason && 'subject' in form.reason}
				<Helper color="red">{form.reason.subject}</Helper>
			{/if}
		</Label>
		<Label class="block space-y-2">
			<span>Message</span>
			<Textarea {...textareaprops} />
			{#if form?.reason && 'message' in form.reason}
				<Helper color="red">{form.reason.message}</Helper>
			{/if}
		</Label>
		<Button class="justify-self-end" type="submit">Send</Button>
	</form>
</div>
