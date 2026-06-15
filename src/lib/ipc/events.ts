import { listen } from '@tauri-apps/api/event';

export type EventCallback<T> = (payload: T) => void;

/**
 * Generic event listener helper.
 */
export async function listenToEvent<T>(eventName: string, callback: EventCallback<T>) {
  return listen<T>(eventName, (event) => {
    callback(event.payload);
  });
}
