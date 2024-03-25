import { invoke } from '@tauri-apps/api/tauri';

export async function invokeAndSet<T>(
  setValue: (value: T) => void,
  command: string,
) {
  try {
    const result: T = await invoke(command);
    setValue(result);
  } catch (err) {
    console.error(err);
  }
}

export async function invokeAndSetWithArgs<T>(
  setValue: (value: T) => void,
  command: string,
  args: Record<string, unknown>,
) {
  try {
    const result: T = await invoke(command, args);
    setValue(result);
  } catch (err) {
    console.error(err);
  }
}
