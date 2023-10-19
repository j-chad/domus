<script lang="ts">
	import { ButtonColours } from '$lib/components/daisyui/constants';
	import type { Colour } from '$lib/components/daisyui/constants';

	export let colour: 'ad' | undefined = undefined;
	export let size: 'lg' | 'md' | 'sm' | 'wide' | 'block' | 'circle' | 'square' = 'md';
</script>

<button class="btn {colour}" on:click>
	<slot />
</button>
