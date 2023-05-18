<script lang='ts'>
	import { onDestroy, onMount } from "svelte";
	import { getTimeForNextGame } from "../apis/game_apis";
    let loading = true;
    let time_left = 0;
    let intervalRef: number | null | undefined = null;
    onMount( async () => {
        time_left = (await getTimeForNextGame()).time_until_next_game;
        loading = false;
        intervalRef = setInterval(() => {
            time_left--;
        }, 1000);
    })

    onDestroy(() => {
        if (intervalRef) clearInterval(intervalRef);
    })

    $: hours = Math.floor(time_left / 3600);
    $: minutes = Math.floor(time_left % 3600 / 60);
    $: seconds = time_left % 60;


    

</script>

<div>
    {#if loading}
        <span>Loading...</span>
    {:else}
    <span>Next game will be available in </span>
    <span>{hours.toString().padStart(2, '0')}:{minutes.toString().padStart(2, '0')}:{seconds.toString().padStart(2, '0')}</span>
    {/if}
</div>


<style lang='scss'>

</style>