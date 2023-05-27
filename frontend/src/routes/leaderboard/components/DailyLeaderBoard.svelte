<script lang="ts">
	import DateInput from 'date-picker-svelte/DateInput.svelte';
	import moment from 'moment';
	import { getDailyLeaderboard } from '../../../apis/leaderboard_apis';

	export let leaderboard: DayLeaderboard;
	let today = moment(leaderboard.day, 'DD/MM/YYYY').add(1, 'days').format('YYYY-MM-DD');
	$: date = new Date(today);
	const min = new Date(moment('02/01/2023', 'DD/MM/YYYY').format('YYYY-MM-DD'));
	$: max = new Date(today);

	$: {
		const currentLeaderboard = getDailyLeaderboard(moment(date).format('DD/MM/YYYY'));
		currentLeaderboard.then((res) => {
			leaderboard = res;
		});
	}

	const getTime = (time: number) => {
		//convert seconds to minutes and seconds
		let minutes = Math.floor(time / 60);
		let seconds = time - minutes * 60;
		return `${minutes}m ${seconds}s`;
	};
</script>

<header>
	<DateInput bind:value={date} {min} {max} format="yyyy-MM-dd" closeOnSelection={true} />
</header>

{#if leaderboard.leaderboard.length == 0}
	<div class="empty">No entries for this day</div>
{:else}
	<article>
		<div class="entry header">
			<div class="rank">Rank</div>
			<div class="name">Username</div>
			<div class="score">Time Taken</div>
		</div>
		{#each leaderboard.leaderboard as entry, i}
			<div class="entry">
				<div class="rank">{i + 1}</div>
				<div class="name" title={entry.username}>
					{entry.username}
				</div>
				<div class="score">{getTime(entry.time_taken)}</div>
			</div>
		{/each}
	</article>
{/if}

<style lang="scss">
	header {
		position: absolute;
		top: 0;
		right: 10px;
		display: flex;
		align-items: center;
		justify-content: flex-end;
		width: 100%;
		height: 32px;
	}
	:global(.picker) {
		transform: translateX(-37%);
	}

	:global(.date-time-field > input) {
		text-align: center;
		outline: none;
		&:focus {
			border: none;
			outline: none;
		}
		&:active {
			border: none;
			outline: none;
		}
	}

	.entry {
		display: grid;
		grid-template-columns: 1fr 2fr 3fr;
		gap: 16px;
		min-width: fit-content;
		width: 100%;
		text-align: center;
		padding: 12px;
		&:nth-child(even) {
			background-color: var(--background-clear);
		}

		div {
			flex-grow: 1;
		}
		.rank {
			font-weight: bold;
			width: 100%;
			min-width: 50px;
			display: flex;
			height: 100%;
			align-items: center;
			justify-content: flex-start;
		}
		.name {
			width: 100%;
			min-width: 100px;
			overflow: hidden;
			text-overflow: ellipsis;
			display: flex;
			height: 100%;
			align-items: center;
			justify-content: flex-start;
		}

		.score {
			width: 100%;
			min-width: 80px;
			display: flex;
			height: 100%;
			align-items: center;
			justify-content: center;
		}
	}

	article {
		width: clamp(200px, 100%, 650px);
		display: flex;
		flex-direction: column;
		border-radius: 8px;
		border: 1px solid var(--shade);
		flex-grow: 1;
		overflow: auto;
		margin: 48px auto;
	}

	.header {
		background-color: var(--shade);
		color: var(--background);
		text-transform: uppercase;
		height: 4rem;
	}

    .empty{
        text-transform: uppercase;
        font-weight: bold;
        font-size: 1.5rem;
        color: var(--shade);
        flex-grow: 1;
        display: flex;
        align-items: center;
        justify-content: center;
    }
</style>
