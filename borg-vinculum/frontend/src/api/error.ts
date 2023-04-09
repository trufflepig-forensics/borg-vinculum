import { Err, Ok, Result } from "../utils/result";

export enum StatusCode {
    ArbitraryJSError = -2,
    JsonDecodeError = -1,

    Unauthenticated = 1000,
    NotFound = 1001,
    InvalidContentType = 1002,
    InvalidJson = 1003,
    PayloadOverflow = 1004,

    LoginFailed = 1005,
    InvalidPassword = 1006,
    NameAlreadyExists = 1007,
    InvalidName = 1008,
    ListRepositoryError = 1009,
    RepositoryAlreadyExists = 1010,
    InvalidUuid = 1011,

    InternalServerError = 2000,
    DatabaseError = 2001,
    SessionError = 2002,
}

export type ApiError = {
    status_code: StatusCode;
    message: string;
};

/**
 * Wraps a promise returned by the generated SDK which handles its errors and returns a {@link Result}
 */
export async function handleError<T>(promise: Promise<T>): Promise<Result<T, ApiError>> {
    try {
        return Ok(await promise);
    } catch (e) {
        if (e instanceof Response) {
            return Err(await parseError(e));
        } else {
            return Err({
                status_code: StatusCode.ArbitraryJSError,
                message: "The server's response was invalid json",
            });
        }
    }
}

/**
 * Parse a response's body into an {@link ApiError}
 *
 * This function assumes but doesn't check, that the response is an error.
 */
export async function parseError(response: Response): Promise<ApiError> {
    try {
        return await response.json();
    } catch {
        console.error("Got invalid json", response.body);
        return {
            status_code: StatusCode.JsonDecodeError,
            message: "The server's response was invalid json",
        };
    }
}
