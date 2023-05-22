<script lang='ts'>
    import moment from 'moment'
    import { DateInput } from 'date-picker-svelte'
    import type { PageServerData } from './$types';
	import { onMount } from 'svelte';
	import { getCurrentGuestGame } from '../../apis/guest_api';
	import { getDayStats } from '../../apis/day_apis';
	import HeatMap from './components.ts/HeatMap.svelte';
	import { getGameByDay } from '../../apis/game_apis';
    
	export let data: PageServerData;
    let today = moment(data.day, "DD/MM/YYYY").format('YYYY-MM-DD');
    $: date = new Date(today);
    const min = new Date(moment("02/01/2023", "DD/MM/YYYY").format('YYYY-MM-DD'));
    $: max = new Date(today);
    let infoPromise: Promise<[
        stats: DayStats,
        board: Game
]> = new Promise((resolve, reject) => {});
    onMount( async () => {
        const hasPlayedToday = await getCurrentGuestGame();
        if(hasPlayedToday) {
            today = moment(data.day, "DD/MM/YYYY").add(1, 'days').format('YYYY-MM-DD');
            date = new Date(today);
            max = new Date(today);
        }
        loadDayInfo();
    })  

    const loadDayInfo = async () => {
        console.log('loading day info')
            infoPromise = Promise.all([getDayStats(moment(date).format('DD/MM/YYYY')), getGameByDay(moment(date).format('DD/MM/YYYY'))]).catch(e =>{
                    infoPromise = new Promise( async (resolve, reject) => {
                    const game =  await getGameByDay(moment(date).format('DD/MM/YYYY'))
                    const stats = {
                                total_games: 0,
                                total_wins: 0,
                                aggregated_board_stats: {},
                                day: moment(date).format('DD/MM/YYYY')
                            }
                    resolve(
                        [
                            stats,
                            game
                        ]
                    )
                })
                return infoPromise;
        })
    }
       

    $ : loadDayInfo(), [date]

</script>

<section>
    <header>
        <h1>
            Stats
        </h1>
        <DateInput bind:value={date} min={min} max={max} format='yyyy-MM-dd' closeOnSelection={true} />
    </header>
    {#await infoPromise}
        Loading
    {:then info} 
   <div style="text-align: center; margin: 24px;">
       Winrate: { info[0].total_games == 0 ? 0 : (info[0].total_wins / info[0].total_games * 100).toFixed(2)}%
       <div style="font-size: 0.8rem; color: var(--shade); margin-top: 8px;">
           {info[0].total_games} games played
        </div>
   </div> 
    <div class="info">
        <HeatMap board={info[1].board} aggregatedBoardStats={info[0].aggregated_board_stats} initialPosition={info[1].initialPosition} total_losses={info[0].total_games - info[0].total_wins} />
    </div>
{/await}
</section>


<style lang='scss'>
    section{
        display: flex;
        flex-direction: column;
        height: 100%;
    }
    header{
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

    :global(.picker){
        transform: translateX(-37%);
    }


    .info{
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: space-evenly;
        height: 100%;
    }

    :global(.date-time-field > input){
        text-align: center;
        outline: none;
        &:focus{
            border: none;
            outline: none;
        }
    }
</style>