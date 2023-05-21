<script lang='ts'>
	import TimeLeft from "./TimeLeft.svelte";
    import Stats from "./Stats.svelte";
	import Share from "./Share.svelte";
	import { createEventDispatcher, onMount } from "svelte";
	import { browser } from "$app/environment";
	import PrimaryButton from "./PrimaryButton.svelte";
    export let result: GameResult;
    export let game : Game;
    const dispatch = createEventDispatcher();
    
    let showShare = false; 
    let height = 300;
    let width = 400;
    onMount(() => {
        if(browser){
            //check component size
            height = document.getElementById('modal')?.clientHeight || 300;
            width = document.getElementById('modal')?.clientWidth || 400;
        }
    });

</script>


<section 
        on:click|stopPropagation={()=>{
            dispatch('close');
        }}
        on:keydown|preventDefault={(e)=>{
            if(e.key === 'Escape'){
                dispatch('close');
            }
        }}>

    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div class="modal" id="modal"
        on:click|stopPropagation={()=>{}}
    >
        <div>
            <h1>Game Over</h1>
            <h2>{result.won ? 'You Won! 🎉' : 'You Lost! 😢'}</h2>
            <span> Wait until tomorrow to play again!</span>
            <TimeLeft/>
        </div>
        <div class="stats">
            <Stats height={height} width={width}/>
            {#if showShare}
                <Share result={result} game={game} on:close={()=>{ showShare = false; }}/>
            {/if}
            <div style="display: flex; gap: 16px;">
                <PrimaryButton on:click={()=>{showShare = true;}}>Share</PrimaryButton>
                <PrimaryButton on:click={()=>{ dispatch('close'); }}>Close</PrimaryButton>
            </div>
        </div>
    </div>
</section>


<style lang='scss'>
        section{
        position: fixed;
        display: grid;
        place-items: center;
        width: 100vw;
        height: 100vh;
        background-color: rgba(0,0,0,0.5);
        top: 0;
        left: 0;
    }

    .modal{
        width: clamp(300px, 50%, 500px);
        background-color: var(--background-clear);    
        border-radius: 10px;
        padding: 32px;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        align-items: center;
        z-index: 100;
    }

    .stats{
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 16px;
    }

</style>