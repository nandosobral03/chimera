<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import PrimaryButton from './PrimaryButton.svelte';
	import { signup } from '../apis/user_apis';
	import { error } from '@sveltejs/kit';
	let dispatch = createEventDispatcher();
	let username: string = '';
	let password: string = '';
	let usernameTouched: boolean = false;
	let passwordTouched: boolean = false;
	let registerPromise: Promise<{ token: string }> | null = null;

	const register = async () => {
		if (!username || !password) return;
        if (username.length < 3 || password.length < 3) return;
		registerPromise = signup(username, password).then((token) => {
            setTimeout(() => {
                dispatch('loginToken', token);
            }, 50);
            return token;
        }).finally(() => {
            setTimeout(() => {
                registerPromise = null;
            }, 1500);
        })
	};
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div class="container">
	<div>
		<input
			type="text"
			placeholder="Username"
			bind:value={username}
			class:error={username.length < 3 && usernameTouched}
			on:blur={() => (usernameTouched = true)}
		/>
		<input
			type="password"
			placeholder="Password"
			bind:value={password}
			class:error={password.length < 3 && passwordTouched}
			on:blur={() => (passwordTouched = true)}
			on:keydown={(e) => {
				if (e.key === 'Enter') {
					register();
				}
			}}
		/>
	</div>
	<div style:flex-grow="1">
		{#if registerPromise}
			{#await registerPromise}
				<PrimaryButton disabled>Registering...</PrimaryButton>
			{:then token}
				<PrimaryButton disabled>Registered!</PrimaryButton>
			{:catch error}
				<PrimaryButton disabled type="error">
					Error: {error.response.data.error}
				</PrimaryButton>
			{/await}
		{:else}
			<PrimaryButton on:click={register}>
				Register <small title="Passwords are hashed and salted but just in case">
					(Don't use real passwords)
				</small>
			</PrimaryButton>
		{/if}
		<PrimaryButton on:click={() => dispatch('login')}>Already have an account? Login</PrimaryButton>
	</div>
</div>

<style lang="scss">
	.container {
		display: flex;
		flex-direction: column;
		gap: 8px;
		height: 100%;
		width: 100%;
		justify-content: space-between;
		div {
			display: flex;
			flex-direction: column;
			gap: 8px;
		}
		:global(button) {
			flex-grow: 1;
		}
	}
	input {
		border: none;
		border-radius: 8px;
		padding: 8px;
		font-size: 1rem;
		line-height: 1.5rem;
	}
	.error {
		border: 2px solid red;
	}
</style>
