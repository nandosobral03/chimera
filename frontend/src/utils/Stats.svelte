<script lang='ts'>
	import { onMount } from "svelte";
	import { getGuestStats } from "../apis/guest_api";
    import 'chart.js/auto';
    import {Pie} from "svelte-chartjs";
    export let height = 300;
    export let width = 400;


    let loading = true;
    let data:any = {};
    let stats = {
        total_games: 0,
        total_wins: 0
    }
    onMount(async ()=>{
        stats = await getGuestStats();
        loading = false;
        data = {
            labels: [
                'Games Won',
                'Games Lost'
            ],
            datasets: [
                {
                    backgroundColor: [
                        'rgba(71, 225, 167, 0.5)',
                        'rgba(194, 116, 161, 0.5)'
                    ],
                    borderColor: [
                        'rgb(71, 225, 167)',
                        'rgb(194, 116, 161)'
                    ],
                    data: [stats.total_wins, stats.total_games - stats.total_wins],
                }
            ],
        }
    })


</script>

<section>
    {#if loading}
    loading
    {:else}
    <Pie 
        data={data} 
        width={width/2} 
        height={width/2} 
        options={{ plugins: { legend: { display: false } } }}
    />
    {/if}
</section>


<style lang='scss'>
    section{
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        font-weight: bold;
        padding: 32px;
    }
</style>
