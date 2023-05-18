<script lang='ts'>
	import TimeLeft from "./TimeLeft.svelte";
    import Stats from "./Stats.svelte";
	import Share from "./Share.svelte";
	import { createEventDispatcher, onMount } from "svelte";
	import { browser } from "$app/environment";
    export let result: GameResult;
    export let game : Game;
    const dispatch = createEventDispatcher();

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
        <div class="">
            <Stats height={height} width={width}/>
            <Share result={result} game={game} />
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
        background-color: white;
        border-radius: 10px;
        padding: 32px;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        align-items: center;
        z-index: 100;
    }

</style>