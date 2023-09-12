<script lang="ts">
	import { page } from '$app/stores';

	interface ErrorDetails {
		title: string;
		description: string;
	}

	const status = $page.status;
	const details = getErrorDetails(status);

	function getErrorDetails(status: number): ErrorDetails {
		if (status === 404) {
			return {
				title: 'Page not found',
				description: "Sorry, we couldn't find the page you’re looking for."
			};
		} else {
			return {
				title: 'Unknown error occurred',
				description: 'Sorry, something went wrong.'
			};
		}
	}
</script>

<main class="grid min-h-full place-items-center bg-base-100 px-6 py-24 sm:py-32 lg:px-8">
	<div class="text-center">
		<p class="text-base-content text-base font-semibold">{status}</p>
		<h1 class="mt-4 text-3xl font-bold tracking-tight text-error sm:text-5xl">{details.title}</h1>
		<p class="mt-6 text-base text-base-content leading-7">
			{details.description}
		</p>
		<div class="mt-10 flex items-center justify-center gap-x-6">
			<a href="/" class="btn btn-primary">Go back home</a>
		</div>
	</div>
</main>
