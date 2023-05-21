<script lang='ts'>
	import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();
</script>
<button on:click|stopPropagation={()=>{(
    dispatch("click")
)}}>
    <slot/>
</button>

<style lang='scss'>
   button{
        padding: 8px 16px;
        border-radius: 8px;
        border: none;
        background-color: var(--dark-accent);
        color: var(--background);
        font-weight: bold;
        cursor: pointer;
        &:hover{
            background-color: var(--light-accent);
            color: var(--background);
        }
    }

</style>