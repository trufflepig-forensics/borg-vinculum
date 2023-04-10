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
 * The request to create a new drone
 * @export
 * @interface CreateDroneRequest
 */
export interface CreateDroneRequest {
    /**
     * 
     * @type {string}
     * @memberof CreateDroneRequest
     */
    name: string;
    /**
     * 
     * @type {string}
     * @memberof CreateDroneRequest
     */
    repository: string;
    /**
     * 
     * @type {string}
     * @memberof CreateDroneRequest
     */
    passphrase: string;
}

export function CreateDroneRequestFromJSON(json: any): CreateDroneRequest {
    return CreateDroneRequestFromJSONTyped(json, false);
}

export function CreateDroneRequestFromJSONTyped(json: any, ignoreDiscriminator: boolean): CreateDroneRequest {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'name': json['name'],
        'repository': json['repository'],
        'passphrase': json['passphrase'],
    };
}

export function CreateDroneRequestToJSON(value?: CreateDroneRequest | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'name': value.name,
        'repository': value.repository,
        'passphrase': value.passphrase,
    };
}


