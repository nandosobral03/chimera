<script lang="ts">
	import { onMount } from 'svelte';
	import { getUserStats } from '../../apis/user_apis';
	import { browser } from '$app/environment';
    import * as jose from "jose"
	let stats = new Promise<any>(() => {});
    let user = "";
	onMount(() => {
        if(!browser) return;
        const token = localStorage.getItem('token');
        if(!token) {
            window.location.href = '/play';
            return;
        }
        
        const decoded = jose.decodeJwt(token);
        user = decoded.sub!;

		stats = getUserStats();
	});
</script>

<section>
	<header>
        {#if user}
		    <h1>{user}'s Profile</h1>
        {/if}
	</header>
	{#await stats then user_stats}
		<div class="stats">
			<div>
                <span class="title">
                    Games Played
                </span>
                <span class="value">
                    {user_stats.total_games}
                </span>
            </div>
			<div>
                <span class="title">
                    Games Won
                </span>
                <span class="value">
                    {user_stats.total_wins}
                </span>
            </div>
			<div>
                <span class="title">
                    Current Win Streak
                </span>
                <span class="value">
                    {user_stats.win_streak}
                </span>
            </div>
            <div>
                <span class="title">
                    Win %
                </span>
                <span class="value">
                    {user_stats.total_games > 0 ? Math.round(user_stats.total_wins / user_stats.total_games * 100) : 0}%
                </span>
            </div>
		</div>
	{/await}
</section>

<style lang="scss">
	section {
		display: flex;
		flex-direction: column;
		height: 100%;
	}
	header {
		display: flex;
		flex-direction: row;
		margin: 24px;
		justify-content: space-between;
		align-items: center;
	}

	h1 {
		color: var(--shade);
		text-transform: uppercase;
		font-size: 1.5rem;
		margin: 0px;
	}

	.stats {
		display: grid;
		max-width: 1500px;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		place-items: center;
		grid-gap: 24px;
		padding: 24px;
		div {
			background-color: var(--shade);
			aspect-ratio: 1/1;
			width: 100%;
			max-width: 350px;
            display: flex;
            flex-direction: column;
            justify-content: flex-start;
            padding: 16px;
            align-items: center;
            color: var(--background);
            .title{
                font-weight: bold;
                text-transform: uppercase;
            }
            .value{
                font-size: 2rem;
                font-weight: bold;
                margin: auto;
            }
		}
	}
</style>
