<script lang='ts'>
	import { onMount } from "svelte";
	import { getGuestStats } from "../apis/guest_api";
    import 'chart.js/auto';
    import {Pie} from "svelte-chartjs";
	import { getUserStats } from "../apis/user_apis";
    export let height = 300;
    export let width = 400;


    let loading = true;
    let data:any = {};
    let statsPromise:Promise<any> = new Promise((resolve, reject) => {});

    function loadStats(){
        const token = localStorage.getItem('token');
        if(token){
            return getUserStats().then((stats) => {
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
                        data: [stats.total_wins, Math.abs(stats.total_games - stats.total_wins)],
                    }
                ],
            }
        })
        }else{
            return getGuestStats().then((stats) => {
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
                        data: [stats.total_wins, Math.abs(stats.total_games - stats.total_wins)],
                    }
                ],
            }
        })
        }
    }

    onMount(()=>{
        statsPromise = loadStats();
    })


</script>

<section>
    {#await statsPromise}
        loading
    {:then}
    <Pie 
        data={data} 
        width={width/2} 
        height={width/2} 
        options={{ plugins: { legend: { display: false } } }}
    />
    {/await}
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
