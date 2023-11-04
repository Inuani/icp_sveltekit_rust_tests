<script>
	export let inline = false;
</script>

<div class={inline ? 'spinner inline' : 'spinner'} />

<style>
	.spinner {
		display: block;
		background: transparent;
		border: 3px solid;
		border-image: linear-gradient(to right, red, yellow) 1;
		width: 50px;
		height: 50px;
		border-radius: 50%;
		animation: spinner-rotate 2s infinite linear;
	}

	.spinner.inline {
		position: static;
	}

	.spinner:not(.inline) {
		position: absolute;
		top: calc(50% - 25px);
		left: calc(50% - 25px);
	}

	@keyframes spinner-rotate {
		0%,
		100% {
			transform: perspective(120px) rotateX(0deg) rotateY(0deg);
		}
		50% {
			transform: perspective(120px) rotateX(-180.1deg) rotateY(0deg);
		}
	}
</style>
