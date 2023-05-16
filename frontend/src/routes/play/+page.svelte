<script lang="ts">
	import { onMount } from 'svelte';
	import type { PageServerData } from './$types';
	import Board from './components/board.svelte';
	import { browser } from '$app/environment';
	import { getCurrentGuestGame } from '../../apis/guest_api';
	export let data: PageServerData;
	let minesRemaining = 0;

	const generateUUID = () => { 
		var d = Date.now();
		var d2 = ((typeof performance !== 'undefined') && performance.now && (performance.now()*1000)) || 0;
		return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
			var r = Math.random() * 16;
			if(d > 0){
				r = (d + r)%16 | 0;
				d = Math.floor(d/16);
			} else {
				r = (d2 + r)%16 | 0;
				d2 = Math.floor(d2/16);
			}
			return (c === 'x' ? r : (r & 0x3 | 0x8)).toString(16);
		});
}



	onMount(async () => {
		minesRemaining = data.mineCount;
		if (browser) {
			let guest_id = localStorage.getItem('guest_id');
			if (!guest_id) {
				guest_id = generateUUID();
				localStorage.setItem('guest_id', guest_id);
			}
			let game = await getCurrentGuestGame();
			if (game){
				// fill board
			}
		}
	});



</script>

<section>
	<div style={ minesRemaining < 0 ? 'color: red' : '' }>
		🚩 : {data.mineCount - minesRemaining}? / {data.mineCount}
	</div>
	<Board board={data.board} initialPosition={data.initialPosition} on:gameover={e => console.log(e.detail)} on:flag={e => minesRemaining--} on:unflag={e => minesRemaining++}/>
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