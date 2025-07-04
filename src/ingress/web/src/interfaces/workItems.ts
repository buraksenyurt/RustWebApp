import {WorkItem} from "./workItem";

export interface WorkItems {
    ready: WorkItem[];
    inProgress: WorkItem[];
    completed: WorkItem[];
}