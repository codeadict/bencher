import axios from "axios";
import { createEffect, createResource } from "solid-js";
import { get_options, pageTitle } from "../../../site/util";

const PerfHeader = (props) => {
	const getProject = async (fetcher) => {
		try {
			const url = props.config?.url(fetcher.project_slug);
			const resp = await axios(get_options(url, fetcher.token));
			return resp.data;
		} catch (error) {
			console.error(error);
			return [];
		}
	};

	const [project_data] = createResource(props.project_fetcher, getProject);

	createEffect(() => {
		pageTitle(project_data()?.name);
	});

	return (
		<nav class="level">
			<div class="level-left">
				<div class="level-item">
					<h3 class="title is-3" style="overflow-wrap:break-word;">
						{project_data()?.name}
					</h3>
				</div>
			</div>

			<div class="level-right">
				{project_data()?.url && (
					<div class="level-item">
						<a
							class="button is-outlined"
							href={project_data()?.url}
							rel="noreferrer nofollow"
							target="_blank"
						>
							<span class="icon">
								<i class="fas fa-globe" aria-hidden="true" />
							</span>
							<span>Website</span>
						</a>
					</div>
				)}
				<div class="level-item">
					<button
						class="button is-outlined"
						onClick={(e) => {
							e.preventDefault();
							navigator.clipboard.writeText(window.location.href);
						}}
					>
						<span class="icon">
							<i class="fas fa-link" aria-hidden="true" />
						</span>
						<span>Copy Link</span>
					</button>
				</div>
				<div class="level-item">
					<button
						class="button is-outlined"
						onClick={(e) => {
							e.preventDefault();
							props.handleRefresh();
						}}
					>
						<span class="icon">
							<i class="fas fa-sync-alt" aria-hidden="true" />
						</span>
						<span>Refresh</span>
					</button>
				</div>
			</div>
		</nav>
	);
};

export default PerfHeader;
