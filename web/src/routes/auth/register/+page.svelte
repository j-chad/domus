<div>
	<a class="text-base-content text-medium" href="/">Domus</a>
	<h2 class="mt-6 text-3xl font-extrabold text-primary">Create an account.</h2>
</div>

<div class="mt-8">
	<form action="#" method="POST" class="space-y-6">
		<div class="form-control">
			<label for="name" class="label label-text"> Full Name </label>
			<input
				id="name"
				name="name"
				type="text"
				autocomplete="name"
				required
				placeholder="Your Name"
				class="input input-bordered input-primary"
			/>
		</div>

		<div class="form-control">
			<label for="email" class="label label-text"> Email address </label>
			<input
				id="email"
				name="email"
				type="email"
				autocomplete="email"
				required
				placeholder="Your Email"
				class="input input-bordered input-primary"
			/>
		</div>

		<div class="form-control">
			<label for="password" class="label label-text"> Password </label>
			<input
				id="password"
				name="password"
				type="password"
				autocomplete="new-password"
				required
				placeholder="Your Password"
				class="input input-bordered input-primary"
			/>
		</div>

		<div class="form-control">
			<label for="confirm-password" class="label label-text"> Confirm Password </label>
			<input
				id="confirm-password"
				name="confirm-password"
				type="password"
				autocomplete="new-password"
				required
				placeholder="Your Password (again)"
				class="input input-bordered input-primary"
			/>
		</div>

		<button type="submit" class="btn btn-lg btn-primary w-full">Register</button>

		<div class="text-sm text-base-content text-center">
			Already have an account? <a href="login" class="text-secondary hover:text-secondary-focus">
				Sign in
			</a>
		</div>
	</form>
</div>
