import Input, { InputConfig, InputValue } from "./kinds/Input";
import Checkbox, { CheckboxConfig, CheckboxValue } from "./kinds/Checkbox";
import Switch from "./kinds/Switch";
import Select from "./kinds/Select";
import FieldKind from "./kind";
import Radio from "./kinds/Radio";

type FieldValue = CheckboxValue | InputValue;

type FieldConfig = CheckboxConfig | InputConfig;

export type FieldHandler = (
	key: string,
	value: FieldValue,
	valid: boolean,
) => void;

export type FieldValueHandler = (value: FieldValue) => void;

const Field = (props: {
	kind: FieldKind;
	fieldKey: string;
	label?: string;
	value: FieldValue;
	valid: boolean;
	config: FieldConfig;
	handleField: FieldHandler;
}) => {
	function handleField(value) {
		switch (props.kind) {
			case FieldKind.CHECKBOX:
				props.handleField(props.fieldKey, value, value);
				break;
			case FieldKind.SWITCH:
			case FieldKind.RADIO:
				props.handleField(props.fieldKey, value, true);
				break;
			case FieldKind.SELECT:
				props.handleField(
					props.fieldKey,
					{ ...props.value, selected: value },
					true,
				);
				break;
			case FieldKind.INPUT:
			case FieldKind.NUMBER:
				const config = props.config as InputConfig;
				props.handleField(
					props.fieldKey,
					value,
					config.validate ? config.validate(value) : true,
				);
				break;
		}
	}

	function getField() {
		switch (props.kind) {
			case FieldKind.CHECKBOX:
				return (
					<Checkbox
						value={props.value as CheckboxValue}
						config={props.config as CheckboxConfig}
						handleField={handleField}
					/>
				);
			case FieldKind.SWITCH:
				return (
					<Switch
						value={props.value}
						config={props.config}
						handleField={handleField}
					/>
				);
			case FieldKind.SELECT:
				return (
					<Select
						value={props.value}
						config={props.config}
						handleField={handleField}
					/>
				);
			case FieldKind.RADIO:
				return (
					<Radio
						value={props.value}
						config={props.config}
						user={props.user}
						path_params={props.path_params}
						handleField={handleField}
					/>
				);
			case FieldKind.INPUT:
			case FieldKind.NUMBER:
				return (
					<Input
						value={props.value as InputValue}
						valid={props.valid}
						config={props.config as InputConfig}
						handleField={handleField}
					/>
				);
			default:
				return <div>UNKNOWN FIELD</div>;
		}
	}

	function shouldValidate() {
		switch (props.kind) {
			case FieldKind.CHECKBOX:
			case FieldKind.SWITCH:
			case FieldKind.SELECT:
			case FieldKind.RADIO:
				return false;
			case FieldKind.INPUT:
			case FieldKind.NUMBER:
				return true;
		}
	}

	return (
		<div class="field">
			{props.label && <label class="label is-medium">{props.label}</label>}
			{getField()}
			{shouldValidate() && props.valid === false && (
				<p class="help is-danger">{props.config.help}</p>
			)}
		</div>
	);
};

export default Field;
