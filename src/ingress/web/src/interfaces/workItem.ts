import {WorkItemStatus} from "./workItemStatus";

export interface WorkItem {
    title: string;
    status: WorkItemStatus;
    size: number;
}