<script lang="ts">
	import type {  LayoutData } from "./$types";

    export let data: LayoutData

	const tabs: {
		name: string;
		path: string;
	}[] = [
		{
			name: 'Play',
			path: '/play'
		},
		{
			name: 'About',
			path: '/about'
		},
		{
			name: 'Stats',
			path: '/stats'
		}
	];





</script>

<div class="container">
	<nav>
		{#each tabs as tab}
			<div class="tab-container">
				<div
					class="divider"
					aria-hidden="true"
					class:active={ data.route.id === tab.path }
				>
					<div class="divider_fill pre" />
				</div>
				<button
					class="tab"
					on:click={() => (window.location.href = tab.path)}
					class:active={  data.route.id === tab.path }
				>
					<span> {tab.name} </span>
				</button>
				<div
					class="divider"
					aria-hidden="true"
					class:active={ data.route.id === tab.path }
				>
					<div class="divider_fill post" />
				</div>
			</div>
		{/each}
	</nav>

	<article class="content">
		<slot />
	</article>
</div>

<style lang="scss">
	.container {
		display: flex;
		flex-direction: column;
		height: 100vh;
		width: 100vw;
		padding: 8px;
		border-radius: 8px;
	}
	nav {
		width: 100%;
		height: 5%;
		padding: 0px 16px;
		min-height: 30px;
		max-height: 50px;
		display: flex;
		justify-content: space-around;
		background-color: var(--shade);
		button {
			appearance: none;
			border: none;
			outline: none;
		}
		.tab-container {
			display: flex;
			flex-grow: 1;
			.tab {
				flex-grow: 1;
				display: flex;
				align-items: center;
				padding: 8px 16px;
				cursor: pointer;
				background-color: var(--shade);
				filter: brightness(1.1);
				color: var(--background);
                font-size: 0.8rem;
                font-weight: thin;
				border-top-left-radius: 16px;
				border-top-right-radius: 16px;
				&.active {
					background: var(--background) !important;
					color: var(--shade);
					filter: none;
				}
			}
			.divider {
				width: 8px;
				height: 100%;
				background-color: #4a3747;
				
				&.active {
					background-color: var(--background);
					display: block;
				}

				.divider_fill {
					width: 100%;
					height: 100%;
					background-color: var(--shade);
					&.pre {
						border-bottom-right-radius: 8px;
					}
					&.post {
						border-bottom-left-radius: 8px;
					}
				}
			}

			&:hover {
				.tab {
					background: linear-gradient(180deg, var(--background-dark) 0%, var(--background) 100%);
					border-top-left-radius: 16px;
					border-top-right-radius: 16px;
					color: var(--shade);
					filter: none;
				}
				.divider {
					background: linear-gradient(180deg, var(--background-dark) 0%, var(--background) 100%);
				}
			}
		}
	}
	article {
		flex-grow: 1;
		background-color: var(--background);
		border-radius: 8px;
		padding: 16px;
	}
</style>
