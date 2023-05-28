<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import PrimaryButton from '../../../utils/PrimaryButton.svelte';
	import loginStore from '../../../stores/login.store';
    const dispatch = createEventDispatcher();
    const openLogin = () => {
        loginStore.set({
            isOpen: true,
            allowClose: true,
        })
    }
</script>

<div class="backdrop">
	<div class="modal">
		<PrimaryButton on:click={() => dispatch('guest')}>
			<div class="user-select">
				Play as Guest
				<small>Play without an account, you can keep track of your scores on this device</small>
			</div>
		</PrimaryButton>
		<PrimaryButton on:click={() => openLogin()}>
			<div class="user-select">
				Login
				<small
					>Create and account or login to keep track of your scores across devices and participate
					in the leaderboards!</small
				>
			</div>
		</PrimaryButton>
	</div>
</div>

<style lang="scss">
	.backdrop {
		position: fixed;
		top: 0;
		left: 0;
		width: 100vw;
		height: 100vh;
		backdrop-filter: blur(8px);
		background-color: rgba(0, 0, 0, 0.5);
		z-index: 2;
	}

	.modal {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		background-color: var(--background);
		border-radius: 8px;
		padding: 16px;
		width: 90vmin;
		max-width: 500px;
		height: 60vh;
		z-index: 3;
		display: flex;
		flex-direction: column;
		gap: 16px;
		:global(button) {
			flex-grow: 1;
		}
		.user-select {
			display: flex;
			flex-direction: column;
			gap: 8px;
            font-size: 16px;
			small {
				font-size: 12px;
				color: var(--light-accent);
			}
		}
	}
</style>
