<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import PrimaryButton from './PrimaryButton.svelte';
	import { login } from '../apis/user_apis';
	let dispatch = createEventDispatcher();
	let username: string;
	let password: string;

    let loginPromise : Promise<{token : string}> | null = null;
    const handleLogin = async () => {
        if (!username || !password) return;
        if (username.length < 3 || password.length < 3) return;
        loginPromise = login(username, password).then((token) => {
            setTimeout(() => {
                dispatch('loginToken', token);
            }, 50);
            return token;
        }).finally(() => {
            setTimeout(() => {
                loginPromise = null;
            }, 1500);
        })
    };
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div class="container">
	<div>
		<input type="text" placeholder="Username" bind:value={username} />
		<input type="password" placeholder="Password" bind:value={password}  on:keyup={(e) => {
			if (e.key === 'Enter') {
				handleLogin();
			}
		}}/>
	</div>
	<div style:flex-grow="1">
        {#if loginPromise}
            {#await loginPromise}
                <PrimaryButton disabled>Loading...</PrimaryButton>
            {:then _} 
                <PrimaryButton disabled>Logged In!</PrimaryButton>
            {:catch error}
                <PrimaryButton disabled type="error">
                    Error: {error.response.data.error}
                </PrimaryButton>
            {/await}
            {:else}
                <PrimaryButton on:click={handleLogin}>
                    Login
                </PrimaryButton>
        {/if}
		<PrimaryButton on:click={() => dispatch('register')}>No Account? Register</PrimaryButton>
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
</style>
