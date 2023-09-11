<section class="overflow-hidden">
	<div class="flex overflow-hidden">
		<div class="relative flex-1 hidden w-0 overflow-hidden lg:block">
			<img
				class="absolute inset-0 object-cover w-full h-full bg-black"
				src="/assets/images/placeholders/rectangleWide.png"
				alt=""
			/>
		</div>
		<div
			class="flex flex-col justify-center flex-1 px-4 py-12 overflow-hidden sm:px-6 lg:flex-none lg:px-20 xl:px-24 bg-base-100"
		>
			<div class="w-full max-w-xl mx-auto lg:w-96">
				<div>
					<a class="text-base-content text-medium" href="/">Domus</a>
					<h2 class="mt-6 text-3xl font-extrabold text-primary">Sign in.</h2>
				</div>

				<div class="mt-8">
					<form action="#" method="POST" class="space-y-6" data-bitwarden-watching="1">
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
								<a href="/account-recovery" class="text-secondary hover:text-secondary-focus">
									Forgot your password?
								</a>
							</div>
						</div>

						<div>
							<button type="submit" class="btn btn-lg btn-primary w-full">Sign in</button>
						</div>

						<div class="text-sm text-base-content text-center">
							Not registered? <a href="/register" class="text-secondary hover:text-secondary-focus">
								Create an account
							</a>
						</div>
					</form>
				</div>
			</div>
		</div>
	</div>
</section>
