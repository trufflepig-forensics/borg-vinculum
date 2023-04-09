import React from "react";
import { Api } from "../api/api";
import { toast } from "react-toastify";
import Input from "../components/input";

import "../styling/login.css";
import { ROUTES } from "../utils/router";

type LoginProps = {};
type LoginState = {
    username: string;
    password: string;
};

export default class Login extends React.Component<LoginProps, LoginState> {
    constructor(props: LoginProps) {
        super(props);

        this.state = {
            username: "",
            password: "",
        };

        this.performLogin = this.performLogin.bind(this);
    }

    async performLogin(e: React.FormEvent<HTMLFormElement>) {
        e.preventDefault();

        (await Api.auth.login(this.state.username, this.state.password)).match(
            async (_) => {
                toast.success("Authenticated successfully");
                document.location.hash = "/";
            },
            (err) => toast.error(err.message)
        );
    }
    render() {
        return (
            <div className="login-container">
                <form method="post" onSubmit={this.performLogin}>
                    <div className="login">
                        <h1 className="heading">Login</h1>
                        <Input
                            required
                            value={this.state.username}
                            onChange={(v: string) => {
                                this.setState({ username: v });
                            }}
                        />
                        <Input
                            required
                            type="password"
                            value={this.state.password}
                            onChange={(v: string) => {
                                this.setState({ password: v });
                            }}
                        />
                        <button className="button">Login</button>
                    </div>
                </form>
            </div>
        );
    }
}
