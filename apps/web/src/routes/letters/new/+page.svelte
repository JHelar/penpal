<script lang="ts">
	import { Button, Heading, Label, Textarea } from 'flowbite-svelte';
	import type { TextareaProps } from 'flowbite-svelte/dist/forms/Textarea.svelte';
	const textareaprops: TextareaProps = {
		id: 'message',
		name: 'message',
		label: 'Your message',
		rows: 4,
		placeholder: 'Write a message...'
	};
</script>

<div class="container mx-auto">
	<div class="max-w-xl grid grid-flow-row">
		<Heading>New letter</Heading>
		<form method="post">
			<div>
				<Label>Message</Label>
				<Textarea {...textareaprops} />
			</div>
			<Button type="submit">Send</Button>
		</form>
	</div>
</div>
