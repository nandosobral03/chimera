<script lang='ts'>
    export let disabled : boolean = false;
	import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();
</script>
<button on:click|stopPropagation={()=>{
    if(!disabled)dispatch("click")
}} {disabled}>
    <slot/>
</button>

<style lang='scss'>
   button{
        padding: 8px 16px;
        border-radius: 8px;
        border: none;
        background-color: var(--shade);
        color: var(--background);
        font-weight: bold;
        cursor: pointer;
        text-transform: uppercase;
        &:hover{
            background-color: var(--dark-accent);
            color: var(--background);
        }
    }

</style>