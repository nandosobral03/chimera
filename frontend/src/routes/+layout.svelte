<script lang="ts">
	import Navigation from "../utils/Navigation.svelte";
	import UserInfo from "../utils/UserInfo.svelte";
	import * as jose from "jose";
	import { user } from "../stores/user.store";
	import { onMount } from "svelte";
	import { browser } from "$app/environment";
	let showNavigation = false;
	let showUserInfo = false;
	onMount(() => {
		if(!browser) return;
		const token = localStorage.getItem("token");
		if (token) {
			const info = jose.decodeJwt(token);
			user.set(info.sub);
		}
		showUserInfo = true;
	});
	

</script>

<div class="container">
	<button class="close" on:click={() => showNavigation=!showNavigation}>&#9776;</button>
	<article class="content">
		<slot />
		<Navigation {showNavigation} />
	</article>
	{#if showUserInfo}
		<UserInfo/>
	{/if}
</div>

<style lang="scss">
	.container {
		display: flex;
		flex-direction: column;
		height: 100vh;
		width: 100vw;
		padding: clamp(8px, 3vw, 32px);
		border-radius: 8px;
		
	}
	
	article {
		flex-grow: 1;
		background-color: var(--background);
		border-radius: 8px;
		padding: 16px;
		overflow: auto;
	}

	button{
		position: absolute;
		top: 16px;
		right: 16px;
		appearance: none;
		border: none;
		background-color: var(--shade);
		border-radius: 8px;
		cursor: pointer;
		font-size: 16px;
		color: var(--background);
		outline: none;
		opacity: 0.8;
		&:hover{
			opacity: 1;
		}
		z-index: 9999;
	}
</style>
