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
/**
 * The response to a get key request
 * @export
 * @interface GetKeyResponse
 */
export interface GetKeyResponse {
    /**
     * 
     * @type {string}
     * @memberof GetKeyResponse
     */
    publicKey: string;
}

export function GetKeyResponseFromJSON(json: any): GetKeyResponse {
    return GetKeyResponseFromJSONTyped(json, false);
}

export function GetKeyResponseFromJSONTyped(json: any, ignoreDiscriminator: boolean): GetKeyResponse {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'publicKey': json['public_key'],
    };
}

export function GetKeyResponseToJSON(value?: GetKeyResponse | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'public_key': value.publicKey,
    };
}


