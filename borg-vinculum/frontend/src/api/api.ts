import { headers } from "./helper";
import { Err, Ok, Result } from "../utils/result";
import { ApiError, parseError, StatusCode } from "./error";
import { toast } from "react-toastify";

export const Api = {
    auth: {
        test: async (): Promise<"logged out" | "logged in" | "verified"> => {
            const res = await fetch("/api/frontend/v1/test");
            if (res.status === 200) {
                return "logged in";
            } else {
                const error = await parseError(res);
                switch (error.status_code) {
                    case StatusCode.Unauthenticated:
                        return "logged out";
                    default:
                        toast.error(error.message);
                        return "logged out";
                }
            }
        },
        login: async (username: string, password: string): Promise<Result<null, ApiError>> => {
            const res = await fetch("/api/frontend/v1/auth/login", {
                method: "post",
                body: JSON.stringify({ username: username, password: password }),
                headers,
            });
            if (res.status === 200) {
                return Ok(null);
            } else {
                return Err(await parseError(res));
            }
        },
        logout: async (): Promise<Result<null, ApiError>> => {
            const res = await fetch("/api/frontend/v1/auth/logout", {
                method: "get",
                headers,
            });
            if (res.status === 200) {
                return Ok(null);
            } else {
                return Err(await parseError(res));
            }
        },
    },
};
