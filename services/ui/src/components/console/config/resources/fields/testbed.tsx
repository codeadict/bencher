import { validate_string } from "../../../../site/util";
import { is_valid_non_empty, is_valid_slug } from "bencher_valid";

const TESTBED_FIELDS = {
	name: {
		type: "text",
		placeholder: "Testbed Name",
		icon: "fas fa-server",
		help: "Must be a non-empty string",
		validate: (input) => validate_string(input, is_valid_non_empty),
	},
	slug: {
		type: "text",
		placeholder: "Testbed Slug",
		icon: "fas fa-exclamation-triangle",
		help: "Must be a valid slug",
		validate: (input) => validate_string(input, is_valid_slug),
	},
};

export default TESTBED_FIELDS;
