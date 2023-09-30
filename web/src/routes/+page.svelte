<script lang="ts">
	const THEMES = ['light', 'dark', 'valentine'];

	function changeTheme() {
		// theme is changed via data-theme attribute on the html tag
		const html = document.querySelector('html');
		if (!html) throw new Error('html tag not found');

		const currentTheme = html.getAttribute('data-theme');
		const nextTheme = THEMES[(THEMES.indexOf(currentTheme ?? 'light') + 1) % THEMES.length];

		if (!nextTheme) throw new Error('next theme not found');

		html.setAttribute('data-theme', nextTheme);
	}
</script>

<a class="btn btn-primary" href="/auth/login">Login</a>
<a class="btn btn-primary" href="/auth/register">Register</a>
<a class="btn btn-primary" href="/registeadsar">404</a>
<button class="btn btn-info" on:click={changeTheme}>Change Theme</button>
