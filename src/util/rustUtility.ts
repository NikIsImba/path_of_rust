import { invoke } from '@tauri-apps/api/tauri';

export async function invokeAndSet<T>(setValue: (value: T) => void, command: string) {
  try {
    const result: T = await invoke(command);
    setValue(result);
  } catch (err) {
    console.error(err);
  }
}