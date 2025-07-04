import {WorkItem} from "./workItem";

export interface WorkItems {
    ready: WorkItem[];
    in_progress: WorkItem[];
    completed: WorkItem[];
}