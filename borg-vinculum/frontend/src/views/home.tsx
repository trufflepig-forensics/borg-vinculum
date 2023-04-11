import React from "react";
import { DroneStat, GetDroneResponse } from "../api/generated";
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
    TimeSeriesScale,
    LineElement,
    PointElement,
    Title,
    Tooltip,
} from "chart.js";
import { Line } from "react-chartjs-2";
import CheckBox from "../components/checkbox";
import { de } from "date-fns/locale";
import "chartjs-adapter-date-fns";

ChartJS.register(CategoryScale, LinearScale, TimeSeriesScale, PointElement, LineElement, Title, Tooltip, Legend);

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
    deleteDronePopup: UUID | null;
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
            deleteDronePopup: null,

            showNfiles: false,
            showOriginalSize: true,
            showCompressedSize: true,
            showDeduplicatedSize: true,
        };

        this.createDrone = this.createDrone.bind(this);
        this.updateDrones = this.updateDrones.bind(this);
        this.transformDroneStats = this.transformDroneStats.bind(this);
        this.deleteDrone = this.deleteDrone.bind(this);
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

    deleteDrone(uuid: UUID) {
        return async () => {
            (await Api.drones.delete(uuid)).match(
                () => {
                    toast.success("Drone got deleted");
                    this.setState({ deleteDronePopup: null });
                    this.updateDrones();
                },
                (err) => toast.error(err.message)
            );
        };
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
            datasets.push({ label: "deduplicated size", data: deduplicatedSize, borderColor: "orange" });
        }

        return {
            labels: this.state.droneData.map((x) => x.createdAt),
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
                <div className={"container-entry"}>
                    <div onClick={this.fetchDroneStats(drone.uuid)}>
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
                    <button
                        className={"delete-button"}
                        type={"button"}
                        onClick={() => {
                            this.setState({ deleteDronePopup: drone.uuid });
                        }}
                    >
                        <svg fill="#eee" width="800px" height="800px" viewBox="0 0 24 24">
                            <path d="M22,5H17V2a1,1,0,0,0-1-1H8A1,1,0,0,0,7,2V5H2A1,1,0,0,0,2,7H3.117L5.008,22.124A1,1,0,0,0,6,23H18a1,1,0,0,0,.992-.876L20.883,7H22a1,1,0,0,0,0-2ZM9,3h6V5H9Zm8.117,18H6.883L5.133,7H18.867Z" />
                        </svg>
                    </button>
                </div>
            );
        }

        let popupDrone;
        if (!!this.state.dronePopup) {
            popupDrone = this.state.drones.filter((x) => x.uuid === this.state.dronePopup)[0];
        } else {
            popupDrone = null;
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
                                <Line
                                    datasetIdKey={"id"}
                                    options={{
                                        responsive: true,
                                        plugins: {
                                            legend: {
                                                position: "top" as const,
                                            },
                                        },
                                        animation: false,
                                        scales: {
                                            x: {
                                                type: "timeseries",
                                                time: {
                                                    unit: "minute",
                                                },
                                                adapters: {
                                                    date: {
                                                        locale: de,
                                                    },
                                                },
                                            },
                                        },
                                    }}
                                    data={this.transformDroneStats()}
                                />
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
                <Popup
                    open={!!this.state.deleteDronePopup}
                    onClose={() => {
                        this.setState({ deleteDronePopup: null });
                    }}
                    closeOnDocumentClick={true}
                    modal={true}
                    nested={true}
                >
                    <h3 className={"heading"}>Confirm deletion</h3>
                    <p>Are you sure that you want to delete the drone?</p>
                    <div className={"confirm-dialog"}>
                        <button
                            className={"button"}
                            onClick={
                                !!this.state.deleteDronePopup
                                    ? this.deleteDrone(this.state.deleteDronePopup)
                                    : undefined
                            }
                        >
                            I'm sure
                        </button>
                        <button
                            className={"button"}
                            onClick={() => {
                                this.setState({ deleteDronePopup: null });
                            }}
                        >
                            Let me reconsider
                        </button>
                    </div>
                </Popup>
            </>
        );
    }
}
