/* tslint:disable */
/* eslint-disable */
/**
 * borg-vinculum
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * Contact: git@omikron.dev
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


import * as runtime from '../runtime';
import {
    ApiErrorResponse,
    ApiErrorResponseFromJSON,
    ApiErrorResponseToJSON,
    GetKeyResponse,
    GetKeyResponseFromJSON,
    GetKeyResponseToJSON,
} from '../models';

/**
 * KeyApi - interface
 * 
 * @export
 * @interface KeyApiInterface
 */
export interface KeyApiInterface {
    /**
     * Request the public key of the server
     * @summary Request the public key of the server
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof KeyApiInterface
     */
    getKeyRaw(): Promise<runtime.ApiResponse<GetKeyResponse>>;

    /**
     * Request the public key of the server
     * Request the public key of the server
     */
    getKey(): Promise<GetKeyResponse>;

}

/**
 * 
 */
export class KeyApi extends runtime.BaseAPI implements KeyApiInterface {

    /**
     * Request the public key of the server
     * Request the public key of the server
     */
    async getKeyRaw(): Promise<runtime.ApiResponse<GetKeyResponse>> {
        const queryParameters: runtime.HTTPQuery = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/frontend/v1/key`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        });

        return new runtime.JSONApiResponse(response, (jsonValue) => GetKeyResponseFromJSON(jsonValue));
    }

    /**
     * Request the public key of the server
     * Request the public key of the server
     */
    async getKey(): Promise<GetKeyResponse> {
        const response = await this.getKeyRaw();
        return await response.value();
    }

}
