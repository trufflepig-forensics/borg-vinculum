import React from "react";
import ReactDOM from "react-dom/client";
import "./index.css";
import "./styling/components.css";
import { ROUTES } from "./utils/router";
import Background from "./views/background";
import Login from "./views/login";

type RouterProps = {};
type RouterState = {
    path: Array<string>;
};

class Router extends React.Component<RouterProps, RouterState> {
    constructor(props: RouterProps) {
        super(props);

        this.state = {
            path: [],
        };
    }

    componentDidMount() {
        // Update state to match url
        const setPath = () => {
            const rawPath = window.location.hash;

            // Ensure well-formed path i.e. always have a #/
            if (!rawPath.startsWith("#/")) {
                window.location.hash = "#/";

                // this method will be immediately triggered again
                return;
            }

            // Split everything after #/
            const path = rawPath.substring(2).split("/");

            // #/ should result in [] not [""]
            if (path.length === 1 && path[0] === "") {
                path.shift();
            }

            this.setState({ path });
        };

        setPath();
        window.addEventListener("hashchange", setPath);
    }

    render() {
        const { path } = this.state;

        if (path[0] && path[0] === "login") {
            return (
                <div className="container">
                    <Login />
                </div>
            );
        }

        let content = undefined;
        for (const route of Object.values(ROUTES)) {
            const params = route.match(path);
            if (params !== undefined) {
                content = route.config.render(params);
                break;
            }
        }

        if (content === undefined) {
            content = <div>Unknown route</div>;
        }

        return <div className="container">{content}</div>;
    }
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <>
        <Router />
        <Background />
    </>
);
