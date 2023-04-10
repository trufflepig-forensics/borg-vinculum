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
 * A single stat record of a drone
 * @export
 * @interface DroneStat
 */
export interface DroneStat {
    /**
     * 
     * @type {number}
     * @memberof DroneStat
     */
    createDuration: number;
    /**
     * 
     * @type {number}
     * @memberof DroneStat
     */
    completeDuration: number;
    /**
     * 
     * @type {number}
     * @memberof DroneStat
     */
    originalSize: number;
    /**
     * 
     * @type {number}
     * @memberof DroneStat
     */
    compressedSize: number;
    /**
     * 
     * @type {number}
     * @memberof DroneStat
     */
    deduplicatedSize: number;
    /**
     * 
     * @type {number}
     * @memberof DroneStat
     */
    nfiles: number;
    /**
     * 
     * @type {Date}
     * @memberof DroneStat
     */
    createdAt: Date;
    /**
     * 
     * @type {number}
     * @memberof DroneStat
     */
    preHookDuration?: number | null;
    /**
     * 
     * @type {number}
     * @memberof DroneStat
     */
    postHookDuration?: number | null;
}

export function DroneStatFromJSON(json: any): DroneStat {
    return DroneStatFromJSONTyped(json, false);
}

export function DroneStatFromJSONTyped(json: any, ignoreDiscriminator: boolean): DroneStat {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'createDuration': json['create_duration'],
        'completeDuration': json['complete_duration'],
        'originalSize': json['original_size'],
        'compressedSize': json['compressed_size'],
        'deduplicatedSize': json['deduplicated_size'],
        'nfiles': json['nfiles'],
        'createdAt': (new Date(json['created_at'])),
        'preHookDuration': !exists(json, 'pre_hook_duration') ? undefined : json['pre_hook_duration'],
        'postHookDuration': !exists(json, 'post_hook_duration') ? undefined : json['post_hook_duration'],
    };
}

export function DroneStatToJSON(value?: DroneStat | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'create_duration': value.createDuration,
        'complete_duration': value.completeDuration,
        'original_size': value.originalSize,
        'compressed_size': value.compressedSize,
        'deduplicated_size': value.deduplicatedSize,
        'nfiles': value.nfiles,
        'created_at': (value.createdAt.toISOString()),
        'pre_hook_duration': value.preHookDuration,
        'post_hook_duration': value.postHookDuration,
    };
}

