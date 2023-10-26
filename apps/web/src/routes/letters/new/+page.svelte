<script lang="ts">
	import { Avatar, Button, Heading, Helper, Input, Label, Select, Textarea } from 'flowbite-svelte';
	import type { TextareaProps } from 'flowbite-svelte/dist/forms/Textarea.svelte';
	import type { ActionData, PageData } from './$types';

	export let data: PageData;
	export let form: ActionData;

	const textareaprops: TextareaProps = {
		id: 'message',
		name: 'message',
		label: 'Your message',
		rows: 4,
		placeholder: 'Write a message...',
		minlength: 250,
		required: true,
		value: form?.formData.message ?? ''
	};
</script>

<div class="container mx-auto">
	<form method="post" class="max-w-xl grid grid-flow-row gap-y-4 mx-auto">
		<Heading>New letter</Heading>
		{#if form?.reason.general}
			<Helper color="red">{form.reason.general}</Helper>
		{/if}
		<Label class="block space-y-2">
			<Avatar rounded src={data.recipient.profile_image} />
			<span>{data.recipient.display_name}</span>
			<input type="hidden" name="to_user_id" value={data.recipient.id} />
		</Label>
		<Label class="block space-y-2">
			<span>Subject</span>
			<Input
				label="Subject"
				id="subject"
				name="subject"
				value={form?.formData.subject ?? ''}
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
