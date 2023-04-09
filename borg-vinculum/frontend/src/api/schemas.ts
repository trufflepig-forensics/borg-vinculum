/** Database id i.e. and u32 */
export type ID = number;

/** Hyphen separated uuid */
export type UUID = string;

/** RFC3339 encoded date time */
export type RFC3339 = string;

/** Like `Partial<T>` but makes fields `| null` instead of `?` */
type Optional<T> = { [P in keyof T]: T[P] | null };
