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

import { exists, mapValues } from '../runtime';
import {
    ApiStatusCode,
    ApiStatusCodeFromJSON,
    ApiStatusCodeFromJSONTyped,
    ApiStatusCodeToJSON,
} from './';

/**
 * The Response that is returned in case of an error
 * 
 * For client errors the HTTP status code will be 400,
 * for server errors the 500 will be used.
 * @export
 * @interface ApiErrorResponse
 */
export interface ApiErrorResponse {
    /**
     * 
     * @type {string}
     * @memberof ApiErrorResponse
     */
    message: string;
    /**
     * 
     * @type {ApiStatusCode}
     * @memberof ApiErrorResponse
     */
    statusCode: ApiStatusCode;
}

export function ApiErrorResponseFromJSON(json: any): ApiErrorResponse {
    return ApiErrorResponseFromJSONTyped(json, false);
}

export function ApiErrorResponseFromJSONTyped(json: any, ignoreDiscriminator: boolean): ApiErrorResponse {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'message': json['message'],
        'statusCode': ApiStatusCodeFromJSON(json['status_code']),
    };
}

export function ApiErrorResponseToJSON(value?: ApiErrorResponse | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'message': value.message,
        'status_code': ApiStatusCodeToJSON(value.statusCode),
    };
}


