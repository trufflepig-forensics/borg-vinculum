import React from "react";
import { DroneStat, GetDroneResponse, GetDroneStats } from "../api/generated";
import { Api } from "../api/api";
import { toast } from "react-toastify";
import Popup from "reactjs-popup";
import "../styling/home.css";
import Input from "../components/input";
import { UUID } from "../api/schemas";
import {
    CategoryScale,
    Chart as ChartJS,
    Legend,
    LinearScale,
    LineElement,
    PointElement,
    Title,
    Tooltip,
} from "chart.js";
import { Line } from "react-chartjs-2";
import CheckBox from "../components/checkbox";

ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Legend);

export const options = {
    responsive: true,
    plugins: {
        legend: {
            position: "top" as const,
        },
    },
};

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
    dronePopup: UUID | null;
    droneData: Array<DroneStat>;

    showNfiles: boolean;
    showOriginalSize: boolean;
    showCompressedSize: boolean;
    showDeduplicatedSize: boolean;
};

export default class Home extends React.Component<HomeProps, HomeState> {
    constructor(props: HomeProps) {
        super(props);

        this.state = {
            publicKey: "",
            drones: [],
            newDrone: null,
            dronePopup: null,
            droneData: [],

            showNfiles: false,
            showOriginalSize: true,
            showCompressedSize: true,
            showDeduplicatedSize: true,
        };

        this.createDrone = this.createDrone.bind(this);
        this.updateDrones = this.updateDrones.bind(this);
        this.transformDroneStats = this.transformDroneStats.bind(this);
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
            async (_) => {
                this.setState({ newDrone: null });
                toast.update(t, { render: "Drone created", type: "success", isLoading: false, autoClose: 3500 });
                this.updateDrones();
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

    fetchDroneStats(uuid: string) {
        return async () => {
            (await Api.drones.stats(uuid)).match(
                (stats) => {
                    this.setState({ dronePopup: uuid, droneData: stats.stats });
                },
                (err) => toast.error(err.message)
            );
        };
    }

    transformDroneStats() {
        let labels = [];

        let nfiles = [];
        let originalSize = [];
        let compressedSize = [];
        let deduplicatedSize = [];

        for (let stats of this.state.droneData) {
            labels.push(stats.createdAt.toLocaleString());
            nfiles.push(stats.nfiles);
            originalSize.push(stats.originalSize);
            compressedSize.push(stats.compressedSize);
            deduplicatedSize.push(stats.deduplicatedSize);
        }

        let datasets = [];

        if (this.state.showNfiles) {
            datasets.push({ label: "nfiles", data: nfiles, borderColor: "white" });
        }

        if (this.state.showOriginalSize) {
            datasets.push({ label: "original size", data: originalSize, borderColor: "purple" });
        }

        if (this.state.showCompressedSize) {
            datasets.push({ label: "compressed size", data: compressedSize, borderColor: "red" });
        }

        if (this.state.showDeduplicatedSize) {
            datasets.push({ label: "deduplicated size", data: compressedSize, borderColor: "orange" });
        }

        return {
            labels: this.state.droneData.map((x) => x.createdAt.toLocaleString()),
            datasets: datasets,
        };
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
                <div className={"container-entry"} onClick={this.fetchDroneStats(drone.uuid)}>
                    <ul>
                        <li key={"name"}>{drone.name}</li>
                        <li key={"repository"}>
                            <p
                                className={"clickable-text"}
                                onClick={async (e) => {
                                    e.stopPropagation();
                                    await navigator.clipboard.writeText(drone.repository);
                                    toast.success("Copied repository to clipboard");
                                }}
                            >
                                {drone.repository}
                            </p>
                        </li>
                        <li key={"token"}>
                            <button
                                className={"icon-button"}
                                type={"button"}
                                onClick={async (e) => {
                                    e.stopPropagation();
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
                        <li key={"last-activity"}>
                            {drone.lastActivity !== undefined ? drone.lastActivity?.toLocaleString() : "N/A"}
                        </li>
                    </ul>
                </div>
            );
        }

        let popupDrone;
        if (!!this.state.dronePopup) {
            popupDrone = this.state.drones.filter((x) => x.uuid === this.state.dronePopup)[0];
        } else {
            popupDrone = null;
        }

        // @ts-ignore
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
                                <li key={"name"}>Name</li>
                                <li key={"repository"}>Repository</li>
                                <li key={"token"}>Token</li>
                                <li key={"last-activity"}>Last Activity</li>
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

                <Popup
                    open={!!this.state.dronePopup}
                    onClose={() => {
                        this.setState({ dronePopup: null });
                    }}
                    closeOnDocumentClick={true}
                    modal={true}
                    nested={true}
                >
                    <div className={"drone-stats-container"}>
                        <h3 className={"heading"}>
                            Statistics for <span className={"monospace"}>{popupDrone?.name}</span>
                        </h3>
                        <div className={"drone-stats"}>
                            <div className={"drone-stats-canvas"}>
                                <Line datasetIdKey={"id"} options={options} data={this.transformDroneStats()} />
                            </div>
                            <div className={"drone-stats-controls"}>
                                <CheckBox
                                    label={"nfiles"}
                                    value={this.state.showNfiles}
                                    onChange={() => {
                                        this.setState({ showNfiles: !this.state.showNfiles });
                                    }}
                                />
                                <CheckBox
                                    label={"original size"}
                                    value={this.state.showOriginalSize}
                                    onChange={() => {
                                        this.setState({ showOriginalSize: !this.state.showOriginalSize });
                                    }}
                                />
                                <CheckBox
                                    label={"compressed size"}
                                    value={this.state.showCompressedSize}
                                    onChange={() => {
                                        this.setState({ showCompressedSize: !this.state.showCompressedSize });
                                    }}
                                />
                                <CheckBox
                                    label={"deduplicated size"}
                                    value={this.state.showDeduplicatedSize}
                                    onChange={() => {
                                        this.setState({ showDeduplicatedSize: !this.state.showDeduplicatedSize });
                                    }}
                                />
                            </div>
                        </div>
                    </div>
                </Popup>
            </>
        );
    }
}
