<script lang="ts">
	import { onMount } from 'svelte';
	import type { PageServerData } from './$types';
	import Board from './components/board.svelte';
	import { browser } from '$app/environment';
	import { getCurrentGuestGame, postGameResultGuest } from '../../apis/guest_api';
	import Modal from '../../utils/Modal.svelte';
	export let data: PageServerData;
	let minesRemaining = 99;
	let loading = true;
	let playable = true;
	let showModal = false;
	let result : GameResult = {
		won: false,
		moves: [],
		flags: []
	}
	let game: Game = {
		board: data.board,
		initialPosition: {
			x: parseInt(data.initialPosition.split(':')[0]),
			y: parseInt(data.initialPosition.split(':')[1])
		}
	}
	const generateUUID = () => {
		var d = Date.now();
		var d2 =
			(typeof performance !== 'undefined' && performance.now && performance.now() * 1000) || 0;
		return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function (c) {
			var r = Math.random() * 16;
			if (d > 0) {
				r = (d + r) % 16 | 0;
				d = Math.floor(d / 16);
			} else {
				r = (d2 + r) % 16 | 0;
				d2 = Math.floor(d2 / 16);
			}
			return (c === 'x' ? r : (r & 0x3) | 0x8).toString(16);
		});
	};
	onMount(async () => {
		minesRemaining = data.mineCount;
		if (browser) {
			let guest_id = localStorage.getItem('guest_id');
			if (!guest_id) {
				guest_id = generateUUID();
				localStorage.setItem('guest_id', guest_id);
				loading = false;
			} else {
				let game = await getCurrentGuestGame();
				if (game) {
					playable = false;
					let played_moves = game.board;
					for (let move of played_moves.split(',')) {
						if (move) {
							let [x, y] = move.split(':');
							result.moves.push({ x: parseInt(x), y: parseInt(y) });
						}
					}
					for (let flag of game.flags.split(',')) {
						if (flag) {
							let [x, y] = flag.split(':');
							result.flags.push({ x: parseInt(x), y: parseInt(y) });
						}
					}
					result.won = game.status === 'won';
					showModal = true;
				}
				minesRemaining = data.mineCount - result.flags.length;
				loading = false;
			}
		}
	});

	const gameover = async (e: GameResult) => {
		showModal = true;
		if(!playable) return;
		await postGameResultGuest(e);
	};
</script>

<section>
	{#if loading}
		<div>Loading...</div>
	{:else}
		<div style={minesRemaining < 0 ? 'color: red' : ''}>
			🚩 {data.mineCount - minesRemaining}? / {data.mineCount}
		</div>
		<Board
			board={data.board}
			initialPosition={data.initialPosition}
			on:gameover={(e) => gameover(e.detail)}
			on:flag={(e) => minesRemaining--}
			on:unflag={(e) => minesRemaining++}
			moves={result.moves}
			flags={result.flags}
		/>
	{/if}


	{#if showModal}
		<Modal
			 result={result}
				on:close={() => showModal = false}
			game= {game}
		/>
	{/if}
</section>

<style lang="scss">
	section {
		height: 100%;
		width: 100%;
		display: flex;
		flex-direction: column;
		justify-content: space-evenly;
		align-items: center;
	}
</style>
