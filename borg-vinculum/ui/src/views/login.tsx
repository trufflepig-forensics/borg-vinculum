import React from "react";

import "../styling/login.css";
import Input from "../components/input";

type LoginProps = {};
type LoginState = {};

export default class Login extends React.Component<LoginProps, LoginState> {
    constructor(props: LoginProps) {
        super(props);

        this.state = {};
    }

    render() {
        return (
            <div className="centered">
                <div className="login">
                    <h1>Login</h1>
                    <Input className="input" placeholder="Username" />
                    <button className="button">Authenticate</button>
                </div>
            </div>
        );
    }
}
