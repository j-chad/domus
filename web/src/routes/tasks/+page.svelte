<script lang="ts">
	interface TaskData {
		completed: boolean;
		taskName: string;
		assignee: string;
		dueDate: string;
		labels: string[];
	}

	const tableHeadings = ['', 'Task', 'Assignee', 'Due Date', 'Tags'];
	const tableData: TaskData[] = [
		{
			completed: false,
			taskName: 'Vaccum',
			assignee: 'Yulia',
			dueDate: '30/09/2022',
			labels: ['Shared']
		},
		{
			completed: false,
			taskName: 'Mop',
			assignee: 'Jackson',
			dueDate: '31/09/2022',
			labels: ['Shared']
		},
		{
			completed: true,
			taskName: 'Clean Bathroom',
			assignee: 'Yulia',
			dueDate: '31/09/2022',
			labels: ["Yulia's Room"]
		}
	];
</script>

<main class="font-body p-10 min-h-full bg-base-100">
	<div class="flex flex-col gap-10">
		<h1 class="text-4xl font-bold flex-none">Task Tracker</h1>
		<div class="w-2/3 grow flex flex-col gap-4">
			<div class="flex-none flex flex-row justify-end gap-2">
                <details class='dropdown'>
                    <summary class='btn btn-sm btn-outline'>Label</summary>
                    <ul class='shadow menu dropdown-content z-[1] bg-base-100 rounded-box'>
                        <li>
                            <span class="badge badge-secondary">Shared</span>
                        </li>
                        <li>
                            <span class="badge badge-secondary">Yulia's Room</span>
                        </li>
                    </ul>
                </details>
				<button class="btn btn-sm btn-primary">New Task</button>
			</div>
				<table class="table grow">
					<thead>
						<tr>
							{#each tableHeadings as heading}
								<th>{heading}</th>
							{/each}
						</tr>
					</thead>
					<tbody>
						{#each tableData as task}
							<tr>
								<td>
									<input type="checkbox" bind:checked={task.completed} />
								</td>
								<td>{task.taskName}</td>
								<td>{task.assignee}</td>
								<td>{task.dueDate}</td>
								<td>
									{#each task.labels as label}
										<span class="badge badge-secondary">{label}</span>
									{/each}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
		</div>
	</div>
</main>
