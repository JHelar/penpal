<script lang="ts">
	import { Button, Card, Heading, Input, Label } from 'flowbite-svelte';
	import { page } from '$app/stores';

	$: user = $page.data.session?.user;
</script>

<div class="mx-auto container">
	<form method="post">
		<Card class="flex flex-col mx-auto max-w-lg gap-y-4">
			<Heading>Settings</Heading>
			<Label class="block space-y-2">
				<span>Email</span>
				<Input
					label="Email"
					id="email"
					name="email"
					value={user?.email}
					required
					placeholder="Email"
					disabled
				/>
			</Label>
			<Label class="block space-y-2">
				<span>Profile image</span>
				<Input
					label="Profile image"
					id="profile_image"
					name="profile_image"
					value={user?.profile_image}
					required
					placeholder="Profile image"
				/>
			</Label>
			<Label class="block space-y-2">
				<span>Username</span>
				<Input
					label="Username"
					id="username"
					name="username"
					value={user?.username}
					required
					placeholder="Username"
				/>
			</Label>
			<Label class="block space-y-2">
				<span>Display name</span>
				<Input
					label="Display name"
					id="display_name"
					name="display_name"
					value={user?.display_name}
					required
					placeholder="Display name"
				/>
			</Label>
			<Button class="self-end" type="submit">Save changes</Button>
		</Card>
	</form>
</div>
