type Success<T> = [data: T, error: never];
type Failure<E> = [data: never, error: E];
export type Result<T, E = Error> = Success<T> | Failure<E>;

/** Implements try/catch for a given promise.
 *
 * If the promise resolves, returns an object with a `data` property. If the promise rejects, returns an object with an `error` property.
 * @template T the type of data to return if the promise resolves successfully.
 * @template E the type of error to return. Defaults to `Error`.
 * @param promise the promise to implement try/catch for.
 * @example
 * const [data, error] = await tryCatch(getData());
 * if (error) return; // handle the error
 * doSomething(data); // data can now be used
 */
export async function tryCatch<T, E = Error>(promise: Promise<T>): Promise<Result<T, E>> {
  try {
    const data = await promise;
    return [data, undefined] as Success<T>;
  } catch (error) {
    return [undefined, error as E] as Failure<E>;
  }
}
