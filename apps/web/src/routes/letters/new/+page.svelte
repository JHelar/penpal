<script lang="ts">
	import { Button, Heading, Helper, Input, Label, Textarea } from 'flowbite-svelte';
	import type { TextareaProps } from 'flowbite-svelte/dist/forms/Textarea.svelte';
	import type { ActionData } from './$types';

	export let form: ActionData;

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
		<Button type="submit">Send</Button>
	</form>
</div>
