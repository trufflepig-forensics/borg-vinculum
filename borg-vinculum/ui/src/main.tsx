import React from "react";
import ReactDOM from "react-dom/client";

type RouterProps = {};
type RouterState = {};

class Router extends React.Component<RouterProps, RouterState> {
    constructor(props: RouterProps) {
        super(props);

        this.state = {};
    }

    render() {
        return <div></div>;
    }
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <>
        <Router />
    </>
);
