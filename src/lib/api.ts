import { invoke } from "@tauri-apps/api/tauri";
import { IProjects } from "../types/Project"
import { ISettings } from "../types/Settings"

export async function get_settings(): Promise<ISettings> {
  return await invoke("get_settings", {});
}

export async function update_projects_list(): Promise<void> {
  return await invoke("update_projects_list", {});
}

export async function get_projects_list(): Promise<IProjects> {
  return await invoke("get_projects_list", {});
}
