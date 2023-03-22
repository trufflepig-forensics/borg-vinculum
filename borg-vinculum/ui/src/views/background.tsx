import React from "react";

import "../styling/background.css";

type BackgroundProps = {};
type BackgroundState = {};

export default class Background extends React.Component<BackgroundProps, BackgroundState> {
    constructor(props: BackgroundProps) {
        super(props);

        this.state = {};
    }

    render() {
        return (
            <div className="background">
                <div className="stars"></div>
                <div className="twinkleMask"></div>
                <div className="twinkleMask2"></div>
            </div>
        );
    }
}
