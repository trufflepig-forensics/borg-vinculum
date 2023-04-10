import React from "react";

type CheckBoxProps = {
    label: string;
    value: boolean;
    onChange: () => void;
};
type CheckBoxState = {};

export default class CheckBox extends React.Component<CheckBoxProps, CheckBoxState> {
    constructor(props: CheckBoxProps) {
        super(props);

        this.state = {};
    }

    render() {
        return (
            <div className={"checkbox"}>
                <div
                    onClick={() => {
                        this.props.onChange();
                    }}
                >
                    <div className={this.props.value ? "checkbox-checked" : ""}></div>
                </div>
                <span>{this.props.label}</span>
            </div>
        );
    }
}
