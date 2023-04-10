import React from "react";
import { GetDroneResponse } from "../api/generated";
import { Api } from "../api/api";
import { toast } from "react-toastify";
import Popup from "reactjs-popup";
import "../styling/home.css";
import Input from "../components/input";

type Drone = {
    name: string;
    repository: string;
    passphrase: string;
};

type HomeProps = {};
type HomeState = {
    publicKey: string;
    drones: Array<GetDroneResponse>;
    newDrone: Drone | null;
};

export default class Home extends React.Component<HomeProps, HomeState> {
    constructor(props: HomeProps) {
        super(props);

        this.state = {
            publicKey: "",
            drones: [],
            newDrone: null,
        };

        this.createDrone = this.createDrone.bind(this);
        this.updateDrones = this.updateDrones.bind(this);
    }

    async createDrone(e: React.FormEvent<HTMLFormElement>) {
        e.preventDefault();

        if (this.state.newDrone === null) {
            console.log("newDrone is null");
            return;
        }

        const t = toast.loading("Creating drone");
        (
            await Api.drones.create({
                name: this.state.newDrone.name,
                repository: this.state.newDrone.repository,
                passphrase: this.state.newDrone.passphrase,
            })
        ).match(
            async (res) => {
                this.setState({ newDrone: null });
                toast.update(t, { render: "Drone created", type: "success", isLoading: false, autoClose: 3500 });
            },
            (err) => toast.update(t, { render: err.message, type: "error", isLoading: false, autoClose: 3500 })
        );
    }

    updateDrones() {
        Api.drones.all().then((x) => {
            x.match(
                (drones) => {
                    this.setState({ drones: drones.drones });
                },
                (err) => toast.error(err.message)
            );
        });
    }

    componentDidMount() {
        this.updateDrones();
        Api.key.get().then((x) =>
            x.match(
                (x) => {
                    this.setState({ publicKey: x.publicKey });
                },
                (err) => toast.error(err.message)
            )
        );
    }

    render() {
        let drones = [];
        for (const drone of this.state.drones) {
            drones.push(
                <div className={"container-entry"}>
                    <ul>
                        <li>{drone.name}</li>
                        <li>
                            <p
                                className={"clickable-text"}
                                onClick={async () => {
                                    await navigator.clipboard.writeText(drone.repository);
                                    toast.success("Copied repository to clipboard");
                                }}
                            >
                                {drone.repository}
                            </p>
                        </li>
                        <li>
                            <button
                                className={"icon-button"}
                                type={"button"}
                                onClick={async () => {
                                    await navigator.clipboard.writeText(drone.token);
                                    toast.success("Copied token to clipboard");
                                }}
                            >
                                <svg fill="#eee" width="800px" height="800px" viewBox="0 0 32 32" version="1.1">
                                    <title>clipboard</title>
                                    <path d="M2.016 30.016q0 0.832 0.576 1.408t1.408 0.576h24q0.832 0 1.408-0.576t0.608-1.408v-26.016q0-0.832-0.608-1.408t-1.408-0.576h-4v4h2.016v21.984h-20v-21.984h1.984v-4h-4q-0.832 0-1.408 0.576t-0.576 1.408v26.016zM8 26.016h16v-18.016h-1.984q0 0.832-0.608 1.44t-1.408 0.576h-8q-0.832 0-1.408-0.576t-0.576-1.44h-2.016v18.016zM10.016 6.016q0 0.832 0.576 1.408t1.408 0.576h8q0.832 0 1.408-0.576t0.608-1.408v-4h-4v-2.016h-4v2.016h-4v4zM14.016 6.016v-2.016h4v2.016h-4z"></path>
                                </svg>
                            </button>
                        </li>
                    </ul>
                </div>
            );
        }

        return (
            <>
                <div className={"container"}>
                    <div className={"container-heading"}>
                        <button
                            className={"button"}
                            onClick={() => {
                                this.setState({ newDrone: { name: "", passphrase: "", repository: "" } });
                            }}
                        >
                            Create drone
                        </button>
                    </div>
                    <div className={"container-list"}>
                        <div>
                            <ul>
                                <li>Name</li>
                                <li>Repository</li>
                                <li>Token</li>
                            </ul>
                        </div>
                        {drones}
                    </div>
                </div>
                <Popup
                    open={!!this.state.newDrone}
                    onClose={() => {
                        this.setState({ newDrone: null });
                    }}
                    closeOnDocumentClick={true}
                    modal={true}
                    nested={true}
                >
                    <form method={"post"} onSubmit={this.createDrone}>
                        <div className={"createDronePopup"}>
                            <label htmlFor={"name"}>Drone name</label>
                            <Input
                                id={"name"}
                                required
                                value={this.state.newDrone?.name}
                                onChange={(v: string) => {
                                    if (!!this.state.newDrone) {
                                        let drone = this.state.newDrone;
                                        drone.name = v;
                                        this.setState({ newDrone: drone });
                                    }
                                }}
                            />
                            <label htmlFor={"repository"}>Borg repository</label>
                            <Input
                                id={"repository"}
                                required
                                value={this.state.newDrone?.repository}
                                onChange={(v: string) => {
                                    if (!!this.state.newDrone) {
                                        let drone = this.state.newDrone;
                                        drone.repository = v;
                                        this.setState({ newDrone: drone });
                                    }
                                }}
                            />
                            <label htmlFor={"passphrase"}>Borg passphrase</label>
                            <Input
                                id={"passphrase"}
                                required
                                type="password"
                                value={this.state.newDrone?.passphrase}
                                onChange={(v: string) => {
                                    if (!!this.state.newDrone) {
                                        let drone = this.state.newDrone;
                                        drone.passphrase = v;
                                        this.setState({ newDrone: drone });
                                    }
                                }}
                            />
                            <p>Make sure the vinculum has access to the repository</p>
                            <div className={"public-key-container"}>
                                <Input editable={false} value={this.state.publicKey} />
                                <button
                                    className={"icon-button"}
                                    type={"button"}
                                    onClick={async () => {
                                        await navigator.clipboard.writeText(this.state.publicKey);
                                        toast.success("Copied to clipboard");
                                    }}
                                >
                                    <svg fill="#eee" width="800px" height="800px" viewBox="0 0 32 32" version="1.1">
                                        <title>clipboard</title>
                                        <path d="M2.016 30.016q0 0.832 0.576 1.408t1.408 0.576h24q0.832 0 1.408-0.576t0.608-1.408v-26.016q0-0.832-0.608-1.408t-1.408-0.576h-4v4h2.016v21.984h-20v-21.984h1.984v-4h-4q-0.832 0-1.408 0.576t-0.576 1.408v26.016zM8 26.016h16v-18.016h-1.984q0 0.832-0.608 1.44t-1.408 0.576h-8q-0.832 0-1.408-0.576t-0.576-1.44h-2.016v18.016zM10.016 6.016q0 0.832 0.576 1.408t1.408 0.576h8q0.832 0 1.408-0.576t0.608-1.408v-4h-4v-2.016h-4v2.016h-4v4zM14.016 6.016v-2.016h4v2.016h-4z"></path>
                                    </svg>
                                </button>
                            </div>
                            <button className={"button"}>Create drone</button>
                        </div>
                    </form>
                </Popup>
            </>
        );
    }
}
