
export interface IProject {
    title: string
    short_description: string
    path: string
    markdown: string
    has_readme: boolean
}

export type IProjects = IProject[]