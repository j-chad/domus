<div>
	<a class="text-base-content text-medium" href="/">Domus</a>
	<h2 class="mt-6 text-3xl font-extrabold text-primary">Sign in.</h2>
</div>

<div class="mt-8">
	<form action="#" method="POST" class="space-y-6">
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
				autocomplete="current-password"
				required
				placeholder="Your Password"
				class="input input-bordered input-primary"
			/>
		</div>

		<div class="flex items-center justify-between">
			<div class="form-control">
				<label class="label cursor-pointer">
					<input type="checkbox" class="checkbox checkbox-primary" />
					<span class="label-text ml-2 text-base-content">Remember me</span>
				</label>
			</div>

			<div class="text-sm">
				<a href="account-recovery" class="text-secondary hover:text-secondary-focus">
					Forgot your password?
				</a>
			</div>
		</div>

		<div>
			<button type="submit" class="btn btn-lg btn-primary w-full">Sign in</button>
		</div>

		<div class="text-sm text-base-content text-center">
			Not registered? <a href="register" class="text-secondary hover:text-secondary-focus">
				Create an account
			</a>
		</div>
	</form>
</div>
