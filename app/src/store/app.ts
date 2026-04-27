import { create } from "zustand";
import { IronbirdProject } from "@/bindings";

interface IronbirdAppState {
  activeProject: IronbirdProject | null
  titlebarLabel: string
  setActiveProject: (project: IronbirdProject | null) => void
  setTitlebarLabel: (label: string) => void
}

export const useIronbirdAppStore = create<IronbirdAppState>((set) => ({
  activeProject: null,
  titlebarLabel: 'Ironbird',
  setActiveProject: (project) => set({ activeProject: project }),
  setTitlebarLabel: (label) => set({ titlebarLabel: label }),
}))
