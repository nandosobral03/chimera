<script lang='ts'>
	import type { PageServerData } from "./$types";
	import AlltimeLeaderBoard from "./components/AlltimeLeaderBoard.svelte";
	import DailyLeaderBoard from "./components/DailyLeaderBoard.svelte";
    export let data: PageServerData;
    let active: "leaderboard" | "daily" = "daily";


</script>


<section>
    <nav>
        <div class:active={active === "leaderboard"} on:click={() => active = "leaderboard"} on:keydown={(e) => {if(e.key === "Enter") active = "leaderboard"}}>
            Leaderboard
        </div>
        <div class:active={active === "daily"} on:click={() => active = "daily"} on:keydown={(e) => {if(e.key === "Enter") active = "daily"}}>
            Daily
        </div>
    </nav>
    <div class="info">

        {#if active == "daily"}
        <DailyLeaderBoard leaderboard={data.current}/>
        {:else}
        <AlltimeLeaderBoard leaderboard={data.alltime}/>
        {/if}
    </div>
    
</section>

<style lang='scss'>
    section{
        display: flex;
        flex-direction: column;
        align-items: center;
        width: 100%;
        height: 100%;
        gap: 16px;
    }
    .info{
        position: relative;
        height: 100%;
        width: 100%;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        overflow: auto;
    }

    nav{
        width: 100%;
        display: flex;
        flex-direction: row-reverse;
        justify-content: space-evenly;
        gap: 16px;
        div{
            cursor: pointer;
            transition: all 0.2s ease-in-out;
            border-radius: 0.5rem;
            padding: 1rem;
            flex-grow: 1;
            text-align: center;
            text-transform: uppercase;
            color: var(--shade);
            background-color: var(--background);
            &.active{
                color: var(--background);
                background-color: var(--shade);                
            }
        }

    }

 
</style>    
